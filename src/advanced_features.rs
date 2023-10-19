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
    {
        use std::ops::Add;
        {
            #[derive(Debug)]
            struct Point<T> {
                x: T,
                y: T
            }
            impl<T> Add<&Point<T>> for Point<T> 
                where T: for<'a> Add<&'a T, Output = T>
            {
                type Output = (T, T);
                fn add(self, rhs: &Point<T>) -> Self::Output {
                    (self.x + &rhs.x, self.y + &rhs.y)
                }
                
            }
            let a = Point { x: 20, y: 30 };
            let b = Point { x: -2, y: 99 };
            println!("{:?}", a + &b);
            println!("{:?}", b);
        }
        {
            #[derive(Debug)]
            struct A(i32);
            #[derive(Debug)]
            struct B(i32);
            impl Add<B> for A {
                type Output = A;
                fn add(self, rhs: B) -> Self::Output {
                    A(self.0 + rhs.0)
                }
            }
            impl Add<&B> for A {
                type Output = B;
                fn add(self, rhs: &B) -> Self::Output {
                    B(self.0 + rhs.0)
                }
            }
            let a = A(70) + B(5);
            let b = A(70) + &B(5);
            println!("{:?}, {:?}", a, b);
        }
    }
    {
        trait MyToString {
            fn to_string(&self) -> String {
                "Default MyToString".to_string()
            }
        }
        struct A(i32);
        impl std::fmt::Display for A {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "A({})", self.0)
            }
        }
        impl MyToString for A {
            fn to_string(&self) -> String {
                format!("[MyToString]A({})", self.0)
            }
        }
        impl A {
            fn to_string(&self) -> String {
                format!("[method]A({})", self.0)                
            }
        }
        let a = A(10);
        println!(
            "{}, {}, {}, {}",
            a,
            a.to_string(),
            MyToString::to_string(&a),
            ToString::to_string(&a)
        );
        println!("{}", <A as MyToString>::to_string(&a));
    }
    {
        trait AbsString: ToString + std::fmt::Display {
            fn to_absstring(&self) -> String {
                println!("{}", self);
                format!("|{}|", self.to_string())                
            }
        }
        impl AbsString for i32 {}
        println!("{}", 123.to_absstring());
    }
    {
        struct Wrapper<T>(Vec<T>);
        impl<T> std::ops::Deref for Wrapper<T> {
            type Target = Vec<T>;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
        impl<T> std::ops::DerefMut for Wrapper<T> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
        impl<T: std::fmt::Display> std::fmt::Display for Wrapper<T> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "[{}]", self.iter().map(T::to_string).collect::<Vec<String>>().join(", "))
            }
        }
        let mut a = Wrapper(vec![1, 2, 3, 4]);
        a.push(0);
        a.sort_by_key(|elem| i32::abs(3 - elem));
        println!("{}", a);
    }
}

fn advanced_types() {
    {
        type I32ptr = Option<std::rc::Rc<std::cell::RefCell<i32>>>;
        fn swap_i32ptr(a: &I32ptr, b: &I32ptr) {
            if let (Some(a1), Some(b1)) = (a.clone(), b.clone()) {
                let mut a2 = a1.borrow_mut();
                let mut b2 = b1.borrow_mut();
                std::mem::swap(&mut *a2, &mut *b2);
            }
        }
        let a = Some(std::rc::Rc::new(std::cell::RefCell::new(123)));
        let b = Some(std::rc::Rc::new(std::cell::RefCell::new(456)));
        println!("{:?}, {:?}", a, b);
        swap_i32ptr(&a, &b);
        println!("{:?}, {:?}", a, b);
    }
}

fn advanced_functions_and_closures() {

}

#[allow(dead_code)]
pub fn main() {
    unsafe_rust();
    advanced_traits();
    advanced_types();
    advanced_functions_and_closures();
}