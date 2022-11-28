use actix_web::{
  get, http::StatusCode, middleware::Logger, App, HttpResponse, HttpServer, Responder
};
use env_logger::Env;

use rust_backend::tiles::prelude::*;

#[get("/")]
/// deserialize `Info` from request's body, max payload size is 4kb
async fn running_server() -> impl Responder {
  HttpResponse::Ok()
    .status(StatusCode::OK)
    .body("Hello World!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  env_logger::init_from_env(Env::default().default_filter_or("info"));
  HttpServer::new(|| {
    App::new()
      .service(running_server)
      .service(router())
      .wrap(Logger::default())
      .wrap(Logger::new("%a %{User-Agent}i"))
  })
  .bind(("127.0.0.1", 8080))?
  .run()
  .await
}

// ================================================== TESTS ==================================================

#[cfg(test)]
mod tests {
  use super::*;
  use actix_web::{http::header::ContentType, test, App};

  #[actix_web::test]
  async fn test_index_get() {
    let app = test::init_service(App::new().service(running_server)).await;
    let req = test::TestRequest::default()
      .insert_header(ContentType::plaintext())
      .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
  }

  #[actix_web::test]
  async fn test_index_post() {
    let app = test::init_service(App::new().service(running_server)).await;
    let req = test::TestRequest::post().uri("/").to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_client_error());
  }
}
