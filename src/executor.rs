use crate::verona_stubs;
use futures::{
    future::{FutureExt, BoxFuture},
    task::{waker_ref, ArcWake},
};
use std::{
    future::Future,
    ffi::c_void,
    sync::Arc,
    task::Context,
};

pub struct Executor {
}

struct MyWaker{
}

impl ArcWake for MyWaker {
    fn wake_by_ref(arc_self: &Arc<Self>) {
    }
}

pub struct Task {
    future: BoxFuture<'static, ()>,
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
        let boxed_future = future.boxed();
        let boxed_task = Box::new(Task{
            future: boxed_future,
        });
        verona_stubs::verona_schedule_task(boxed_task);
    }
}

#[no_mangle]
pub extern "C" fn foo_rust(task: *mut c_void) {
    println!("This is a rust function called from C++");
    unsafe {
        let raw_ptr = task as *mut Task;
        let boxed_task = Box::from_raw(raw_ptr);

        let mut boxed_future = boxed_task.future;
        let mw = Arc::new(MyWaker{});
        let waker = waker_ref(&mw);
        let context = &mut Context::from_waker(&waker);

        if boxed_future.as_mut().poll(context).is_pending() {
            println!("Task is not finished yet");
        }
        else {
            println!("Task is finished");
        }
    }
}
