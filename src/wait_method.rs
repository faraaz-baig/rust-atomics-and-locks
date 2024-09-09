use std::{
    collections::VecDeque,
    sync::{Mutex, Condvar},
    thread,
    time::Duration,
};

pub fn wait_notify() {
    let queue = Mutex::new(VecDeque::<u32>::new());
    let condvar = Condvar::new();
    let pair = (Mutex::new(false), condvar);

    thread::scope(|s| {
        let consumer = s.spawn(|| loop {
            let (lock, cvar) = &pair;
            let mut guard = lock.lock().unwrap();
            
            while queue.lock().unwrap().is_empty() {
                guard = cvar.wait(guard).unwrap();
            }

            if let Some(item) = queue.lock().unwrap().pop_front() {
                dbg!(item);
            }
        });

        for i in 1.. {
            queue.lock().unwrap().push_back(i);
            pair.1.notify_one();
            thread::sleep(Duration::from_secs(1));
        }
    });
}