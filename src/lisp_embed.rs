use rust_lisp::{lisp, default_env};
use rust_lisp::interpreter::eval;
use std::rc::Rc;
use std::cell::RefCell;

#[allow(dead_code)]
pub fn main() {
    let tmp = lisp! {
        (list 
          (* 1 2)
          (/ 6 3 "foo"))
    };
    let res = eval(Rc::new(RefCell::new(default_env())), &tmp).unwrap();
    println!("{}", res);
    println!("{}", eval(Rc::new(RefCell::new(default_env())), &lisp! {(/ 6 3 "foo")}).unwrap());
}