//Primitive strings = Immutable fixed-length string somewhere in memory
//String = Growable, heap-allocated data structure - Use when you need to modify or own string data at runtime

pub fn run() {
    let mut hello = String::from("Hello ");

    //Get Length 
    println!("Length: {}", hello.len());

    //Push Char
    hello.push('W');


    //Push String
    hello.push_str("orld!");


    //Capacity in bytes
    println!("Capacity: {}", hello.capacity());

    //Check if Empty
    println!("Is Empty: {}", hello.is_empty());

    //Contains 
    println!("Contains 'World' {}", hello.contains("World"));

    //Replace 
    println!("Replace: {}", hello.replace("World", "There"));

    //loop through string by whitespace
    for word in hello.split_whitespace() {
        println!("{}", word);
    }


    //Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

  
    //Assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    

    println!("{}", s);
}


