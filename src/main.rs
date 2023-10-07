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
    #[derive(Clone)]
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
fn 组织管理() {
    use second::fib;
    println!("{}", fib(100));
    {
        println!("{}", f64::cos(std::f64::consts::PI));
    }
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
}