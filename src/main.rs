pub mod config;
pub mod duration;

use crate::config::WaylaidConfig;

fn main() {
    let config = toml::from_str::<WaylaidConfig>(
        &std::fs::read_to_string("./sample_config.toml").expect("sample config not present"),
    );
    dbg!(config.unwrap());
}
