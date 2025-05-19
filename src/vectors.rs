//Vectors -- Dynamically sized arrays


use std::mem;

pub   fn run () {
    let mut numbers: Vec<i32> = vec![1,2,3,4,5];

    //Re-assign value
    numbers[2] = 20;


    //Add on to vector
    numbers.push(5);
    numbers.push(6);

    //pop off last value
    numbers.pop();


    //get value of Numbers Vector
    println!("{:?}", numbers);


    //Get single val
    println!("Single Value: {}", numbers[0]);


    //Get Vector length
    println!("Vector Length: {}", numbers.len());


    //Vectors are stack allocated
    println!("Vectors occupies {} bytes", mem::size_of_val(&numbers));

    //Get Slice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);


    //loop through vector values
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    //loop and mutate values
    for x in numbers.iter_mut() {
        *x *= 2;
    }

    println!("Numbers Vec: {:?}", numbers);
    



}