mod rs_nano;

fn main() {
    rs_nano::init();
    rs_nano::run();
    rs_nano::cleanup();
    std::process::exit(0);
}
