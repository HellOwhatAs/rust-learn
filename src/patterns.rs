fn all_places() {
    {
        enum W {
            A(i32),
            B(i64)
        }
        fn type_name<T>(_x: &T) -> &'static str {
            std::any::type_name::<T>()
        }
        let mut a = W::A(10);
        fn func(x: &W) {
            if let W::A(x) = x {
                println!("{} {}", type_name(x), x);
            }
            else if let W::B(x) = x {
                println!("{} {}", type_name(x), x);
            }
        }
        func(&a);
        a = match a {
            W::A(x) => W::B(x as i64 + i32::MAX as i64),
            W::B(x) => W::B(x),
        };
        func(&a);
    }
    {
        let mut stack = vec![1, 2, 3];
        while let Some(top) = stack.pop() {
            print!("{}, ", top);
        }
        println!();
    }
    {
        let s = String::from("HellOwhatAs");
        let mut d = std::collections::HashMap::new();
        for (idx, val) in s.chars().enumerate() {
            d.entry(val).and_modify(|x: &mut Vec<usize>| x.push(idx)).or_insert(vec![idx]);
        }
        println!("{:?}", d);
    }
}

fn pattern_syntax() {
    {
        let x = 20;
        match &x {
            0 | 1 => println!("A"),
            21.. => println!("B"),
            _ => println!("C")
        }
    }
    {
        struct Point<T> {
            x: T,
            y: T
        }
        let mut val = Point {x: 20, y: 20};
        let Point {x,  y: _ } = &mut val;
        *x += 20;
        println!("{}, {}", val.x, val.y);
    }
    {
        let nums = (1, 2, 3, 4, 5, 6);
        let (first, .., last) = nums;
        println!("{}, .., {}", first, last);
    }
    {
        let x = Some(235);
        let y = match &x {
            Some(val) if val & 1 == 0 => "A",
            Some(_) => "B",
            None => "C"
        };
        println!("{y}");
    }
    {
        let a = Some(20);
        if let Some(x @ 21..) = &a {
            println!("{x} in 21..");
        }
        else if let Some(x) = &a {
            println!("{x} not in 21..");
        }
        else {
            println!("None...");
        }
    }
}

#[allow(dead_code)]
pub fn main() {
    all_places();
    pattern_syntax();
}