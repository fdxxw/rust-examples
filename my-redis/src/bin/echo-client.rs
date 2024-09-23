use tokio::net::TcpStream;
use tokio::io::{self, AsyncWriteExt, AsyncReadExt};
#[tokio::main]
async fn main() -> io::Result<()> {
  let socket = TcpStream::connect("127.0.0.1:6142").await?;
  let (mut rd, mut wr) = io::split(socket);
  tokio::spawn(async move {
    wr.write_all(b"hello\r\n").await?;
    wr.write_all(b"world\r\n").await?;
    // 有时，我们需要给予 Rust 一些类型暗示，它才能正确的推导出类型
    Ok::<_, io::Error>(())
  });
  let mut buf = vec![0;128];
  loop {
    let n = rd.read(&mut buf).await?;
    if n == 0 {
      break;
    }
    println!("GOT = {:?}", &buf[..n]);
  }
  Ok(())
}