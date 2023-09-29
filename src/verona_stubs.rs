#[link(name = "verona")]
extern "C" {
    fn marios_print();
    fn runtime_init();
    fn scheduler_run();
}

pub fn verona_marios_println() {
    unsafe {
        marios_print();
    }
}

pub fn verona_runtime_init() {
    unsafe {
        runtime_init();
    }
}

pub fn verona_scheduler_run() {
    unsafe {
        scheduler_run();
    }
}
