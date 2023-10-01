mod executor;
mod timerfuture;
mod verona_stubs;

use std::time::Duration;

async fn say_hello() {
    println!("hello");
    println!("Will create a time future and await it");
    timerfuture::TimerFuture::new(Duration::new(2, 0)).await;
    println!("world");
}

fn main() {
    let exec = executor::Executor::new();

    exec.spawn(say_hello());

    exec.run();
}
