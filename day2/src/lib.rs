use anyhow::{Result};

pub fn part1(s: &str) -> Result<i32> {
    let mut sum = 0;
    for line in s.to_lowercase().lines() {
        let (p1, p2) = line.split_once(" ").unwrap();
        sum += match p1{
            "a" => {
                match p2{
                    "x" => {4}
                    "y" => {8}
                    "z" => {3}
                    _ => {panic!("");}
                }
            }
            "b" =>{
                match p2{
                    "x" => {1}
                    "y" => {5}
                    "z" => {9}
                    _ => {panic!("");}
                }
            }
            "c" => {
                match p2{
                    "x" => {7}
                    "y" => {2}
                    "z" => {6}
                    _ => {panic!("");}
                }
            }
            _ => {panic!("");}
        }
    }
    Ok(sum)
}

pub fn part2(s: &str) -> Result<i32> {
    let mut sum = 0;
    for line in s.to_lowercase().lines() {
        let (p1, p2) = line.split_once(" ").unwrap();
        sum += match p1{
            "a" => {
                match p2{
                    "x" => {3}
                    "y" => {4}
                    "z" => {8}
                    _ => {panic!("");}
                }
            }
            "b" =>{
                match p2{
                    "x" => {1}
                    "y" => {5}
                    "z" => {9}
                    _ => {panic!("");}
                }
            }
            "c" => {
                match p2{
                    "x" => {2}
                    "y" => {6}
                    "z" => {7}
                    _ => {panic!("");}
                }
            }
            _ => {panic!("");}
        }
    }
    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    const CONTENT: &str = "A Y
B X
C Z";

    #[test]
    fn test_part1() {
        assert_eq!(part1(CONTENT).expect(""), 15);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(CONTENT).expect(""), 12);
    }
}