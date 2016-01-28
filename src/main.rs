use std::fmt; //Imports fmt from std::

fn main() {
    let test_int = 2;
    let other_int = 3;
    println!("This program contains a bunch of functions pertaining to features of this language.");
    println!("Starting\n");
    hello_world();
    println!("Adding!:\t{} + {} = {}", test_int, other_int, add(test_int, other_int));
    println!("--------------------END ADD----------------------");
    objects();
}

fn hello_world() {
    println!("--------------------BEGIN HELLO WORLD--------------------");
    //println! is a macro
    //macros are denoted by the suffix '!'
    println!("Hello world!");
    //Variables are immutable by default
    //In Rust, it is convention to use snake case (my_int instead of myInt)
    let my_int: i32 = 10; 
    //Variables may be displayed by using the {}. Variables are then listed in the proper order
    //after the string.
    println!("my_int: {}", my_int);
    //```myInt+=10;``` Is _invalid_!
    //To change a value, mark it as mutable:
    let mut my_second_int: i32 = 1;
    println!("my_second_int before: {}", my_second_int);
    my_second_int+=10; //Adds 10 to my_second_int
    println!("my_second_int after: {}", my_second_int);
    println!("--------------------END HELLO WORLD----------------------");
}

//This function takes two i32 arguments and returns an i32
fn add(x: i32, y: i32) -> i32 {
    println!("--------------------BEGIN ADD--------------------");
    //You can use ```return x+y;```, but it is not recommended for small functions like this.
    //Instead, write the return value as an expression
    //Expressions in Rust are like statements, but they are not delimited by a semicolor ";"
    //This function will return the value of x+y as an i32
    x+y
    //You can cast this to a different numeric:
    //(x+y) as i64
    //This expression will return the value of (x+y) as an i64 instead of an i32
}

//A struct is Rust's way of OOP
struct MyObject {
    //All values stored in MyObject
    x: i32,
    y: i32,
    name: &'static str,
    //&'static str is an owned string
}

//Implements functions for MyObject
impl MyObject {
    
    fn new() -> MyObject {
        //Common convention for objects in Rust
        //Used to make a default copy of this object
        //MyObject::new();
        //Again, no explicit return statement is required.
        //This will return a copy of MyObject with default values.
        MyObject {
            x: 10,
            y: 10,
            name: "My object!",
        }
    }
}

impl fmt::Display for MyObject {
    //This is used to allow printing the object to the console using print! and println! macros

    //&self is a reference to the object. The '&' prefix denotes a reference.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MyObject: Position: ({}, {}), name: \"{}\"", self.x, self.y, self.name)
    }
}

fn objects() {
    println!("--------------------BEGIN OBJECTS--------------------");
    let object = MyObject::new();
    println!("{}", object);
    println!("--------------------END OBJECTS----------------------");
}
