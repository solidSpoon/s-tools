use nom::{
    bytes::complete::{take_while, take_while1},
    character::complete::{char, multispace1},
    combinator::map_res,
    multi::separated_list1,
    sequence::terminated,
    IResult,
};
use chrono::NaiveTime;

/// The `Timestamp` struct represents a segment in a video file.
///
/// It contains the following fields:
/// - `index`: A unique identifier for each segment. It is a 32-bit unsigned integer.
/// - `start`: The start time of the segment. It is represented as a `NaiveTime` object.
/// - `end`: The end time of the segment. It is represented as an `Option<NaiveTime>` object. This field is optional and can be `None`. If it is `None`, it means that the segment extends to the start time of the next segment or to the end of the video if there is no next segment.
/// - `title`: The title of the segment. It is a `String`.
///
/// # Examples
///
/// ```
/// let timestamp = Timestamp {
///     index: 1,
///     start: NaiveTime::from_hms(0, 0, 0),
///     end: Some(NaiveTime::from_hms(0, 1, 55)),
///     title: "Intro".to_string(),
/// };
/// ```
#[derive(Debug, PartialEq)]
pub struct Timestamp {
    pub index: u32,
    pub start: NaiveTime,
    pub end: Option<NaiveTime>,
    pub title: String,
}

/// This function checks if a character is numeric.
///
/// # Arguments
///
/// * `c` - A character that needs to be checked.
///
/// # Returns
///
/// This function returns a boolean value. It returns `true` if the character is numeric, otherwise it returns `false`.
fn is_numeric(c: char) -> bool {
    c.is_numeric()
}

/// This function is a parser that parses a duration from a string.
///
/// The input string should be in the format "HH:MM:SS,mmm", where:
/// - HH represents the hours,
/// - MM represents the minutes,
/// - SS represents the seconds,
/// - mmm represents the milliseconds.
///
/// # Arguments
///
/// * `input` - A string slice that holds the duration in the format "HH:MM:SS,mmm".
///
/// # Returns
///
/// This function returns a `Result` which is an `Ok` if the parsing is successful. The `Ok` variant wraps a tuple containing:
/// - the remaining part of the input string after parsing,
/// - a `Duration` instance representing the parsed duration.
///
/// If the parsing is unsuccessful, it returns an `Err` variant wrapping a `nom::Err` instance.
///
/// # Errors
///
/// This function will return an error if the input string is not in the correct format or if the parsing of the hours, minutes, seconds or milliseconds fails.
fn duration(input: &str) -> IResult<&str, NaiveTime> {
    let (input, hours) = map_res(take_while(is_numeric), |s: &str| s.parse::<u64>())(input)?;
    let (input, _) = char(':')(input)?;
    let (input, minutes) = map_res(take_while(is_numeric), |s: &str| s.parse::<u64>())(input)?;
    let (input, _) = char(':')(input)?;
    let (input, seconds) = map_res(take_while(is_numeric), |s: &str| s.parse::<u64>())(input)?;
    let time = NaiveTime::from_hms_opt(hours as u32, minutes as u32, seconds as u32).unwrap();
    return Ok((input, time));
}
fn timestamp_item(input: &str) -> IResult<&str, Timestamp> {
    let (input, timestamp) = terminated(duration, multispace1)(input)?;
    let (input, title) = take_while1(|c: char| c != '\n')(input)?;
    Ok((
        input,
        Timestamp {
            index: 0,
            start: timestamp,
            end: None,
            title: title.to_string(),
        },
    ))
}

fn parse_timestamps(input: &str) -> IResult<&str, Vec<Timestamp>> {
    let input = input.trim();
    separated_list1(multispace1, timestamp_item)(input)
}

fn process_timestamps(timestamps: &mut Vec<Timestamp>)  {
    let start_times: Vec<_> = timestamps.iter().map(|t| t.start).collect();
    let len = timestamps.len();
    for (index, timestamp) in timestamps.iter_mut().enumerate() {
        timestamp.index = (index + 1) as u32;
        if index < len - 1 {
            timestamp.end = Some(start_times[index + 1]);
        }
    }
}

/// Parses a string of timestamps into a vector of `Timestamp` structs.
///
/// This function takes a string of timestamps as input and attempts to parse it into a vector of `Timestamp` structs.
/// Each timestamp in the input string should be in the format "HH:MM:SS,mmm" and separated by whitespace.
/// The function first calls `parse_timestamps` to parse the input string into a vector of `Timestamp` structs.
/// Then it calls `process_timestamps` to process the parsed timestamps, setting the `index` and `end` fields for each `Timestamp`.
///
/// # Arguments
///
/// * `input` - A string slice that holds the timestamps.
///
/// # Returns
///
/// This function returns a `Result` which is an `Ok` if the parsing and processing is successful. The `Ok` variant wraps a vector of `Timestamp` structs.
/// If the parsing or processing is unsuccessful, it returns an `Err` variant wrapping an `anyhow::Error` instance.
///
/// # Errors
///
/// This function will return an error if the input string is not in the correct format or if the parsing or processing of the timestamps fails.
pub fn parse(input: &str) -> anyhow::Result<Vec<Timestamp>> {
    let result = parse_timestamps(input);
    match result {
        Ok((_, timestamps)) => {
            let mut timestamps = timestamps;
            process_timestamps(&mut timestamps);
            Ok(timestamps)
        },
        Err(e) => Err(anyhow::anyhow!("Error parsing timestamps: {:?}", e)),
    }
}
