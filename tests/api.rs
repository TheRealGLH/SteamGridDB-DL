#[cfg(test)]
mod tests {
    use steamgriddb_dl::connectors::http::*;

    #[test]
    fn collection_info_http_request_from_id() {
        let id = "123";
        let request = HttpRequest::collection_info_request(&id);
        match request {
            Ok(request) => {
                assert!(
                    matches!(request.method, HttpRequestMethod::GET),
                    "Method is not GET"
                );
                assert!(request.headers.len() < 1, "Headers are not empty");
                assert_eq!(
                    "https://www.steamgriddb.com/api/public/collection/".to_string() + id,
                    request.url
                );
            }
            Err(e) => panic!("{e}"),
        };
    }

    #[test]
    fn collection_info_http_request_without_id() {
        let id = "";
        let request = HttpRequest::collection_info_request(&id);

        assert!(
            request.is_err(),
            "Request was formed, while it shouldn't have been."
        );
    }
    #[test]
    fn hero_info_http_request_from_id() {
        let id = "123";
        let request = HttpRequest::hero_info_request(&id);
        match request {
            Ok(request) => {
                assert!(
                    matches!(request.method, HttpRequestMethod::GET),
                    "Method is not GET"
                );
                assert!(request.headers.len() < 1, "Headers are not empty");
                assert_eq!(
                    "https://www.steamgriddb.com/api/public/asset/hero/".to_string() + id,
                    request.url
                );
            }
            Err(e) => panic!("{e}"),
        };
    }
    #[test]
    fn hero_info_http_request_without_id() {
        let id = "";
        let request = HttpRequest::hero_info_request(&id);

        assert!(
            request.is_err(),
            "Request was formed, while it shouldn't have been."
        );
    }
    #[test]
    fn grid_info_http_request_from_id() {
        let id = "123";
        let request = HttpRequest::grid_info_request(&id);
        match request {
            Ok(request) => {
                assert!(
                    matches!(request.method, HttpRequestMethod::GET),
                    "Method is not GET"
                );
                assert!(request.headers.len() < 1, "Headers are not empty");
                assert_eq!(
                    "https://www.steamgriddb.com/api/public/asset/grid/".to_string() + id,
                    request.url
                );
            }
            Err(e) => panic!("{e}"),
        };
    }
    #[test]
    fn grid_info_http_request_without_id() {
        let id = "";
        let request = HttpRequest::grid_info_request(&id);

        assert!(
            request.is_err(),
            "Request was formed, while it shouldn't have been."
        );
    }
    #[test]
    fn logo_info_http_request_from_id() {
        let id = "123";
        let request = HttpRequest::logo_info_request(&id);
        match request {
            Ok(request) => {
                assert!(
                    matches!(request.method, HttpRequestMethod::GET),
                    "Method is not GET"
                );
                assert!(request.headers.len() < 1, "Headers are not empty");
                assert_eq!(
                    "https://www.steamgriddb.com/api/public/asset/logo/".to_string() + id,
                    request.url
                );
            }
            Err(e) => panic!("{e}"),
        };
    }
    #[test]
    fn logo_info_http_request_without_id() {
        let id = "";
        let request = HttpRequest::logo_info_request(&id);

        assert!(
            request.is_err(),
            "Request was formed, while it shouldn't have been."
        );
    }
    #[test]
    fn icon_info_http_request_from_id() {
        let id = "123";
        let request = HttpRequest::icon_info_request(&id);
        match request {
            Ok(request) => {
                assert!(
                    matches!(request.method, HttpRequestMethod::GET),
                    "Method is not GET"
                );
                assert!(request.headers.len() < 1, "Headers are not empty");
                assert_eq!(
                    "https://www.steamgriddb.com/api/public/asset/icon/".to_string() + id,
                    request.url
                );
            }
            Err(e) => panic!("{e}"),
        };
    }
    #[test]
    fn icon_info_http_request_without_id() {
        let id = "";
        let request = HttpRequest::icon_info_request(&id);

        assert!(
            request.is_err(),
            "Request was formed, while it shouldn't have been."
        );
    }
}
