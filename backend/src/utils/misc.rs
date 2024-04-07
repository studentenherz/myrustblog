use rand::{distributions::Alphanumeric, Rng};

use crate::database::DBHandler;

pub fn generate_random_alphanumeric_str(len: usize) -> String {
    let rng = rand::thread_rng();
    rng.sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}

fn title_to_slug(title: &str) -> String {
    title
        .to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
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
