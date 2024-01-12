use crate::parser::srt_parser::SubtitleItem;
pub use timestamp_parser::Timestamp;
pub use file_parser::FileHolder;
pub use file_parser::FileType;
mod timestamp_parser;
mod srt_parser;
mod file_parser;

pub use timestamp_parser::parse as parse_timestamp;
pub use srt_parser::parse as parse_srt;
pub use file_parser::parse as parse_path;