use futures::executor::block_on;

pub struct Song {
    author: String,
    name: String,
}

async fn learn_song() -> Song {
    Song {
        author: "周杰伦".to_string(),
        name: String::from("《菊花台》"),
    }
}

async fn sing_song(song: Song) {
  println!("给大家献上一首{}的{} ~ {}", song.author, song.name, "菊花残，满地伤~ ~");
}

async fn  dance() {
  println!("唱到情深处，身体不由自主的动了起来~ ~");
}

async fn learn_and_sing() {
  // 总之，在async fn函数中使用.await可以等待另一个异步调用的完成。但是与block_on不同，.await并不会阻塞当前的线程，而是异步的等待Future A的完成，在等待的过程中，该线程还可以继续执行其它的Future B，最终实现了并发处理的效果
  let song = learn_song().await;
  sing_song(song).await;
}

fn simple_run() {
  let song = block_on(learn_song());
  block_on(sing_song(song));
  block_on(dance());
}

async fn async_run() {
  let f1 = learn_and_sing();
  let f2 = dance();
  futures::join!(f1, f2);
}

pub fn run() {
  block_on(async_run());
}