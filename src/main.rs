use std::io::{self, Read};

use vivoxrs::hello_vivox;

#[tokio::main]
async fn main() -> io::Result<()> {
  let mut buffer = String::new();
  let stdin = io::stdin();
  let mut handle = stdin.lock();

  tokio::spawn(async move {
    hello_vivox();
  });

  handle.read_to_string(&mut buffer)?;
  Ok(())
}
