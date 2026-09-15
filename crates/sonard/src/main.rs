use sonard::{Fsm, NORMAL_EXIT_CODE, Output};

fn main() {
    let mut fsm = Fsm::new();
    loop {
        match fsm.next_step() {
            Ok(Output::Continue) => continue,
            Ok(Output::Exit) => std::process::exit(NORMAL_EXIT_CODE),

            Err(err) => std::process::exit(fsm.handle_error(err)),
        }
    }
}
