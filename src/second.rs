pub fn fib(x: u128) -> u128 {
    let mut last1: u128 = 1;
    if x == 1 {return  last1;}
    let mut last2: u128 = 0;
    if x == 0 {return  last2;}
    let mut count: u128 = 1;
    while x > count {
        let tmp: u128 = &last1 + last2;
        last2 = last1;
        last1 = tmp;
        count += 1;
    }
    return last1;
}