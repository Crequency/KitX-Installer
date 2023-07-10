mod app_info;
mod patch_info;
mod patch_task;
mod utils;

use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::io::Write;

use crate::app_info::AppInfo;
use crate::patch_info::PatchInfo;
use crate::patch_task::PatchTask;

// Default patch task:
// $$_!_%Version%_@_$$                                        #
// $$_!_%Profile%_@_$$                                        #

fn main() {
    let args = arguments::parse(env::args()).unwrap();

    let app_info = AppInfo::default();
    let mut patch_info = PatchInfo::default();

    println!("KitX Installer Patcher");
    println!("Version: v{}", app_info.version);
    println!();
    println!("Current directory: {}", app_info.current_directory);
    println!("Current exe: {}", app_info.current_exe);
    println!();

    patch_info.patch_file_path = args.get::<String>("patch");
    patch_info.patch_tasks.push(PatchTask {
        patch_source: args
            .get::<String>("from")
            .unwrap_or("".to_string())
            .into_bytes(),
        patch_target: args
            .get::<String>("to")
            .unwrap_or("".to_string())
            .into_bytes(),
    });

    println!("File to patch: {:?}", patch_info.patch_file_path);
    println!("Running default patch strategy:");
    println!();

    if patch_info.patch_file_path.is_none() {
        println!("Please provide path of file to patch !");
        return;
    }

    let file = File::open(patch_info.patch_file_path.clone().unwrap());
    let mut reader = BufReader::new(file.unwrap());
    let mut buffer = Vec::new();

    reader.read_to_end(&mut buffer).unwrap();

    println!("File loaded into RAM with size: {} bytes\n", buffer.len());

    for task in patch_info.patch_tasks {
        println!(
            "Patching {} bytes to {} bytes:",
            task.patch_source.len(),
            task.patch_target.len()
        );
        println!("  - Source: {:X?}", task.patch_source);
        println!("  - Target: {:X?}", task.patch_target);

        let mut new_file_buffer = Vec::new();
        let mut patched = false;

        for i in 0..buffer.len() {
            if buffer[i] == task.patch_source[0] {
                let mut found = true;

                for j in 0..task.patch_source.len() {
                    if buffer[i + j] != task.patch_source[j] {
                        found = false;
                        break;
                    }
                }

                if found {
                    println!("  - Found at index: {}", i);

                    for j in 0..i {
                        new_file_buffer.push(buffer[j]);
                    }

                    for j in 0..task.patch_target.len() {
                        new_file_buffer.push(task.patch_target[j]);
                    }

                    for j in i + task.patch_source.len()..buffer.len() {
                        new_file_buffer.push(buffer[j]);
                    }

                    println!("  - Patched !");

                    // println!("  - Origin: {:X?}", buffer);
                    // println!("  - Result: {:X?}", new_file_buffer);
                    // println!("  - Origin string: {}", String::from_utf8(buffer.clone()).unwrap());
                    // println!("  - Result string: {}", String::from_utf8(new_file_buffer.clone()).unwrap());

                    println!("  - Written in ...");

                    let file = File::create(patch_info.patch_file_path.clone().unwrap());
                    patched = file.unwrap().write_all(&new_file_buffer).is_ok();

                    break;
                }
            }
        }

        if !patched {
            println!("  - Not found ! Patching failed .");
        }
    }
}
