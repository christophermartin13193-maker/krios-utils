use sonard::{Automate, EXIT_CODE_STANDARD, Output};

fn main() {
    let mut automate = Automate::default();
    loop {
        match automate.run() {
            Ok(Output::Continue) => continue,
            Ok(Output::Exit) => std::process::exit(EXIT_CODE_STANDARD),

            Err(error) => std::process::exit(Automate::handle_error(error)),
        }
    }
}
