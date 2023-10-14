fn syntax() {
    struct Point<T> {
        x: T,
    }
    impl<T> Point<T> {
        fn x(&mut self) -> &mut T {
            &mut self.x
        }
    }
    impl Point<String> {
        fn push_str(&mut self, string: &str) {
            self.x.push_str(string)
        }
    }
    {
        let mut p = Point { x: 5 };
        println!("p.x = {}", p.x());
        *p.x() += 10;
        println!("p.x = {}", p.x());
    }
    {
        let mut p = Point { x: "HellO".to_string() };
        println!("p.x = {}", p.x());
        p.x().push_str("what");
        p.push_str("As");
        println!("p.x = {}", p.x());
    }
}

fn traits() {
    trait MyToString {
        fn my_to_string(&self) -> String;
    }
    impl<T: std::fmt::Display> MyToString for Vec<T> {
        fn my_to_string(&self) -> String {
            format!("vec![{}]", self.iter().map(|x|x.to_string()).collect::<Vec<String>>().join(", "))
        }
    }
    {
        fn func<T: MyToString>(x: &T) {
            println!("{}", x.my_to_string());
        }
        func(&vec![1, 2, 3, 4, 5]);
    }
    {
        fn func() -> impl MyToString {
            vec![1, 1, 0]
        }
        let res = func();
        println!("{}", res.my_to_string());
    }
}

/// # 生命周期省略规则
/// 1. 编译器为每一个引用参数都分配一个生命周期参数
/// 2. 如果只有一个输入生命周期参数，那么它被赋予所有输出生命周期参数
/// 3. 如果方法有多个输入生命周期参数并且其中一个参数是 &self 或 &mut self，说明是个对象的方法，那么所有输出生命周期参数被赋予 self 的生命周期  
fn lifetime() {
    {
        fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
            if a.len() > b.len() {a} else {b}
        }
        {
            let a = "Hello";
            let b = "whatAs";
            println!("{}", longest(&a, &b));
        }
        {
            // let string1 = String::from("long string is long");
            // let result;
            // {
            //     let string2 = String::from("xyz");
            //     result = longest(string1.as_str(), string2.as_str());
            // }
            // println!("The longest string is {}", result);
        }
    }
    {
        struct StrRef<'a> {
            s: &'a str
        }
        let raw_s = "HellOwhatAs";
        let s = StrRef{s: raw_s};
        println!("{}, {}", s.s, raw_s);

        impl<'a> StrRef<'a> {
            fn len(&'a self) -> usize {
                self.s.len()
            }
            fn longest(&'a self, other: &'a str) -> &'a str {
                if self.s.len() > other.len() {self.s} else {other}
            }
            fn return_other<'b>(&'a self, other: &'b str) -> &'b str {
                other
            }
        }
        println!(
            "{} {} {}",
            s.len(),
            s.longest("Shanghai Jiao Tong University (SJTU) was founded in Shanghai, China, in 1896,"),
            s.return_other("with the goal of cultivating talented professionals for the nation.")
        );
    }
}

#[allow(dead_code)]
pub fn main() {
    syntax();
    traits();
    lifetime();
}