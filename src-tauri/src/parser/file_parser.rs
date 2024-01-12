use std::path::Path;
use std::ffi::OsStr;

#[derive(Debug, PartialEq)]
pub enum FileType {
    Video,
    Subtitle,
    None,
}

#[derive(Debug, PartialEq)]
pub struct FileHolder {
    /// Full file path
    pub full_path: String,

    /// File name
    pub name: String,

    /// Directory path
    pub directory: String,

    /// File type
    pub file_type: FileType,
}

pub fn parse(path_str: &str) -> FileHolder {
    let path = Path::new(path_str);
    let file_name = path.file_name().and_then(OsStr::to_str).unwrap_or("");
    let directory = path.parent().and_then(|p| p.to_str()).unwrap_or("");
    let extension = path.extension().and_then(OsStr::to_str).unwrap_or("");

    let file_type = match extension {
        "mp4" | "avi" | "mkv" => FileType::Video,
        "srt" | "ass" | "vtt" => FileType::Subtitle,
        _ => FileType::None,
    };

    FileHolder {
        full_path: String::from(path_str),
        name: String::from(file_name),
        directory: String::from(directory),
        file_type,
    }
}
