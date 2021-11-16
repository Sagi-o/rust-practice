// For workign with files and Read/Write
// use std::fs::File;
// use std::io::prelude::*;

// For CMD Arguments
// use std::env;

// For input
// use std::io;

// Hashmaps
// use std::collections::HashMap;

// Use an external dependency
extern crate rand;
use rand::Rng;

enum Direction {
    Up,
    Down,
    Right,
    Left,
}

const MAX_NUMBER : u8 = 100;

struct Color {
    red: u8, // u8 -> 0-255
    green: u8,
    blue: u8,
}

struct TupleColor(u8, u8, u8);

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn print_description(&self) {
        println!("Rectangle: {} x {}", self.width, self.height);
    }

    fn is_square(&self) -> bool {
        return self.width == self.height;
    }
}

struct Person {
    name: String,
    age: u8,
}

trait ToString {
    fn to_string(&self) -> String;
}

impl ToString for Person {
    fn to_string(&self) -> String {
        return format!("My name is {} and I am {}", self.name, self.age);
    }
}

trait HasVoiceBox {
    fn speak(&self);
    fn can_speak(&self) -> bool;
}

fn main() {

    // - Arrays and variables
    // let mut x: u32 = 10;

    // let nums = 1..11; // [..)

    // let animals = vec!["Rabbit", "Dog", "Cat"];

    // for a in animals.iter() {
    //     println!("Animal is {}", a)
    // }

    // for (index, value) in animals.iter().enumerate() {
    //     println!("Animal index {}: value {}", index, value);
    // }

    // for i in nums {
    //     println!("The number is {}", i);
    // }

    // - Enums and match/switch statement
    // let player_direction: Direction = Direction::Right;

    // match player_direction {
    //     Direction::Up => println!("Heading Up!"),
    //     Direction::Down => println!("Heading Down!"),
    //     Direction::Left => println!("Heading Left!"),
    //     Direction::Right => println!("Heading Right!"),
    // }

    // - Constants
    // for n in 1..MAX_NUMBER {
    //     println!("{}", n);
    // }

    // - Tuples
    // let tuple = (10, 20, 30, ("Hello", "World"));
    // let (a, b, c) = tuple; // Destructure to a, b and c variables
    // println!("{}", (tuple.3).1);

    // - Functions
    // print_numbers_to(10);

    // - Code Blocks (y exists inside block only)

    // let x = 10;
    // {
    //     let y = 5;
    // }

    // - Shadowing
    // let mut x = 10;

    // {
    //     let x = 5;
    //     // x here will not change that outer x, because it is using 'let' keyword
    // }
    // // x == 10 -> true

    // - References

    // let mut x = 10;

    // let dom = &mut x; // Mutable reference, can change it's value
    // *dom += 1; // De-reference and change x value

    // // {
    // // Or put dom init inside this scope to use x variable
    // // }

    // println!("x is {}", dom);

    // - Structs
    // let mut bg = Color { red: 255, green: 70, blue: 15 };
    // bg.blue = 100;
    // println!("R: {}, G: {}, B: {}", bg.red, bg.green, bg.blue);

    // - Tuple structs
    // let mut red = TupleColor(255, 0, 0);
    // red.2 = 60;
    // println!("Red is ({}, {}, {})", red.0, red.1, red.2);

    // - Pass by reference
    // let blue = Color { red: 0, green: 0, blue: 255 };
    // print_color(&blue);

    // - Arrays
    // let nums: [i32; 5] = [1, 2, 3, 4, 5];
    // let numsWithDefaults = [1; 400]; // Array with 400 items and each one is value = 1

    // println!("Index 0 value: {}", nums[0]);

    // for n in nums.iter() {
    //     println!("{}", n);
    // }

    // for i in 0..nums.len() {
    //     println!("{}", nums[i]);
    // }

    // - Impl keyword
    // let rect = Rectangle { width: 10, height: 5 };
    // rect.print_description();
    // println!("Rectangle is a square: {}", rect.is_square());

    // - Strings
    // let mut my_str = String::from("How are you?");
    // println!("Length: {}", my_str.len());

    // println!("Empty? {}", my_str.is_empty());

    // for token in my_str.split_whitespace() {
    //     println!("{}", token);
    // }

    // println!("Containes 'Dom'? {}", my_str.contains("Dom"));

    // my_str.push_str(" I'm good!"); // str needs to be mut

    // println!("my_str: {}", my_str);

    // - Traits
    // let person = Person { name: String::from("John"), age: 27 };
    // println!("{}", person.to_string());

    // - Vectors
    // let my_vector: Vec<i32> = Vec::new();
    // let mut my_vector: Vec<i32> = vec![1, 2, 3];

    // // my_vector[2];

    // my_vector.push(4); // add to the end
    // my_vector.remove(1); // remove '2'

    // for num in my_vector.iter() {
    //     println!("{}", num);
    // }

    // - Read files
    // let mut file = File::open("../info.txt").expect("Can't open file");
    // let mut content = String::new();
    // file.read_to_string(&mut content).expect("Can't read the file to string");
    // println!("File content: \n{}", content);

    // - Command line arguments
    // let args: Vec<String> = env::args().collect();

    // for arg in args {
    //     println!("Argument: {}", arg);
    // }

    // - Write files
    // let mut file = File::create("output.txt").expect("Can't create file");
    // file.write_all(b"Welcome to your first Rust application!").expect("Can't write to output.txt");
    
    // Match statement
    // let num = 20;
    // match num {
    //     1 | 4 => println!("One or Four!"),
    //     10 => println!("Ten!"),
    //     12..=20 => println!("In range of 12 to 20!"), // Inclusive [...]
    //     _ => println!("Nothing from above!"),
    // }

    // Read user input
    // let mut input = String::new();
    // println!("Hey mate! Say something:");
    // match io::stdin().read_line(&mut input) { // Returns result, success or failure
    //     Ok(_) => {
    //         println!("Brilliant! you said: {}", input.to_uppercase());
    //     },
    //     Err(e) => {
    //         println!("Oops, something went wrong! error: {}", e);
    //     }
    // }

    // Hashmaps
    // let mut map = HashMap::new();
    // map.insert("Math", 90);
    // map.insert("English", 95);
    // map.insert("Web", 92);
    // println!("How many keys here? {}", map.len());

    // // Get single value
    // match map.get("Math") {
    //     Some(value) => println!("Value for 'Math' is {}", value),
    //     None => println!("No value for 'Math'"),
    // }

    // map.remove("Math");

    // for (key, value) in &map {
    //     println!("Key: {}, Value: {}", key, value);
    // }

    // println!("Is 'Cpp' exists on map? {}", map.contains_key("Cpp"));
}

fn print_color(color: &Color) {
    println!("R: {}, G: {}, B: {}", color.red, color.green, color.blue);
}

fn print_numbers_to(num: u32) {
    for n in 1..num {
        if is_even(n) {
            println!("Even! {}", n);
        } else {
            println!("Not even! {}", n);
        }
    }
}

fn is_even(num: u32) -> bool {
    return num % 2 == 0;
}
