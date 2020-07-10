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
  let bill_count:u32 = read();
  let total_money:u32 = read();
  let mut ten_thousand:u32 = 0;
  let mut five_thousand:u32 = 0;
  let mut thousand:u32 = 0;
  let mut result:u32 = 0;

  for i in 0..bill_count+1 {
    for j in 0..bill_count+1 {
      if i+j > bill_count {
        continue;
      }
      result = (i*10000) + (j*5000) + ((bill_count-i-j)*1000);
      if result == total_money {
        ten_thousand = i;
        five_thousand = j;
        thousand = bill_count-i-j;
        println!("{} {} {}", ten_thousand, five_thousand, thousand);
        return;
      }
    }
  }

  println!("-1 -1 -1");


}