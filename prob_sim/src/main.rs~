use std::io;
use std::time::{Instant};

fn main() {
    println!("Enter which probability test you would like to do (dice, coin, monty): ");
    let mut pref = String::new();
    io::stdin().read_line(&mut pref).expect("Thats not an option...");
    let prefs = &pref.trim()[..];
    match prefs {
        "dice" => dicetest(),
        "coin" => cointest(),
        "monty" => montyhalltest(),
        "fib" => fibtest(),
        _ => println!("Thats not an option..."),
    }
}

fn dicetest() {
    println!("How many dice would you like to roll? ");
    let mut s_num = String::new();
    io::stdin().read_line(&mut s_num).expect("failed to read line");
    println!("How many sides would you like your dice to have? ");
    let mut s_sides = String::new();
    io::stdin().read_line(&mut s_sides).expect("failed to read line");
    let num: i32 = match s_num.trim().parse::<i32>() {
        Err(_) => 0,
        Ok(n) => n,
    };
    let sides: f32 = match s_sides.trim().parse::<f32>() {
        Err(_) => 0.0,
        Ok(n) => n,
    };
    let start = Instant::now();
    let expect: f32 = num as f32 * (sides + 1.0) / 2.0;
    let time1 = start.elapsed();
    let mut actual = 0.0;
    for _ in 0..num {
        actual += rand::random_range(1.0..=sides);
    };
    let time2 = start.elapsed() - time1;
    let err: f32 = ((actual - expect) / expect * 100.0).abs();
    println!("The expected total is {}, and the actual total is {}.", expect, actual);
    println!("The time it took to calculate the expectation was {:?}, and the time for calculating the actual total was {:?}.", time1, time2);
    println!("The error was {}%.", err);
}

fn cointest() {
    println!("How many coins would you like to flip? ");
    let mut s_num = String::new();
    io::stdin().read_line(&mut s_num).expect("failed to read line");
    println!("What is the chance of landing on heads (written as a decimal between 0 and 1)? ");
    let mut s_chance = String::new();
    io::stdin().read_line(&mut s_chance).expect("failed to read line");
    let num: i32 = match s_num.trim().parse::<i32>() {
        Err(_) => 0,
        Ok(n) => n,
    };
    let chance: f32 = match s_chance.trim().parse::<f32>() {
        Err(_) => 0.0,
        Ok(n) => n,
    };
    let start = Instant::now();
    let expect: f32 = num as f32 * chance;
    let time1 = start.elapsed();
    let mut actual = 0.0;
    for _ in 0..num {
        actual += if rand::random_range(0_f32..=1_f32) <= chance {1.0} else {0.0};
    };
    let time2 = start.elapsed() - time1;
    let err: f32 = ((actual - expect) / expect * 100.0).abs();
    println!("The expected total amount of heads is {}, and the actual total is {}.", expect, actual);
    println!("The time it took to calculate the expectation was {:?}, and the time for calculating the actual total was {:?}.", time1, time2);
    println!("The error was {}%.", err);
}

fn montyhalltest() {
    println!("this is the monty hall problem test");
}

fn fibtest() {
    println!("Enter which # Fibonnaci number you want: ");
    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("failed to read line");
    let start = Instant::now();
    let result1 = match num.trim().parse::<i32>() {
        Err(_) => "nonexistent",
        Ok(n) => match fibhard(n) {
            None => "nonexistent",
            Some(res) => &res.to_string(),
        },
    };
    let time1 = start.elapsed();
    let result2 = match num.trim().parse::<i32>() {
        Err(_) => "nonexistent",
        Ok(n) => match fib(n) {
            None => "nonexistent",
            Some(res) => &res.to_string(),
        },
    };
    let time2 = start.elapsed() - time1;
    let err: f32 = match result1.trim().parse::<f32>() {
        Err(_) => 0.0,
        Ok(n) => match result2.trim().parse::<f32>() {
            Err(_) => 0.0,
            Ok(n2) => ((n - n2) / n * 100.0).abs(),
        },
    };   
    println!("The {}th Fibonnaci number is {} with the long method and {} with the short method.", num.trim(), result1, result2);
    println!("The long method took {:?} to complete, and the short method took {:?}.", time1, time2);
    println!("The error in the short version is {}%.", err);


}

fn fibhard(n: i32) -> Option<i32> {
    if (n == 0) || (n == 1) {
        Some(1)
    } else if n < 0 {
        None
    } else {
        match (fib(n-1),fib(n-2)) {
            (Some(a),Some(b)) => Some(a + b),
            _ => None,
        }
    }
}

fn fib(n: i32) -> Option<i32> {
    if (n == 0) || (n == 1) {
        Some(1)
    } else if n < 0 {
        None
    } else {
        let n1: f32 = (1.0 + 5_f32.sqrt()) / 2.0;
        let n2: f32 = (1.0 - 5_f32.sqrt()) / 2.0;
        Some(((n1.powi(n) - n2.powi(n)) / 5_f32.sqrt()) as i32)
    }
}
