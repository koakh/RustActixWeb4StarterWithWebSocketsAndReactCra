use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ConfigItem {
  pub key: Rc<RefCell<Option<String>>>,
  pub label: Rc<RefCell<Option<String>>>,
  pub filter_file: Rc<RefCell<Option<String>>>,
  pub filter_line: Rc<RefCell<Option<String>>>,
  pub start_command: Rc<RefCell<Option<String>>>,
  pub stop_command: Rc<RefCell<Option<String>>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigState {
  pub filter_file: Rc<RefCell<Option<String>>>,
  pub filter_line: Rc<RefCell<Option<String>>>,
  pub input_files: Rc<RefCell<Option<Vec<PathBuf>>>>,
  pub configuration: Rc<RefCell<Vec<ConfigItem>>>,
}
