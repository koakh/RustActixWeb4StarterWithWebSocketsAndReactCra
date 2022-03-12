#![allow(unused_imports)]
use crate::app::{config::ConfigItem, ConfigState, LOG_HEADER_LINE_CHAR, LOG_HEADER_LINE_LEN};
use chrono::{DateTime, Utc};
use log::{debug, error};
use regex::Regex;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::json;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::MutexGuard;
use rand::Rng;

/// generate a fixed size char line
pub fn gen_line_char(character: char, width: u8) -> String {
  let mut buf = String::new();
  for _ in 0..width {
    buf.push(character);
  }
  buf
}

/// simple helper, just to prevent extra code lines
pub fn out_message(message: String, indent: u8) {
  debug!("{}", &message);
  println!("{}{}", gen_line_char(' ', indent), &message);
}

/// check if file exists
pub fn file_exists(file_path: &str) -> bool {
  let package_exist = Path::new(&file_path).exists();
  package_exist
}

/// check if path/dir exists
pub fn _path_exists(path: &str) -> bool {
  Path::new(&path).exists()
}

pub fn _log_header(content: &str) {
  let line = gen_line_char(*LOG_HEADER_LINE_CHAR, *LOG_HEADER_LINE_LEN);
  // let indent = gen_line_char(' ', 30);
  debug!("{}", line);
  debug!("{}", content);
  debug!("{}", line);
  println!("{}", content);
}

/// strip trailing newline *nix and windows
pub fn strip_trailing_newline(input: &str) -> &str {
  input.strip_suffix("\r\n").or(input.strip_suffix("\n")).unwrap_or(&input)
}

/// get current formatted date
// https://rust-lang-nursery.github.io/rust-cookbook/datetime/parse.html#display-formatted-date-and-time
pub fn get_current_formatted_date(format: &str) -> String {
  let now: DateTime<Utc> = Utc::now();
  now.format(format).to_string()
}

/// This function reads a JSON file from disk.
///
/// # Arguments
/// * file_path (String): the path to the file being read
///
/// # Returns
/// (serde_json::state_model::State): the values from the JSON file
pub fn read_generic_type<T: DeserializeOwned>(file_path: &str) -> Result<T, String> {
  if !file_exists(file_path) {
    return Err(format!("state file '{}' not found!", file_path));
  }
  let mut file = File::open(file_path).unwrap();
  let mut data = String::new();
  file.read_to_string(&mut data).unwrap();
  // let json: Value = serde_json::from_str(&data).unwrap();
  // let state: Map<String, Value> = json.as_object().unwrap().clone();
  // let state: State = serde_json::from_str(&data).expect("invalid json file!");
  match serde_json::from_str::<T>(&data) {
    Ok(data) => Ok(data),
    Err(e) => Err(format!("invalid json file: {}", e.to_string())),
  }
}

/// This function writes a JSON map to file on disk.
/// update bow use generics
///
/// # Arguments
/// * file_path (String): the path to the file being read
/// * state (state_model::State): the data being written to disk
///
/// # Returns
/// None
pub fn _write_generic_type<T: Serialize>(file_path: String, data: &T) -> Result<(), String> {
  let new_data = json!(data);
  match fs::write(file_path, new_data.to_string()) {
    Ok(_) => Ok(()),
    Err(e) => Err(format!("error saving file: {}", e.to_string())),
  }
}

// reverse use PathBuf::from
pub fn pathbuf_to_str(path_buf: &PathBuf) -> String {
  path_buf.display().to_string()
}

pub fn read_config(file_path: &str) -> Result<ConfigState, String> {
  match read_generic_type::<ConfigState>(file_path) {
    Ok(loaded_config) => {
      // out_message(format!("config: {:?}", config), 0);
      Ok(loaded_config)
    }
    Err(e) => {
      // error!("error trying to read file '{}' {:?}", CONFIG_FILE_PATH, e);
      Err(e)
    }
  }
}

/// get config state from config_file_mutex_guard
pub fn get_config_state(config_file_mutex_guard: MutexGuard<Option<String>>) -> Result<ConfigState, String> {
  match config_file_mutex_guard.as_ref() {
    Some(current_config_file) => match read_config(current_config_file) {
      Ok(c) => Ok(c),
      Err(e) => Err(format!("can't read config file: {}, error: {}", current_config_file, e.to_string())),
    },
    None => Err(format!("can't read config file")),
  }
}

/// get config item from config state and find key
pub fn get_config_item(config: &ConfigState, find_key: String) -> Option<ConfigItem> {
  // let input_files = config.input_files.borrow().as_ref().unwrap().clone();
  let configuration = config.configuration.borrow().clone();
  // let configuration: Vec<ConfigItem> = config.configuration.unwrap();
  for value in configuration {
    let key = value.key.borrow().as_ref().unwrap().clone().to_string();
    if key.eq(&find_key) {
      return Some(value);
    }    
  }
  None
}

/// get config item from config state and find key
pub fn get_config_files_from_regex(config: &ConfigState, regex: Regex) -> Option<String> {
// let match_file_re = ref_regex_file.lock().unwrap().is_match(&line.source().display().to_string());
  let mut result = String::from("");
  let input_files = config.input_files.borrow().as_ref().unwrap().clone();
  for file in input_files {
    let match_file_re = regex.is_match(pathbuf_to_str(&file).as_str());
    let exist = file_exists(pathbuf_to_str(&file).as_str());
    if match_file_re && exist {
      result += format!(" {}", pathbuf_to_str(&file)).as_str();
      debug!("match file: {}, result: {}", pathbuf_to_str(&file), result);
    }
  }
  if result.eq("") {
    None
  } else {
    Some(result.trim_start().to_string())
  }
}

// https://rust-lang-nursery.github.io/rust-cookbook/algorithms/randomness.html
// Create random passwords from a set of user-defined characters
pub fn generate_random_string(seed: &[u8], size: usize) -> String {
  let mut rng = rand::thread_rng();

  let password: String = (0..size)
      .map(|_| {
          let idx = rng.gen_range(0..seed.len());
          seed[idx] as char
      })
      .collect();

  password
}