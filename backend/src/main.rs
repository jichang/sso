extern crate dotenv;
extern crate backend;

use dotenv::dotenv;

fn main() {
    dotenv().ok();

    backend::create().launch();
}
