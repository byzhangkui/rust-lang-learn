fn main() {
    let s1 = String::from("world");
    let len = get_length(&s1);
    println!("The length of {} is {}", s1, len)
}

fn get_length(s : &String) -> usize {
    s.len() // s does not have ownership of what it refer to.
}