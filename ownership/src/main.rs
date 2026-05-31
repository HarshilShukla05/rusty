// fn main() {
//let s = "hello"; This cant be mutated and passed on to stack in scope ,
//hence here we are using the String type and passing to heap

// let mut s = String::from("hello");
// //Allocating memory to binary -  need a way to free
// //this when done to the alloaction
// //(There is a memory allocater we have to borrow from it am
// //gyessing right now)
// s.push_str(", world!");
// println!("{s}");

// let s1 = String::from("hello");
// let s2 = s1;//Moved
//
// println!("s1 = {s1}, s2 = {s2}");

// let s1 = String::from("hello");
// let s2 = s1.clone();
//
// println!("s1 = {s1}, s2 = {s2}");
//
// let x = 5;
// let y = x;
//
// println!("x = {x}, y = {y}");
// }

// fn main() {
//     let s = String::from("hello"); // s comes into scope
//
//     takes_ownership(s); // s's value moves into the function...
//     // ... and so is no longer valid here
//
//     let x = 5; // x comes into scope
//
//     makes_copy(x); // Because i32 implements the Copy trait,
//     // x does NOT move into the function,
//     // so it's okay to use x afterward.
//     let y = x;
//     println!("y = {y} , x = {x}");
// } // Here, x goes out of scope, then s. However, because s's value was moved,
// // nothing special happens.
//
// fn takes_ownership(some_string: String) {
//     // some_string comes into scope
//     println!("{some_string}");
// } // Here, some_string goes out of scope and `drop` is called. The backing
// // memory is freed.
//
// fn makes_copy(some_integer: i32) {
//     // some_integer comes into scope
//     println!("{some_integer}");
// } // Here, some_integer goes out of scope. Nothing special happensf

// fn main() {
//     //     let _s1 = give_ownership();
//     //     let s2 = String::from("hello");
//     //     let (s3, len) = takes_gives_back_with_length(s2);
//     //     println!("s3 = {s3}, len = {len}");
//     // }
//     //
//     // fn give_ownership() -> String {
//     //     let some_string = String::from("yours");
//     //     some_string
//     // }
//     //
//     // fn takes_gives_back_with_length(a_string: String) -> (String, usize) {
//     //     let length = a_string.len();
//     //     (a_string, length)
//
//     let s1 = String::from("hello");
//     let len = calculate_length(&s1);
//
//     println!("Size of {s1} is {len}")
// }
//
// fn calculate_length(s: &String) -> usize {
//     s.len()
// }

fn main() {
    let mut s = String::from("hello");

    change(&mut s);

    println!("Value of s is {s}");
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
