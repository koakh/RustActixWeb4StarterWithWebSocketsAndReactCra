use serde::{Serialize};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BackupLogResponse {
  pub file_name: String,
  pub file_path: String,
  pub file_url: String,
}
