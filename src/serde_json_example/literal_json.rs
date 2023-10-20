#[allow(dead_code)]
pub fn from_literal() {
    use serde_json::json;
    let book_demo = json!(
        [
            [
                "cpt1",
                [
                    "line1",
                    "line2"
                ]
            ],
            [
                "cpt2",
                [
                    "line1",
                    "line2"
                ]
            ]
        ]
    );
    println!("{}", book_demo);
}