#[cfg(test)]
mod tests {
    // use super::*;
    use std::sync::mpsc::channel;
    use std::thread;
    use std::time::Duration;
    use web_server::ThreadPool;

    // thread

    #[test]
    fn test_create_threadpool() {
        let pool = ThreadPool::new(4);
        assert_eq!(pool.workers.len(), 4);
    }

    #[test]
    #[should_panic]
    fn test_create_threadpool_with_zero_size() {
        ThreadPool::new(0);
    }

    #[test]
    fn test_execute_single_job() {
        let pool = ThreadPool::new(4);
        let (tx, rx) = channel();

        pool.execute(move || {
            tx.send(true).unwrap();
        });

        assert!(rx.recv_timeout(Duration::from_secs(2)).is_ok());
    }

    #[test]
    fn test_execute_multiple_jobs() {
        let pool = ThreadPool::new(4);
        let (tx, rx) = channel();
        let tx1 = tx.clone();
        let tx2 = tx.clone();

        pool.execute(move || {
            tx.send(1).unwrap();
        });
        pool.execute(move || {
            tx1.send(2).unwrap();
        });
        pool.execute(move || {
            tx2.send(3).unwrap();
        });

        let mut results = vec![];
        for _ in 0..3 {
            if let Ok(result) = rx.recv_timeout(Duration::from_secs(2)) {
                results.push(result);
            }
        }

        results.sort();
        assert_eq!(results, vec![1, 2, 3]);
    }

    #[test]
    fn test_threadpool_drop() {
        let pool = ThreadPool::new(4);
        pool.execute(move || {
            thread::sleep(Duration::from_secs(1));
        });

        // Pool should drop and all threads should be joined without panic
        drop(pool);
    }
}
