extern crate language;

use language::dtypes::Val;

extern "C" {
    fn entry() -> Val;
}

fn main() {
    let result = unsafe { entry() };
    println!("{}", result.to_string());
}
