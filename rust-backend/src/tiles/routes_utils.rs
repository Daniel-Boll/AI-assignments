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

    impl Tile {
        pub fn new(content: String) -> Self {
            Tile { content }
        }

        pub fn is_empty(&self) -> bool {
            self.content == ""
        }

        pub fn goal_position(&self) -> usize {
            match self.content.as_str() {
                "1" => 0,
                "2" => 1,
                "3" => 2,
                "4" => 3,
                "5" => 4,
                "6" => 5,
                "7" => 6,
                "8" => 7,
                _ => 8,
            }
        }
    }

    #[derive(Serialize, Deserialize)]
    pub struct SolveInfo {
        pub board: [Tile; 9],
        pub method: String,
    }

    #[derive(Serialize)]
    pub struct SolveResult {
        pub solution_steps: Vec<[Tile; 9]>,
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
