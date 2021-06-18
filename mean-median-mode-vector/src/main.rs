mod vec_operation;

use vec_operation::*;

fn main() {
    let my_vec = vec![5, 3, 7, 9, 1, 11, 25, 5, 11, 11];

    for elem in &my_vec { print!("{} ", elem);}
    println!("");

    println!("Mean: {}", calculate_mean(&my_vec));

    for elem in &my_vec { print!("{} ", elem);}
    println!("");

    println!("Median: {}", calculate_median(&my_vec));

    for elem in &my_vec { print!("{} ", elem);}
    println!("");

    let (val, count) = calculate_mode(&my_vec);

    println!("Mode: Value {} shown {} times", val, count);

    for elem in &my_vec { print!("{} ", elem);}
    println!("");
}
