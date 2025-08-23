mod setup;
mod startup;

use std::path::Path;

fn main() {
    let config_path = "config.json";

    if Path::new(config_path).exists() {
        startup::run();
    } else {
        setup::run();
    }
}
