use actix::Addr;
use actix_multipart::Multipart;
use actix_web::{
  get,
  http::{header, StatusCode},
  post, web, Error as ActixError, HttpRequest, HttpResponse, Responder, Result,
};
use awc::Client;
use log::{debug, error};
#[allow(unused_imports)]
use regex::Regex;
use serde_json::json;

use crate::{
  app::{AppState, AppStateGlobal, ENDPOINT_REDIRECT_TO, RANDOM_STRING_GENERATOR_CHARSET, RANDOM_STRING_GENERATOR_SIZE},
  enums::MessageToClientType,
  requests::{PostAwcUriRequest, PostStateRequest, PostWsEchoRequest},
  responses::{ApiKeyResponse, AppStateResponse, ErrorMessageResponse, GetStateResponse, MessageResponse, PostStateResponse, PostWsEchoResponse},
  server::save_file,
  util::{generate_random_string, get_config_state},
  websocket::{MessageToClient, Server as WebServer},
};

pub async fn not_found() -> Result<HttpResponse, ActixError> {
  Ok(HttpResponse::build(StatusCode::NOT_FOUND).json(MessageResponse { message: String::from("not found") }))
}

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

/// GET:/redirect
#[get("/redirect")]
pub async fn redirect() -> HttpResponse {
  HttpResponse::Found()
    // optional
    .append_header(header::ContentType(mime::TEXT_HTML))
    .append_header(("location", ENDPOINT_REDIRECT_TO))
    .finish()
}

/// POST:/ws-echo
#[post("/ws-echo")]
pub async fn ws_echo(msg: web::Json<PostWsEchoRequest>, websocket_srv: web::Data<Addr<WebServer>>) -> HttpResponse {
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

  // BOF : UNCOMMENT to use config
  // if !msg.filter_file.eq("") {
  //   let mut filter_file = app_data.filter_file.lock().unwrap();
  //   let mut regex_file = app_data.regex_file.lock().unwrap();
  //   // access filter inside MutexGuard
  //   *filter_file = msg.filter_file.clone();
  //   *regex_file = Regex::new(msg.filter_file.clone().as_str()).unwrap();
  // }
  // EOF : UNCOMMENT to use config

  // BOF : UNCOMMENT to use config
  // if !msg.filter_line.eq("") {
  //   let mut filter_line = app_data.filter_line.lock().unwrap();
  //   let mut regex_line = app_data.regex_line.lock().unwrap();
  //   // access filter inside MutexGuard
  //   *filter_line = msg.filter_line.clone();
  //   *regex_line = Regex::new(msg.filter_line.clone().as_str()).unwrap();
  // }
  // EOF : UNCOMMENT to use config

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

  // BOF : UNCOMMENT to use config
  // if !msg.filter_file.eq("") {
  //   let mut filter_file = app_data.filter_file.lock().unwrap();
  //   let mut regex_file = app_data.regex_file.lock().unwrap();
  //   // access filter inside MutexGuard
  //   *filter_file = msg.filter_file.clone();
  //   match Regex::new(msg.filter_file.clone().as_str()) {
  //     Ok(r) => *regex_file = r,
  //     Err(e) => return HttpResponse::InternalServerError().json(MessageResponse { message: format!("{}", e) }),
  //   }
  // }
  // EOF : UNCOMMENT to use config

  // BOF : UNCOMMENT to use config
  // if !msg.filter_line.eq("") {
  //   let mut filter_line = app_data.filter_line.lock().unwrap();
  //   let mut regex_line = app_data.regex_line.lock().unwrap();
  //   // access filter inside MutexGuard
  //   *filter_line = msg.filter_line.clone();
  //   match Regex::new(msg.filter_line.clone().as_str()) {
  //     Ok(r) => *regex_line = r,
  //     Err(e) => return HttpResponse::InternalServerError().json(ErrorMessageResponse { message: format!("{}", e) }),
  //   }
  // }
  // EOF : UNCOMMENT to use config

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
    // BOF : UNCOMMENT to use config
    // filter_file: app_data.filter_file.lock().unwrap().to_string(),
    // filter_line: app_data.filter_line.lock().unwrap().to_string(),
    // EOF : UNCOMMENT to use config
    config_file: config_file.to_string(),
  }))
}

// GET:config
#[get("/config")]
pub async fn get_config(app_data: web::Data<AppStateGlobal>) -> impl Responder {
  let current_config_file_mutex_guard = app_data.config_file.lock().unwrap();
  match get_config_state(current_config_file_mutex_guard) {
    Ok(c) => HttpResponse::Ok().json(c),
    Err(e) => HttpResponse::InternalServerError().json(ErrorMessageResponse { message: format!("{:?}", e) }),
  }
}

/// POST:/upload
#[post("/upload")]
pub async fn upload(payload: Multipart) -> Result<HttpResponse, ActixError> {
  let upload_status = save_file(payload).await;

  match upload_status {
    Ok(_) => Ok(HttpResponse::build(StatusCode::OK).json(MessageResponse {
      message: "successful upload".to_string(),
    })),
    Err(e) => Ok(HttpResponse::build(StatusCode::NOT_FOUND).json(MessageResponse {
      message: format!("failed upload: {}", e),
    })),
  }
}

/// POST:/test-awc
#[post("/test-awc")]
pub async fn test_awc(msg: web::Json<PostAwcUriRequest>) -> Result<HttpResponse, ActixError> {
  let client = Client::default();
  let res = client
    // create request builder
    .get(msg.uri.as_str())
    .insert_header(("User-Agent", "Actix-web"))
    // send http request
    .send()
    .await;
  match res {
    Ok(_) => Ok(HttpResponse::build(StatusCode::OK).json(MessageResponse {
      message: "request successful".to_string(),
    })),
    Err(e) => Ok(HttpResponse::build(StatusCode::NOT_FOUND).json(MessageResponse {
      message: format!("request failed to '{}' error {}", msg.uri.as_str(), e),
    })),
  }
}
