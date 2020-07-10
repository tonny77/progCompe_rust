use std::io::*;
use std::str::FromStr;
use std::collections::HashSet;

fn read<T: FromStr>() -> T {
    let stdin = stdin();
    let stdin = stdin.lock();
    let token: String = stdin
        .bytes()
        .map(|c| c.expect("failed to read char") as char)
        .skip_while(|c| c.is_whitespace())
        .take_while(|c| !c.is_whitespace())
        .collect();
    token.parse().ok().expect("failed to parse token")
}

fn main() {
  let n:usize = read();
  let mut a: Vec<u32> = (0..n).map(|_| read()).collect();
  a.sort_by(|x, y| y.cmp(x));

  let uniq: HashSet<u32> = a.into_iter().collect();
  let answer:usize = uniq.len();

  println!("{}", answer);
}