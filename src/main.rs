use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Config {
    database_url: String,
    #[serde(default = "default_min_block_confirmations")]
    min_block_confirmations: u8,
}

/// provides default value for min_block_confirmations
/// if MIN_BLOCK_CONFIRMATIONS env var is not set
fn default_min_block_confirmations() -> u8 {
    100
}

fn main() {
    dotenvy::dotenv().ok();
    let config = envy::prefixed("APP_").from_env::<Config>().unwrap();
    println!("APP_DATABASE_URL {:?}", config.database_url);
    println!(
        "APP_MIN_BLOCK_CONFIRMATIONS {:?}",
        config.min_block_confirmations
    );
}
