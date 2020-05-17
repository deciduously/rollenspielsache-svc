//! App-wide tests

use super::*;
use actix_web::{http, test, web};
use pretty_assertions::assert_eq;

#[actix_rt::test]
async fn test_index_redirect() {
    let mut app = test::init_service(App::new().route("/", web::get().to(index))).await;
    let req = test::TestRequest::with_header("content-type", "text/plain").to_request();
    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), http::StatusCode::SEE_OTHER);
}
