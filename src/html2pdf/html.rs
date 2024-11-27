use std::fs::File;
use std::io::Write;
use std::error::Error;
use actix_web::web;
use headless_chrome::{Browser, LaunchOptions};
use headless_chrome::browser::default_executable;
use anyhow::{anyhow, Result};
use crate::vo::vo_html2pdf::VoHtml2pdf;
use log::info;

pub fn save_vec_to_file(file_path: &str, data: Vec<u8>) -> std::io::Result<()> {
    // 打开文件，如果文件不存在则创建
    let mut file = File::create(file_path)?;
    // 写入 Vec<u8> 数据到文件
    file.write_all(&data.clone())?;
    Ok(())
}

pub async fn h2pdf_test(vo: web::Json<VoHtml2pdf>) -> Result<Vec<u8>, Box<dyn Error>> {

    let ele: Option<&str>;
    if let Some(e) = &vo.wait_ele {
        ele = Some(e.as_str())
    } else {
        ele = None;
    }

    let res = html2pdf(vo.url.as_str(), ele).await;
    match res {
        // Ok(r) => { Ok(base64::engine::general_purpose::STANDARD.encode(r)) }
        Ok(r) => { Ok(r) }
        Err(e) => {
            return Err(e);
        }
    }
}


pub async fn html2pdf(url: &str, ele: Option<&str>) -> Result<Vec<u8>, Box<dyn Error>> {
    info!("---->url: {}", url);
    info!("---->ele: {:?}", ele);
    let launch_options = LaunchOptions::default_builder()
        .ignore_certificate_errors(false)
        .sandbox(false)
        .path(Some(default_executable().map_err(|e| anyhow!(e))?))
        .build()?;
    let browser = Browser::new(launch_options)?;
    let tab = browser.new_tab()?;
    tab.set_default_timeout(std::time::Duration::from_secs(300));
    tab.reload(true, None)?;
    tab.navigate_to(url)?;
    tab.wait_until_navigated()?;
    if let Some(e) = ele {
        info!("等待元素: {}", e);
        let res = tab.wait_for_elements(e);
        match res {
            Ok(e) => {
                info!("找到元素: {:?}", e)
            }
            Err(e) => {
                info!("未找到元素: {:?}", e)
            }
        }
    };
    let html2 = tab.get_document()?;
    info!("----------> html: {:?}", html2);
    let c = tab.get_cookies()?;
    info!("-----cookie: {:?}", c);
    let viewport = tab.print_to_pdf(None)?;
    // let _ = save_vec_to_file("hhhh3.pdf", viewport.clone());
    Ok(viewport)
}
