mod cli_error;
mod commands;

fn main() {
    commands::exec().unwrap();
}
