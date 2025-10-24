use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    thread::{self},
};

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let map: HashMap<char, usize> = HashMap::new();
    let counter = Arc::new(Mutex::new(map));

    let mut str_per_worker = input.len() / worker_count;
    if str_per_worker == 0 {
        str_per_worker = input.len();
    }

    let mut handles = Vec::with_capacity(worker_count);
    for i in 0..worker_count {
        let start = i * str_per_worker;
        if start >= input.len() {
            break;
        }
        let mut end = (i + 1) * str_per_worker;
        if i == worker_count-1 {
            end = input.len();
        }

        let slice: Vec<String> = input[start..end]
            .iter()
            .map(|s| s.to_lowercase().to_string())
            .collect();
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for s in slice {
                for c in s.chars() {
                    if !c.is_alphabetic() {
                        continue;
                    }
                    let mut m = counter.lock().unwrap();
                    if m.contains_key(&c) {
                        m.entry(c).and_modify(|v| *v += 1);
                    } else {
                        m.insert(c, 1);
                    }
                }
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    counter.lock().unwrap().clone()
}
