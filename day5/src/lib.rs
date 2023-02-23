use anyhow::{Result};

pub fn part1(s:&str) -> Result<String> {
    let mut res: String = String::new();
    let (mut stacks, moves) = parse(s);
    for line in moves.iter(){
        for _ in 0..line[0]{
            let s = stacks[(line[1]-1)].pop().expect("");
            stacks[(line[2]-1)].push(s);
        }
    }
    for stack in stacks.iter_mut(){
        res.push(stack.pop().expect(""));
    }
    Ok(res)
}

pub fn part2(s:&str) -> Result<String> {
    let mut res: String = String::new();
    let (mut stacks, moves) = parse(s);
    for line in moves.iter(){
        let (mut t,mut s) = stacks[(line[1]-1)].split_at(stacks[
            (line[1]-1)].len()
            -1-line[0]);
        stacks[(line[2]-1)].push_str(s);
        let s = t.clone();
        stacks[(line[1]-1)] = s.to_string();
    }
    for stack in stacks.iter_mut(){
        res.push(stack.pop().expect(""));
    }
    Ok(res)
}

fn parse(s:&str)->(Vec<String>, Vec<Vec<usize>>){
    let mut stacks: Vec<String> = vec!["".to_string(),"".to_string()];
    let (t1, t2) = s.split_once("\n\n").expect("");
    for line in t1.lines(){
        let gaps = line.len()/4;
        for i in 0..=gaps{
            match stacks.get_mut(i){
                Some( s) => {s.push(line.as_bytes()[1+i*4] as char)},
                None => {stacks.push((line.as_bytes()[1+i*4] as char).to_string())},
            }
        }
    }
    stacks = stacks.iter().map(|stack| stack.chars().rev().collect::<String>().trim().to_string()).collect();
    let mut moves: Vec<Vec<usize>> = vec![];
    for line in t2.lines(){
        let tmp = line
            .replace("move ", "") //
            .replace("from ", "") //
            .replace("to ","") //
            .splitn(3, " ") //
            .map(|s,| s.parse().expect("")) //
            .collect::<Vec<usize>>(); //
        moves.push(tmp);
    }
    /*
    let moves: Vec<Vec<u32>> = t2
        .replace("move ", "")
        .replace("from ", "")
        .replace("to ","")
        .lines()
        .map(|s|{
            s.splitn(3, " ")
                .map(|s| {
                    s.parse().expect("")})
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();
        */
    (stacks, moves)
}

#[cfg(test)]
mod tests {
    use super::*;
    const CONTENT: &str ="    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
    #[test]
    fn test_part1() {
        assert_eq!(part1(CONTENT).expect(""), "CMZ".to_string());
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(CONTENT).expect(""), "MCD".to_string());
    }
}