extern "C" {
    fn entry() -> u64;
}

fn main() {
    let result = unsafe { entry() };
    println!("{}", result);
}
