use std::{
    collections::HashSet,
    sync::{LazyLock, Mutex},
};

use rand::{Rng, distr::Alphabetic};

static GLOBAL_STATE: LazyLock<Mutex<HashSet<String>>> =
    LazyLock::new(|| Mutex::new(HashSet::new()));

pub struct Robot {
    name: String,
}

impl Robot {
    pub fn new() -> Self {
        Self { name: generate_name() }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn reset_name(&mut self) {
        self.name = generate_name();
    }
}

fn generate_name() -> String {
    let mut name = generate_name_impl();
    loop {
        if GLOBAL_STATE.lock().unwrap().insert(name.clone()) {
            break;
        }
        name = generate_name_impl()
    }
    name
}

fn generate_name_impl() -> String {
    let mut rng = rand::rng();
    let suffix = rng.random_range(100..1000);
    let prefix = rng
        .sample_iter(Alphabetic)
        .take(2)
        .map(char::from)
        .map(|c| char::to_ascii_uppercase(&c))
        .collect::<String>();
    format!("{prefix}{suffix}")
}
