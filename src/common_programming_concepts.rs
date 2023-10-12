fn variables() {
    let x = 120;
    {
        let x = 30;
        println!("x = {x};");
    }
    println!("x = {x};");   
}

fn data_types() {
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("{}, {}, {}", c, z, heart_eyed_cat);
}

fn function() {
    /// Return the name String of input variable.
    fn type_name<T>(_x: &T) -> String {
        std::any::type_name::<T>().to_owned()
    }
    fn func() -> i128 {
        let res = 5;
        res
    }
    let var = func();
    println!("{}", type_name(&var));
}

fn control_flow() {
    let val1 = loop {break "loop return value"};
    let val2 = 'loop_outer: loop {
        let res = 'loop_inner: loop {
            loop {
                break 'loop_inner "nested loops";
            }
        };
        break 'loop_outer res;
    };
    println!("{}, {}", val1, val2);
    'for_outer: for i in (0..5).rev() {
        for j in 0..5 {
            if j + i == 5 {
                println!("{i}, {j}");
                break 'for_outer;
            }
        }
    }
}

#[allow(dead_code)]
pub fn main() {
    variables();
    data_types();
    function();
    control_flow();
}