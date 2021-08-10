#![feature(rustc_private)]

extern crate rustc_driver;

use rustc_driver::{Callbacks, RunCompiler};

use std::{env, iter, process::Command};

struct StandardCompiler;

impl Callbacks for StandardCompiler {}

fn main() {
    let args = env::args()
        .skip(1)
        .chain(iter::once(get_sysroot_arg()))
        .collect::<Vec<_>>();

    RunCompiler::new(args.as_slice(), &mut StandardCompiler)
        .run()
        .unwrap();
}

fn get_sysroot_arg() -> String {
    let out = Command::new("rustc")
        .arg("+nightly")
        .arg("--print=sysroot")
        .current_dir(".")
        .output()
        .unwrap()
        .stdout;

    let out = String::from_utf8(out).unwrap();

    format!("--sysroot={}", out.trim())
}
