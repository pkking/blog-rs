use super::comment::Comment;
use rocket_contrib::json::Json;

use super::user::User;
use ::chrono::{DateTime, Utc};
use rocket::http::RawStr;
use rocket::request::{Form, FromForm, FromFormValue};
use rocket::{get, post, routes};
use serde::{Deserialize, Serialize};
use serde_json::{from_str, Value};
#[derive(FromForm, Deserialize, Serialize, Debug)]
pub struct Post {
    id: String,
    name: String,
    content: String,
    //author: User,
    //comment: Comment,
    timestamp: String,
}
/*
fn new(
    id: String,
    name: String,
    content: String,
    author: User,
    comment: Comment,
    timestamp: String,
) -> Post {
    Post {
        id,
        name,
        content,
        author,
        comment,
        timestamp,
    }
}*/
#[post("/post", data = "<p>")]
pub fn create_post(p: Form<Post>) -> String {
    format!("Hello, {:?}!", p)
}

#[get("/post")]
pub fn batch_query_post() -> String {
    format!(" batch query post")
}

#[get("/post/<id>")]
pub fn query_post(id: &RawStr) -> String {
    format!("query {} post", id.as_str())
}

#[post("/postjson", format = "json", data = "<p>")]
pub fn new_post(p: Json<Post>) -> String {
    format!("Hello, {:?}!", p)
}
impl<'v> FromFormValue<'v> for Post {
    type Error = serde_json::error::Error;

    fn from_form_value(form_value: &'v RawStr) -> Result<Post, serde_json::error::Error> {
        let u: Post = serde_json::from_str(form_value.as_str().into())?;
        Ok(u)
    }
}
