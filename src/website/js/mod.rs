use std::fs;

use axum::http::{header::CONTENT_TYPE, Response, StatusCode};

pub fn get_js_file(file_name: &str) -> Result<Response<String>, (StatusCode, String)> {
    let mut res = Response::new(String::new());
    res.headers_mut()
        .insert(CONTENT_TYPE, "text/javascript".parse().unwrap());
    let file_name = format!("./src/website/js/{}", file_name);
    let body = match fs::read_to_string(&file_name) {
        Ok(contents) => contents,
        Err(e) => {
            println!("File Not Found: {}", file_name);
            return Err((StatusCode::NOT_FOUND, e.to_string()));
        }
    };
    res.body_mut().push_str(&body);
    Ok(res)
}
