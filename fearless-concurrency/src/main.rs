use std::sync::{Arc, Mutex};

fn main() {
    let s = Arc::new(Mutex::new(String::from("Fearless")));

    let sleep_duration = std::time::Duration::from_secs(1);
    let mut handlers = vec![];
    for _ in 0..10 {
        let s = Arc::clone(&s);
        handlers.push(std::thread::spawn(move || {
            let mut val = s.lock().unwrap();
            val.push('!');
            println!("String is: {}", val);
        }));
        std::thread::sleep(sleep_duration);
    }
    for h in handlers {
        h.join();
    }
}
