#![feature(proc_macro_hygiene, decl_macro)]
mod comment;
mod post;
mod user;
use rocket::{get, post, routes};

use post::*;
fn main() {
    rocket::ignite()
        .mount(
            "/",
            routes![create_post, new_post, batch_query_post, query_post],
        )
        //.mount("/", routes![test])
        .launch();
}
