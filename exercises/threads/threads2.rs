// threads2.rs
// 执行 `rustlings hint threads2` 或在观察模式下使用 `hint` 子命令来获取提示。
// 基于上一个练习，我们想要所有线程完成它们的工作，但是这回
// 派生线程需要需要负责更新一个共享的值： JobStatus.jobs_completed

use std::sync::Arc;
use std::thread;
use std::time::Duration;
use std::sync::atomic::{AtomicU32, Ordering};

struct JobStatus {
    jobs_completed: AtomicU32,
}

fn main() {
    let status = Arc::new(JobStatus { jobs_completed: AtomicU32::new(0)  });
    let mut handles = vec![];
    for i in 0..10 {
        // let status_shared = Arc::clone(&status);
        let status_shared = status.clone();
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            // TODO: 在更新共享值之前，你还需要做点什么
            // let mut job_status = status_shared.lock().unwrap();
            // job_status.jobs_completed += 1;
            status_shared.jobs_completed.fetch_add(1, Ordering::Relaxed);
            println!("thread {} is complete {:?}", i, status_shared.jobs_completed);
        });
        handles.push(handle);
    }
    while status.jobs_completed.load(Ordering::Relaxed) < 10 {
        println!("waiting... {:?}", status.jobs_completed);
        thread::sleep(Duration::from_millis(100));
    }
    for handle in handles {
        // handle.join().unwrap();
    //     // TODO: 打印 JobStatus.jobs_completed 的值。你可以从输出中注意到有趣的东西吗？
    //     // 你必须要 'join' 所有的线程句柄吗？
        println!("jobs completed {:?}", status.jobs_completed);
    }
}
