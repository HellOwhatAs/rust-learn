fn hygienes() {
    let local;
    macro_rules! use_local {
        () => { local = 42; }
    }
    use_local!();
    println!("{}", local);
}

#[allow(dead_code)]
pub fn main() {
    hygienes();
}