use std::io::Cursor;

use bytes::{Buf, Bytes, BytesMut};
use tokio::{
    io::{AsyncReadExt, BufWriter},
    net::{TcpListener, TcpStream},
};

pub type Error = Box<dyn std::error::Error + Send + Sync>;
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:6142").await?;

    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            process_socket(socket).await;
        });
    }
}

async fn process_socket(socket: TcpStream) {
    let mut conn = Connection::new(socket);
    loop {
        if let Ok(Some(frame)) = conn.read_frame().await {
            println!("GOT = {:?}", frame);
        }
    }
}

struct Connection {
    stream: BufWriter<TcpStream>,
    buffer: BytesMut,
}
impl Connection {
    pub fn new(stream: TcpStream) -> Self {
        Connection {
            stream: BufWriter::new(stream),
            buffer: BytesMut::with_capacity(4096), // 缓冲区
        }
    }
    pub async fn read_frame(&mut self) -> Result<Option<Frame>, Error> {
        loop {
            if let Some(frame) = self.parse_frame()? {
                return Ok(Some(frame));
            }
            if 0 == self.stream.read_buf(&mut self.buffer).await? {
                if self.buffer.is_empty() {
                    return Ok(None);
                } else {
                    return Err("Connection reset by peer".into());
                }
            }
        }
    }
    fn parse_frame(&mut self) -> Result<Option<Frame>, Error> {
        let mut buf = Cursor::new(&self.buffer[..]);
        buf.set_position(0);
        let len = buf.get_ref().len();
        let frame = Frame::parse(&mut buf)?;
        self.buffer.advance(len);
        return Ok(Some(frame));
    }
}

#[derive(Debug)]
enum Frame {
    Bulk(Bytes),
}

impl Frame {
    pub fn parse(src: &mut Cursor<&[u8]>) -> Result<Frame, Error> {
        if !src.has_remaining() {
            return Err("Error::Incomplete".into());
        }
        // let start = src.position() as usize;
        let len = src.get_ref().len() - 1;
        let data = Bytes::copy_from_slice(&src.chunk()[..len]);
        Ok(Frame::Bulk(data))
    }
}

/// Find a line
fn get_line<'a>(src: &mut Cursor<&'a [u8]>) -> Result<&'a [u8], Error> {
    // Scan the bytes directly
    let start = src.position() as usize;
    // Scan to the second to last byte
    let end = src.get_ref().len() - 1;

    for i in start..end {
        if src.get_ref()[i] == b'\r' && src.get_ref()[i + 1] == b'\n' {
            // We found a line, update the position to be *after* the \n
            src.set_position((i + 2) as u64);

            // Return the line
            return Ok(&src.get_ref()[start..i]);
        }
    }

    Err("Error::Incomplete".into())
}
