mod serde_json_example;

fn main() {
    serde_json_example::book2txt::book2txt("assets/Cycle_of_the_Werewolf.json", "assets/Cycle_of_the_Werewolf.txt");
}