fn unsafe_rust() {
    {
        let mut num: [i128; 7] = [1, 2, 3, 4, 5, 6, 7];
        let mut p = num.as_mut_ptr(); // or (&mut num as *mut i128);
        for _ in 0..=num.len() {
            unsafe {
                println!("{:?} {}", p, *p);
                *p = *p * *p;
                p = p.wrapping_add(1);
            }
        }
        println!("[{}]", num.iter().map(i128::to_string).collect::<Vec<String>>().join(", "));
    }
    {
        unsafe fn dangerous() {}
        unsafe { dangerous() };
    }
    {
        let mut a = ['H', 'e', 'l', 'l', 'o', ' ', 'W', 'o', 'r', 'l', 'd', '!'];
        fn split_at(arr: &mut [char], idx: usize) -> Option<(&mut [char], &mut [char], &mut [char])> {
            if idx >= arr.len() {
                return None;
            }
            return Some(unsafe {(
                std::slice::from_raw_parts_mut(arr.as_mut_ptr(), 5),
                std::slice::from_raw_parts_mut(arr.as_mut_ptr().add(5), 1),
                std::slice::from_raw_parts_mut(arr.as_mut_ptr().add(6), 6)
            )});
        }
        println!("{}", 
            match split_at(&mut a, 6).unwrap() {
                (a, b, c) => c.iter().map(char::to_string).collect::<Vec<String>>().join("") + 
                            &b.iter().map(char::to_string).collect::<Vec<String>>().join("") + 
                            &a.iter().map(char::to_string).collect::<Vec<String>>().join("")
            }
        );
    }
    {
        let (mut a, b) = (100, 10);
        println!("{}", b);
        {
            let ptr = unsafe { std::slice::from_raw_parts_mut(&mut a as *mut i32, 2) };
            ptr[0] += 20;
            let tmp = &mut a;
            *tmp += 20;
            println!("{}, {}", ptr[0], a);
            println!("{:?}", ptr);
            ptr.sort();
            println!("{:?}", ptr);
            println!("{}, {}", ptr[0], a);
        }
        println!("{}", b);
    }
    {
        extern "C" {
            fn rand() -> i32;
            fn srand(seed: u32);
            fn time(ptr: u32) -> u32;
        }
        unsafe {
            let seed = time(0);
            println!("seed = {seed}");
            srand(seed);
            for _ in 0..100 {
                print!("{} ", rand());
            }
            println!();
        }
    }
    {
        static mut X: Vec<i32> = Vec::new();
        unsafe { X.push(75) };
        println!("{:?}", unsafe { X.clone() });
    }
}

fn advanced_traits() {

}

#[allow(dead_code)]
pub fn main() {
    unsafe_rust();
    advanced_traits();
}