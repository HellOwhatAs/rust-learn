fn recoverable() {
    {
        fn thrower() -> Result<(), i32> {
            Err(123)
        }
        fn caller() -> Result<String, i32> {
            thrower()?;
            Ok("Ok".to_string())
        }
        match caller() {
            Ok(msg) => println!("Ok: {msg}"),
            Err(e) => println!("Err: {e}"),
        }
    }
    {
        fn noner() -> Option<()> {
            None
        }
        fn caller() -> Option<String> {
            noner()?;
            Some("Not None".to_string())
        }
        match caller() {
            Some(msg) => println!("Some: {msg}"),
            None => println!("None"),
        }
    }
    {
        fn last_char_of_first_line(text: &str) -> Option<char> {
            text.lines().next()?.chars().last()
        }
        match last_char_of_first_line("Hello, world\nHow are you today?") {
            Some(c) => println!("last char of first line is {c}"),
            None => println!("Not any line"),
        }
    }
}

#[allow(dead_code)]
pub fn main() {
    recoverable();
}