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
  let n = read();
  println!(
      "{}",
      (0..n)
          .map(|_| read::<u32>().trailing_zeros())
          .min()
          .unwrap()
  );
}
