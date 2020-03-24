use std::io::Cursor;

use rocket::http::{ContentType};
use rocket::request::Request;
use rocket::response::{self, Responder, Response};
use crate::types::ManifestHistory;

use serde_json;

impl<'r> Responder<'r> for ManifestHistory {
    fn respond_to(self, _req: &Request) -> response::Result<'r> {

        let json = serde_json::to_string(&self).unwrap();

        Response::build()
            .header(ContentType::JSON)
            .sized_body(Cursor::new(json))
            .ok()
    }
}
