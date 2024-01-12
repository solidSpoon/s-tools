use nom::IResult;
use crate::parser::srt_parser::SubtitleItem;
use crate::parser::timestamp_parser::Timestamp;

mod timestamp_parser;
mod srt_parser;

pub fn parse_timestamp(input: &str) -> anyhow::Result<Vec<Timestamp>> {
    timestamp_parser::parse(input)
}

pub fn parse_srt(input: &str) -> anyhow::Result<Vec<SubtitleItem>> {
    srt_parser::parse(input)
}
