#![allow(dead_code)]
// app
pub const APP_NAME: &'static str = "actixweb4-starter";
pub const DEFAULT_HTTP_SERVER_URI: &'static str = "0.0.0.0:8543";
pub const HTTP_SERVER_API_KEY: &'static str = "uOtXEZXYslKyB0n3g3xRmCaaNsAwB5KmgFcy1X7bbcbtS9dhOpKuhZ04Mfr2OKGL";
pub const HTTP_SERVER_KEEP_ALIVE: u64 = 30;
pub const CONFIG_FILE_PATH: &'static str = "/etc/actixweb4-starter/config.json";
// pub const LOG_FILE_PATH: &'static str = "/var/log/actixweb4-starter.log";
pub const LOG_FILE_PATH: &'static str = "./actixweb4-starter.log";
// DEBUG
pub const LOG_DEFAULT_LEVEL: &'static str = "ERROR";
pub const LOGFILE_DEFAULT_LEVEL: &'static str = "ERROR";
pub const LOG_ACTIXWEB_MIDDLEWARE_FORMAT: &'static str = r#""%r" %s %b "%{User-Agent}i" %D"#;

// pub const DEFAULT_RUST_LOG: &'static str = "error,actix_server=error,actix_web=error";
// misc
pub const LOG_HEADER_LINE_CHAR: &'static char = &'-';
pub const LOG_HEADER_LINE_LEN: &'static u8 = &120;
// filters
pub const DEFAULT_FILTER_FILE: &'static str = r"^.*c3-.*\.log$";
pub const DEFAULT_FILTER_LINE: &'static str = r"(?i)(.*)";
// certificates
pub const DEFAULT_CONFIG_PATH_SSL: &'static str = "./config/ssl";
pub const DEFAULT_CERT_FILE_NAME_KEY: &'static str = "key.pem";
pub const DEFAULT_CERT_FILE_NAME_CERT: &'static str = "cert.pem";
// files - /static/
pub const DOWNLOAD_FILES_PATH: &'static str = "./static";
pub const DOWNLOAD_URI_PATH: &'static str = "/downloads";
pub const DOWNLOAD_URI_PATH_ABSOLUTE: &'static str = "https://localhost:8543/downloads";
pub const PUBLIC_URI_PATH: &'static str = "/";
// date
pub const FORMAT_DATE_TIME: &'static str = "%Y-%m-%d %H:%M:%S";
pub const FORMAT_DATE_TIME_FILE_NAME: &'static str = "%Y-%m-%d_%H-%M-%S";
// random charset
pub const RANDOM_STRING_GENERATOR_CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789)(*&^%$#@!~";
pub const RANDOM_STRING_GENERATOR_SIZE: usize = 64;
// redirect
pub const ENDPOINT_REDIRECT_TO: &'static str = "https://koakh.com";
// spawn thread
pub const SPAWN_THREAD_ENABLED: bool = false;
pub const SPAWN_THREAD_DURATION_SECONDS: u64 = 30;
