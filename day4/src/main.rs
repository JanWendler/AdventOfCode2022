use std::fs;
use anyhow::{Context, Result};

fn main()->Result<()>{
    let content = fs::read_to_string("./src/input.txt")?.replace("\r\n", "\n");
    println!("Result of part 1 = {}", day4::part1(content.as_str()).context("")?);
    println!("Result of part 2 = {}", day4::part2(content.as_str()).context("")?);
    Ok(())
}
