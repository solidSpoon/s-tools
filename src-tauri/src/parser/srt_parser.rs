use nom::{
    bytes::complete::{take_while, take_while1},
    character::complete::{char, multispace1},
    combinator::map_res,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use chrono::NaiveTime;
use nom::bytes::complete::tag;

/// `SubtitleItem` is a structure that represents a subtitle item in an SRT file.
///
/// # Fields
///
/// * `id` - A unique identifier for the subtitle item.
/// * `start` - The start time of the subtitle item as a `Duration`.
/// * `end` - The end time of the subtitle item as a `Duration`.
/// * `content` - The actual content of the subtitle item as a vector of strings.
///
/// # Example
///
/// ```
/// let subtitle_item = SubtitleItem {
///     id: 1,
///     start: Duration::from_secs(0),
///     end: Duration::from_secs(2),
///     content: vec!["Hello, world!".to_string(), "你好世界！".to_string()],
/// };
/// ```
#[derive(Debug, PartialEq)]
pub struct SubtitleItem {
    pub id: u32,
    pub start: NaiveTime,
    pub end: NaiveTime,
    pub content: Vec<String>,
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
    let (input, _) = char(',')(input)?;
    let (input, millis) = map_res(take_while(is_numeric), |s: &str| s.parse::<u64>())(input)?;
    let time = NaiveTime::from_hms_milli_opt(hours as u32, minutes as u32, seconds as u32, millis as u32).unwrap();
    Ok((input, time))
}
fn subtitle_item(input: &str) -> IResult<&str, SubtitleItem> {
    let (input, id) = map_res(take_while(is_numeric), |s: &str| s.parse::<u32>())(input)?;
    let (input, _) = multispace1(input)?;
    let (input, (start, end)) = separated_pair(duration, tag(" --> "), duration)(input)?;
    let (input, _) = multispace1(input)?;
    let (input, content) = separated_list1(char('\n'), take_while1(|c: char| c != '\n'))(input)?;
    Ok((
        input,
        SubtitleItem {
            id,
            start,
            end,
            content: content.into_iter()
                .map(|s| s.trim())
                .map(String::from)
                .collect(),
        },
    ))
}

fn parse_srt(input: &str) -> IResult<&str, Vec<SubtitleItem>> {
    let input = input.trim();
    separated_list1(multispace1, subtitle_item)(input)
}

pub fn parse(input: &str) -> anyhow::Result<Vec<SubtitleItem>> {
    let result = parse_srt(input);
    match result {
        Ok((_, timestamps)) => Ok(timestamps),
        Err(e) => Err(anyhow::anyhow!("Error parsing srt: {:?}", e)),
    }
}
