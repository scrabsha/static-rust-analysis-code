#![feature(rustc_private)]

extern crate rustc_ast;
extern crate rustc_driver;
extern crate rustc_hir;
extern crate rustc_interface;
extern crate rustc_middle;

use std::{env, iter, process::Command};

use rustc_driver::{Callbacks, Compilation, RunCompiler};
use rustc_hir::{def_id::DefId, ItemId, ItemKind, Mod};
use rustc_middle::ty::{TyCtxt, Visibility};

fn main() {
    if must_compile_target_crate() {
        run_instrumented_compiler()
    } else {
        run_standard_compiler()
    }
}

fn must_compile_target_crate() -> bool {
    let current_crate_name = get_current_crate();
    let target_crate_name = get_target_crate();

    current_crate_name == target_crate_name
}

fn get_current_crate() -> String {
    env::args().nth(3).unwrap()
}

fn get_target_crate() -> String {
    env::var("TARGET_CRATE").unwrap()
}

fn run_standard_compiler() {
    let args = env::args()
        .skip(1)
        .chain(iter::once(get_sysroot_arg()))
        .collect::<Vec<_>>();

    RunCompiler::new(args.as_slice(), &mut StandardCompiler)
        .run()
        .unwrap();
}

struct StandardCompiler;

impl Callbacks for StandardCompiler {}

fn run_instrumented_compiler() {
    let args = env::args()
        .skip(1)
        .chain(iter::once(get_sysroot_arg()))
        .collect::<Vec<_>>();

    let mut instrumented_compiler = InstrumentedCompiler::new();

    RunCompiler::new(args.as_slice(), &mut instrumented_compiler)
        .run()
        .unwrap();

    let functions = instrumented_compiler.public_fns();

    functions.iter().for_each(|f| println!("{}", f));
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

struct InstrumentedCompiler {
    discovered_fns: Vec<String>,
}

impl InstrumentedCompiler {
    fn new() -> InstrumentedCompiler {
        InstrumentedCompiler {
            discovered_fns: Vec::new(),
        }
    }

    fn public_fns(&self) -> &[String] {
        self.discovered_fns.as_slice()
    }

    fn add_pub_fns(&mut self, tcx: TyCtxt) {
        let root_module = tcx.hir().krate().module();
        self.visit_pub_mod(&tcx, root_module);
    }

    fn visit_pub_mod(&mut self, tcx: &TyCtxt, mod_: &Mod) {
        mod_.item_ids
            .iter()
            .filter(|item| tcx.visibility(item.def_id.to_def_id()) == Visibility::Public)
            .for_each(|item| self.visit_pub_item(tcx, *item))
    }

    fn visit_pub_item(&mut self, tcx: &TyCtxt, item: ItemId) {
        let item = tcx.hir().item(item);

        match &item.kind {
            ItemKind::Fn(_, _, _) => self.add_fn(tcx, item.def_id.to_def_id()),
            ItemKind::Mod(mod_) => self.visit_pub_mod(tcx, &mod_),
            _ => {}
        }
    }

    fn add_fn(&mut self, tcx: &TyCtxt, item: DefId) {
        self.discovered_fns.push(tcx.def_path_str(item))
    }
}

impl Callbacks for InstrumentedCompiler {
    fn after_analysis<'tcx>(
        &mut self,
        _compiler: &rustc_interface::interface::Compiler,
        queries: &'tcx rustc_interface::Queries<'tcx>,
    ) -> Compilation {
        queries.global_ctxt().unwrap().take().enter(|tcx| {
            self.add_pub_fns(tcx);
        });

        // We don't need the compiler to actually generate any code.
        Compilation::Stop
    }
}

pub fn foo() {}

pub mod pub_mod {
    pub fn bar() {}
}

mod non_pub_mod {
    pub fn bar() {}
}
