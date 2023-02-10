use anyhow::{Result};

pub fn part1(s:&str) -> Result<u32> {
    let mut sum: u32 = 0;
    let offset1 = 'A' as u32;
    let offset2 = 'a' as u32;
    for rucksack in s.lines(){
        let (comp1, comp2) = rucksack.split_at(rucksack.len()/2);
        let mut item = ' ' as u32;
        for letter in comp2.chars(){
            if comp1.contains(letter) {
                item = letter as u32;
                break;
            }
        }
        sum += if item < offset2 {
            item-offset1+26+1
        }
        else {
            item-offset2+1
        }
    }
    Ok(sum)
}

pub fn part2(s:&str) -> Result<u32> {
    let mut sum: u32 = 0;
    let offset1 = 'A' as u32;
    let offset2 = 'a' as u32;
    for rucksack in s.lines(){
        let (comp1, comp2) = rucksack.split_at(rucksack.len()/2);
        let mut item = ' ' as u32;
        for letter in comp2.chars(){
            if comp1.contains(letter) {
                item = letter as u32;
                break;
            }
        }
        sum += if item < offset2 {
            item-offset1+26+1
        }
        else {
            item-offset2+1
        }
    }
    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;
    const CONTENT: &str ="vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
    #[test]
    fn test_part1() {
        assert_eq!(part1(CONTENT).unwrap(), 157);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(CONTENT).unwrap(), 70);
    }
}