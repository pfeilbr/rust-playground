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

fn main() {
    // hello();
    // for_loop();
    // user_input();
    // generate_random_number();
}
