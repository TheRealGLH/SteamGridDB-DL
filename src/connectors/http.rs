pub struct HttpRequest {
    method: HttpRequestMethod,
}

#[derive(Debug)]
struct HttpHeader {
    key: String,
    value: String,
}

#[derive(Debug)]
enum HttpRequestMethod {
    GET,
    POST,
    DELETE,
}


#[cfg(test)]
mod test {

    #[test]
    fn make_get_request() {
        //todo!();

    }
}
