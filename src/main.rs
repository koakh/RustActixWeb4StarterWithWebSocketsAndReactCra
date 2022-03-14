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
use actix_web_static_files::ResourceFiles;
use actixweb4_starter::{
  app::{
    config::ConfigItem, init_log4rs, AppState, AppStateGlobal, Cli, ConfigState, APP_NAME, CONFIG_FILE_PATH, DEFAULT_CERT_FILE_NAME_CERT, DEFAULT_CERT_FILE_NAME_KEY, DEFAULT_CONFIG_PATH_SSL,
    DEFAULT_FILTER_FILE, DEFAULT_FILTER_LINE, DEFAULT_HTTP_SERVER_URI, DOWNLOAD_FILES_PATH, DOWNLOAD_URI_PATH, DOWNLOAD_URI_PATH_ABSOLUTE, FORMAT_DATE_TIME_FILE_NAME, HTTP_SERVER_API_KEY,
    HTTP_SERVER_KEEP_ALIVE, LOG_ACTIXWEB_MIDDLEWARE_FORMAT, PUBLIC_URI_PATH, RANDOM_STRING_GENERATOR_CHARSET, RANDOM_STRING_GENERATOR_SIZE,
  },
  enums::MessageToClientType,
  requests::{PostStateRequest, PostWsEchoRequest, PostbackupLogRequest},
  responses::{ApiKeyResponse, AppStateResponse, BackupLogResponse, ErrorMessageResponse, GetStateResponse, MessageResponse, PostStateResponse, PostWsEchoResponse},
  server::{get_config, get_state, health_check, not_found, post_backup_log, post_state, post_state_full, redirect, upload},
  util::{
    execute_command, execute_command_shortcut, generate_random_string, get_config_files_from_regex, get_config_item, get_config_state, get_current_formatted_date, out_message, pathbuf_to_str,
    read_config, read_generic_type, ExecuteCommandOutcome,
  },
  websocket::{ws_index, MessageToClient, Server as WebServer},
};

// for static files
include!(concat!(env!("OUT_DIR"), "/generated.rs"));

static SERVER_COUNTER: AtomicUsize = AtomicUsize::new(0);

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

  // default config, must be implicit override
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
        // override default config
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
          // TODO: don't enable this, it will pollute and create some extra flood on syslog
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
    // init actix_web_static_files generated
    let generated = generate();

    App::new()
      .wrap(cors)
      // enable logger
      // .wrap(middleware::Logger::default())
      .wrap(middleware::Logger::new(LOG_ACTIXWEB_MIDDLEWARE_FORMAT))
      // new actixweb MUST USE everything wrapped in Data::new() this is the solution for websockets connection error
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
      .service(redirect)
      // disabled
      // .service(api_key)
      // TODO: can't show listing yet
      // we allow the visitor to see an index of the images at `/downloads`.
      .service(Files::new(format!("{}", DOWNLOAD_FILES_PATH).as_str(), format!("{}{}", DOWNLOAD_FILES_PATH, DOWNLOAD_URI_PATH).as_str()).show_files_listing())
      // TODO: download path is a ENV VAR
      // without see an index of the images at `/downloads`.
      .service(Files::new(format!("{}", DOWNLOAD_URI_PATH).as_str(), format!("{}{}", DOWNLOAD_FILES_PATH, DOWNLOAD_URI_PATH).as_str()))
      // scoped
      .service(
        // TODO: use /api on constants and ENV VAR ex /api/v1
        web::scope("/api")
          // authentication middleware, warn: Bearer must be uppercased Bearer to work with actix-web-httpauth, bearer fails
          .wrap(HttpAuthentication::bearer(validator))
          .service(post_state_full)
          .service(get_state)
          .service(post_state)
          .service(get_config)
          .service(post_backup_log)
          .service(upload),
        // .service(ws_echo)
        // .route("/{name}", web::get().to(greet))
        // static, leave / route to the end, else it overrides all others
        // .route("/", web::get().to(greet)),
      )
    // static, leave / route to the end, else it overrides all others
    .service(ResourceFiles::new("/", generated).resolve_not_found_to_root())
    // after all is default_service if above / is not used only
    // .default_service(web::route().to(not_found))
  })
  // .workers(2)
  .keep_alive(Duration::from_secs(HTTP_SERVER_KEEP_ALIVE))
  // .bind(http_server_uri)?
  .bind_openssl(http_server_uri, builder)?
  .run()
  .await
}
