enum Cell{
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let v: Vec<i32> = Vec::new();
    let mut v = vec![1,2,3];
    v.push(4);
    v.push(5);

    let mut v1 = &mut v;
    v.push(9);
    //v1.push(10);
    let third: &i32 = &v[2];
    //v.push(6);
    println!("The third element is {}", third);
   
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    for i in &v {
        println!("{}", i);
    }

    for i in &mut v {
        *i += 50;
    }

    for i in &v {
        println!("{}", i);
    }

    let row = vec![
        Cell::Int(1),
        Cell::Float(2.2),
        Cell::Text(String::from("3")),
    ];

    for cell in &row {
        match cell {
            Cell::Int(x) =>  println!("{}", x),
            Cell::Float(x) =>  println!("{}", x),
            Cell::Text(x) =>  println!("{}", x),
        }
    }
}
