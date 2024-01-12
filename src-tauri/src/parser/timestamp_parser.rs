use nom::{
    bytes::complete::{take_while, take_while1},
    character::complete::{char, multispace1},
    combinator::map_res,
    multi::separated_list1,
    sequence::terminated,
    IResult,
};
use std::time::Duration;

#[derive(Debug, PartialEq)]
pub struct Timestamp {
    pub start: Duration,
    pub end: Option<Duration>,
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
fn duration(input: &str) -> IResult<&str, Duration> {
    let (input, hours) = map_res(take_while(is_numeric), |s: &str| s.parse::<u64>())(input)?;
    let (input, _) = char(':')(input)?;
    let (input, minutes) = map_res(take_while(is_numeric), |s: &str| s.parse::<u64>())(input)?;
    let (input, _) = char(':')(input)?;
    let (input, seconds) = map_res(take_while(is_numeric), |s: &str| s.parse::<u64>())(input)?;
    Ok((input, Duration::from_secs(hours * 60 * 60 + minutes * 60 + seconds)))
}
fn timestamp_item(input: &str) -> IResult<&str, Timestamp> {
    let (input, timestamp) = terminated(duration, multispace1)(input)?;
    let (input, title) = take_while1(|c: char| c != '\n')(input)?;
    Ok((
        input,
        Timestamp {
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

fn process_timestamps(input: &str) -> Vec<Timestamp> {
    let mut timestamps = parse_timestamps(input).unwrap().1;
    let mut prev_timestamp = None;
    for timestamp in timestamps.iter_mut() {
        if let Some(prev_timestamp) = prev_timestamp {
            prev_timestamp.end = Some(timestamp.start);
        }
        prev_timestamp = Some(timestamp);
    }
    timestamps
}

pub fn parse(input: &str) -> anyhow::Result<Vec<Timestamp>> {
    let result = parse_timestamps(input);
    match result {
        Ok(timestamps) => Ok(timestamps),
        Err(e) => Err(anyhow::anyhow!("parse timestamp failed! {}", e)),
    }
}
