// // aliemjay — Hoje às 12:46
// // the other variant in BlockingError was removed because it is no longer needed. You can take a look on how web::block signature changed in v4.
// // impl From<BlockingError> for Error {
// //   fn from(error: BlockingError) -> Error {
// //     Error::BlockingError("Thread blocking error".into())
// //   }
// // }

// use actix_web::{
//   error::{BlockingError, ResponseError},
//   Error as ActixError, HttpResponse,
// };

// impl From<BlockingError> for Error {
//   fn from(error: BlockingError) -> Error {
//     Error::BlockingError("Thread blocking error".into())
//   }
// }
