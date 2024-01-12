use anyhow::{Context, Result};
use std::path::PathBuf;
use crate::parser::{FileHolder, FileType, parse_path, parse_timestamp, Timestamp};
use crate::util::escape_special_chars;

pub fn split_file(file_names: Vec<String>, time_stamp: String) -> Result<()> {
    let timestamps: Vec<Timestamp> = parse_timestamp(&time_stamp).context("parse timestamp failed!")?;
    let file_holders = file_names.iter().map(|file_name| parse_path(file_name)).collect::<Vec<_>>();
    for file_holder in file_holders {
        split_one_file(&file_holder, &timestamps)?;
    }
    Ok(())
}

fn split_one_file(file_holder: &FileHolder, time_stamps: &Vec<Timestamp>) -> Result<()> {
    for time_stamp in time_stamps {
        split_video(file_holder, time_stamp)?;
    }
    Ok(())
}

fn split_video(file_holder: &FileHolder, time_stamp: &Timestamp) -> Result<()> {
    if file_holder.file_type != FileType::Video {
        return Ok(());
    }
    // 在视频文件所在目录创建同名文件夹
    let bath_path = PathBuf::from(file_holder.directory.clone());
    let output_folder = bath_path.join(&file_holder.name.replace(".", "_"));
    std::fs::create_dir_all(&output_folder)?;

    let output_filename = format!(
        "{}/{}_{}_{}.mp4",
        output_folder.to_str().unwrap(),
        time_stamp.index,
        time_stamp.start.format("%H-%M-%S"),
        escape_special_chars(time_stamp.title.as_ref())
    );
    println!("Splitting {} to {}", file_holder.full_path, output_filename);

    let cmd = match time_stamp.end {
        Some(end_time) => format!(
            "/opt/homebrew/bin/ffmpeg -y -ss {} -t {} -accurate_seek -i {} -codec copy  -avoid_negative_ts 1 {}",
            time_stamp.start.format("%H:%M:%S"),
            end_time.signed_duration_since(time_stamp.start).num_seconds(),
            file_holder.full_path,
            output_filename
        ),
        None => format!(
            "/opt/homebrew/bin/ffmpeg -y -ss {} -accurate_seek -i {} -codec copy  -avoid_negative_ts 1 {}",
            time_stamp.start.format("%H:%M:%S"),
            file_holder.full_path,
            output_filename
        ),
    };

    // info!("cmd: {}", cmd);
    let cmd_clone = cmd.clone();
    std::env::set_var("RUST_BACKTRACE", "full");
    let output = std::process::Command::new("sh")
        // .env("PATH", "/opt/homebrew/bin")
        .arg("-c")
        .arg(&cmd_clone)
        // .output().unwrap();
        .output()?;

    // info!("output: {:?}", output);
    if !output.status.success() {
        return Err(anyhow::anyhow!("split video failed! {}", String::from_utf8(output.stderr).unwrap()));
    }
    Ok(())
}