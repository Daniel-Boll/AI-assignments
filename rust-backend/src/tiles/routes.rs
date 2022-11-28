pub mod tile_router {
  use actix_web::{get, post, web, HttpResponse, Responder};

  use crate::tiles::{
    generate_and_test::*,
    hill_climbing::*,
    routes_utils::routes_utils::{SolveErrors, SolveInfo, SolveResult},
  };
  #[post("/solve")]
  async fn solve(info: web::Json<SolveInfo>) -> Result<impl Responder, SolveErrors> {
    let res = match info.method.as_str() {
      "generate_and_test" => Ok(generate_and_test::solver(info.board.clone())),
      "hill_climbing" => Ok(hill_climbing::solver(info.board.clone())),
      _ => Err(SolveErrors::MethodNotSupported {
        method: info.method.clone(),
      }),
    };

    if res.is_err() {
      return Err(res.err().unwrap());
    }

    let result = SolveResult {
      board: res.unwrap(),
    };

    Ok(HttpResponse::Ok().json(result))
  }

  #[get("")]
  async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello from tiles!")
  }
  pub fn router() -> actix_web::Scope {
    web::scope("tiles").service(index).service(solve)
  }
}

// ================================================== TESTS ==================================================

#[cfg(test)]
mod tests {
  use actix_web::{test, web, App};

  use crate::tiles::prelude::{routes_utils::SolveInfo, *};

  #[actix_web::test]
  async fn test_index_get() {
    let app = test::init_service(App::new().service(web::scope("tiles").service(index))).await;
    let req = test::TestRequest::get().uri("/tiles").to_request();
    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_success());

    assert_eq!(test::read_body(resp).await, "Hello from tiles!");
  }

  #[actix_web::test]
  async fn test_index_post() {
    let app = test::init_service(App::new().service(web::scope("tiles").service(index))).await;
    let req = test::TestRequest::post().uri("/tiles").to_request();
    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_client_error());
  }

  #[actix_web::test]
  async fn test_solve() {
    let app = test::init_service(App::new().service(web::scope("tiles").service(solve))).await;
    let payload = r#"{
      "method": "generate_and_test",
      "board": [
        { "content": "1" },
        { "content": "2" },
        { "content": "3" },
        { "content": "4" },
        { "content": "5" },
        { "content": "6" },
        { "content": "7" },
        { "content": "8" },
        { "content": "" }
      ]
    }"#;

    let json_payload = serde_json::from_str::<SolveInfo>(payload).unwrap();

    assert_eq!(json_payload.board.len(), 9);

    let req = test::TestRequest::post()
      .uri("/tiles/solve")
      .set_json(&json_payload)
      .to_request();
    let resp = test::call_service(&app, req).await;

    assert!(resp.response().status().is_success());
  }
}
