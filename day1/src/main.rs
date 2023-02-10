
use std::fs;

fn main() {
    let content = fs::read_to_string("./src/input.txt").expect("").replace("\r\n", "\n");;
    println!("{}", most_calories(content.as_str()));
}

fn most_calories(s: &str) -> u32 {
    let mut biggest = 0;
    let mut temp = 0;
    for elf in s.split("\n\n"){
        temp = elf.lines().map(|line| line.parse::<u32>().expect("")).sum();
        if temp > biggest{
            biggest = temp;
        }
    }
    biggest
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn biggestElf() {
        let str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

        assert_eq!(most_calories(str), 24000);
    }


}