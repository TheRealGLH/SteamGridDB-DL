#[cfg(test)]
mod bodies {
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
                    "https://www.steamgriddb.com/api/public/collection/".to_string() + id + "/home",
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

    #[test]
    fn game_info_http_request_from_id() {
        let id = "123";
        let request = HttpRequest::game_info_request(&id);
        let expected_acc_key = "Accept";
        let expected_acc_val = "application/json, text/plain, */*";
        let expected_ref_key = "Referer";
        let expected_ref_val = "https://www.steamgriddb.com/";
        match request {
            Ok(request) => {
                assert!(
                    matches!(request.method, HttpRequestMethod::GET),
                    "Method is not GET"
                );
                let ref_header = request.headers.get(0).unwrap();
                let acc_header = request.headers.get(1).unwrap();
                assert_eq!(expected_ref_key, ref_header.key);
                assert_eq!(expected_ref_val, ref_header.value);
                assert_eq!(expected_acc_key, acc_header.key);
                assert_eq!(expected_acc_val, acc_header.value);
                assert_eq!(
                    "https://www.steamgriddb.com/api/public/game/".to_string() + id,
                    request.url
                );
            }
            Err(e) => panic!("{e}"),
        };
    }
    #[test]
    fn game_info_http_request_without_id() {
        let id = "";
        let request = HttpRequest::game_info_request(&id);

        assert!(
            request.is_err(),
            "Request was formed, while it shouldn't have been."
        );
    }
}

#[cfg(test)]
mod connections {
    use steamgriddb_dl::connectors::http::*;
    #[test]
    fn get_request_without_headers() {
        let expected_status: u16 = 200;
        let url = "https://jsonplaceholder.typicode.com/posts/1".to_string();
        let expected_body = "{
  \"userId\": 1,
  \"id\": 1,
  \"title\": \"sunt aut facere repellat provident occaecati excepturi optio reprehenderit\",
  \"body\": \"quia et suscipit\\nsuscipit recusandae consequuntur expedita et cum\\nreprehenderit molestiae ut ut quas totam\\nnostrum rerum est autem sunt rem eveniet architecto\"
}";
        let request: HttpRequest = HttpRequest {
            method: HttpRequestMethod::GET,
            url,
            headers: vec![],
        };
        let result = handle_get_request(request);
        match result {
            Ok(result) => {
                assert_eq!(expected_status, result.status());
                let result_body = result.into_string();
                assert!(result_body.is_ok());
                //"uhmmm this will panic if the body is None 🤓🤓"
                //yes, that's why it's in the test. in the intended flow the body is not meant to
                //be empty, thus a panic happens when the test is meant to fail regardless
                assert_eq!(expected_body, result_body.ok().unwrap());
            }
            Err(e) => panic!("HTTP request failed: {e}"),
        };
    }
    #[test]
    fn get_request_with_unresolvable_url() {
        let url = "coocoo231".to_string();
        let request: HttpRequest = HttpRequest {
            method: HttpRequestMethod::GET,
            url,
            headers: vec![],
        };
        let result = handle_get_request(request);
        assert!(result.is_err());
    }

    #[test]
    fn game_info_request_ok() {
        let request = HttpRequest::game_info_request("123");

        match request {
            Ok(r) => {
                match handle_get_request(r) {
                    Ok(response) => {
                        assert_eq!(response.status(), 200);
                        let response_body = response.into_string();
                        assert!(response_body.is_ok());
                        response_body.unwrap();
                    }
                    Err(e) => panic!("Http error: {e}"),
                };
            }
            Err(e) => panic!("Failed to create request object {e}"),
        };
    }
    #[test]
    fn game_info_request_without_header() {
        let request = HttpRequest::game_info_request("123");

        match request {
            Ok(mut r) => {
                r.headers = vec![];
                assert!(handle_get_request(r).is_err(), "HTTP request was successful, whereas it should not have been!");
            }
            Err(e) => panic!("Failed to create request object {e}"),
        };
    }
    #[test]
    fn hero_info_request_ok() {
        let request = HttpRequest::hero_info_request("123");

        match request {
            Ok(r) => {
                match handle_get_request(r) {
                    Ok(response) => {
                        assert_eq!(response.status(), 200);
                        let response_body = response.into_string();
                        assert!(response_body.is_ok());
                        response_body.unwrap();
                    }
                    Err(e) => panic!("Http error: {e}"),
                };
            }
            Err(e) => panic!("Failed to create request object {e}"),
        };
    }
    #[test]
    fn logo_info_request_ok() {
        let request = HttpRequest::logo_info_request("123");

        match request {
            Ok(r) => {
                match handle_get_request(r) {
                    Ok(response) => {
                        assert_eq!(response.status(), 200);
                        let response_body = response.into_string();
                        assert!(response_body.is_ok());
                        response_body.unwrap();
                    }
                    Err(e) => panic!("Http error: {e}"),
                };
            }
            Err(e) => panic!("Failed to create request object {e}"),
        };
    }
}
