use actix_web::{HttpResponse, web, Result, error::Error};
use serde::{Deserialize, Serialize};
use serde_json::{Value};


#[derive(Deserialize, Debug, Serialize)]
pub struct JsonResponse {
    pub code: i32,
    pub message: String,
    pub success: bool,
    pub data: Option<Value>,
}


impl JsonResponse {
    pub async fn ok() -> Result<HttpResponse, Error> {
        Ok(HttpResponse::Ok().json(web::Json(JsonResponse {
            code: 200,
            success: true,
            message: "操作成功!".to_string(),
            data: None,
        })))
    }

    pub async fn error(code: i32, message: String) -> Result<HttpResponse, Error> {
        Ok(HttpResponse::Ok().json(web::Json(JsonResponse {
            code,
            success: false,
            message,
            data: None,

        })))
    }

    pub async fn ok_with_data(data: Value) -> Result<HttpResponse, Error> {
        Ok(HttpResponse::Ok().json(web::Json(JsonResponse {
            code: 200,
            success: true,
            data: Some(data),
            message: "success".to_string(),
        })))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_response() {
        let resp = JsonResponse::ok();
    }
}
