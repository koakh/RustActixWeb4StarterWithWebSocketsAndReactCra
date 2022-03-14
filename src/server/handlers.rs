use actix::Addr;
use actix_multipart::Multipart;
use actix_web::{
  get,
  http::{header, StatusCode},
  post, web, Error as ActixError, HttpRequest, HttpResponse, Result,Responder
};
use serde_json::json;
use log::{debug, error};
use regex::Regex;

use crate::{
  app::{ENDPOINT_REDIRECT_TO, RANDOM_STRING_GENERATOR_CHARSET, RANDOM_STRING_GENERATOR_SIZE, AppStateGlobal, AppState, config::ConfigItem, FORMAT_DATE_TIME_FILE_NAME, DOWNLOAD_FILES_PATH, DOWNLOAD_URI_PATH_ABSOLUTE},
  enums::MessageToClientType,
  responses::{ApiKeyResponse, MessageResponse, AppStateResponse, PostStateResponse, ErrorMessageResponse, GetStateResponse, BackupLogResponse, PostWsEchoResponse},
  requests::{PostStateRequest, PostWsEchoRequest, PostbackupLogRequest},
  server::save_file,
  util::{generate_random_string, get_config_state, get_config_item, get_config_files_from_regex, execute_command_shortcut, get_current_formatted_date, execute_command, ExecuteCommandOutcome},
  websocket::{MessageToClient, Server as WebServer},
};

/// GET:/ping
#[get("/ping")]
pub async fn health_check(_: HttpRequest) -> Result<web::Json<MessageResponse>> {
  Ok(web::Json(MessageResponse { message: "pong".to_string() }))
}

/// GET:/secret-api-key
#[get("/secret-api-key")]
async fn _api_key(_: HttpRequest) -> Result<web::Json<ApiKeyResponse>> {
  Ok(web::Json(ApiKeyResponse {
    api_key: generate_random_string(RANDOM_STRING_GENERATOR_CHARSET, RANDOM_STRING_GENERATOR_SIZE).to_string(),
  }))
}

#[get("/redirect")]
pub async fn redirect() -> HttpResponse {
  HttpResponse::Found()
    // optional
    .append_header(header::ContentType(mime::TEXT_HTML))
    .append_header(("location", ENDPOINT_REDIRECT_TO))
    .finish()
}

pub async fn not_found() -> Result<HttpResponse, ActixError> {
  Ok(HttpResponse::build(StatusCode::NOT_FOUND).json(MessageResponse { message: String::from("not found") }))
}

#[post("/upload")]
pub async fn upload(payload: Multipart) -> Result<HttpResponse, ActixError> {
  let upload_status = save_file(payload).await;

  match upload_status {
    Ok(_) => Ok(HttpResponse::build(StatusCode::OK).json(MessageResponse {
      message: "successful upload".to_string(),
    })),
    Err(e) => Ok(HttpResponse::build(StatusCode::NOT_FOUND).json(MessageResponse { message: format!("failed upload: {}", e) })),
  }
}

/// POST:/ws-echo
#[post("/ws-echo")]
pub async fn _ws_echo(msg: web::Json<PostWsEchoRequest>, websocket_srv: web::Data<Addr<WebServer>>) -> HttpResponse {
  // The type of `j` is `serde_json::Value`
  let json = json!({ "fingerprint": "0xF9BA143B95FF6D82", "message": msg.message });
  // let wsm: WebSocketMessage = serde_json::from_value(json).unwrap();
  let msg_type = &format!("{}", MessageToClientType::Echo)[..];
  let message_to_client = MessageToClient::new(msg_type, json);
  // let message_to_client = MessageToClient::new("echo", json);
  // websocket_srv.do_send(message_to_client);
  match websocket_srv.send(message_to_client).await {
    Ok(ok) => debug!("{:?}", ok),
    Err(e) => error!("{:?}", e),
  };
  HttpResponse::Ok().json(PostWsEchoResponse { message: msg.message.clone() })
}

/// GET:/ | GET:/name
pub async fn _greet(req: HttpRequest) -> impl Responder {
  let name = req.match_info().get("name").unwrap_or("World");
  format!("Hello {}!", &name)
}

