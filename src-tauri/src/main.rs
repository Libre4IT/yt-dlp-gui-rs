// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::{Command, ExitStatus};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(url: &str, musiconly: &str) -> String {

    let status = download(url, musiconly);
    if status.success() {
        format!("Download complete!")
    } else {
        format!("Unable to download. Is this web address valid?")
    }
}

/// download 
/// TODO: change target folder for download: now downloads to src-tauri/
/// TODO: refactor duplicate code 
fn download(url: &str, musiconly: &str) -> ExitStatus {
    // yt-dlp download 
    if musiconly == "y" {
        if url.contains("playlist"){
            let mut the_process = Command::new("yt-dlp")
            .arg("--extract-audio")
            .arg("--audio-format")
            .arg("mp3")
            .arg("--output")
            .arg("%(title)s.%(ext)s")
            .arg("--yes-playlist")
            .arg(url) // the URL given
            .spawn()
            .expect("yt-dlp command failed to start");
            // Wait for the process.
            let the_status = the_process.wait()
                .ok().expect("Couldn't wait for process.");
            // Output some exit information.
            // println!("process finished with: {the_status}");
            the_status
        } else {
            let mut the_process = Command::new("yt-dlp")
            .arg("--extract-audio")
            .arg("--audio-format")
            .arg("mp3")
            .arg("--output")
            .arg("%(title)s.%(ext)s")
            .arg(url) // the URL given
            .spawn()
            .expect("yt-dlp command failed to start");
            // Wait for the process.
            let the_status = the_process.wait()
                .ok().expect("Couldn't wait for process.");
            // Output some exit information.
            // println!("process finished with: {the_status}");
            the_status   
        }
    }
    else {
        if url.contains("playlist") {
            let mut the_process = Command::new("yt-dlp")
            .arg("-i")
            .arg("-f")
            .arg("mp4")
            .arg("--output")
            .arg("%(title)s.%(ext)s")
            .arg("--yes-playlist")
            .arg(url) // the URL given
            .spawn()
            .expect("yt-dlp command failed to start");
            // Wait for the process.
            let the_status = the_process.wait()
                .ok().expect("Couldn't wait for process.");
            // Output some exit information.
            // println!("process finished with: {the_status}");
            the_status
        } else {
            let mut the_process = Command::new("yt-dlp")
            .arg("-i")
            .arg("-f")
            .arg("mp4")
            .arg("--output")
            .arg("%(title)s.%(ext)s")
            .arg(url) // the URL given
            .spawn()
            .expect("yt-dlp command failed to start");
            // Wait for the process.
            let the_status = the_process.wait()
                .ok().expect("Couldn't wait for process.");
            // Output some exit information.
            // println!("process finished with: {the_status}");
            the_status
        }
    }
}


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
