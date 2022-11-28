pub mod routes_utils {
  use actix_web::{
    error,
    http::{header::ContentType, StatusCode},
    HttpResponse,
  };
  use derive_more::{Display, Error};
  use serde::{Deserialize, Serialize};

  #[derive(Serialize, Deserialize, Clone, Debug)]
  pub struct Tile {
    pub content: String,
  }

  #[derive(Serialize, Deserialize)]
  pub struct SolveInfo {
    pub board: [Tile; 9],
    pub method: String,
  }

  #[derive(Serialize)]
  pub struct SolveResult {
    pub board: [Tile; 9],
  }

  #[derive(Debug, Display, Error)]
  pub enum SolveErrors {
    #[display(fmt = "The method {} is not supported", method)]
    MethodNotSupported { method: String },
  }

  impl error::ResponseError for SolveErrors {
    fn error_response(&self) -> HttpResponse {
      HttpResponse::build(self.status_code())
        .insert_header(ContentType::html())
        .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
      match *self {
        SolveErrors::MethodNotSupported { .. } => StatusCode::BAD_REQUEST,
      }
    }
  }
}
