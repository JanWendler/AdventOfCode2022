use anyhow::{Result};

pub fn part1(s: &str) -> Result<usize> {
    const PACKET_LENGTH: usize = 4;
    for i in 0..s.len() {
        if is_different(&s[i..(i + PACKET_LENGTH)]) {
            return Ok(i + PACKET_LENGTH);
        }
    }
    Ok(0)
}

pub fn part2(s: &str) -> Result<usize> {
    const MESSAGE_LENGTH: usize = 14;
    for i in 0..s.len() {
        if is_different(&s[i..(i + MESSAGE_LENGTH)]) {
            return Ok(i + MESSAGE_LENGTH);
        }
    }
    Ok(0)
}

fn is_different(s: &str) -> bool {
    for i in 0..s.len() - 1 {
        for j in i + 1..s.len() {
            if s.as_bytes()[i] == s.as_bytes()[j] {
                return false;
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    const CONTENT: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";

    #[test]
    fn test_part1() {
        assert_eq!(part1(CONTENT).expect(""), 7);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(CONTENT).expect(""), 19);
    }
}