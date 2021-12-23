#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(warnings, unused)]

use std::char;
use std::env;
use std::fs;
use std::io;
use std::mem;
use std::process::Command;
use std::str::FromStr;

#[test]
fn test_hello_format() {
    print!("hello");
    let name = "brian";
    assert_eq!(format!("Hello {}!", name), "Hello brian!");
}

#[test]
fn test_mem_size() {
    let x = 5;
    assert_eq!(4, std::mem::size_of_val(&x))
}

#[test]
fn for_loop() {
    let mut s = String::new();
    for i in 0..3 {
        s.push_str(i.to_string().as_str())
    }
    assert_eq!("012", s.as_str())
}

#[test]
fn test_read_file() {
    let filename = "./README.md";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    assert!(contents.contains("rust-playground"));
}

#[test]
fn test_read_dir() {
    let mut files = Vec::new();
    let paths = fs::read_dir("./").unwrap();

    for path in paths {
        files.push(format!("{}", path.unwrap().path().display()))
    }
    assert!(files.len() > 0)
}

fn user_input() {
    println!("Pick a number.");

    let mut number = String::new();

    match io::stdin().read_line(&mut number) {
        Ok(_) => println!("You guessed: {}", number),
        Err(e) => println!("Failed to read line: {}", e),
    }
}

#[test]
fn test_struct() {
    #[derive(Debug)]
    struct Person<'a> {
        name: &'a str,
        age: u8,
    }

    let p = Person {
        name: "brian",
        age: 42,
    };
    assert_eq!(format!("{:?}", p), "Person { name: \"brian\", age: 42 }")
}

#[test]
fn test_strings() {
    let s = String::from_str("foo").unwrap();
    let ss = &s;
    assert_eq!(ss, "foo")
}

#[test]
fn test_example() {
    assert!(true);
    assert_eq!(1, 1)
}

#[test]
fn test_checked() {
    let i: i32 = 3;
    let product = i.checked_mul(2).unwrap();
    assert_eq!(product, 6);
    let a: i32 = 2;
    assert_eq!(a.checked_add(3), Some(5))
}

#[test]
fn test_format() {
    let name = "world";
    assert_eq!(format!("hello {}", name), "hello world")
}

#[test]
#[should_panic]
fn test_todo() {
    todo!("boom")
}

#[test]
#[ignore]
fn ignored_test() {
    let a = 0;
    assert_eq!(a, 0);
}

#[test]
fn test_child_process() {
    let output = Command::new("echo")
        .arg("-n")
        .arg("foo")
        .output()
        .unwrap_or_else(|e| panic!("failed to execute process: {}", e));

    if output.status.success() {
        let s = String::from_utf8_lossy(&output.stdout);
        assert_eq!(s, "foo");
    } else {
        let s = String::from_utf8_lossy(&output.stderr);
        panic!("echo failed and stderr was:\n{}", s);
    }
}

#[test]
fn test_variables() {
    let x = 42;
    let y = 43;
    let var1 = &x;
    let mut var2 = &x;
    var2 = &y;
}

#[test]
fn test_tuples() {
    let rect1 = (30, 50);
    assert_eq!(30, rect1.0);
    assert_eq!(50, rect1.1)
}

#[test]
fn test_traits() {
    struct Player {
        first_name: String,
        last_name: String,
    }

    impl Player {
        fn new(first_name: String, last_name: String) -> Player {
            Player {
                first_name: first_name,
                last_name: last_name,
            }
        }
    }

    trait FullName {
        fn full_name(&self) -> String;
    }

    impl FullName for Player {
        fn full_name(&self) -> String {
            format!("{} {}", self.first_name, self.last_name)
        }
    }

    let player_2 = Player::new("Roger".to_string(), "Federer".to_string());

    assert_eq!(
        format!("Player 02: {}", player_2.full_name()),
        "Player 02: Roger Federer"
    )
}

#[test]
fn test_command_line_args() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
}

#[test]
fn test_current_exe() {
    let exe = env::current_exe().expect("current_exe() failed");
    println!("{:?}", exe)
}

#[test]
fn test_json() {
    use serde::{Deserialize, Serialize};
    use serde_json::{json, Result, Value};

    #[derive(Serialize, Deserialize)]
    struct Person {
        name: String,
        age: u8,
        phones: Vec<String>,
    }

    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    let v: Value = serde_json::from_str(data).unwrap();
    assert_eq!(v["name"], "John Doe");
    assert_eq!(v["age"], 43);

    let p: Person = serde_json::from_str(data).unwrap();
    assert_eq!(p.name, "John Doe");
    assert_eq!(p.age, 43);

    let john = json!({
        "name": "John Doe",
        "age": 43,
        "phones": [
            "+44 1234567",
            "+44 2345678"
        ]
    });
    assert_eq!(john["name"], "John Doe");
    assert_eq!(john["age"], 43);

    #[derive(Serialize, Deserialize)]
    struct Address {
        street: String,
        city: String,
    }
    let address = Address {
        street: "10 Downing Street".to_owned(),
        city: "London".to_owned(),
    };
    let j = serde_json::to_string(&address).unwrap();
    assert_eq!(j, "{\"street\":\"10 Downing Street\",\"city\":\"London\"}")
}

fn main() {}
