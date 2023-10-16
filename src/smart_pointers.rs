fn boxs() {
    {
        let mut b = Box::new(5);
        *b += 70;
        println!("{}", b);
    }
    {
        #[derive(Debug)]
        enum List<T> {
            Cons(T, Box<List<T>>),
            Nil
        }
        impl<T> List<T> {
            fn from_vec(v: Vec<T>) -> Box<List<T>> {
                let mut res = Box::new(List::Nil);
                for elem in v.into_iter().rev() {
                    res = Box::new(List::Cons(elem, res));
                }
                res
            }
        }
        let tmp = List::from_vec(vec!["123", "456", "789"]);
        println!("{:?}", tmp);
    }
}

fn derefs() {
    {
        struct DerefableEmpty();
        impl std::ops::Deref for DerefableEmpty {
            type Target = i32;
            fn deref(&self) -> &Self::Target {
                return &10;
            }
        }
        let a = DerefableEmpty{};
        let b = *a;
        println!("{}", b);
    }
    {
        // &Box<String> -> &String -> &str
        // Deref 强制转换可以将 &String 转换为 &str，因为 String 实现了 Deref trait 因此可以返回 &str
        let x = Box::new(String::from("Rust"));
        let y = &x;
        let z: &str = &x;
        println!("{}, {}", y, z);
    }
    {
        struct DerefMultable<T>(T);
        impl<T> std::ops::Deref for DerefMultable<T> {
            type Target = T;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
        impl<T> std::ops::DerefMut for DerefMultable<T> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
        impl<T: std::fmt::Display> std::fmt::Display for DerefMultable<T> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }
        let mut a = DerefMultable(100);
        *a += 10;
        println!("{}", a);
    }
}

fn drops() {
    struct Sb{}
    impl Drop for Sb {
        fn drop(&mut self) {
            println!("== drop of variable ==");
        }
    }
    let a = Sb{};
    drop(a);
    println!("== End of block ==");
}

fn reference_counting() {
    use std::rc::Rc;
    let x = Rc::new(100);
    println!("{}", Rc::strong_count(&x));
    let x1 = Rc::clone(&x);
    println!("{}", Rc::strong_count(&x));
    let x2 = Rc::clone(&x);
    println!("{}, {}, {}", x1, x2, Rc::strong_count(&x));
    drop(x);
    drop(x1);
    println!("{}", Rc::strong_count(&x2));
}

fn interior_mutability() {
    use std::cell::RefCell;
    {
        let x = RefCell::new("HellO".to_string());
        {
            let mut y = x.borrow_mut();
            (*y).push_str("what");
        }
        println!("{}", x.borrow());
    }
    {
        fn add1(x: &RefCell<i32>) {
            *x.borrow_mut() += 1;
        }
        let x = RefCell::new(10);
        add1(&x);
        println!("{}", x.borrow());
    }
    {
        // Definition for a binary tree node.
        // https://leetcode.cn/problems/invert-binary-tree/
        #[derive(Debug, PartialEq, Eq)]
        pub struct TreeNode {
            pub val: i32,
            pub left: Option<Rc<RefCell<TreeNode>>>,
            pub right: Option<Rc<RefCell<TreeNode>>>,
        }
        
        impl TreeNode {
            #[inline]
            #[allow(dead_code)]
            pub fn new(val: i32) -> Self {
                TreeNode {
                val,
                left: None,
                right: None
                }
            }
        }
        use std::rc::Rc;
        struct Solution;
        impl Solution {
            pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
                let r = root?;
                let mut r1 = r.borrow_mut();
                let mut tmp = None;
                std::mem::swap(&mut r1.left, &mut tmp);
                tmp = Solution::invert_tree(tmp);
                std::mem::swap(&mut r1.right, &mut tmp);
                tmp = Solution::invert_tree(tmp);
                std::mem::swap(&mut r1.left, &mut tmp);
                drop(r1);
                Some(r)
            }
        }
        let t = |
            val: i32, 
            left, 
            right
        | Some(
            Rc::new(
                RefCell::new(
                    TreeNode {val, left, right}
                )
            )
        );
        let root = t(
            4, 
            t(
                2, 
                t(1, None, None),
                t(3, None, None)
            ),
            t(
                7, 
                t(6, None, None),
                t(9, None, None)
            )
        );
        println!("{:#?}", Solution::invert_tree(root));
    }
}

fn reference_cycles() {
    {
        struct Node<'a> {
            val: &'a str,
            next: Option<std::rc::Rc<std::cell::RefCell<Node<'a>>>>
        }
        impl<'a> Drop for Node<'a> {
            fn drop(&mut self) {
                println!("Drop {}", self.val);
            }
        }
        {
            let a = std::rc::Rc::new(std::cell::RefCell::new(Node { val: "A", next: None }));
            let b = std::rc::Rc::new(std::cell::RefCell::new(Node { val: "B", next: None }));
            let c = std::rc::Rc::new(std::cell::RefCell::new(Node { val: "C", next: None }));
            a.borrow_mut().next = Some(b.clone());
            b.borrow_mut().next = Some(a.clone());
            println!("ref-count of A = {}, ref-count of B = {}, ref-count of C = {}", std::rc::Rc::strong_count(&a), std::rc::Rc::strong_count(&b), std::rc::Rc::strong_count(&c));

            let mut p = a.clone();
            for _ in 0..10 {
                match &p.clone().borrow().next {
                    Some(strong_next) => {
                        println!("The strong-next of {} is {}", p.borrow().val, strong_next.borrow().val);
                        p = strong_next.clone();
                    },
                    None => {},
                }
            }
        }
        println!("See? A and B never dropped!!");
    }
    {
        struct Node<'a> {
            val: &'a str,
            next: Option<std::rc::Weak<std::cell::RefCell<Node<'a>>>>
        }
        impl<'a> Drop for Node<'a> {
            fn drop(&mut self) {
                println!("Drop {}", self.val);
            }
        }
        let d = std::rc::Rc::new(std::cell::RefCell::new(Node { val: "D", next: None }));
        let e = std::rc::Rc::new(std::cell::RefCell::new(Node { val: "E", next: None }));
        d.borrow_mut().next = Some(std::rc::Rc::downgrade(&e));
        e.borrow_mut().next = Some(std::rc::Rc::downgrade(&d));
        
        let mut p = std::rc::Rc::downgrade(&d);
        for _ in 0..10 {
            // emm... but why?
            match &p.upgrade().unwrap().borrow().next {
                Some(weak_next) => {
                    let next = weak_next.upgrade().unwrap();
                    println!("The weak-next of {} is {}", p.upgrade().unwrap().borrow().val, next.borrow().val);
                    p = std::rc::Rc::downgrade(&next);
                }
                None => {}
            }
        }
    }
}

#[allow(dead_code)]
pub fn main() {
    boxs();
    derefs();
    drops();
    reference_counting();
    interior_mutability();
    reference_cycles();
}