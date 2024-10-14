use catr::Config;

fn main() {
    if let Err(e) = Config::get_args().and_then(|config| config.run()) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
