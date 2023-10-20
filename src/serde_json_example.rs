pub mod book2txt;
pub mod literal_json;

#[allow(dead_code)]
pub fn parse_book_test() {
    use book2txt::*;
    let raw_content = std::fs::read_to_string("assets/Cycle_of_the_Werewolf.json").unwrap();
    let res1 = parse_book(&raw_content).unwrap();
    let res2 = parse_book2(&raw_content).unwrap();
    assert_eq!(res1, res2);
    std::fs::write("assets/Cycle_of_the_Werewolf.txt", book2txt(&res1).unwrap()).unwrap();
}