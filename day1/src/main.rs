
use std::fs;

fn main() {
    let content = fs::read_to_string("./src/input.txt").expect("").replace("\r\n", "\n");
    println!("{}", most_calories(content.as_str()));
}

fn most_calories(s: &str) -> u32 {
    let mut biggest:[u32;3] = [0,0,0];
    let mut temp = 0;
    for elf in s.split("\n\n"){
        temp = elf.lines().map(|line| line.parse::<u32>().expect("")).sum();
        if temp > biggest[0]{
            biggest[2] = biggest[1];
            biggest[1] = biggest[0];
            biggest[0] = temp;
        } else if temp > biggest[1]{
            biggest[2] = biggest[1];
            biggest[1] = temp;
        } else if  temp > biggest[2]{
            biggest[2] = temp;
        }

    }
    biggest.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    const CONTENT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
    #[test]
    fn biggest_elf() {
        assert_eq!(most_calories(CONTENT), 24000);
    }
    #[test]
    fn part2() {
        assert_eq!(most_calories(CONTENT), 45000);
    }


}