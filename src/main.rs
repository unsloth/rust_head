fn main() {
    if let Err(e) = rust_head::run() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
