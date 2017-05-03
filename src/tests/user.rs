use rocket;
use rocket::testing::MockRequest;
use rocket::http::Method::*;
use rocket::http::{Status, Header};
use rocket::Response;

use serde_json;

use helpers::error::Error;

use services::users::User;

use tests::helpers;
use self::helpers::testdata;

#[test]
fn test_me() {
    let test_user = testdata::recreate().user;
    let auth_token = testdata::normal_user_auth_token(test_user.id, &test_user.username);
    let auth_header = Header::new("Authorization", "Bearer ".to_string() + &auth_token);
    let rocket = rocket();

    let mut req = MockRequest::new(Get, "/users/me");
    req.add_header(auth_header);
    run_test!(&rocket, req, |mut response: Response| {
        let body = body_string!(response);
        let user: User = serde_json::from_str(&body).unwrap();
        assert_eq!(user.id, test_user.id);
        assert_eq!(user.username, test_user.username);
        assert_eq!(user.email, test_user.email);
    });

    // without token
    let req = MockRequest::new(Get, "/users/me");
    run_test!(&rocket, req, |mut response: Response| {
        let body = body_string!(response);
        let err: Error = serde_json::from_str(&body).unwrap();
        assert_eq!(err.code, Status::Unauthorized.code);
        assert_eq!(err.msg, "token not found");
    });

    // invalid token
    let wrong_token = Header::new("Authorization", "Bearer wrongtoken");
    let mut req = MockRequest::new(Get, "/users/me");
    req.add_header(wrong_token);
    run_test!(&rocket, req, |mut response: Response| {
        let body = body_string!(response);
        let err: Error = serde_json::from_str(&body).unwrap();
        assert_eq!(err.code, Status::Unauthorized.code);
        assert_eq!(err.msg, "invalid token");
    });
}

#[test]
fn test_get_users() {
    let test_user = testdata::recreate().user;
    let auth_token = testdata::admin_user_auth_token(1, "admin");
    let auth_header = Header::new("Authorization", "Bearer ".to_string() + &auth_token);
    let rocket = rocket();

    let mut req = MockRequest::new(Get, "/users");
    req.add_header(auth_header);
    run_test!(&rocket, req, |mut response: Response| {
        let body = body_string!(response);
        let users: Vec<User> = serde_json::from_str(&body).unwrap();
        assert_eq!(users.len(), 1);
        assert_eq!(users[0].id, test_user.id);
        assert_eq!(users[0].username, test_user.username);
        assert_eq!(users[0].email, test_user.email);
    });

    // without token
    let req = MockRequest::new(Get, "/users");
    run_test!(&rocket, req, |mut response: Response| {
        let body = body_string!(response);
        let err: Error = serde_json::from_str(&body).unwrap();
        assert_eq!(err.code, Status::Unauthorized.code);
        assert_eq!(err.msg, "token not found");
    });

    // invalid token
    let wrong_token = Header::new("Authorization", "Bearer wrongtoken");
    let mut req = MockRequest::new(Get, "/users");
    req.add_header(wrong_token);
    run_test!(&rocket, req, |mut response: Response| {
        let body = body_string!(response);
        let err: Error = serde_json::from_str(&body).unwrap();
        assert_eq!(err.code, Status::Unauthorized.code);
        assert_eq!(err.msg, "invalid token");
    });

    // normal user token
    let auth_token = testdata::normal_user_auth_token(test_user.id, &test_user.username);
    let auth_header = Header::new("Authorization", "Bearer ".to_string() + &auth_token);
    let mut req = MockRequest::new(Get, "/users");
    req.add_header(auth_header);
    run_test!(&rocket, req, |mut response: Response| {
        let body = body_string!(response);
        let err: Error = serde_json::from_str(&body).unwrap();
        assert_eq!(err.code, Status::Forbidden.code);
        assert_eq!(err.msg, "permission denied");
    });
}