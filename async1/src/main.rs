use futures::executor::block_on;
mod song;
fn main() {
    // hello();
    song::run();
}

fn hello() {
    let future = hello_world();
    block_on(future);
}

async fn hello_world() {
    hello_cat().await;
    println!("hello, world!");
}

async fn hello_cat() {
    println!("hello, cat")
}