#![allow(dead_code)]
// app
pub const APP_NAME: &'static str = "actixweb4-starter";
pub const DEFAULT_HTTP_SERVER_URI: &'static str = "0.0.0.0:8470";
pub const HTTP_SERVER_API_KEY: &'static str = "5KmgFcy1X7bbcbtSuOtXEZXYslKyB0n3g3xRmCaaNsAwB9dhOpKuhZ04Mfr2OKGL";
pub const CONFIG_FILE_PATH: &'static str = "/etc/actixweb4-starter/config.json";
pub const LOG_FILE_PATH: &'static str = "/tmp/actixweb4-starter.log";
// DEBUG
pub const LOG_DEFAULT_LEVEL: &'static str = "ERROR";
pub const LOGFILE_DEFAULT_LEVEL: &'static str = "ERROR";
// pub const DEFAULT_RUST_LOG: &'static str = "error,actix_server=error,actix_web=error";
// misc
pub const LOG_HEADER_LINE_CHAR: &'static char = &'-';
pub const LOG_HEADER_LINE_LEN: &'static u8 = &120;
// filters
pub const DEFAULT_FILTER_FILE: &'static str = r"^.*c3-.*\.log$";
pub const DEFAULT_FILTER_LINE: &'static str = r"(?i)(.*)";
// certificates
// ./config/ssl
pub const DEFAULT_CONFIG_PATH_SSL: &'static str = "/etc/apache2/ssl";
// key.pem
pub const DEFAULT_CERT_FILE_NAME_KEY: &'static str = "c3edu.online.key";
// cert.pem
pub const DEFAULT_CERT_FILE_NAME_CERT: &'static str = "c3edu.online.crt";
// files - /static/
pub const DOWNLOAD_FILES_PATH: &'static str = "/tmp";
pub const DOWNLOAD_URI_PATH: &'static str = "/downloads";
pub const DOWNLOAD_URI_PATH_ABSOLUTE: &'static str = "https://c3edu.online:8470/downloads";

// date
pub const FORMAT_DATE_TIME: &'static str = "%Y-%m-%d %H:%M:%S";
pub const FORMAT_DATE_TIME_FILE_NAME: &'static str = "%Y-%m-%d_%H-%M-%S";
