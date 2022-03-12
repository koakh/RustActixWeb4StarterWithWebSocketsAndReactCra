#![allow(unused_imports)]
// trick for .start() is start using actix::prelude::* and find the required import, ex Actor
// find what is the required trait to not include all of them
use actix::prelude::{Actor, Addr};
// use actix::prelude::*;
use actix_cors::Cors;
use actix_files::Files;
use actix_web::dev::{AppService, HttpServiceFactory, ResourceDef, ServiceRequest};
use actix_web::error::BlockingError;
use actix_web::rt::{spawn, time};
use actix_web::web::Data;
use actix_web::{get, middleware, post, web, App, Error as ActixError, HttpRequest, HttpResponse, HttpServer, Responder, Result};
use actix_web_actors::ws;
use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
use actix_web_httpauth::extractors::AuthenticationError;
use actix_web_httpauth::middleware::HttpAuthentication;
use linemux::MuxedLines;
use log::{debug, error, info};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::cell::RefCell;
use std::fmt::Display;
use std::path::PathBuf;
use std::process::exit;
use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::MutexGuard;
use std::time::Duration;
use std::{
  cell::Cell,
  env,
  sync::{Arc, Mutex},
};
use structopt::StructOpt;
// // use local modules from mod.rs
use actixweb4_starter::{
  app::{
    config::ConfigItem, init_log4rs, AppState, AppStateGlobal, Cli, ConfigState, APP_NAME, CONFIG_FILE_PATH, DEFAULT_CERT_FILE_NAME_CERT, DEFAULT_CERT_FILE_NAME_KEY, DEFAULT_CONFIG_PATH_SSL,
    DEFAULT_FILTER_FILE, DEFAULT_FILTER_LINE, DEFAULT_HTTP_SERVER_URI, DOWNLOAD_FILES_PATH, DOWNLOAD_URI_PATH, DOWNLOAD_URI_PATH_ABSOLUTE, FORMAT_DATE_TIME_FILE_NAME, HTTP_SERVER_API_KEY,
  },
  enums::MessageToClientType,
  requests::{PostStateRequest, PostWsEchoRequest, PostbackupLogRequest},
  responses::{AppStateResponse, BackupLogResponse, ErrorMessageResponse, GetStateResponse, PingResponse, PostStateResponse, PostWsEchoResponse},
  util::{
    execute_command, execute_command_shortcut, get_config_files_from_regex, get_config_item, get_config_state, get_current_formatted_date, out_message, pathbuf_to_str, read_config, read_generic_type,
    ExecuteCommandOutcome,
  },
  websocket::{ws_index, MessageToClient, Server as WebServer},
};

static SERVER_COUNTER: AtomicUsize = AtomicUsize::new(0);

/// GET:/ping
#[get("/ping")]
async fn health_check(_: HttpRequest) -> Result<web::Json<PingResponse>> {
  Ok(web::Json(PingResponse { message: "pong".to_string() }))
}

