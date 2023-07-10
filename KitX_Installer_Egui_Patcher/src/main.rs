mod app_info;
mod patch_info;
mod patch_task;

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
        alignment: args.get::<bool>("align").unwrap_or(true),
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
        let mut new_file_buffer = Vec::new();
        let mut patched = false;
        let mut matched = false;
        let source = task.patch_source.clone();
        let mut target = task.patch_target.clone();

        if task.alignment {
            for _i in 0..(source.len() - target.len()) {
                target.push(" ".as_bytes()[0]);
            }
        }

        println!("Patching {} bytes to {} bytes:", source.len(), target.len());
        println!("  - Source: {:X?}", source);
        println!("  - Target: {:X?}", target);

        if task.alignment {
            println!(
                "  - Target is aligned to source, origin target length is {} bytes.",
                task.patch_target.len()
            );
        }

        for i in 0..buffer.len() {
            if buffer[i] == source[0] {
                let mut found = true;

                for j in 0..source.len() {
                    if buffer[i + j] != source[j] {
                        found = false;
                        break;
                    }
                }

                if found {
                    matched = found;

                    println!("  - Found at index: {}", i);

                    for j in 0..i {
                        new_file_buffer.push(buffer[j]);
                    }

                    for j in 0..target.len() {
                        new_file_buffer.push(target[j]);
                    }

                    for j in i + source.len()..buffer.len() {
                        new_file_buffer.push(buffer[j]);
                    }

                    println!("  - New file composed in RAM !");

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

        if !matched {
            println!("  - Pattern not found .");
        }

        if patched {
            println!("  - Patched !");
        } else {
            println!("  - Patching failed .");
        }
    }

    println!();
    println!("All patching tasks done !");
    println!("Enjoy :)");
}
