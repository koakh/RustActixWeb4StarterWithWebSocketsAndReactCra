use actix_multipart::Multipart;
use actix_web::{Error as ActixError, HttpResponse, web};
use uuid::Uuid;
use std::io::Write;
use futures_util::TryStreamExt as _;

// the following trait is implemented but not in scope; perhaps add a `use` for it:: `use std::io::Write;
pub async fn save_file(mut payload: Multipart) -> Result<HttpResponse, ActixError> {
  // iterate over multipart stream
  while let Some(mut field) = payload.try_next().await? {
    // A multipart/form-data stream has to contain `content_disposition`
    let content_disposition = field.content_disposition();

    let filename = content_disposition.get_filename().map_or_else(|| Uuid::new_v4().to_string(), sanitize_filename::sanitize);
    let filepath = format!("/tmp/{}", filename);

    // File::create is blocking operation, use threadpool
    let mut f = web::block(|| std::fs::File::create(filepath)).await??;

    // Field in turn is stream of *Bytes* object
    while let Some(chunk) = field.try_next().await? {
      // filesystem operations are blocking, we have to use threadpool
      f = web::block(move || f.write_all(&chunk).map(|_| f)).await??;
    }
  }

  // here can be unit ()....leaved this way to work passing to .route and .service at same time
  Ok(HttpResponse::Ok().into())
}