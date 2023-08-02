use std::{
    io::{BufRead, BufReader},
    process::{Command, Stdio},
};

pub fn decompress_7z_with_7z<RP: FnMut(f32) -> f32, RD: Fn(&str)>(
    v7z_file: String,
    working_dir: String,
    zip_file_name: String,
    _report_progress: &mut RP,
    report_detail: RD,
) {
    let child = Command::new(v7z_file)
        .arg("x") // Extract files with full paths
        .arg("-y") // Assume Yes on all queries
        .arg(zip_file_name.as_str()) // Specify archive filename
        .arg("-bb3") // Set output log level
        .stdout(Stdio::piped()) // Capture stdout
        .current_dir(working_dir) // Set working directory
        .spawn();

    if child.is_err() {
        report_detail("Failed to spawn 7z process.");
        return;
    }

    let child = child.unwrap();

    if child.stdout.is_none() {
        report_detail("Failed to get stdout.");
        return;
    }

    let stdout = child.stdout.unwrap();
    let reader = BufReader::new(stdout);

    reader.lines().for_each(|line| {
        let line = line.unwrap_or("Failed to read from 7z process stdout.".to_string());
        if line.starts_with("- ") {
            let content = line[2..].to_string();
            report_detail(format!("├ Extracting `{}`", content).as_str());
        }
    });
}
