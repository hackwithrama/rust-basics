pub fn test_vec_int() {
    let mut my_ints = Vec::<i32>::new();
    my_ints.push(30);
    my_ints.push(25);
    my_ints.push(13);
    my_ints.push(1);
    my_ints.push(2);


    println!("Size of vec: {:?}", my_ints.len());
    println!("Capacity of vec: {:?}", my_ints.capacity());
    println!("Vec : {:?}\n", my_ints);

    // println!("First item in vec: {:?}", &(&my_ints).as_slice()[0..8]);

    println!("Element from vec: {:?}", my_ints.get(10));
}

pub fn test_vec_string(){
    let first_names = vec!["rama", "chandran", "ranga", "ramakrishnan", "varadaraju"];

    for first_name in first_names.clone() {
        println!("The first name is: {}", first_name);
    }

    println!("The first names of vec: {:?}", first_names);
}