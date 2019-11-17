use super::user::User;
use ::chrono::{DateTime, Utc};
use rocket::http::RawStr;
use rocket::request::{Form, FromForm, FromFormValue};
use rocket::{get, post, routes};
use serde::{Deserialize, Serialize};
use serde_json::{from_str, Value};
#[derive(FromForm, Deserialize, Serialize)]
pub struct Comment {
    author: User,
    content: String,
    timestamp: String,
}

impl<'v> FromFormValue<'v> for Comment {
    type Error = serde_json::error::Error;

    fn from_form_value(form_value: &'v RawStr) -> Result<Comment, serde_json::error::Error> {
        let u: Comment = serde_json::from_str(form_value.as_str().into())?;
        Ok(u)
    }
}
