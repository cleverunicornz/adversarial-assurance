//! Thin process boundary for the `assurance` binary.

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let code = match assurance::cli::parse(&args).and_then(assurance::cli::run) {
        Ok(code) => code,
        Err(error) => {
            eprintln!("{error}");
            1
        }
    };
    std::process::exit(code);
}
