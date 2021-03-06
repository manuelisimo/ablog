use actix_web::{web, error, Error, Result};
use actix_files::NamedFile;
use serde::Deserialize;

use crate::db;

#[derive(Deserialize)]
pub struct ImageParams {
    id: i32,
}

pub async fn get_image(
    pool: web::Data<db::LitePool>,
    info: web::Path<ImageParams>,
) -> Result<NamedFile, Error> {
    let image = db::get_image_record(info.id, &pool)
        .map_err(|e| error::ErrorNotFound(e.to_string()))?;
    Ok(NamedFile::open(image.path)?)
}