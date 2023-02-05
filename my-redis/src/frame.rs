use bytes::Bytes;
pub enum Frame {
  Simple(String),
  Error(String),
  Integer(String),
  Bulk(Bytes),
  Null,
  Array(Vec<Frame>)
}