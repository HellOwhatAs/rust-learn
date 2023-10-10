use std::io::Read;

fn 输出到命令行() {
    let a = 123;
    println!("{{{}, {1}, {0}}}", a, 456);
}

fn 基础语法() {
    let a = 123;
    let mut b = 456;
    b *= 2;
    const C: i32 = 789;
    let b = b / 2;
    println!("a = {}, b = {}, c = {}", a, b, C);
}

fn 数据类型() {
    let a = 123_456;
    let b = 0b1001011;
    let c = 0o114514;
    let d = 0xABCDE;
    println!("a = {}, b = {}, c = {}, d = {}", a, b, c, d);
    let e: f64 = 2.71828;
    let pi: f32 = 3.1415926;
    println!("e = {}, pi = {}", e, pi);
    println!("a / b = {}, e / pi = {}", a / b, e / (pi as f64));

    {
        let a = "123";
        fn get(s: &str){
            fn get2(s: &str){
                println!("{} in get2", s);
            }
            get2(s);
        }
        get(a);
        println!("{a}");
    }

    let mut a_tuple = (a, e);
    let a_array = [a, b, c, d];
    a_tuple.0 += 20;
    println!("({}, {})", a_tuple.0, a_tuple.1);
    let a = 0;
    println!("[{}, {}, {}, {}]", a_array[a], a_array[1], a_array[2], a_array[3]);
    let _array = vec![vec![0; 100]; 100];
}

/// 这是一个函数的注释（必须以 “///” 开头）
fn 注释() -> () {return ();}

fn 函数() {
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }
    println!("{}", add(114514, 1919810));
    fn func() -> &'static str {
        return "return val";
    }
    println!("{}", func());
    let block_val = {
        let a = 10010;
        a
    };
    println!("{}", block_val);
}

fn 条件语句(x: i32) {
    if x > 5 {
        println!("{x} > 5");
    }
    else if x < 0 {
        println!("{x} < 0");
    }
    else {
        println!("0 <= {x} <= 5");
    }
    println!("{}", if false {0} else if true {1} else {-1});
}

fn 循环() {
    let mut i = 0;
    while i < 100 {
        i = i + 1;
    }
    println!("{}", i);
    let mut arr = [1, 2, 3, 4, 5, 7, 9, 11, 13, 15];
    for i in arr {
        print!("{}, ", i);
    }
    println!();
    for i in 0..arr.len() {
        print!("[{}]{}, ", i, arr[i]);
        arr[i] *= 20;
    }
    println!();
    for sb in arr.iter() {
        print!("{}, ", sb);
    }
    println!();
    i = 0;
    let idx = loop {
        if i >= arr.len() {
            break -1;
        }
        if arr[i] > 10 * 20 {
            break i as i32;
        }
        i += 1;
    };
    println!("{idx}")
}

fn 所有权() {
    let mut a = vec![1, 0, 0, 1, 0, 1, 1];
    let b = &mut a;
    b[0] += 20;
    println!("{}", b[0]);
    println!("{}", a[0]);
    let s = String::from("薛家奇");
    let s2 = s;
    println!("{}", s2);
    let s3 = s2.clone();
    println!("{}, {}", s3, s2);
    {
        let a = vec![5, 3, 1, 2, 4, 8];
        let b = [1, 0, 0, 1, 0];
        fn func1(a: Vec<i32>) -> i32 {
            let mut ret = 0;
            for elem in a {
                ret += elem;
            }
            return ret;
        }
        fn fnuc2(b: [i32; 5]) -> i32 {
            let mut ret = 0;
            for elem in b {
                ret += elem;
            }
            ret
        }
        func1(a);
        fnuc2(b);
        // a;
        println!("{}", b[0]);
    }
    {
        fn func1() -> Vec<String> {
            let a = vec![String::from("value")];
            return a;
        }
        let b = func1();
        println!("{}", b[0]);
    }
    {
        let s1 = String::from("HellOwhatAs");
        let s2 = &s1;
        println!("{}, {}", s1, s2);

        let s1 = String::from("hello");
        let s2 = &s1;
        let s3 = s1.clone();
        println!("{}", s2);
        println!("{}", s3);
    }
    {
        let mut s1 = String::from("薛家奇");
        let s2 = &mut s1;
        s2.push_str(String::from("value").as_str());
        println!("{}", s1);
        let s2 = &mut s1;
        println!("{}", s2);
    }
    {
        let mut arr = vec![1, 2, 3, 4, 5];
        fn print_arr(arr: &mut Vec<i32>, idx: i32) {
            if idx as usize == arr.len() {
                println!();    
                return;
            }
            print!("{}, ", arr[idx as usize]);
            arr[idx as usize] += 1;
            print_arr(arr, idx + 1);

        }
        let b = &mut arr;
        print_arr(b, 0);
        print_arr(b, 0);
    }
}

fn 切片类型() {
    let s = String::from("HellOwhatAs");
    let p1 = &s[1..];
    println!("{}, {}", p1, s);

    let arr = [1, 2, 3, 4, 5, 6];
    let arr1 = &arr[1..3];
    for i in arr1 {
        print!("{}, ", i);
    }
    println!();

    let s_str = s.as_str();
    println!("{}", s_str);
}

