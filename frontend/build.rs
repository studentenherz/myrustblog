use dotenv::dotenv;

fn main() {
    // println!("cargo:rerun-if-changed=.env");
    dotenv().ok();

    for (key, value) in std::env::vars() {
        if key.starts_with("APP_") {
            println!("cargo:rustc-env={}={}", key, value);
        }
    }
}
