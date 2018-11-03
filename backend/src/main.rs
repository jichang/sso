extern crate backend;

fn main() {
    let config_file = "./src/config.toml";
    backend::create(config_file).launch();
}
