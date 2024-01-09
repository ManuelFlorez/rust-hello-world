fn main() {    
    
    let vector = vec![
        "monday",
        "tuesday",
        "wednesday",
        "thursday",
        "friday",
        "saturday",
        "sunday"
    ];

    let mut counter = 0;
    loop {
        println!("{}", vector[counter]);
        counter += 1;
        if counter == vector.capacity() {
            break;
        }
    }

    println!("--------------------------1");

    counter = 0;
    while counter < vector.capacity() {
        println!("{}", vector[counter]);
        counter += 1;
    }

    println!("--------------------------2");

    for dia in 0..vector.capacity() {
        println!("{}", vector[dia]);
    }

    println!("--------------------------3");

    for dia in vector.iter() {
        println!("{}", dia);
    }

}
