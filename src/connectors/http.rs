pub struct HttpRequest {
    pub method: HttpRequestMethod,
    pub url: String,
    pub headers: Vec<HttpHeader>,
}

impl HttpRequest {
    pub fn collection_info_request(id: &str) -> Result<Self, &'static str> {
        if id.len() < 1 {
            Err("Invalid ID supplied")
        } else {
            Ok(HttpRequest {
                method: HttpRequestMethod::GET,
                url: "https://www.steamgriddb.com/api/public/collection/".to_string() + id,
                headers: vec![],
            })
        }
    }

    pub fn hero_info_request(id: &str) -> Result<Self, &'static str> {
        todo!();
    }

    pub fn logo_info_request(id: &str) -> Result<Self, &'static str> {
        todo!();
    }

    pub fn grid_info_request(id: &str) -> Result<Self, &'static str> {
        todo!();
    }

    pub fn icon_info_request(id: &str) -> Result<Self, &'static str> {
        todo!();
    }

    pub fn game_info_request(id: &str) -> Result<Self, &'static str> {
        todo!();
    }
}

pub fn handle_get_request(request: HttpRequest) -> String {
    todo!();
}

pub struct HttpHeader {
    pub key: String,
    pub value: String,
}

pub enum HttpRequestMethod {
    GET,
    POST,
    DELETE,
}
