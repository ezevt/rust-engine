pub use log::*;

pub fn init() {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
}
