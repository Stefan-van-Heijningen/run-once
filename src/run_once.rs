#[macro_export]
macro_rules! run_once {
    ($expr:expr) => {{
        static IS_FIRST_CALL: std::sync::Once = std::sync::Once::new();
        IS_FIRST_CALL.call_once(|| $expr);
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
            run_once!(counter.call());
        }

        assert_eq!(counter.get_count(), 1);
    }

    #[test]
    fn test_independent_invocations() {
        let counter = Counter::new();

        for _ in 0..2 {
            run_once!(counter.call());
            run_once!(counter.call());
        }

        assert_eq!(counter.get_count(), 2);
    }

    #[test]
    fn test_multiple_os_threads() {
        let counter = Counter::new();

        for _ in 0..2 {
            let counter_clone = counter.clone();
            let handle = thread::spawn(move || {
                run_once!(counter_clone.call());
            });
            handle.join().unwrap();
        }

        assert_eq!(counter.get_count(), 1);
    }

    #[tokio::test]
    async fn test_multiple_tokio_threads() {
        let counter = Counter::new();

        for _ in 0..2 {
            let counter_clone = counter.clone();
            let handle = tokio::spawn(async move {
                run_once!(counter_clone.call());
            });
            handle.await.unwrap();
        }

        assert_eq!(counter.get_count(), 1);
    }
}
