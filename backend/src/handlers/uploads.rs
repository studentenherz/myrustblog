use std::fs;
use std::path::Path;

use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use actix_web::{web, HttpResponse, Responder};
use bson::uuid::Uuid;
use serde_json::json;

use crate::Config;

#[derive(MultipartForm, Debug)]
pub struct UploadForm {
    #[multipart(limit = "3MB")]
    file: TempFile,
}

pub async fn upload(
    MultipartForm(form): MultipartForm<UploadForm>,
    config: web::Data<Config>,
) -> impl Responder {
    let file = form.file;
    let name = file.file_name.unwrap_or_else(|| {
        let uuid = Uuid::new();
        format!("file-{}.tmp", uuid)
    });
    let file_path = Path::new(&config.FILE_UPLOAD_PATH).join(&name);

    match fs::copy(&file.file.path(), &file_path) {
        Ok(_) => HttpResponse::Ok().json(json!({
            "url": Path::new(&config.FILE_UPLOAD_URL)
                .join(&name)
                .to_str()
                .unwrap_or(""),
        })),
        Err(err) => {
            println!("Failed to persist file: {:?}", err);
            return HttpResponse::InternalServerError().finish();
        }
    }
}
