#[macro_export]
macro_rules! run_once_thread {
    ($expr:expr) => {{
        thread_local! {
            static IS_FIRST_CALL: std::cell::Cell<bool> = std::cell::Cell::new(true);
        }
        if IS_FIRST_CALL.with(|current_call| {
            if current_call.get() {
                current_call.set(false);
                true
            } else {
                false
            }
        }) {
            $expr
        }
    }};
}

#[cfg(test)]
mod tests {
    use std::thread;

    use crate::test_helpers::Counter;

    #[test]
    fn test_single_thread() {
        let counter = Counter::new();

        for _ in 0..2 {
            run_once_thread!(counter.increment());
        }

        assert_eq!(counter.get_count(), 1);
    }

    #[test]
    fn test_independent_invocations() {
        let counter = Counter::new();

        for _ in 0..2 {
            run_once_thread!(counter.increment());
            run_once_thread!(counter.increment());
        }

        assert_eq!(counter.get_count(), 2);
    }

    #[test]
    fn test_multiple_os_threads() {
        let counter = Counter::new();

        for _ in 0..2 {
            let counter_clone = counter.clone();
            let handle = thread::spawn(move || {
                run_once_thread!(counter_clone.increment());
            });
            handle.join().unwrap();
        }

        assert_eq!(counter.get_count(), 2); // Is 2 in comparison to run_once expected behavior
    }

    #[tokio::test]
    async fn test_multiple_tokio_threads() {
        let counter = Counter::new();

        for _ in 0..2 {
            let counter_clone = counter.clone();
            let handle = tokio::spawn(async move {
                run_once_thread!(counter_clone.increment());
            });
            handle.await.unwrap();
        }

        assert_eq!(counter.get_count(), 1); // Is still 1 since tokio threads are user threads, not kernel threads
    }
}
