#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(warnings, unused)]

use std::char;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::io;
use std::io::Error;
use std::mem;
use std::process::Command;
use std::str::FromStr;
use chrono::NaiveDate;
use serde::Deserialize;

#[test]
fn test_hello_format() {
    print!("hello");
    let name = "brian!";
    assert_eq!(format!("Hello {}", name), "Hello brian!");
}

#[test]
fn test_mem_size() {
    let x = 6;
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
    // exe is something like "/workspaces/rust-playground/target/debug/deps/rust_playground-0668dbea08391dc4"
    let exe = env::current_exe().expect("current_exe() failed");
    let path_str = exe.to_string_lossy().to_string();
    let file_name_substring = "rust_playground";
    assert!(path_str.contains(file_name_substring), "Path does not contain the expected substring");
    println!("{:?}", exe)
}

#[test]
fn test_json_serialization() {
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
    assert_eq!(v["phones"][0], "+44 1234567");
    assert_eq!(v["phones"][1], "+44 2345678");

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

#[test]
fn test_dir_listing_for_src_main() {
    let mut files = Vec::new();
    let dir = Path::new("./src");

    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(_) => {
            panic!("Could not read directory");
        }
    };

    for entry_result in entries {
        // Handle the Result for each entry
        match entry_result {
            Ok(entry) => {
                let path = entry.path();
                if path.is_file() {
                    files.push(path.to_string_lossy().into_owned());
                }
            }
            Err(_) => {
                // Handle the error here if needed
            }
        }
    }

    assert_eq!(files[0], "./src/main.rs")
    
}

#[test]
fn test_create_and_read_sample_csv() -> Result<(), Error> {

    #[derive(Debug, Deserialize)]
    struct Record {
        name: String,
        age: u8,
        date_of_birth: NaiveDate,
    }


    // Create a temporary directory for the test
    let mut tmp_dir = std::env::temp_dir();
    tmp_dir.push("rust_csv_test");
    if !tmp_dir.exists() {
        fs::create_dir(&tmp_dir)?;
    }

    // Create a sample CSV file inside the temporary directory
    let mut file_path = tmp_dir.clone();
    file_path.push("sample.csv");
    let mut wtr = csv::Writer::from_path(&file_path)?;

    // Write the headers
    wtr.write_record(&["name", "age", "date_of_birth"])?;

    // Write sample data
    wtr.write_record(&["Alice", "30", "1991-02-01"])?;
    wtr.write_record(&["Bob", "40", "1981-03-15"])?;
    wtr.write_record(&["Charlie", "25", "1996-07-21"])?;

    wtr.flush()?;

    // Verify that the file has been created
    assert!(file_path.exists(), "CSV file should exist");

    // Read the CSV into a Vec of Record structs
    let mut rdr = csv::Reader::from_path(&file_path)?;
    let mut records: Vec<Record> = Vec::new();

    for result in rdr.deserialize() {
        let record: Record = result?;
        records.push(record);
    }

    // Verify that the records are correct (replace with your actual test logic)
    assert_eq!(records.len(), 3, "Should have read 3 records");
    assert_eq!(records[0].name, "Alice");
    assert_eq!(records[0].age, 30);
    assert_eq!(records[0].date_of_birth, NaiveDate::from_ymd(1991, 2, 1));

    Ok(())
}

fn main() {}
