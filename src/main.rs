mod verona_stubs;
mod executor;

fn main() {
    println!("Hello, world!");
    verona_stubs::verona_marios_println();

    let exec = executor::Executor::new();

    exec.run()
}
