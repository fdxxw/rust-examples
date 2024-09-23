use bytes::Bytes;
use mini_redis::client;
use tokio::sync::{mpsc, oneshot};

type Responder<T> = oneshot::Sender<mini_redis::Result<T>>;

#[derive(Debug)]
enum Command {
    Set {
        key: String,
        val: Bytes,
        resp: Responder<()>,
    },
    Get {
        key: String,
        resp: Responder<Option<Bytes>>,
    },
}

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(32);
    // tokio::spawn(async move {
    //     for i in 0..10 {
    //         if let Err(_) = tx.send(i).await {
    //             println!("receiver dropped");
    //             return;
    //         }
    //     }
    // });
    // let tx2 = tx.clone();
    // tokio::spawn(async move {
    //   tx.send("sending fron first handle").await.unwrap();
    // });
    // tokio::spawn(async move {
    //   tx2.send("sending from second handle").await.unwrap();
    // });
    // while let Some(i) = rx.recv().await {
    //   println!("got = {}", i);
    // }

    let manager = tokio::spawn(async move {
        let mut client = client::connect("127.0.0.1:6379").await.unwrap();
        while let Some(cmd) = rx.recv().await {
            use Command::*;
            match cmd {
                Get { key, resp } => {
                    let res = client.get(&key).await;
                    let _ = resp.send(res);
                }
                Set { key, val, resp } => {
                    let res = client.set(&key, val).await;
                    let _ = resp.send(res);
                }
            }
        }
    });
    let tx2 = tx.clone();
    let t1 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Command::Get {
            key: "hello".to_string(),
            resp: resp_tx,
        };
        tx.send(cmd).await.unwrap();
        let res = resp_rx.await;
        println!("GOT = {:?}", res);
    });
    let t2 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Command::Set {
            key: "hello".to_string(),
            val: "world".into(),
            resp: resp_tx,
        };
        tx2.send(cmd).await.unwrap();
        let res = resp_rx.await;
        println!("GOT = {:?}", res);
    });
    t1.await.unwrap();
    t2.await.unwrap();
    manager.await.unwrap();
}
