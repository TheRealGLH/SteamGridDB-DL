use ureq::{get, Error, Response};

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
                url: "https://www.steamgriddb.com/api/public/collection/".to_string() + id + "/home",
                headers: vec![],
            })
        }
    }

    pub fn hero_info_request(id: &str) -> Result<Self, &'static str> {
        if id.len() < 1 {
            Err("Invalid ID supplied")
        } else {
            Ok(HttpRequest {
                method: HttpRequestMethod::GET,
                url: "https://www.steamgriddb.com/api/public/asset/hero/".to_string() + id,
                headers: vec![],
            })
        }
    }

    pub fn logo_info_request(id: &str) -> Result<Self, &'static str> {
        if id.len() < 1 {
            Err("Invalid ID supplied")
        } else {
            Ok(HttpRequest {
                method: HttpRequestMethod::GET,
                url: "https://www.steamgriddb.com/api/public/asset/logo/".to_string() + id,
                headers: vec![],
            })
        }
    }

    pub fn grid_info_request(id: &str) -> Result<Self, &'static str> {
        if id.len() < 1 {
            Err("Invalid ID supplied")
        } else {
            Ok(HttpRequest {
                method: HttpRequestMethod::GET,
                url: "https://www.steamgriddb.com/api/public/asset/grid/".to_string() + id,
                headers: vec![],
            })
        }
    }

    pub fn icon_info_request(id: &str) -> Result<Self, &'static str> {
        if id.len() < 1 {
            Err("Invalid ID supplied")
        } else {
            Ok(HttpRequest {
                method: HttpRequestMethod::GET,
                url: "https://www.steamgriddb.com/api/public/asset/icon/".to_string() + id,
                headers: vec![],
            })
        }
    }

    pub fn game_info_request(id: &str) -> Result<Self, &'static str> {
        if id.len() < 1 {
            Err("Invalid ID supplied")
        } else {
            Ok(HttpRequest {
                method: HttpRequestMethod::GET,
                url: "https://www.steamgriddb.com/api/public/game/".to_string() + id,
                headers: vec![
                    HttpHeader {
                        key: "Referer".to_string(),
                        value: "https://www.steamgriddb.com/".to_string(),
                    },
                    HttpHeader {
                        key: "Accept".to_string(),
                        value: "application/json, text/plain, */*".to_string(),
                    },
                ],
            })
        }
    }
}

pub fn handle_get_request(request: HttpRequest) -> Result<Response, Error> {
    let mut req = get(&request.url);
    for header in request.headers.iter() {
        req = req.set(&header.key, &header.value);
    }
    req.call()
}

pub struct HttpHeader {
    pub key: String,
    pub value: String,
}

pub enum HttpRequestMethod {
    GET,
    POST,
    DELETE,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PUT,
    PATCH,
}
