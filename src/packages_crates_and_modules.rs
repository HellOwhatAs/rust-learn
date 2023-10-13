pub mod modules;
pub mod paths;

fn modules() {
    use modules::fibonacci::fib;
    println!("{}", fib(186 as u128));
}

fn paths() {
    {
        use crate::packages_crates_and_modules::modules::fibonacci::fib as func;
        println!("{}", func(75 as u64));
    }
    {
        use paths::fib_from_super;
        println!("{}", fib_from_super::fib(100 as u128));
    }
}

fn uses() {
    use std::collections::*;
    use std::any::type_name;
    println!("{}", type_name::<HashMap<(i32, i32), u128>>())
}

#[allow(dead_code)]
pub fn main() {
    modules();
    paths();
    uses();
}