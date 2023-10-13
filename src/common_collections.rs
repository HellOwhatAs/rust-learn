fn vectors() {
    let mut v = vec![1, 2, 3, 4];
    // let v0 = &mut v[0];
    v.push(5);
    let tmp = v.iter().map(|x|x.to_string()).collect::<Vec<String>>().join(", ");
    println!("[{}]", tmp);
    // println!("{}", v0);
}

fn strings() {
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("{s1}, {s2}");
}

fn hash_maps() {
    use std::collections::HashMap;
    let mut x = HashMap::new();
    x.insert("k".to_string(), "v".to_string());
    x.insert("k".to_string(), "v2".to_string());
    println!("{{{}}}", x.iter().map(|x|format!("{}: {}", x.0, x.1)).collect::<Vec<String>>().join(", "));
    x.entry("user name".to_string()).or_insert("HellOwhatAs".to_string());
    println!("{{{}}}", x.iter().map(|x|format!("{}: {}", x.0, x.1)).collect::<Vec<String>>().join(", "));
    x.entry("k".to_string()).and_modify(|old|{
        let i = old.pop().unwrap().to_digit(10).unwrap();
        old.push_str(&(i + 1).to_string());
    });
    println!("{{{}}}", x.iter().map(|x|format!("{}: {}", x.0, x.1)).collect::<Vec<String>>().join(", "));
}

mod practices {
    pub fn q1<T>(arr: &Vec<T>) -> (T, T) 
        where T: Copy + std::cmp::Ord + std::ops::Add<Output = T> + std::ops::Div<Output = T> + std::hash::Hash + From<i8> + std::fmt::Display
    {
        let mut arr1 = arr.clone();
        arr1.sort();
        println!("[{}]", arr1.iter().map(|x|x.to_string()).collect::<Vec<String>>().join(", "));
        let first = match arr1.len() % 2 {
            1 => arr1[arr1.len() / 2],
            _ => (arr1[arr1.len() / 2 - 1] + arr1[arr1.len() / 2]) / T::from(2)
        };
        let mut counter = std::collections::HashMap::new();
        for elem in arr {
            counter.entry(elem).and_modify(|x| {*x += 1;}).or_insert(0);
        }
        let second = counter.iter().reduce(|acc, e| {if acc.1 > e.1 {acc} else {e}}).unwrap().0;
        (first, **second)
    }
    pub fn q2(s: &str) -> String {
        s.split_ascii_whitespace().map(|word|{
            let s: Vec<char> = word.chars().collect();
            if "aeiouAEIOU".contains(s[0]) {
                return s.iter().collect::<String>() + "-hay";
            }
            else {
                let mut y = (&s[1..]).iter().collect::<String>();
                y.push('-');
                y.push(s[0]);
                y.push_str("ay");
                y
            }
        }).collect::<Vec<String>>().join(" ")
    }
}

#[allow(dead_code)]
pub fn main() {
    vectors();
    strings();
    hash_maps();
    {
        use rand::Rng;
        let mut input: Vec<i32> = vec![];
        let mut rng = rand::thread_rng();
        for _ in 0..rng.gen_range(8..12) {
            input.push(rng.gen_range(0..10));
        }
        let res = practices::q1(&input);
        println!("{}, {}", res.0, res.1);
    }
    {
        let input = String::from("Empowering everyone to build reliable and efficient software");
        println!("{}", practices::q2(&input));
    }
}