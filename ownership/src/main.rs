fn main() {
    //let s = "hello"; This cant be mutated and passed on to stack in scope ,
    //hence here we are using the String type and passing to heap

    // let mut s = String::from("hello");
    // //Allocating memory to binary -  need a way to free
    // //this when done to the alloaction
    // //(There is a memory allocater we have to borrow from it am
    // //gyessing right now)
    // s.push_str(", world!");
    // println!("{s}");

    let s1 = String::from("hello");
    let _s2 = s1.clone();

    println!("{s1}, world!");
}
