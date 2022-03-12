use regex::Regex;
use serde::Serialize;
use std::{
  cell::Cell,
  sync::{Arc, Mutex},
};

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AppState {
  // Mutex is necessary to mutate safely across threads
  pub server_id: usize,
  // worker, used for every thread/workers
  pub request_count: Cell<usize>,
}

#[derive(Debug)]
pub struct AppStateGlobal {
  // global, used for all workers
  pub counter: Mutex<i32>,
  pub filter_file: Arc<Mutex<String>>,
  pub filter_line: Arc<Mutex<String>>,
  pub regex_file: Arc<Mutex<Regex>>,
  pub regex_line: Arc<Mutex<Regex>>,
  pub config_file: Arc<Mutex<Option<String>>>
}
