use std::{collections::VecDeque, sync::Mutex, thread, time::Duration};

pub fn park_unpark() {
    let queue = Mutex::new(VecDeque::<u32>::new());

    thread::scope( |s| {
        let consumer = s.spawn(|| loop {
            let item = queue.lock().unwrap().pop_front();
            if let Some(item) = item {
                dbg!(item);
            } else {
                thread::park();
            }
        });

        for i in 1.. {
            queue.lock().unwrap().push_back(i);
            consumer.thread().unpark();
            thread::sleep(Duration::from_secs(1));
            
        }
    });

}
