fn trait_objects() {
    let t: Vec<Box<dyn std::fmt::Display>> = vec![
        Box::new(1), 
        Box::new(2.3), 
        Box::new("HellOwhatAs")
    ];
    println!("{}", t.into_iter().map(|x| x.to_string()).collect::<Vec<String>>().join(", "));
}

fn oo_design_patterns() {
    struct A(String);
    impl A {
        fn new() -> A {
            A(String::new())
        }
        fn add_text(&mut self, text: &str) {
            self.0.push_str(text);
        }
        fn to_b(self) -> B {
            B(self.0)
        }
    }
    struct B(String);
    impl B {
        fn to_c(self) -> C {
            C(self.0)
        }
    }
    struct C(String);
    impl C {
        fn content(&self) -> &str {
            &self.0
        }
    }
    let mut post = A::new();
    post.add_text("I ate a salad for lunch today");
    let post = post.to_b();
    let post = post.to_c();
    println!("{}", post.content());
}

#[allow(dead_code)]
pub fn main() {
    trait_objects();
    oo_design_patterns();
}