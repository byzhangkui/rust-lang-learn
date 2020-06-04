fn main() {
    let mut s = String::from("hello");
    
    changes(&mut s);
    println!("{}", s);

    {
        let mut s = String::from("hello");
        let r1 = &mut s;
        //let r2 = &mut s;// error , you can have only one mutable reference to a particular piece of data in a particular scope 
        //println!("{}, {}", r1, r2);
    }

    {
        let mut s = String::from("hello");
        let r1 = &mut s;
        println!("{}", r1);
        let r2 = &mut s;
        println!("{}", r2);
    }
    {
        let mut s = String::from("hello");
        let r1 = &mut s;
        println!("{}", r1);
        let r2 = &mut s;
        println!("{}", r2);
    }
    {
        let r1 = &s;//no problem
        let r2 = &s;//no problem
        //let r3 = &mut s;//BIG PROBLEM
        //println!("{}, {}, {}", r1, r2, r3);
    }
    {
        let r1 = &s;//no problem
        let r2 = &s;//no problem
        println!("{}, {}", r1, r2);
        // r1 and r2 are no longer used after this point

        let r3 = &mut s;//no problem
        println!("{}", r3);
    }
}

fn changes(s : &mut String) {
    s.push_str(", world!");
}
