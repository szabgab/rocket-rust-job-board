use std::env;

use rocket::http::{ContentType, Status};
use rocket::local::blocking::Client;

#[test]
fn hello_world() {
    let client = Client::tracked(super::rocket()).unwrap();
    let response = client.get("/").dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(
        response.headers().get_one("Content-Type").unwrap(),
        "text/html; charset=utf-8"
    );
    let html = response.into_string().unwrap();
    assert!(html.contains("Hello <b>Rocket with Tera!</b>"));
}

#[test]
fn test_add() {
    let tmp_dir = tempfile::tempdir().unwrap();
    let db_path = tmp_dir.path().join("db");
    env::set_var("DB_PATH", &db_path);

    let client = Client::tracked(super::rocket()).unwrap();
    let response = client.get("/add").dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(
        response.headers().get_one("Content-Type").unwrap(),
        "text/html; charset=utf-8"
    );
    let html = response.into_string().unwrap();

    assert!(html.contains("<title>Add a Rust job</title>"));

    assert!(html.contains(r#"<a href="/">list</a>"#));
    // TODO: missing title field
    // let response = client
    //     .post("/add")
    //     .header(ContentType::Form)
    //     .body("")
    //     .dispatch();

    let response = client
        .post("/add")
        .header(ContentType::Form)
        .body("title=Sn. Backend Engineer with Rust&company=IBM")
        .dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(
        response.headers().get_one("Content-Type").unwrap(),
        "text/html; charset=utf-8"
    );

    let html = response.into_string().unwrap();
    //assert!(html.contains("<title>Job added</title>"));

    assert!(html.contains(r#"<a href="/">list</a>"#));
}
