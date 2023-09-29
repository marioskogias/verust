mod verona_stubs;
mod executor;

async fn say_hello() {
    println!("hello");
    println!("world");
}

fn main() {
    println!("Hello, world!");
    verona_stubs::verona_marios_println();

    let exec = executor::Executor::new();

    exec.spawn(say_hello());

    exec.run()
}
