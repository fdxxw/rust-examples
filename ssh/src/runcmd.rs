use ssh2::Session;
use std::io::prelude::*;
use std::net::TcpStream;
pub fn run() {
  // Connect to the local SSH server
  let tcp = TcpStream::connect("192.168.13.67:22").unwrap();
  let mut sess = Session::new().unwrap();
  sess.set_tcp_stream(tcp);
  sess.handshake().unwrap();
  sess.userauth_password("root", "123456").unwrap();
  let mut channel = sess.channel_session().unwrap();
  channel.exec("ls").unwrap();
  let mut s = String::new();
  channel.read_to_string(&mut s).unwrap();
  println!("{}", s);
  channel.wait_close().expect("close exception");
  println!("{}", channel.exit_status().unwrap());
}
