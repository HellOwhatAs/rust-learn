#[allow(dead_code)]
pub fn parse_book(input: &str) -> Option<Vec<(String, Vec<String>)>> {
    let v: serde_json::Value = serde_json::from_str(input).expect("invalid json");
    let mut s = Vec::new();
    if let serde_json::Value::Array(arr) = v {
        for cpt in arr.into_iter() {
            if let serde_json::Value::Array(mut cpt) = cpt {
                let cptlines = cpt.pop()?;
                let cptname = cpt.pop()?;
                if let (serde_json::Value::String(cptname), serde_json::Value::Array(cptlines)) = (cptname, cptlines) {
                    let cptlines: Vec<String> = cptlines.into_iter().filter_map(
                        |elem| if let serde_json::Value::String(elem) = elem { Some(elem) } else { None }).collect();
                    s.push((cptname, cptlines));
                } else {
                    return None;
                }
            } else {
                return None;
            }
        }
        return Some(s);
    } else {
        return None;
    }
}

#[allow(dead_code)]
pub fn parse_book2(input: &str) -> Option<Vec<(String, Vec<String>)>> {
    let res: Vec<(String, Vec<String>)> = serde_json::from_str(input).ok()?;
    Some(res)
}

#[allow(dead_code)]
pub fn book2txt(book: &Vec<(String, Vec<String>)>) -> Option<String> {
    use std::fmt::Write;
    let mut res = String::new();
    for (cptname, cptlines) in book.iter() {
        writeln!(res, "{}", cptname).ok()?;
        for line in cptlines {
            writeln!(res, "{}", line).ok()?;
        }
    }
    Some(res)
}