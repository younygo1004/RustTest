fn main() {
    let mut s = String::from("hello");

    s.push_str(", world!");

    println!("{}", s);

    let s1 = String::from("hello");
    let s2 = s1;

    // println!(s1, s2);
    println!("{s2}");

    let x = "test";
    let y = x;

    println!("x = {}, y = {}", x, y);

    let s = String::from("hello");
    takes_ownership(s); // 소유권(Ownership)이 이동(move)됨

    // println!("{s}"); // 유효하지 않음!

    let x = 5;

    // 소유권이 이동되지만, stack 영역의 변수이므로 copy된 값이 이동됨
    makes_copy(x);
    println!("{}", x); // 따라서 x는 유효함
}

fn takes_ownership(some_string: String) {
    println!("{some_string}");
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}
