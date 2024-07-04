pub fn init_logger() {
    use std::env;

    // Set default log level if not already set
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }

    env_logger::init();
}
