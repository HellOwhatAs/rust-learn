fn macros_methodical() {
    {
        #[macro_use]
        mod symrs {
            #[derive(Debug)]
            pub struct Symbols(pub String);
            macro_rules! symbols {
                ($($i:ident),*) => {
                    $( let $i = Symbols(stringify!($i).to_string()); )*
                }
            }
        }
        use symrs::*;
        symbols!(x, y, z);
        println!("{:?}", (x, y, z));
    }
}

#[allow(dead_code)]
pub fn main() {
    macros_methodical();
}