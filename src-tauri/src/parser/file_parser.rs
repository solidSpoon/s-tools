use std::path::Path;
use std::ffi::OsStr;

/// `FileType` is an enumeration that represents the type of a file.
/// It can be one of the following:
/// - `Video`: Represents video files
/// - `Subtitle`: Represents subtitle files
/// - `None`: Represents any other type of file
#[derive(Debug, PartialEq)]
pub enum FileType {
    Video,
    Subtitle,
    None,
}

/// `FileHolder` is a structure that holds information about a file.
/// It contains the following fields:
/// - `full_path`: A string representing the full path of the file.
/// - `name`: A string representing the name of the file.
/// - `directory`: A string representing the directory path of the file.
/// - `file_type`: A `FileType` enumeration representing the type of the file.
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
/// Parses a given file path string and returns a `FileHolder` struct.
///
/// This function takes a string representing a file path, and parses it to create a `FileHolder` struct.
/// The `FileHolder` struct contains the full file path, the file name, the directory path, and the file type.
///
/// The file type is determined by the file extension. The following extensions are recognized:
/// - "mp4", "avi", "mkv" are recognized as `FileType::Video`
/// - "srt", "ass", "vtt" are recognized as `FileType::Subtitle`
/// - Any other extension is recognized as `FileType::None`
///
/// # Arguments
///
/// * `path_str` - A string slice that holds the file path to be parsed.
///
/// # Returns
///
/// Returns a `FileHolder` struct containing the parsed file information.
///
/// # Examples
///
/// ```
/// let file_holder = parse("/path/to/file.mp4");
/// assert_eq!(file_holder.file_type, FileType::Video);
/// ```
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
