#![no_main]
use libfuzzer_sys::fuzz_target;

use rustpython::vm::compile::Mode;
use rustpython_vm::{
    builtins::{PyBaseException, PyCode},
    compile::CompileError,
    prelude::*,
};

fuzz_target!(|src: &str| {
    let _ = fuzz(0, src);
});

fn fuzz(
    opt: u8,
    src: &str,
) -> (
    Result<PyRef<PyCode>, CompileError>,
    Option<Result<PyObjectRef, PyRef<PyBaseException>>>,
) {
    let mut options: Settings = Default::default();
    options.interactive = false;
    options.optimize = opt;

    Interpreter::without_stdlib(options).enter(|vm| {
        for item in ["__loader__", "input", "print", "__import__", "open"] {
            vm.builtins.dict().del_item(item, vm).unwrap();
        }
        let scope = vm.new_scope_with_builtins();
        let a = vm.compile(src, Mode::Exec, "<fuzz>".to_owned());
        let b = if let Ok(code_obj) = &a {
            Some(vm.run_code_obj(code_obj.clone(), scope))
        } else {
            None
        };
        (a, b)
    })
}
