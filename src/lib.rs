use std::path::Path;
use std::time::Instant;

pub mod day_01_secret_entrance;

pub fn solve_all() {
    println!("Advent of Code 2024\n");

    day_01();
    // day_02();
    // day_03();
    // day_04();
    // day_05();
    // day_06();
    // day_07();
    // day_08();
    // day_09();
    // day_10();
    // day_11();
    // day_12();
}


fn day_01() {
    let file = Path::new("./data/day_01_secret_entrance.txt").to_str().unwrap();
    let timer = Instant::now();
    let value = day_01_secret_entrance::solve_day_01(file).unwrap();

    println!(
        "Day 1: Secret Entrance\n\
        Run Time: {:?}\n\
        Number of Times on Zero 1: {}\n\
        Number of Zero Crosses 2: {}\n",
        timer.elapsed(), value.0, value.1
    );
}
