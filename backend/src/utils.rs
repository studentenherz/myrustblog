use rand::{distributions::Alphanumeric, Rng};

pub fn generate_random_alphanumeric_str(len: usize) -> String {
    let rng = rand::thread_rng();
    rng.sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}