/// POST:/state : same as /filter but change filters and respond with full state
#[post("/state-full")]
pub async fn post_state_full(msg: web::Json<PostStateRequest>, data: web::Data<AppState>, app_data: web::Data<AppStateGlobal>) -> Result<web::Json<AppStateResponse>> {
  // global get counter's MutexGuard
  let mut counter = app_data.counter.lock().unwrap();
  // access counter inside MutexGuard
  *counter += 1;

  if !msg.filter_file.eq("") {
    let mut filter_file = app_data.filter_file.lock().unwrap();
    let mut regex_file = app_data.regex_file.lock().unwrap();
    // access filter inside MutexGuard
    *filter_file = msg.filter_file.clone();
    *regex_file = Regex::new(msg.filter_file.clone().as_str()).unwrap();
  }

  if !msg.filter_line.eq("") {
    let mut filter_line = app_data.filter_line.lock().unwrap();
    let mut regex_line = app_data.regex_line.lock().unwrap();
    // access filter inside MutexGuard
    *filter_line = msg.filter_line.clone();
    *regex_line = Regex::new(msg.filter_line.clone().as_str()).unwrap();
  }

  // workers state
  let request_count = data.request_count.get() + 1;
  data.request_count.set(request_count);

  debug!("{:?}", msg);
  // HttpResponse::Ok().json(PostFilterResponse {
  //   message: String::from(request_count.to_string()),
  // })
  Ok(web::Json(AppStateResponse {
    server_id: data.server_id,
    request_count,
    counter: *counter,
    filter_file: String::from(&msg.filter_file),
    filter_line: String::from(&msg.filter_line),
  }))
}

/// POST:/filter
#[post("/state")]
pub async fn post_state(msg: web::Json<PostStateRequest>, data: web::Data<AppState>, app_data: web::Data<AppStateGlobal>) -> impl Responder /*Result<web::Json<PostFilterResponse>>*/ {
  // global get counter's MutexGuard
  let mut counter = app_data.counter.lock().unwrap();
  // access counter inside MutexGuard
  *counter += 1;

  if !msg.filter_file.eq("") {
    let mut filter_file = app_data.filter_file.lock().unwrap();
    let mut regex_file = app_data.regex_file.lock().unwrap();
    // access filter inside MutexGuard
    *filter_file = msg.filter_file.clone();
    match Regex::new(msg.filter_file.clone().as_str()) {
      Ok(r) => *regex_file = r,
      Err(e) => return HttpResponse::InternalServerError().json(MessageResponse { message: format!("{}", e) }),
    }
  }

  if !msg.filter_line.eq("") {
    let mut filter_line = app_data.filter_line.lock().unwrap();
    let mut regex_line = app_data.regex_line.lock().unwrap();
    // access filter inside MutexGuard
    *filter_line = msg.filter_line.clone();
    match Regex::new(msg.filter_line.clone().as_str()) {
      Ok(r) => *regex_line = r,
      Err(e) => return HttpResponse::InternalServerError().json(ErrorMessageResponse { message: format!("{}", e) }),
    }
  }

  // workers state
  let request_count = data.request_count.get() + 1;
  data.request_count.set(request_count);

  // output changed filters: leave stdout clean for output filtered log lines only
  // out_message(format!("filters changed file: {}, line: {}", &msg.filter_file, &msg.filter_line), 0);

  HttpResponse::Ok().json(PostStateResponse {
    filter_file: String::from(&msg.filter_file),
    filter_line: String::from(&msg.filter_line),
  })
}

/// GET:/state
#[get("/state")]
pub async fn get_state(app_data: web::Data<AppStateGlobal>) -> Result<web::Json<GetStateResponse>> {
  // extract config_file from mutexGuard
  let current_config_file_mutex_guard = app_data.config_file.lock().unwrap();
  let mut config_file = "";
  if let Some(c) = current_config_file_mutex_guard.as_ref() {
    config_file = c;
  }

  Ok(web::Json(GetStateResponse {
    filter_file: app_data.filter_file.lock().unwrap().to_string(),
    filter_line: app_data.filter_line.lock().unwrap().to_string(),
    config_file: config_file.to_string(),
  }))
}

