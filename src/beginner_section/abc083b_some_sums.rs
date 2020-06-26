// ----------------------------------------------------------------
// [コピペ](https://qiita.com/tubo28/items/e6076e9040da57368845)
// 標準入力を行うツール
// ----------------------------------------------------------------
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

// ----------------------------------------------------------------
// main
// ----------------------------------------------------------------
fn main() {
  let num:u32 = read();
  let min:u32 = read();
  let max:u32 = read();
  let mut sum:u32 = 0;
  let mut target_num:u32 = 0;
  let mut answer:u32 = 0;

  for i in 1..num+1 {
    target_num = i;
    sum += i % 10;
    while target_num / 10 != 0 {
      target_num /= 10;
      sum += target_num % 10;
    }
    if min <= sum && sum <= max {
      answer += i;
    }
    sum = 0;
  }

  println!("{}", answer);
}