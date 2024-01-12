use crate::parser::srt_parser::SubtitleItem;
pub use timestamp_parser::Timestamp;
pub use file_parser::FileHolder;
pub use file_parser::FileType;
mod timestamp_parser;
mod srt_parser;
mod file_parser;

pub fn parse_timestamp(input: &str) -> anyhow::Result<Vec<Timestamp>> {
    timestamp_parser::parse(input)
}

pub fn parse_srt(input: &str) -> anyhow::Result<Vec<SubtitleItem>> {
    srt_parser::parse(input)
}

pub fn parse_path(path_str: &str) -> FileHolder {
    file_parser::parse(path_str)
}