use rocket::http::RawStr;
use rocket::request::{Form, FromForm, FromFormValue};
use rocket::{get, post, routes};
use serde::{Deserialize, Serialize};
use serde_json::{from_str, Value};

#[derive(FromForm, Deserialize, Serialize, Debug)]
pub struct User {
    name: String,
    email: String,
    id: String,
}
impl<'v> FromFormValue<'v> for User {
    type Error = serde_json::error::Error;

    fn from_form_value(form_value: &'v RawStr) -> Result<User, serde_json::error::Error> {
        let u: User = serde_json::from_str(form_value.as_str().into())?;
        Ok(u)
    }
}
