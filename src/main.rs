mod executor;
mod verona_stubs;

async fn say_hello() {
    println!("hello");
    println!("world");
}

fn main() {
    let exec = executor::Executor::new();

    exec.spawn(say_hello());

    exec.run()
}
