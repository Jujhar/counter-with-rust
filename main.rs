use std::io;

fn main() {
	let mut count = 0;
    println!("Add one? (Y/n): ");
	loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .ok()
            .expect("Couldn't read line");
        if input.trim() == "y" || input.trim() == "" {
            count += 1;
        } else if input.trim() == "yy" {
            count += 2;
        } else if input.trim() == "yyy" {
            count += 3;
        } else if input.trim() == "yyyy" {
            count += 4;
        } else if input.trim() == "yyyyy" {
            count += 5;
        } else if input.trim() == "yyyyyy" {
            count += 6;
        } else if input.trim() == "yyyyyyy" {
            count += 7;
        } else if input.trim() == "yyyyyyyy" {
            count += 8;
        } else if input.trim() == "yyyyyyyyy" {
            count += 9;
        } else if input.trim() == "yyyyyyyyyy" {
            count += 10;
        } else if input.trim() == "n" {
            count -= 1;
        }

        print!(" {}", count);
        println!();
        println!();
        println!("Add one? (Y/n): ");
    }
}

// Made by Rebabre 2019
