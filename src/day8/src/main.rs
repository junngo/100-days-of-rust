fn main() {
    let s1 = String::from("hello");
    let len = caculate_length(&s1);
    println!("The length of {} is {}", s1, len);

    let mut s2 = String::from("Hello");
    change_string_value(&mut s2);
    println!("The s2 value is `{}`", s2);
}

fn caculate_length(s: &String) -> usize {
    s.len()
}

fn change_string_value(s: &mut String) {
    s.push_str(", world");
}
