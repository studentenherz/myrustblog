use rand::{distr::Alphanumeric, Rng};

use crate::database::DBHandler;
use common::utils::title_to_slug;

pub fn generate_random_alphanumeric_str(len: usize) -> String {
    let rng = rand::rng();
    rng.sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}

pub async fn generate_unique_slug(db_handler: &impl DBHandler, title: &str) -> Result<String, ()> {
    let original_slug = title_to_slug(title);
    let mut slug = original_slug.clone();
    let mut counter = 1;

    loop {
        match db_handler.get_post(&slug).await {
            Ok(Some(_)) => {
                slug = format!("{}-{}", title_to_slug(title), counter);
                counter += 1;
            }
            Ok(None) => {
                return Ok(slug);
            }
            Err(_) => return Err(()),
        }
    }
}
