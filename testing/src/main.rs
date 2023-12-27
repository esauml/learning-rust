

fn main() {
    // let mut name = String::from("Esau Salvador Martinez Lopez");
    // let name = read(); // read(&mut name);
    // println!("Hello, {}!", name);

    let result = maxElement();
    println!("The result is {}", result);
}

// fn read<'a>() -> &'a str {
//     let name = String::from("Esau Salvador Martinez Lopez");
//     let first_name = name.split(" ").next().expect("No first name found");

//     return first_name;
// }

fn maxElement() -> i32 {
    let arr = [10, 20, 30, 40, 50];

    let mut max = &arr[0];

    for i in arr.iter() {
        if i > max {
            max = i;
        }
    }

    *max
}