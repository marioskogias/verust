use crate::verona_stubs;

pub struct Executor {
}

impl Executor {
    pub fn new() -> Executor {
        verona_stubs::verona_runtime_init();
        Executor{}
    }

    pub fn run(&self) {
        verona_stubs::verona_scheduler_run();
    }
}
