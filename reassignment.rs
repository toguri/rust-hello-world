fn reassignment() {
    let first_name = String::from("toru");
    let second_name = String::from("oguri");

    first_name = String::from("tadahiro");

    println!("{} {}", first_name, second_name);
}

fn shadowing() {
    let first_name = String::from("toru");
    let first_name = String::from("tadahiro");

    println!("{}", first_name);
}

fn shadowing2() {
    let first_name = String::from("toru");
    let first_name = 1;

    println!("{}", first_name);
}

fn name(arg: Type) -> RetType {
    unimplemented!();
}

fn main() {
    reassignment();
}
