pub fn book2txt<P: AsRef<std::path::Path>>(input: P, output: P) {
    use std::fmt::Write;
    let content = std::fs::read_to_string(input).expect("failed to read");
    let v: serde_json::Value = serde_json::from_str(&content).expect("invalid json");
    let mut s = String::new();
    for cpt in v.as_array().expect("invalid book format") {
        let cptname = cpt[0].as_str().expect("invalid book format");
        let cptlines = cpt[1].as_array().expect("invalid book format");
        writeln!(s, "{}", cptname).unwrap();
        for line in cptlines {
            writeln!(s, "{}", line.as_str().expect("invalid book format")).unwrap();
        }
    }
    std::fs::write(output, s).expect("failed to write");
}