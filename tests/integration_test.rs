use actix_web::{test, App};
use lightning_nodes_api::health_check; // Imports the public health_check function from your crate

#[actix_rt::test]
async fn test_health_endpoint() {
    let app = test::init_service(App::new().service(health_check)).await;
    let req = test::TestRequest::get().uri("/health").to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
    assert_eq!(resp.status().as_u16(), 200);
    let response_body = test::read_body(resp).await;
    assert_eq!(response_body, "OK");
}