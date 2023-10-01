use crate::verona_stubs;
use futures::{
    future::{BoxFuture, FutureExt},
    task::{waker_ref, ArcWake},
};
use std::{
    ffi::c_void,
    future::Future,
    sync::{Arc, Mutex},
    task::Context,
};

pub struct Executor {}

pub struct Task {
    future: Mutex<BoxFuture<'static, ()>>,
}

impl ArcWake for Task {
    fn wake_by_ref(_arc_self: &Arc<Self>) {}
}

impl Executor {
    pub fn new() -> Executor {
        verona_stubs::verona_runtime_init();
        Executor {}
    }

    pub fn run(&self) {
        verona_stubs::verona_scheduler_run();
    }

    pub fn spawn(&self, future: impl Future<Output = ()> + 'static + Send) {
        let boxed_future = future.boxed();
        let boxed_task = Arc::new(Task {
            future: Mutex::new(boxed_future),
        });
        verona_stubs::verona_schedule_task(boxed_task);
    }
}

#[no_mangle]
pub extern "C" fn poll_future_in_rust(task: *mut c_void) {
    unsafe {
        let raw_ptr = task as *mut Task;
        let boxed_task = Arc::from_raw(raw_ptr);

        let mut boxed_future = boxed_task.future.lock().unwrap();
        //let mw = Arc::new(MyWaker{});
        //let waker = waker_ref(&mw);
        let waker = waker_ref(&boxed_task);
        let context = &mut Context::from_waker(&waker);

        if boxed_future.as_mut().poll(context).is_pending() {
            println!("Task is not finished yet");
        } else {
            println!("Task is finished");
        }
    }
}
