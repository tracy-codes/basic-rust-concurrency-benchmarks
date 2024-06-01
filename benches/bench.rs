use criterion::{criterion_group, criterion_main, Criterion};
use std::sync::{mpsc, Arc, Mutex, RwLock};
use std::thread;
use std::time::Duration;

/// Introduces a delay of 25ms for every even iteration.
/// This function simulates potential stalls in the execution of threads,
/// adding a delay to every even-numbered iteration.
fn even_iteration_delay(i: usize) {
    if i % 2 == 0 {
        thread::sleep(Duration::from_millis(25));
    }
}

/// Benchmark for read-heavy workloads using Arc<Mutex>.
/// This function measures the performance of multiple threads
/// concurrently reading from an Arc-wrapped Mutex-protected integer,
/// with a delay added for every even iteration.
fn arc_mutex_read_heavy(c: &mut Criterion) {
    let data = Arc::new(Mutex::new(0));
    c.bench_function("arc_mutex_read_heavy", |b| {
        b.iter(|| {
            let mut handles = vec![];
            for i in 0..10 {
                let data_clone = Arc::clone(&data);
                let handle = thread::spawn(move || {
                    let _unused = data_clone.lock().unwrap();
                    even_iteration_delay(i);
                });
                handles.push(handle);
            }
            for handle in handles {
                handle.join().unwrap();
            }
        });
    });
}

/// Benchmark for write-heavy workloads using Arc<Mutex>.
/// This function measures the performance of multiple threads
/// concurrently writing to an Arc-wrapped Mutex-protected integer,
/// with a delay added for every even iteration.
fn arc_mutex_write_heavy(c: &mut Criterion) {
    let data = Arc::new(Mutex::new(0));
    c.bench_function("arc_mutex_write_heavy", |b| {
        b.iter(|| {
            let mut handles = vec![];
            for i in 0..10 {
                let data_clone = Arc::clone(&data);
                let handle = thread::spawn(move || {
                    let mut num = data_clone.lock().unwrap();
                    *num += 1;
                    even_iteration_delay(i);
                });
                handles.push(handle);
            }
            for handle in handles {
                handle.join().unwrap();
            }
        });
    });
}

/// Benchmark for read-heavy workloads using Arc<RwLock>.
/// This function measures the performance of multiple threads
/// concurrently reading from an Arc-wrapped RwLock-protected integer,
/// with a delay added for every even iteration.
fn arc_rwlock_read_heavy(c: &mut Criterion) {
    let data = Arc::new(RwLock::new(0));
    c.bench_function("arc_rwlock_read_heavy", |b| {
        b.iter(|| {
            let mut handles = vec![];
            for i in 0..10 {
                let data_clone = Arc::clone(&data);
                let handle = thread::spawn(move || {
                    let _unused = data_clone.read().unwrap();
                    even_iteration_delay(i);
                });
                handles.push(handle);
            }
            for handle in handles {
                handle.join().unwrap();
            }
        });
    });
}

/// Benchmark for write-heavy workloads using Arc<RwLock>.
/// This function measures the performance of multiple threads
/// concurrently writing to an Arc-wrapped RwLock-protected integer,
/// with a delay added for every even iteration.
fn arc_rwlock_write_heavy(c: &mut Criterion) {
    let data = Arc::new(RwLock::new(0));
    c.bench_function("arc_rwlock_write_heavy", |b| {
        b.iter(|| {
            let mut handles = vec![];
            for i in 0..10 {
                let data_clone = Arc::clone(&data);
                let handle = thread::spawn(move || {
                    let mut num = data_clone.write().unwrap();
                    *num += 1;
                    even_iteration_delay(i);
                });
                handles.push(handle);
            }
            for handle in handles {
                handle.join().unwrap();
            }
        });
    });
}

/// Benchmark for mixed read/write workloads using Arc<Mutex>.
/// This function measures the performance of multiple threads
/// performing both reads and writes to an Arc-wrapped Mutex-protected integer,
/// with a delay added for every even iteration.
fn arc_mutex_mixed(c: &mut Criterion) {
    let data = Arc::new(Mutex::new(0));
    c.bench_function("arc_mutex_mixed", |b| {
        b.iter(|| {
            let mut handles = vec![];
            for i in 0..10 {
                let data_clone = Arc::clone(&data);
                let handle = if i % 2 == 0 {
                    thread::spawn(move || {
                        let _unused = data_clone.lock().unwrap();
                        even_iteration_delay(i);
                    })
                } else {
                    thread::spawn(move || {
                        let mut num = data_clone.lock().unwrap();
                        *num += 1;
                        even_iteration_delay(i);
                    })
                };
                handles.push(handle);
            }
            for handle in handles {
                handle.join().unwrap();
            }
        });
    });
}

