use std::{env, process};

use chrono::{Datelike, Local};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 || args[1].len() != 6 {
        println!("Please input {} 198202", args[0]);
        process::exit(1);
    }
    let year_start: i32 = args[1][..4].parse().unwrap();
    let month_start: i32 = args[1][4..].parse().unwrap();
    let local = Local::now();
    let year_now = local.year();
    let month_now: i32 = local.month().try_into().unwrap();
    let year_80 = year_start + 80;
    let percent = (((year_now - year_start) * 12 + (month_now - month_start)) * 100) / (80 * 12);
    for _ in 0..168 {
        print!("-");
    }
    println!("");
    print!("|");
    let mut x_count = (percent * 168) / 100;
    if x_count > 160 {
        x_count = 160;
    }
    for _ in 0..x_count {
        print!("=");
    }
    print!(">[{percent}%]");
    for _ in 0..(160 - (percent * 168) / 100) {
        print!("-");
    }
    println!("|");
    for _ in 0..168 {
        print!("-");
    }
    println!("");
    println!("       01 02 03 04 05 06 07 08 09 10 11 12       01 02 03 04 05 06 07 08 09 10 11 12       01 02 03 04 05 06 07 08 09 10 11 12       01 02 03 04 05 06 07 08 09 10 11 12");
    let mut line = 0;
    for year in year_start..year_80 {
        print!("  {year} ");
        for month in 1..13 {
            if month == 1 {
                print!(" ");
            } else {
                print!("  ");
            }
            if year == year_start {
                if month < month_start {
                    print!("O");
                } else {
                    print!("X");
                }
            } else if year < year_now {
                print!("X");
            } else if year == year_now {
                if month < month_now {
                    print!("X");
                } else {
                    print!("O");
                }
            } else {
                print!("O");
            }
        }
        line += 1;
        if line > 3 {
            line = 0;
            print!("\n");
        }
    }
}
