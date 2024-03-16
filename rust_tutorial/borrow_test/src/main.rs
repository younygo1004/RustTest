fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    // -------------------------------------

    let mut s = String::from("hello");

    change(&mut s);
    change(&mut s);

    println!("{}", s);

    // --------------------------------------
    let reference_to_nothing = not_dangle();
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// fn dangle() -> &String {
//     let s = String::from("hello");

//     &s
// }

fn not_dangle() -> String {
    let s = String::from("hello");

    s
}
