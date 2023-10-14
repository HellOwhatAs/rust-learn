fn input_argv(args: std::env::Args) -> Result<(String, String), ()> {
    let mut argv = args;
    argv.next();
    Ok((argv.next().ok_or(())?, argv.next().ok_or(())?))
}

fn search(query: String, content: &String) -> (usize, Vec<(usize, String, &str)>) {
    let mut line_num = 0;
    let mut indent: usize = 0;
    let mut results = vec![];
    for line in content.lines() {
        if line.contains(&query) {
            let tmp = line_num.to_string();
            let indent_len = tmp.len();
            indent = std::cmp::max(indent, indent_len);
            results.push((indent_len, tmp, line));
        }
        line_num += 1;
    }
    (indent, results)
}

fn output(indent: usize, results: Vec<(usize, String, &str)>) {
    for (indent_len, tmp, line) in results {
        println!("{}{}|{}", tmp, " ".repeat(indent - indent_len), line);
    }
}

pub fn main() {
    let (query, file_path) = match input_argv(std::env::args()) {
        Ok(result) => result,
        Err(_) => {
            eprintln!("Usage: <query> <file_path>");
            std::process::exit(1);
        },
    };

    let content = match std::fs::read_to_string(&file_path) {
        Ok(result) => result,
        Err(e) => {
            eprintln!("Error: Cannot read from {} {:?}", &file_path, e);
            std::process::exit(1);
        },
    };

    let (indent, results) = search(query, &content);
    output(indent, results);
}
