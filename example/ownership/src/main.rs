fn main() {
    {
        let x = 5;
        let mut y = x;
        println!("x,y {} {}",x,y);
        y = 6;
        println!("x,y {} {}",x,y);
    }

    //String
    {
        let mut s = String::from("hello");
        s.push_str(", world!"); // push_str() appends a literal to a String
        println!("{}", s); // This will print `hello, world!`
    
    }
    //move
    {
        let s1 = String::from("hello");
        let s2 = s1;
        //println!("{}, world!", s1);//got build error.
    }
    //clone
    {
        let s1 = String::from("hello");
        let s2 = s1.clone();
        println!("s1 = {}, s2 = {}", s1, s2);
    }
    {
        let s = String::from("hello");
        takes_ownership(s);
        //println!("{}", s);//got build error

        let i = 5;
        makes_copy(5);
        println!("{}", i);
    }
    {
        let s1 = get_ownership();
        let s2 = String::from("world");
        let len = get_length(s2);
        //println!("{}", s2);//got build error

    }
}

fn takes_ownership(s : String) {
    println!("{}", s);
}

fn makes_copy(i : i32) {
    println!("{}", i)
}

fn get_ownership() -> String {
    String::from("hello")
}

fn get_length(s : String) -> usize {
    s.len()
}