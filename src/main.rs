#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(warnings, unused)]

use rand::Rng;
use std::io;

fn hello() {
    let name = "brian";
    println!("Hello {}!", name);
}

fn for_loop() {
    for i in 0..9 {
        println!("i: {}", i);
    }
}

fn user_input() {
    println!("Pick a number.");

    let mut number = String::new();

    match io::stdin().read_line(&mut number) {
        Ok(_) => println!("You guessed: {}", number),
        Err(e) => println!("Failed to read line: {}", e),
    }
}

fn generate_random_number() {
    let num: i32 = rand::thread_rng().gen();
    println!("random number: {}", num);

    let low = 0;
    let high = 9;
    let ranged_num = rand::thread_rng().gen_range(low, high);
    println!("ranged random number ({}-{}): {}", low, high, ranged_num);
}

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

fn struct_example() {
    let p = Person {
        name: "brian",
        age: 42,
    };
    println!("{:?}", p);
}

// async fn app() -> Result<()> {
//     //todo!()
//     future::ok("Hello world".to_string()).await
// }

// fn async_example_start() {
//     let mut rt = tokio::runtime::Runtime::new().unwrap();
//     let future = app();
//     rt.block_on(future);
// }

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
#[ignore]
fn ignored_test() {
    assert_eq!(0, 0);
}

#[test]
fn test_child_process() {
    use std::process::Command;
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

fn main() {
    // hello();
    // for_loop();
    // user_input();
    // generate_random_number();
    struct_example();
    
}
