mod executor;
mod timerfuture;
mod verona_stubs;

use std::time::Duration;

async fn say_hello() {
    println!("hello");
    timerfuture::TimerFuture::new(Duration::new(2, 0)).await;
    println!("world");

    executor::spawn(async {
        println!("I can spawn like this too");
    })
}

fn main() {
    let exec = executor::Executor::new();

    exec.spawn(say_hello());

    exec.run();
}
