// For working with files and read/write
use std::fs::File;
use std::io::prelude::*;

// For CMD Arguments
use std::env;

// Inputs
use std::io;

// Hashmaps
use std::collections::HashMap;

// Use an external dependency
extern crate rand;
use rand::Rng;

// Use HTTP requests
extern crate reqwest;

// Execute another languages commands
use std::process::Command;

// Work with JSONs
extern crate serde_json;
use serde_json::Value as JsonValue;

// JSONs serialization to structs
extern crate serde;

#[macro_use]
extern crate serde_derive;

// Use utils.rs
mod utils;

// Nested modules
mod outer {
    pub fn print_message() {
        println!("Hello fom outer module!");
        private_func();
    }

    fn private_func() {
        println!("I am an outer private function!");
    }

    pub mod inner {
        pub fn inner_print_message() {
            println!("Hello from inner module!");
        }
    }
}

const MAX_NUMBER : u8 = 100;

enum Direction {
    Up,
    Down,
    Right,
    Left,
}

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

trait HasVoiceBox {
    fn speak(&self);
    fn can_speak(&self) -> bool;
}

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    is_male: bool,
}

trait ToString {
    fn to_string(&self) -> String;
}

impl ToString for Person {
    fn to_string(&self) -> String {
        return format!("My name is {} and I am {}", self.name, self.age);
    }
}