/// POST:/ws-echo
#[post("/ws-echo")]
async fn _ws_echo(msg: web::Json<PostWsEchoRequest>, websocket_srv: web::Data<Addr<WebServer>>) -> HttpResponse {
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
async fn _greet(req: HttpRequest) -> impl Responder {
  let name = req.match_info().get("name").unwrap_or("World");
  format!("Hello {}!", &name)
}

/// POST:/state : same as /filter but change filters and respond with full state
#[post("/state-full")]
async fn post_state_full(msg: web::Json<PostStateRequest>, data: web::Data<AppState>, app_data: web::Data<AppStateGlobal>) -> Result<web::Json<AppStateResponse>> {
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
async fn post_state(msg: web::Json<PostStateRequest>, data: web::Data<AppState>, app_data: web::Data<AppStateGlobal>) -> impl Responder /*Result<web::Json<PostFilterResponse>>*/ {
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
      Err(e) => return HttpResponse::InternalServerError().json(ErrorMessageResponse { message: format!("{}", e) }),
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

  // output chaneged filters: leave stdout clean for output filtered log lines only
  // out_message(format!("filters changed file: {}, line: {}", &msg.filter_file, &msg.filter_line), 0);

  HttpResponse::Ok().json(PostStateResponse {
    filter_file: String::from(&msg.filter_file),
    filter_line: String::from(&msg.filter_line),
  })
}

/// GET:/state
#[get("/state")]
async fn get_state(app_data: web::Data<AppStateGlobal>) -> Result<web::Json<GetStateResponse>> {
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
async fn get_config(app_data: web::Data<AppStateGlobal>) -> impl Responder {
  let current_config_file_mutex_guard = app_data.config_file.lock().unwrap();
  match get_config_state(current_config_file_mutex_guard) {
    Ok(c) => HttpResponse::Ok().json(c),
    Err(e) => HttpResponse::InternalServerError().json(ErrorMessageResponse { message: format!("{:?}", e) }),
  }
}

#[post("/backup-log")]
async fn post_backup_log(msg: web::Json<PostbackupLogRequest>, app_data: web::Data<AppStateGlobal>) -> impl Responder {
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  // env vars
  let http_server_uri = env::var("HTTP_SERVER_URI").unwrap_or(DEFAULT_HTTP_SERVER_URI.to_string());
  let config_path_ssl = env::var("CONFIG_PATH_SSL").unwrap_or(DEFAULT_CONFIG_PATH_SSL.to_string());
  let default_cert_file_name_key = env::var("CERT_FILE_NAME_KEY").unwrap_or(DEFAULT_CERT_FILE_NAME_KEY.to_string());
  let default_cert_file_name_cert = env::var("CERT_FILE_NAME_CERT").unwrap_or(DEFAULT_CERT_FILE_NAME_CERT.to_string());
  // init_log()
  // init log4rs
  init_log4rs().expect("can't initialize logger");

  // default config, must be implicit overrided
  let mut config = ConfigState {
    filter_file: Rc::new(RefCell::new(Some(String::from(DEFAULT_FILTER_FILE)))),
    filter_line: Rc::new(RefCell::new(Some(String::from(DEFAULT_FILTER_LINE)))),
    input_files: Rc::new(RefCell::new(Some(vec![
      // PathBuf::from("/var/log/syslog".to_string()),
    ]))),
    configuration: Rc::new(RefCell::new(vec![])),
  };

  // config https ssl keys
  let cert_file_name_key = format!("{}/{}", config_path_ssl, default_cert_file_name_key);
  let cert_file_name_cert = format!("{}/{}", config_path_ssl, default_cert_file_name_cert);
  let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
  builder.set_private_key_file(cert_file_name_key.clone(), SslFiletype::PEM).unwrap();
  builder.set_certificate_chain_file(cert_file_name_cert.clone()).unwrap();
  // builder.set_private_key_file(format!("{}/key.pem", config_path_ssl), SslFiletype::PEM).unwrap();
  // builder.set_certificate_chain_file(format!("{}/cert.pem", config_path_ssl)).unwrap();

  // the trick for not lost connections sessions, is create ws_server outside of HttpServer::new, and use `move ||`
  let ws_server = WebServer::new().start();
  let ws_server_spawn = ws_server.clone();

  let cli = Cli::from_args();
  let mut current_config_file = String::from("");
  match &cli {
    Cli::StartHttpServer {
      config_file,
      input_files,
      filter_file,
      filter_line,
    } => {
      // use files and filters: priority is the files and filters
      if !input_files.is_empty() {
        // override deffult config
        config.filter_file = Rc::new(RefCell::new(Some(String::from(filter_file))));
        config.filter_line = Rc::new(RefCell::new(Some(String::from(filter_line))));
        config.input_files = Rc::new(RefCell::new(Some(input_files.to_vec())));
      }
      // use config
      else {
        current_config_file = pathbuf_to_str(config_file.as_ref().unwrap());
        match read_config(current_config_file.as_str()) {
          // override default config
          Ok(c) => {
            config = c;
          }
          // clean exit
          Err(e) => {
            eprintln!("error: {:?}", e);
            exit(0x0100);
          }
        };
      };
    }
  }

  // init initial_filter_file and initial_filter_line from config references
  let initial_filter_file = config.filter_file.borrow().as_ref().unwrap().clone();
  let initial_filter_line = config.filter_line.borrow().as_ref().unwrap().clone();
  // declare muxed lines
  let mut lines = MuxedLines::new()?;
  // extract a new vec from config input_files
  let input_files = config.input_files.borrow().as_ref().unwrap().clone();
  for f in input_files.to_vec() {
    match lines.add_file(&f).await {
      Ok(r) => debug!("{:?}", r),
      Err(e) => error!("{:?}", e),
    };
  }

  //command line validation
  // bellow validation is handled by structOps with The argument '--files <input-files>...' requires at least 1 values, but only 0 was provided
  // if config.input_files.borrow().as_ref().unwrap().is_empty() {
  //   panic!("{}", "You must supply input file(s) in config file or passed by --input-files flag");
  // }

  // let initial_filter_file_re = Regex::new(initial_filter_file.as_str()).unwrap();
  // let initial_filter_line_re = Regex::new(initial_filter_line.as_str()).unwrap();
  // let match_file_re = initial_filter_file_re.is_match(r"c3-microcloud-backend.log");
  // let match_line_re = iniPathBuf to Stringtial_filter_line_re.is_match(r"Jan 10 18:09:24 c3 docker/c3-microcloud-backend[941]: [Nest] 11  - 01/10/2022, 6:09:24 PM     LOG [HttpModule] POST https://172.17.0.1:8410/api/action 11ms}");
  let data = web::Data::new(AppStateGlobal {
    counter: Mutex::new(0),
    filter_file: Arc::new(Mutex::new(String::from(initial_filter_file.clone()))),
    filter_line: Arc::new(Mutex::new(String::from(initial_filter_line.clone()))),
    regex_file: Arc::new(Mutex::new(Regex::new(initial_filter_file.as_str()).unwrap())),
    regex_line: Arc::new(Mutex::new(Regex::new(initial_filter_line.as_str()).unwrap())),
    config_file: Arc::new(Mutex::new(Some(current_config_file))),
  });

  // check current config
  // out_message(format!("config: {:?}", config), 0);
  // out_message(format!("lines: {:?}", lines), 0);
  out_message(format!("initial filters file: '{}', line: '{}'", initial_filter_file.clone(), initial_filter_line.clone()), 0);
  // the real and hard trick to use references of AppStateGlobal is clone the arc, one tip of mighty The0x539
  let _ref_filter_file = data.filter_file.clone();
  let _ref_filter_line = data.filter_line.clone();
  let ref_regex_file = data.regex_file.clone();
  let ref_regex_line = data.regex_line.clone();
  // spawn loop in parallel thread with async
  spawn(async move {
    while let Ok(Some(line)) = lines.next_line().await {
      // filter file
      // out_message(
      //   format!("filter file: {}, filter line: {}", &ref_filter_file.lock().unwrap().as_str(), &ref_filter_line.lock().unwrap().as_str()),
      //   0,
      // );
      // without regex
      // if line.source().display().to_string().to_lowercase().eq("") || line.source().display().to_string().to_lowercase().contains(&ref_filter_file.lock().unwrap().as_str()) {

      // with regex
      let match_file_re = ref_regex_file.lock().unwrap().is_match(&line.source().display().to_string());
      if line.source().display().to_string().to_lowercase().eq("") || match_file_re {
        // filter line
        // without regex
        // if line.line().to_string().to_lowercase().eq("") || line.line().to_string().to_lowercase().contains(&ref_filter_line.lock().unwrap().as_str()) {
        // with regex
        let match_line_re = ref_regex_line.lock().unwrap().is_match(&line.line().to_string());
        if line.line().to_string().to_lowercase().eq("") || match_line_re {
          // out_message(format!("source: {}, line: {}", line.source().display(), line.line()), 0);
          // TOOD: don't enable this will polute and creat some extra flood on syslog
          // out_message(format!("{}", line.line()), 0);
          // send message to client
          let json = json!({ "message": line.line() });
          // let wsm: WebSocketMessage = serde_json::from_value(json).unwrap();
          let msg_type = &format!("{}", MessageToClientType::Echo)[..];
          let message_to_client = MessageToClient::new(msg_type, json);
          // let message_to_client = MessageToClient::new("echo", json);
          // websocket_srv.do_send(message_to_client);
          match ws_server_spawn.send(message_to_client).await {
            Ok(_) => {}
            Err(e) => error!("{:?}", e),
          };
        }
      }
      // out_message(format!("data: {:?}", &ref_filter_file.lock().unwrap()), 0);
    }
  });

  // authentication validator
  // required to implement ResponseError in src/app/errors.rs else we have a error
  // Err(AuthenticationError::from(config).into())
  async fn validator(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, ActixError> {
    if credentials.token() == HTTP_SERVER_API_KEY.to_string() {
      Ok(req)
    } else {
      let config = req
        .app_data::<Config>()
        .map(|data| data.clone())
        .unwrap_or_else(Default::default)
        .scope("urn:example:channel=HBO&urn:example:rating=G,PG-13");
      out_message("invalid authorization api key".to_string(), 0);
      Err(AuthenticationError::from(config).into())
      // with this output unauthorized in response, we keep with silence message and error code 401
      // use actix_web::{error::ErrorUnauthorized};
      // Err(ErrorUnauthorized("unauthorized"))
    }
  }

  info!(
    "start {} rest server at: '{}' with certificates {}, {}",
    APP_NAME, http_server_uri, cert_file_name_key, cert_file_name_cert
  );

  HttpServer::new(move || {
    // cors
    let cors = Cors::default().allow_any_origin().allow_any_header().allow_any_method().supports_credentials();

    App::new()
      .wrap(cors)
      // enable logger
      .wrap(middleware::Logger::default())
      // new actixweb MUST USE everything wrapped in Data::new() this is the solution for webosckets connection error
      .app_data(Data::new(AppState {
        server_id: SERVER_COUNTER.fetch_add(1, Ordering::SeqCst),
        request_count: Cell::new(0),
        // filter,
      }))
      // global data: don't wrap it in data::new() it's already wrapped above
      .app_data(data.clone())
      // inject ws_server in context
      .app_data(Data::new(ws_server.clone()))
      // webSockets: TRICK /ws/ route must be before / and others to prevent problems
      .service(web::resource("/ws/").route(web::get().to(ws_index)))
      .service(health_check)
      // we allow the visitor to see an index of the images at `/downloads`.
      // .service(Files::new(format!("/{}", DOWNLOAD_FILES_PATH).as_str(), format!("static/{}", DOWNLOAD_FILES_PATH).as_str()).show_files_listing())      
      // without see an index of the images at `/downloads`.
      .service(Files::new(format!("{}", DOWNLOAD_URI_PATH).as_str(), format!("{}", DOWNLOAD_FILES_PATH).as_str()))      
      // scoped
      .service(
        web::scope("/api")
          // authentication middleware, warn: Bearer must be uppercased Bearer to work with actix-web-httpauth, bearer fails
          .wrap(HttpAuthentication::bearer(validator))
          .service(post_state_full)
          .service(get_state)
          .service(post_state)
          .service(get_config)
          .service(post_backup_log)
          // .service(ws_echo)
          // .route("/{name}", web::get().to(greet))
          // static, leave / route to the end, else it overrides all others
          // .route("/", web::get().to(greet)),
      )
  })
  // .workers(2)
  // .bind(http_server_uri)?
  .bind_openssl(http_server_uri, builder)?
  .run()
  .await
}
