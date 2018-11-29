///
///

mod runner {
    fn _line(){
        println!("{:?}", "---------------------------------------");
    }

    fn study_threads_case_sample() {
        // 学习简单的线程处理

        use std::thread;
        use std::time::Duration;

        thread::spawn(|| {
            for i in 1..10 {
                println!("hi number {} from the spawned thread!", i);
                thread::sleep(Duration::from_millis(5));
            }
        });

        for i in 1..10 {
            println!("You number {} from the main thread!", i);
            thread::sleep(Duration::from_millis(5));
        }

        _line();

    }

    fn study_threads_case_join() {
        // 使用join() 函数，让主线程等待新线程执行完成
        use std::thread;
        use std::time::Duration;

        let sub_handle = thread::spawn(|| {
            for i in 1..20 {
                println!("hi number {} from the spawned thread!", i);
                thread::sleep(Duration::from_millis(5));
            }
        });

        for i in 1..10 {
            println!("You number {} from the main thread!", i);
            thread::sleep(Duration::from_millis(5));
        }

        sub_handle.join().unwrap();
        _line();
    }

    fn study_threads_case_move() {
        // 学习：在一个线程中使用另一个线程的数据
        // 重点在Move

        use std::thread;
        use std::time::Duration;

        let v = vec![1,2,5,7,8,9];


        let sub_handle = thread::spawn(move || {
            for i in &v {
                println!("V number {} from the spawned thread!", i);
                thread::sleep(Duration::from_millis(5));
            }
            // 以下代码
            println!("Here's a vector: {:?}", &v); // value used here after move
        });

        sub_handle.join().unwrap();
        _line();
    }

    fn study_threads_case_sync_mutex() {
        // 主要说明： 多线程之间共享内存、共享所有权
        // 使用 Mutex<T> 与 Arc<T>


        use std::sync::{Mutex, Arc};
        use std::thread;

        // counter 本身是不可变的，需要改变其内部值的可变引用来处理
        // Mutex<T> 提供了内部可变性
        let counter = Arc::new(Mutex::new(0));

        let mut handlers = vec![];

        for _ in 0..10 {
            let counter = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                let mut num = counter.lock().unwrap();;

                *num += 1;
            });

            handlers.push(handle);
        }

        for handler in handlers {
            handler.join().unwrap();
        }

        println!("Result: {:?}", *counter.lock().unwrap()); // 10
    }


    pub fn run() {
        study_threads_case_sample();
        study_threads_case_join();
        study_threads_case_move();
        study_threads_case_sync_mutex();
    }
}


pub fn run() {
    runner::run();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
