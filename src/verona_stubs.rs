use std::ffi::c_void;
use crate::executor::Task;

#[link(name = "verona")]
extern "C" {
    fn marios_print();
    fn runtime_init();
    fn scheduler_run();
    fn schedule_task(task: *mut c_void);
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

pub fn verona_schedule_task(task: Box<Task>) {
    let task_ptr = Box::into_raw(task) as *mut c_void;
    unsafe {
        schedule_task(task_ptr);
    }
}