fn 结构体() {
    struct Site {
        domain: String,
        name: String,
        nation: String,
        found: u32
    }
    impl Site{
        fn to_string(&self) -> String {
            "Site {\n    ".to_string() + 
                "domain: " + &self.domain + ",\n    " +
                "name: " + &self.name + ",\n    " +
                "nation: " + &self.nation + ",\n    " +
                "found: " + &self.found.to_string() + "\n}"
        }
        fn add(a: i32, b: i32) -> i32 {
            return a + b;
        }
    }
    let mut my_site = Site {
        domain: String::from("hellowhatas.github.io"),
        name: String::from("from mp3"),
        nation: String::from("CN"),
        found: 2021
    };
    let my_site2 = &mut my_site;
    my_site2.name = "129".into();
    println!("{}", my_site2.to_string());
    println!("{}", my_site.to_string());
    println!("1 + 2 = {}", Site::add(1, 2));
}

fn 枚举类() {
    enum Book {
        Paper { id: i32 },
        Electronic(String)
    }
    impl Book {
        fn to_string(&self) -> String {
            match &self {
                Book::Paper {id} => "Paper(".to_string() + id.to_string().as_str() + ")",
                Book::Electronic(s) => "Electronic(".to_string() + &s + ")",
            }
        }
    }
    let book = Book::Paper{id: 10010};
    let book2 = Book::Electronic("https://myurl".to_string());
    println!("{}, {}", book.to_string(), book2.to_string());
    {
        let opt: Option<&str> = Some("HellOwhatAs");
        let res: Option<String> = match opt {
            Some(s) => {println!("{}", s); None},
            None => {println!("None"); None}
        };
        println!("{}", res.unwrap_or("None".to_string()));
        match opt {
            Some("123") => println!("Some(123)"),
            None => println!("None"),
            _ => println!("Some({})", &opt.unwrap())
        }
        if let Some("1234") = opt {
            println!("1234");
        }
        else {
            println!("Not Some(1234)");
        }
    }
}

mod second;
mod third{
    pub fn fib(x: i32) -> i32 {
        if x < 2 {x} else {fib(x - 1) + fib(x - 2)}
    }
    pub use std::f64::consts::PI;
}

fn 组织管理() {
    use second::fib;
    use third::fib as rfib;
    println!("{}", fib(10));
    println!("{}", rfib(10));
    println!("{}", f64::cos(third::PI));
}

fn 错误处理() {
    if false {
        panic!("Never {}{}{}{}", 'H', 'e', 'r', 'e');
    }
    let f = std::fs::File::open("Cargo.toml");
    let mut buf = String::new();
    match f {
        Ok(mut f) => {
            f.read_to_string(&mut buf).expect("Failed to Read");
        },
        Err(e) => {
            println!("Failed to Open: {}", e);
            return;
        }
    }
    let lines: Vec<&str> = buf.split('\n').collect();
    for line in lines {
        let line = line.trim();
        if line.len() > 0 {
            println!("{}", "\"".to_string() + &line.replace("\"", "\\\"") + "\"");
        }
    }
    {
        fn error_happen() -> Result<i32, String> {
            Err("Error".to_string())
        }
        fn error_recver() -> Result<i64, String> {
            error_happen()?; // 可以在 Result 对象后添加 ? 操作符将同类的 Err 直接return出去
            println!("Ok(20)");
            Ok(20)
        }
        if let Err(msg) = error_recver() {
            println!("{}", msg);
        }
    }
    // kind
    {
        fn read_text_from_file(path: &str) -> Result<String, std::io::Error> {
            let mut f = std::fs::File::open(path)?;
            let mut s = String::new();
            f.read_to_string(&mut s)?;
            Ok(s)
        }
        fn func() {
            let str_file = read_text_from_file("hello.txt");
            match str_file {
                Ok(s) => println!("{}", s),
                Err(e) => {
                    match e.kind() {
                        std::io::ErrorKind::NotFound => {
                            println!("No such file");
                        },
                        _ => {
                            println!("Cannot read the file");
                        }
                    }
                }
            }
        }
        func()
    }
}

fn 泛型与特性() {
    fn max_arr_t<T: std::cmp::Ord + Clone>(arr: &[T]) -> T {
        let mut idx: usize = 0;
        for i in 1..arr.len() {
            idx = if arr[i] > arr[idx] {i} else {idx};
        }
        return arr[idx].clone();
    }
    println!("{}", max_arr_t(&[1, 2, 3, 4, 5, 4, 3, 2, 1]));
    use std::fmt;
    struct Point<T1, T2> {
        x: T1,
        y: T2
    }
    impl<T: std::ops::Add<Output = T> + Copy> Point<T, T> {
        fn sum(&self) -> T {
            return self.x + self.y;
        }
    }
    let p = Point {x: 10, y: 20};
    impl<T1: fmt::Display, T2: fmt::Display> fmt::Display for Point<T2, T1> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Point{{x: {}, y: {}}}", self.x, self.y)
        }
    }
    println!("{}, {}", p, p.sum());
    impl<T, U> Point<T, U> {
        fn mixup<V, W>(&self, other: &Point<V, W>) -> Point<T, W> 
            where T: Clone, U: Clone, V: Clone, W: Clone {
            Point {
                x: self.x.clone(),
                y: other.y.clone(),
            }
        }
    }
    println!("{}", p.mixup(&Point { x: "Hello", y: "World" }));
    println!("{}", Point::mixup(&Point { x: "Hello", y: "World" }, &p));
}

fn 生命周期() {
    fn longer<'a>(s1: &'a str, s2: &'a str) -> &'a str {
        if s2.len() > s1.len() {
            s2
        }
        else {
            s1
        }
    }
    let r;
    {
        let s1 = "Hello";
        let s2 = "World!";
        r = longer(s1, s2);
    }
    println!("{}", r);
}

fn main() {
    输出到命令行();
    基础语法();
    数据类型();
    注释();
    函数();
    条件语句(3);
    循环();
    所有权();
    切片类型();
    结构体();
    枚举类();
    组织管理();
    错误处理();
    泛型与特性();
    生命周期();
}