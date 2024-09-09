// src/lib.rs

mod park_and_unpark_threads;
mod wait_method;

pub fn run_code() {
    //park_and_unpark_threads::park_unpark();
    wait_method::wait_notify()
}