use std::io::*;
use std::str::FromStr;


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

  // 可変の値を読み込む場合
  let mut a: Vec<u32> = (0..n).map(|_| read()).collect();
  a.sort_by(|x, y| y.cmp(x));
  // println!("{:?}", a);
  let mut alice = 0;
  let mut bob = 0;

  for (index, x) in a.iter().enumerate() {
    if index % 2 == 0 {
      alice += x;
    } else {
      bob += x;
    }
  }

  println!("{}", alice - bob);
}