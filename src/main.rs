use miro::run;

fn main() {
    if let Err(error) = run() {
        eprintln!("miro: {error:#}");
        std::process::exit(1);
    }
}
