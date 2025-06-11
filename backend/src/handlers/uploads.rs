use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use actix_web::{web, HttpResponse, Responder};
use bson::uuid::Uuid;

use common::UploadResponse;

use crate::database::DBHandler;
use crate::Config;

#[derive(MultipartForm, Debug)]
pub struct UploadForm {
    #[multipart(limit = "3MB")]
    file: TempFile,
}

pub async fn upload<T: DBHandler>(
    MultipartForm(form): MultipartForm<UploadForm>,
    config: web::Data<Config>,
    db_handler: web::Data<T>,
) -> impl Responder {
    let file = form.file;
    let filename = format!(
        "{}{}",
        Uuid::new(),
        file.file_name.unwrap_or(String::from(".tmp"))
    );

    if let Ok((_, path)) = file.file.keep() {
        if db_handler.create_temp_file(&path, &filename).await.is_ok() {
            return HttpResponse::Ok().json(UploadResponse {
                parent_path: config.FILE_UPLOAD_URL.clone(),
                filename,
            });
        }
    };

    HttpResponse::BadRequest().finish()
}
