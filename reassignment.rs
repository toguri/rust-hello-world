fn reassignment() {
    let first_name = String::from("toru");
    let second_name = String::from("oguri");

    first_name = String::from("tadahiro");

    println!("{} {}", first_name, second_name);
}

fn main() {
    reassignment();
}
