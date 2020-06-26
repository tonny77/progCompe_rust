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
  let five_hundred:i32 = read();
  let hundred:i32 = read();
  let fifty:i32 = read();
  let target_num:i32 = read();
  let mut count:u32 = 0;

  for i in 0..fifty + 1 {
    for j in 0..hundred + 1 {

      if (target_num - (i*50) - (j*100)) < 0 {
        break;
      }

      if ((target_num - (i*50) - (j*100)) % 500 == 0) && (five_hundred >= ((target_num - (i*50) - (j*100)) / 500)) {
        count += 1;
      }
    }
  }


  println!("{}", count);

}