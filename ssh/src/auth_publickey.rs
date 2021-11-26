use ssh2::Session;
use std::net::TcpStream;
extern crate dirs;
pub fn run() {
  // Connect to the local SSH server
  let tcp = TcpStream::connect("192.168.13.67:22").unwrap();
  let mut sess = Session::new().unwrap();
  sess.set_tcp_stream(tcp);
  sess.handshake().unwrap();

  // Try to authenticate with the first identity in the agent.
  sess
    .userauth_pubkey_file(
      "root",
      None,
      dirs::home_dir()
        .unwrap()
        .join(".ssh")
        .join("id_rsa").as_path(),
      None,
    )
    .unwrap();
  // Make sure we succeeded
  assert!(sess.authenticated());
}
