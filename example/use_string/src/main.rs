fn main() {
    let mut s = String::new();

    // initial with contents
    let data = "initial contents";
    let s = data.to_string();
    let s = "initial content".to_string();

    let s = String::from("initial contents");

    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");;
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    //append to a string
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("s:{}", s);

    let mut s1 = String::from("foo");
    let s2 = String::from("bar");
    s1.push_str(&s2);
    println!("s1:{}", s1);
    println!("s2:{}", s2);
    s1.push('!');
    println!("s1:{}", s1);
}
