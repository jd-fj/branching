// Basic if/else block
// fn main() {
//     let number = 3;

//     if number < 5 {
//         println!("condition met");
//     } else {
//         println!("condition NOT met");
//     }
// }
// --------------------------------------------------

// Basic else if branching
// fn main() {
//     let number = 6;

//     if number % 4 == 0 {
//         println!("number is divisible by 4");
//     } else if number % 3 == 0 {
//         println!("number is divisible by 3");
//     } else if number % 2 == 0 {
//         println!("number divisible by 2");
//     } else {
//         println!("number not divisible by 4, 3, or 2");
//     }
// }
// --------------------------------------------------

// If in a let statement. The values that have the potential to be results from each arm of the if MUST be the same type! It CANNOT be `if condition { 5 } else { "string" }`
fn main() {
    let condition = false;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);
}
