use std::collections::HashMap;
use std::any::Any;

enum Method {
    GET, POST,
}

struct Response {
    status_code: u16,
    body: String,
}

trait HTTPClient {
    fn do_request(
        url: &str,
        method: Method,
        body: Option<HashMap<String,Box<dyn Any>>>
    ) -> Response;
}

pub struct Client {}

impl Client {
    pub const fn new() -> Self {
        Client {}
    }
}

impl HTTPClient for Client {

    fn do_request(
        url: &str,
        method: Method,
        body: Option<HashMap<String, Box<dyn Any>>>
    ) -> Response {
        Response {
            status_code: 200,
            body: String::from("body")
        }
    }
}
