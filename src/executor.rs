use crate::verona_stubs;
use futures::{
    future::{FutureExt,UnsafeFutureObj},
};
use std::{
    future::Future,
};

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

    pub fn spawn(&self, future: impl Future<Output = ()> + 'static + Send) {
        let raw_ptr = future.boxed().into_raw();
        verona_stubs::verona_schedule_task(raw_ptr);
    }

}
