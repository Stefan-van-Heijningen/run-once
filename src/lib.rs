pub mod run_once;
pub mod run_once_thread;

#[cfg(test)]
mod test_helpers {
    use std::sync::{Arc, Mutex};

    #[derive(Debug, Clone)]
    pub struct Counter {
        count: Arc<Mutex<i32>>,
    }

    impl Counter {
        pub fn new() -> Self {
            Counter {
                count: Arc::new(Mutex::new(0)),
            }
        }

        pub fn increment(&self) {
            let mut count = self.count.lock().unwrap();
            *count += 1;
        }

        pub fn get_count(&self) -> i32 {
            *self.count.lock().unwrap()
        }
    }
}
