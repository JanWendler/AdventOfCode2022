use anyhow::{Result};

pub fn part1(s: &str) -> Result<i32> {
    let mut sum= s.lines()
        .map(|line| {
            line.split_once(",").unwrap()})
        .map(|(p1, p2)| {
            (p1.split_once("-").unwrap(),
            p2.split_once("-").unwrap())})
        .map(|((p11, p12), (p21, p22))|
                 ((p11.parse::<u32>().expect(""),
                 p12.parse::<u32>().expect("")),
                  (p21.parse::<u32>().expect(""),
                 p22.parse::<u32>().expect(""))))
        .filter(|((p11, p12),(p21,p22))| {
            contains(p11, p12, p21, p22)})
        .count();
    Ok(sum as i32)
}

fn contains(p11: &u32, p12: &u32, p21: &u32, p22: &u32) -> bool {
    if *p11 == *p21 {return true; }
    if *p11 <= *p21 {
        if *p12 >= *p22 {
            return true;
        }
    } else {
        if *p12 <= *p22 {
            return true;
        }
    }
    false
}

pub fn part2(s: &str) -> Result<i32> {
    let mut sum= s.lines()
                  .map(|line| {
                      line.split_once(",").unwrap()})
                  .map(|(p1, p2)| {
                      (p1.split_once("-").unwrap(),
                       p2.split_once("-").unwrap())})
                  .map(|((p11, p12), (p21, p22))|
                      ((p11.parse::<u32>().expect(""),
                        p12.parse::<u32>().expect("")),
                       (p21.parse::<u32>().expect(""),
                        p22.parse::<u32>().expect(""))))
                  .filter(|((p11, p12),(p21,p22))| {
                      contains2(p11, p12, p21, p22)})
                  .count();
    Ok(sum as i32)
}
fn contains2(p11: &u32, p12: &u32, p21: &u32, p22: &u32) -> bool {
    if *p11 == *p21 {
        return true;
    }
    if *p11 <= *p21 {
        if *p12 >= *p21{
            return true;
        }
    }
    if *p11 >= *p21 {
        if *p11 <= *p22{
            return true;
        }
    }
    false
}
#[cfg(test)]
mod tests {
    use super::*;

    const CONTENT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
4-5,4-5
3-3,3-3
3-6,3-5
2-5,3-5
5-6,5-8
5-6,3-8";

    #[test]
    fn test_part1() {
        assert_eq!(part1(CONTENT).unwrap(), 8);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(CONTENT).unwrap(), 10);
    }
}