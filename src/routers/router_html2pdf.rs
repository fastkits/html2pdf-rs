


use crate::vo::vo_html2pdf::VoHtml2pdf;
use crate::html2pdf::html::h2pdf_test;
use actix_web::web::Bytes;
use actix_web::{Responder, web, post, Result, HttpResponse};
use serde_json::json;
use crate::infra::resp::JsonResponse;
use log::info;

/// html转pdf
/// POST
/// /html2pdf
#[post("/html2pdf")]
async fn html2pdf_post(vo: web::Json<VoHtml2pdf>) -> impl Responder {
    info!("接收参数: {:?}", vo);
    let res = h2pdf_test(vo).await;
    match res {
        Ok(r) => {
            HttpResponse::Ok()
                .content_type("application/octet-stream")
                .append_header(("Content-Disposition", "attachment; filename=report.pdf"))
                .body(Bytes::from(r))
        }
        Err(e) => {
            // JsonResponse::error(-1, e.to_string()).await
            HttpResponse::Ok().json(web::Json(JsonResponse {
                code: 30000,
                success: false,
                message: e.to_string(),
                data: None,
            }))
        }
    }
}
