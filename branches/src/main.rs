// fn main() {
//     let number = 10;
//         if number < 5 {
//             println!("Condition was true");
//         }else {
//             println!("Condition was false");
//         }
// }
//
// fn main() {
//     let number = 3;
//
//     if number != 0 {
//         println!("number was something other than 0");
//     }
// }

// fn main() {
//     let condition = true;
//
//     let number = if condition { "five" } else { "six" };
//
//     println!("The value of number is: {number}");
// }
//

// fn main() {
//     loop {
//         println!("again!");
//     }
// }


// fn main() {
//     let mut count = 0;
//     'counting_up: loop {
//         println!("count = {count}");
//         let mut remaining = 10;
//
//         loop {
//             println!("remaining = {remaining}");
//             if remaining == 9 {
//                 break;
//             }
//             if count == 2 {
//                 break 'counting_up;
//             }
//             remaining -= 1;
//         }
//
//         count += 1;
//     }
//     println!("End count = {count}");
// }

//
// fn main() {
//     let mut number = 3;
//
//     while number != 0 {
//         println!("{number}!");
//
//         number -= 1;
//     }
//
//     println!("LIFTOFF!!!");
// }

// fn main() {
//     let a = [10, 20, 30, 40, 50];
//
//     for element in a {
//         println!("the value is: {element}");
//     }
// }

fn main() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