#[get("/config")]
pub async fn get_config(app_data: web::Data<AppStateGlobal>) -> impl Responder {
  let current_config_file_mutex_guard = app_data.config_file.lock().unwrap();
  match get_config_state(current_config_file_mutex_guard) {
    Ok(c) => HttpResponse::Ok().json(c),
    Err(e) => HttpResponse::InternalServerError().json(ErrorMessageResponse { message: format!("{:?}", e) }),
  }
}

#[post("/backup-log")]
pub async fn post_backup_log(msg: web::Json<PostbackupLogRequest>, app_data: web::Data<AppStateGlobal>) -> impl Responder {
  let current_config_file_mutex_guard = app_data.config_file.lock().unwrap();
  // read config state
  let config_state;
  match get_config_state(current_config_file_mutex_guard) {
    Ok(c) => config_state = c,
    Err(e) => return HttpResponse::InternalServerError().json(ErrorMessageResponse { message: format!("{:?}", e) }),
  };
  // get config item from config state
  let config_item;
  match get_config_item(&config_state, msg.key.clone()) {
    Some(c) => config_item = c,
    None => {
      return HttpResponse::InternalServerError().json(ErrorMessageResponse {
        message: format!("can't get config item key '{}' from config state", msg.key.clone()),
      })
    }
  };
  let key = config_item.key.borrow().as_ref().unwrap().clone();
  let filter_file = config_item.filter_file.borrow().as_ref().unwrap().clone();
  let filter_file_re = Regex::new(filter_file.as_str()).unwrap();
  let files;
  match get_config_files_from_regex(&config_state, filter_file_re) {
    Some(c) => files = c,
    None => {
      return HttpResponse::InternalServerError().json(ErrorMessageResponse {
        message: format!("can't get config files from from config state with item key '{}'", msg.key.clone()),
      })
    }
  };

  // TODO: add to notes
  // stop command closure
  let stop_command = |config_item: &ConfigItem| {
    if config_item.stop_command.borrow().as_ref().is_some() {
      let command = config_item.stop_command.borrow().as_ref().unwrap().clone();
      match execute_command_shortcut(&command) {
        Ok(_) => {}
        // TODO: send back HTTP RESPONSE
        Err(err) => debug!("{:?}", err),
      };
    };
  };
  // start command closure
  let start_command = |config_item: &ConfigItem| {
    if config_item.start_command.borrow().as_ref().is_some() {
      let command = config_item.start_command.borrow().as_ref().unwrap().clone();
      match execute_command_shortcut(&command) {
        Ok(_) => {}
        // TODO: send back HTTP RESPONSE
        Err(err) => debug!("{:?}", err),
      };
    };
  };

  // stop command
  if key.eq("all") {
    for config_item_vec in config_state.configuration.borrow().to_vec() {
      stop_command(&config_item_vec);
    }
  } else {
    stop_command(&config_item);
  };

  // let key = config_item.key.lock().unwrap();
  let date = get_current_formatted_date(FORMAT_DATE_TIME_FILE_NAME);
  let file_name = format!("logs_{}_{}.tgz", key, date);
  let file_path = format!("{}/{}", DOWNLOAD_FILES_PATH, file_name);
  let file_url = format!("{}/{}", DOWNLOAD_URI_PATH_ABSOLUTE, file_name);
  let command = format!("tar -zcf {} --ignore-failed-read --absolute-names {}", file_path, files);
  let command_args = &[String::from("-c"), String::from(command)];
  // debug!("{:?}", command_args);
  let command_outcome: ExecuteCommandOutcome = execute_command(command_args, false);

  // start command
  if key.eq("all") {
    for config_item_vec in config_state.configuration.borrow().to_vec() {
      start_command(&config_item_vec);
    }
  } else {
    start_command(&config_item);
  };

  if command_outcome.error_code != 0 {
    error!("error_code: {}, stderr: {}", command_outcome.error_code, command_outcome.stderr_string);
    HttpResponse::InternalServerError().json(ErrorMessageResponse {
      message: format!("{:?}", command_outcome.stderr_string),
    })
  } else {
    HttpResponse::Ok().json(BackupLogResponse { file_name, file_path, file_url })
  }
}