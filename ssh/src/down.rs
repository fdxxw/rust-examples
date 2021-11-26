use ssh2::Session;
use std::io::prelude::*;
use std::net::TcpStream;
use std::path::Path;
pub fn run() {
  // Connect to the local SSH server
  let tcp = TcpStream::connect("192.168.13.67:22").unwrap();
  let mut sess = Session::new().unwrap();
  sess.set_tcp_stream(tcp);
  sess.handshake().unwrap();
  sess.userauth_password("root", "123456").unwrap();

  let (mut remote_file, stat) = sess.scp_recv(Path::new("remote")).unwrap();
  println!("remote file size: {}", stat.size());
  let mut contents = Vec::new();
  remote_file.read_to_end(&mut contents).unwrap();

  // Close the channel and wait for the whole content to be tranferred
  remote_file.send_eof().unwrap();
  remote_file.wait_eof().unwrap();
  remote_file.close().unwrap();
  remote_file.wait_close().unwrap();
}
