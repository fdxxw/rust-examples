use std::io::Cursor;

use bytes::{Buf, BufMut, Bytes, BytesMut};
use mini_redis::{Result, Frame};
use tokio::{
    io::{AsyncReadExt, BufWriter, AsyncWriteExt},
    net::TcpStream,
};

// use crate::Frame;

pub struct Connection {
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
    pub async fn read_frame(&mut self) -> Result<Option<mini_redis::Frame>> {
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
    pub async fn write_frame(&mut self, frame: &mini_redis::Frame) -> Result<()> {
        match frame {
          Frame::Simple(val) => {
            self.stream.write_u8(b'+').await?;
            self.stream.write_all(val.as_bytes()).await?;
            self.stream.write_all(b"\r\n").await?;
          },
          Frame::Error(val) => {
            self.stream.write_u8(b'-').await?;
            self.stream.write_all(val.as_bytes()).await?;
            self.stream.write_all(b"\r\n").await?;
          },
          Frame::Null => {
            self.stream.write_all(b"$-1\r\n").await?;

          },
          Frame::Integer(val) => {
            self.stream.write_u8(b':').await?;
            // self.write_decimal(*val).await?;
          },
          Frame::Bulk(val) => {
            let len = val.len();

            self.stream.write_u8(b'$').await?;
            // self.write_decimal(len as u64).await?;
            self.stream.write_all(val).await?;
            self.stream.write_all(b"\r\n").await?;
          },
          Frame::Array(_) => unimplemented!()
        }
        self.stream.flush().await?;
        Ok(())
    }
    fn parse_frame(&mut self) -> Result<Option<mini_redis::Frame>> {
        let mut buf = Cursor::new(&self.buffer[..]);
        match mini_redis::Frame::check(&mut buf) {
            Ok(_) => {
                let len = buf.position() as usize;
                buf.set_position(0);
                let frame = mini_redis::frame::Frame::parse(&mut buf)?;
                self.buffer.advance(len);
                Ok(Some(frame))
            }
            Err(mini_redis::frame::Error::Incomplete) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }
}
