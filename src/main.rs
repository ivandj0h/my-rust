fn main() {
    //Match function
    //let num = 5;
    //match num {
    //  1 => println!("One"),
    //  2 => println!("Two"),
    //  3 => println!("Three"),
    //  4 => println!("Four"),
    //  5 => println!("Five"),
    //  _ => println!("Something else"),
    //}

    // For loop
    //for n in 1..100 {
    //  print!("{} ", n);
    //}

    // Loop Statement
    let mut count = 0;
    loop {
        count += 1;
        if count == 10 {
            break;
        }
        println!("Count: {}", count);
    }
}
