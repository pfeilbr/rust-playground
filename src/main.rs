#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(warnings, unused)]

use std::io;
use std::char;
use std::str::FromStr;
use std::process::Command;

#[test]
fn test_hello_format() {
    let name = "brian";
    assert_eq!(format!("Hello {}!", name), "Hello brian!");
}

#[test]
fn for_loop() {
    let mut s = String::new();
    for i in 0..3 {
        s.push_str(i.to_string().as_str())
    }
    assert_eq!("012", s.as_str())
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
    assert_eq!(1,1)
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
fn ignored_test() {
    assert_eq!(0, 0);
}

#[test]
fn test_child_process() {
    let output = Command::new("echo")
        .arg("-n")
        .arg("foo")
        .output().unwrap_or_else(|e| {
            panic!("failed to execute process: {}", e)
    });

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
                first_name : first_name,
                last_name : last_name,
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

    let player_2 = Player::new("Roger".to_string(),"Federer".to_string());

    assert_eq!(format!("Player 02: {}", player_2.full_name()), "Player 02: Roger Federer")
        
}


fn main() {
    
}
