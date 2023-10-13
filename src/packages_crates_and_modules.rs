pub mod mylibs;

pub fn main() {
    use mylibs::fibonacci::fib;
    println!("{}", fib(186 as u128));
}