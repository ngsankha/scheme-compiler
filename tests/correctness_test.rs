extern crate glob;
extern crate language;
extern crate interpreter;

use std::str;
use std::fs;
use std::process::Command;
use glob::glob;

#[test]
fn compile_interp_consistent() {
  for entry in glob("tests/*.rkt").unwrap() {
    let src = fs::canonicalize(entry.unwrap()).unwrap().display().to_string();
    let ast = language::parse_from_file(&src);
    let mut child = Command::new("cargo")
      .current_dir("runtime")
      .arg("build")
      .env("SRC", &src)
      .spawn().expect("failed to build binary");
    let ecode = child.wait().expect("failed to wait on compilation");
    assert!(ecode.success());
    let output = Command::new("runtime/target/debug/runtime").output().expect("failed to execute process");
    let cout = str::from_utf8(&output.stdout).unwrap();
    let iout = interpreter::to_human_str(interpreter::interp(ast));
    assert_eq!(iout, cout);
  }
}
