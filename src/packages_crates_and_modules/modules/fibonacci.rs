pub fn fib<T>(x: T) -> T
where
    T: Copy
        + From<u8>
        + Into<u128>
        + PartialOrd
        + std::ops::Sub<Output = T>
        + std::ops::Add<Output = T>
        + std::ops::AddAssign
{
    let mut last1 = T::from(1);
    if x == T::from(1) {
        return last1;
    }
    let mut last2 = T::from(0);
    if x == T::from(0) {
        return last2;
    }
    let mut count = T::from(1);
    while x > count {
        let tmp = last1 + last2;
        last2 = last1;
        last1 = tmp;
        count += T::from(1);
    }
    return last1;
}
