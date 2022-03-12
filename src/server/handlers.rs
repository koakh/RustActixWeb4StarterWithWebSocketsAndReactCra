use actix_multipart::Multipart;
use actix_web::{
  get,
  http::{header, StatusCode},
  post, web, Error as ActixError, HttpRequest, HttpResponse, Result,
};

use crate::{
  app::{ENDPOINT_REDIRECT_TO, RANDOM_STRING_GENERATOR_CHARSET, RANDOM_STRING_GENERATOR_SIZE},
  responses::{ApiKeyResponse, MessageResponse},
  server::save_file,
  util::generate_random_string,
};

/// GET:/ping
#[get("/ping")]
async fn health_check(_: HttpRequest) -> Result<web::Json<MessageResponse>> {
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
    Err(_) => Ok(HttpResponse::build(StatusCode::NOT_FOUND).json(MessageResponse { message: "failed upload".to_string() })),
  }
}