fn main() {
    // - Arrays and variables
    let mut x: u32 = 10;

    let nums = 1..11; // [..)

    let animals = vec!["Rabbit", "Dog", "Cat"];

    for a in animals.iter() {
        println!("Animal is {}", a)
    }

    for (index, value) in animals.iter().enumerate() {
        println!("Animal index {}: value {}", index, value);
    }

    for i in nums {
        println!("The number is {}", i);
    }

    // - Enums and match/switch statement
    let player_direction: Direction = Direction::Right;

    match player_direction {
        Direction::Up => println!("Heading Up!"),
        Direction::Down => println!("Heading Down!"),
        Direction::Left => println!("Heading Left!"),
        Direction::Right => println!("Heading Right!"),
    }

    // - Constants
    for n in 1..MAX_NUMBER {
        println!("{}", n);
    }

    // - Tuples
    let tuple = (10, 20, 30, ("Hello", "World"));
    let (a, b, c) = tuple; // Destructure to a, b and c variables
    println!("{}", (tuple.3).1);

    // - Functions
    print_numbers_to(10);

    // - Code Blocks (y exists inside block only)

    let x = 10;
    {
        let y = 5;
    }

    // - Shadowing
    let mut x = 10;

    {
        // x here will not change that outer x, because it is using 'let' keyword
        let x = 5;
    }
    // x == 10 -> true

    // - References
    let mut x = 10;

    let dom = &mut x; // Mutable reference, can change it's value
    *dom += 1; // De-reference and change x value

    // {
    // Or put dom init inside block scope to use x variable
    // }

    println!("x is {}", dom);

    // - Structs
    let mut bg = Color { red: 255, green: 70, blue: 15 };
    bg.blue = 100;
    println!("R: {}, G: {}, B: {}", bg.red, bg.green, bg.blue);

    // - Tuple structs
    let mut red = TupleColor(255, 0, 0);
    red.2 = 60;
    println!("Red is ({}, {}, {})", red.0, red.1, red.2);

    // - Pass by reference
    let blue = Color { red: 0, green: 0, blue: 255 };
    print_color(&blue);

    // - Arrays
    let nums: [i32; 5] = [1, 2, 3, 4, 5];
    let numsWithDefaults = [1; 400]; // Array with 400 items and each one is value = 1

    println!("Index 0 value: {}", nums[0]);

    for n in nums.iter() {
        println!("{}", n);
    }

    for i in 0..nums.len() {
        println!("{}", nums[i]);
    }

    // - Impl keyword
    let rect = Rectangle { width: 10, height: 5 };
    rect.print_description();
    println!("Rectangle is a square: {}", rect.is_square());

    // - Strings
    let mut my_str = String::from("How are you?");

    println!("Length: {}", my_str.len());

    println!("Empty? {}", my_str.is_empty());

    for token in my_str.split_whitespace() {
        println!("{}", token);
    }

    println!("Containes 'Dom'? {}", my_str.contains("Dom"));

    my_str.push_str(" I'm good!"); // str needs to be mut

    // -- Replace
    println!("{}", my_str.replace("How", "What"));

    // -- Lines
    let mut my_str2 = String::from("Line1\nLine2\nLine3");

    for line in my_str2.lines() {
        println!("[ {} ]", line);
    }

    // -- Split
    let mut my_str3 = String::from("How+are+you?");
    let tokens:Vec<&str> = my_str3.split("+").collect(); // Collect iterator to a Vector
    println!("{:?}", tokens);

    // -- Trim
    let mut my_str4 = String::from("  My name is John   ");
    println!("my_str4: {}", my_str.trim());

    // -- Character at index
    let mut my_str5 = String::from("ABCDE");
    let char_index = 3;

    match my_str5.chars().nth(char_index) { // chars() is an iterator
        Some(c) => println!("Character at index {} is: {}", char_index, c), // Or c.to_string()
        None => println!("No character at index {}", char_index),
    }

    // - Traits
    let person = Person { name: String::from("John"), age: 27 };
    println!("{}", person.to_string());

    // - Vectors
    let my_vector: Vec<i32> = Vec::new();
    let mut my_vector: Vec<i32> = vec![1, 2, 3];

    println!("Item on index 2: {}", my_vector[2])

    my_vector.push(4); // add to the end
    my_vector.remove(1); // remove '2'

    for num in my_vector.iter() {
        println!("{}", num);
    }

    // - Read files
    let mut file = File::open("../info.txt").expect("Can't open file");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Can't read the file to string");
    println!("File content: \n{}", content);

    // - Command line arguments
    let args: Vec<String> = env::args().collect();

    for arg in args {
        println!("Argument: {}", arg);
    }

    // - Write files
    let mut file = File::create("output.txt").expect("Can't create file");
    file.write_all(b"Welcome to your first Rust application!").expect("Can't write to output.txt");
    
    // - Match statement
    let num = 20;
    match num {
        1 | 4 => println!("One or Four!"),
        10 => println!("Ten!"),
        12..=20 => println!("In range of 12 to 20!"), // Inclusive [...]
        _ => println!("Nothing from above!"),
    }

    // - Read user input
    let mut input = String::new();
    println!("Hey mate! Say something:");
    match io::stdin().read_line(&mut input) { // Returns result, success or failure
        Ok(_) => {
            println!("Brilliant! you said: {}", input.to_uppercase());
        },
        Err(e) => {
            println!("Oops, something went wrong! error: {}", e);
        }
    }

    // - Hashmaps
    let mut map = HashMap::new();
    map.insert("Math", 90);
    map.insert("English", 95);
    map.insert("Web", 92);
    println!("How many keys here? {}", map.len());

    // Get single value
    match map.get("Math") {
        Some(value) => println!("Value for 'Math' is {}", value),
        None => println!("No value for 'Math'"),
    }

    map.remove("Math");

    for (key, value) in &map {
        println!("Key: {}, Value: {}", key, value);
    }

    println!("Is 'Cpp' exists on map? {}", map.contains_key("Cpp"));

    // - Random numbers
    let random_number = rand::thread_rng().gen_range(1, 11); // 1 - 10
    println!("Random number: {}", random_number);

    // Random true boolean
    let random_bool = rand::thread_rng().gen_weighted_bool(2); // 1/2 chance to get 'true'
    println!("Random bool: {}", random_bool);

    // - Use another modules
    println!("2 + 5 = {}", utils::add(2, 5));
    outer::print_message();
    outer::inner::inner_print_message();

    // - Option
    let name = "Michael";

    println!("Occupation of {} is: {}", 
    name,
    match get_occupation_option(&name) {
        Some(val) => val,
        None => "No occuaption found!",
    });

    // - HTTP requests
    let endpoint = "https://baconipsum.com/api/?type=meat-and-filter";

    // Long way to make requests
    match reqwest::get(endpoint) {
        Ok(mut response) => {
            // Check if 200 status
            if response.status() == reqwest::StatusCode::Ok {
                match response.text() {
                    Ok(text) => println!("Response text: {}", text),
                    Err(e) => println!("Could not read response text: {}", e)
                }
            } else {
                println!("Response was {}", response.status());
            }
        },
        Err(e) => println!("Error on request: {}", e),
    }

    // Short way to make requests
    let response_text = reqwest::get(endpoint)
        .expect("Could not make request!")
        .text()
        .expect("Could not read response text!");

    println!("{}", response_text);

    // - Execute terminal commands

    // build command: python script.py
    let mut cmd = Command::new("python");
    cmd.arg("script.py");

    // execute the command
    match cmd.output() {
        Ok(o) => {
            unsafe {
                println!("Output from script.py: {}", String::from_utf8_unchecked(o.stdout));
            }
        },
        Err(e) => {
            println!("Error: {}", e);
        }
    }

    // Work with JSONs
    let json_str = r#"
        {
            "name": "John",
            "age": 27,
            "is_male": true
        }
    "#;

    let result = serde_json::from_str(json_str);

    // First way - with serde_json    
    if result.is_ok() {
        let object: JsonValue = result.unwrap();
        println!("The name is {}", object["name"].as_str().unwrap());
    } else {
        println!("Could not parse JSON");
    }

    // Second way - with serde, serde_derive (#[macro_use])
    if result.is_ok() {
        let object: Person = result.unwrap();
        println!("The name is {}", object.name);
    } else {
        println!("Could not parse JSON");
    }
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

fn get_occupation_option(name: &str) -> Option<&str> {
    match name {
        "John" => Some("Frontend Developer"),
        "Michael" => Some("Backend Developer"),
        _ => None
    }
}
