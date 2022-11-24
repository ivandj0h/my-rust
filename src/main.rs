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
    //let mut count = 0;
    //loop {
    //  count += 1;
    //  if count == 10 {
    //      break;
    //  }
    //  println!("Count: {}", count);
    //}

    // While loop
    //let mut count = 0;

    //while count <= 10 {
    //println!("Count: {}", count);
    //  count += 1;
    //}

    // If Statement
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
