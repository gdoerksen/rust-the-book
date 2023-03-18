use std::io::{self, BufRead};

fn vec_minus(mut vector: Vec<u32>)->Option<u32> 
    {
    let second = vector.pop();
    let first = vector.pop();
    match (first, second) {
        (Some(x), Some(y)) =>  Some(y - x + 1),
        _ => None,

    }
}

fn solve(line: &str) -> u32 {
    let mut count = 0;
    let parts = line.split(';');
    for part in parts  {
        if part.contains('-') {
            let sub_parts: Vec<u32> = part
                                      .split('-')
                                      .map(|x| x.parse::<u32>().unwrap()) //#TODO unwrap
                                      .collect();

            count += vec_minus(sub_parts).ok_or( 0).unwrap();
        } else {
            count += 1;
        }

    }
    count
}

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines().map(|l| l.unwrap()) {
        let solution = solve(&line);
        println!("{solution}");
    }
}

# [cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_1() {
        assert_eq!( 9, solve("1-3;5;7;10-13"))
    }

    #[test]
    fn sample_2() {
        assert_eq!( 8, solve("3-10"))
    }

    #[test]
    fn sample_3() {
        assert_eq!( 5, solve("1;3;5;7;13"))
    }

}