/// Benchmark for mixed read/write workloads using Arc<RwLock>.
/// This function measures the performance of multiple threads
/// performing both reads and writes to an Arc-wrapped RwLock-protected integer,
/// with a delay added for every even iteration.
fn arc_rwlock_mixed(c: &mut Criterion) {
    let data = Arc::new(RwLock::new(0));
    c.bench_function("arc_rwlock_mixed", |b| {
        b.iter(|| {
            let mut handles = vec![];
            for i in 0..10 {
                let data_clone = Arc::clone(&data);
                let handle = if i % 2 == 0 {
                    thread::spawn(move || {
                        let _unused = data_clone.read().unwrap();
                        even_iteration_delay(i);
                    })
                } else {
                    thread::spawn(move || {
                        let mut num = data_clone.write().unwrap();
                        *num += 1;
                        even_iteration_delay(i);
                    })
                };
                handles.push(handle);
            }
            for handle in handles {
                handle.join().unwrap();
            }
        });
    });
}

/// Benchmark for read-heavy workloads using mpsc channels.
/// This function measures the performance of multiple threads
/// sending and receiving messages through mpsc channels,
/// with a delay added for every even iteration.
fn mpsc_read_heavy(c: &mut Criterion) {
    c.bench_function("mpsc_read_heavy", |b| {
        b.iter(|| {
            let (tx, rx) = mpsc::channel();
            let rx = Arc::new(Mutex::new(rx));
            let mut handles = vec![];
            for i in 0..10 {
                let tx_clone = tx.clone();
                let handle = thread::spawn(move || {
                    tx_clone.send(i).unwrap();
                    even_iteration_delay(i);
                });
                handles.push(handle);
            }
            for handle in handles {
                handle.join().unwrap();
            }
            let mut recv_handles = vec![];
            for i in 0..10 {
                let rx_clone = Arc::clone(&rx);
                let handle = thread::spawn(move || {
                    let _unused = rx_clone.lock().unwrap().recv().unwrap();
                    even_iteration_delay(i);
                });
                recv_handles.push(handle);
            }
            for handle in recv_handles {
                handle.join().unwrap();
            }
        });
    });
}

/// Benchmark for write-heavy workloads using mpsc channels.
/// This function measures the performance of multiple threads
/// sending and receiving messages through mpsc channels,
/// with a delay added for every even iteration.
fn mpsc_write_heavy(c: &mut Criterion) {
    c.bench_function("mpsc_write_heavy", |b| {
        b.iter(|| {
            let (tx, rx) = mpsc::channel();
            let rx = Arc::new(Mutex::new(rx));
            let mut handles = vec![];
            for i in 0..10 {
                let tx_clone = tx.clone();
                let handle = thread::spawn(move || {
                    tx_clone.send(i).unwrap();
                    even_iteration_delay(i);
                });
                handles.push(handle);
            }
            for handle in handles {
                handle.join().unwrap();
            }
            let mut recv_handles = vec![];
            for i in 0..10 {
                let rx_clone = Arc::clone(&rx);
                let handle = thread::spawn(move || {
                    let _unused = rx_clone.lock().unwrap().recv().unwrap();
                    even_iteration_delay(i);
                });
                recv_handles.push(handle);
            }
            for handle in recv_handles {
                handle.join().unwrap();
            }
        });
    });
}

/// Benchmark for mixed read/write workloads using mpsc channels.
/// This function measures the performance of multiple threads
/// performing both sending and receiving operations through mpsc channels,
/// with a delay added for every even iteration.
fn mpsc_mixed(c: &mut Criterion) {
    c.bench_function("mpsc_mixed", |b| {
        b.iter(|| {
            let (tx, rx) = mpsc::channel();
            let rx = Arc::new(Mutex::new(rx));
            let mut handles = vec![];
            for i in 0..10 {
                let tx_clone = tx.clone();
                let rx_clone = Arc::clone(&rx);
                let handle = if i % 2 == 0 {
                    thread::spawn(move || {
                        let _unused = rx_clone.lock().unwrap().recv().unwrap();
                        even_iteration_delay(i);
                    })
                } else {
                    thread::spawn(move || {
                        tx_clone.send(i).unwrap();
                        even_iteration_delay(i);
                    })
                };
                handles.push(handle);
            }
            for handle in handles {
                handle.join().unwrap();
            }
        });
    });
}

criterion_group!(
    benches,
    arc_mutex_read_heavy,
    arc_mutex_write_heavy,
    arc_rwlock_read_heavy,
    arc_rwlock_write_heavy,
    arc_mutex_mixed,
    arc_rwlock_mixed,
    mpsc_read_heavy,
    mpsc_write_heavy,
    mpsc_mixed
);
criterion_main!(benches);
