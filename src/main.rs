mod args;
mod profile;
mod rule;

fn main() {
    let dofi: args::Dofi = argh::from_env();
    match dofi.run() {
        Ok(_) => std::process::exit(0),
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1)
        }
    }
}
