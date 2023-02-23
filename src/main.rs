use std::{env, process};

use chrono::{Datelike, Local};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 || args[1].len() != 6 {
        println!("Please input {} 198202", args[0]);
        process::exit(1);
    }
    let year_start: i32 = args[1][..4].parse().unwrap();
    let month_start: u32 = args[1][4..].parse().unwrap();
    println!("year: {}, month: {}", year_start, month_start);
    let local = Local::now();
    let year_now = local.year();
    let month_now = local.month();
    let year_80 = year_start + 80;
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
            if year < year_now {
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
    println!("       01 02 03 04 05 06 07 08 09 10 11 12       01 02 03 04 05 06 07 08 09 10 11 12       01 02 03 04 05 06 07 08 09 10 11 12       01 02 03 04 05 06 07 08 09 10 11 12");
}
