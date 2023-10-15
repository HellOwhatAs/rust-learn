fn closures() {
    {
        let f = |v: &Vec<i32>| "[".to_string() + &v.iter().map(i32::to_string).collect::<Vec<String>>().join(", ") + "]";
        let mut v = vec![1, 2, 3];
        let f1 = || v.last();
        f1();
        println!("{}", f(&v));
        let mut f2 = || v.push(4);
        f2();
        println!("{}", f(&v));
        let f3 = move || {v.push(5); v};
        let res = f3();
        println!("{}", f(&res));
    }
    {
        let mut x: Vec<&str> = "
        Shanghai Jiao Tong University (SJTU) was founded in Shanghai, China, in 1896,
        with the goal of cultivating talented professionals for the nation.
        ".split_whitespace().map(|elem| elem.trim_matches(|c: char| c.is_ascii_punctuation())).collect();
        let mut count = 0;
        x.sort_by_key(|elem| {count += 1; elem.len()});
        println!("{}", count);
        println!("{}", x.join(", "));
    }
}

fn iterators() {
    let mut v = vec!["Hello".to_string(), "World".to_string(), "!!!".to_string()];
    {
        let mut v_i = v.iter();
        println!("{}", v_i.next().unwrap());
    }
    {
        let mut v_m_i = v.iter_mut();
        v_m_i.next().unwrap().push_str(", ");
        println!("{}", v[0]);
    }
    {
        let _v_owned_i = v.into_iter();
        // println!("{}", v.join(""));
    }
}

#[allow(dead_code)]
pub fn main() {
    closures();
    iterators();
}