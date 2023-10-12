fn ownership() {
    let x = "Hello";
    let _y = x;
    println!("{}", x);
}

fn references_and_borrowing() {
    let mut s = "World!".to_owned();
    let s1 = &s;
    print!("{}, ", s1);
    let s2 = &mut s;
    println!("{}", s2);
}

fn slices() {
    let input = "Hello  World !!!";
    let result: Vec<&str> = input.split(' ').filter(|x| x.len() > 0).collect();
    println!("{}", result.get(0).unwrap_or(&"[Empty str]"));
    let mut s = 0..5;
    s.start += 1;
    println!("{}", &input[s]);
}

#[allow(dead_code)]
pub fn main() {
    ownership();
    references_and_borrowing();
    slices();
}