use sao::{Fsm, Output};

fn main() {
    let mut fsm = Fsm::default();
    loop {
        match fsm.next_step() {
            Ok(Output::Continue) => continue,
            Ok(Output::Exit) => break,

            Err(err) => std::process::exit(fsm.handle_error(err)),
        }
    }
}
