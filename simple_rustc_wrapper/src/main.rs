use std::{env, process::Command};

fn main() {
    let args = env::args().skip(2);

    Command::new("rustc").args(args).status().unwrap();
}
