use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use bytes::Bytes;
use mini_redis::{
    Command,
    Command::{Get, Set},
    Connection, Frame,
};
use tokio::net::{TcpListener, TcpStream};

type Db = Arc<Mutex<HashMap<String, Bytes>>>;
// type ShardedDb = Arc<Vec<Mutex<HashMap<String, Vec<u8>>>>>;


#[tokio::main]
async fn main() {
    // 监听指定地址，等待 TCP 连接进来
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    println!("Listening");
    let db: Db = Arc::new(Mutex::new(HashMap::new()));

    loop {
        // 第二个被忽略的项中包含有新连接的 `IP` 和端口信息

        let (socket, _) = listener.accept().await.unwrap();
        // 将 handle 克隆一份

        let db = db.clone();
        tokio::spawn(async move {
            process(socket, db).await;
        });
    }
}

async fn process(socket: TcpStream, db: Db) {
    let mut connection = Connection::new(socket);
    // let mut db = HashMap::new();
    while let Some(frame) = connection.read_frame().await.unwrap() {
        let res = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                let mut db = db.lock().unwrap();
                db.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                let db = db.lock().unwrap();
                if let Some(val) = db.get(cmd.key()) {
                    Frame::Bulk(val.clone().into())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented {:?}", cmd),
        };
        connection.write_frame(&res).await.unwrap();
    }
}
