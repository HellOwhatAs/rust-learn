fn threads(x: i32) {
    use std::thread;
    use std::time::Duration;
    fn task(n: i32, idx: i32) -> impl Fn() {
        move || {
            for i in 0..n {
                thread::sleep(Duration::from_millis(100));
                println!("{} from Thread {}", i, thread::current().name().unwrap_or(&format!("[Unnamed {}]", idx)));
            }
        }
    }
    let mut ts = vec![];
    for i in 0..4 { ts.push(thread::spawn(task(x * 3, i))) }
    task(x, -1)();
    ts.into_iter().map(|t| t.join().unwrap()).count();
}

fn message_passing(x: i32) {
    use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;
    let (s, r) = mpsc::channel();
    let s1 = s.clone();
    let t1 = thread::spawn(move ||{
        for i in 0..x {
            s1.send(format!("Msg {} from Thread 1", i)).unwrap();
            thread::sleep(Duration::from_nanos(1));
        }
    });
    let t2 = thread::spawn(move || {
        let res = match r.recv() {
            Ok(msg) => msg,
            Err(_) => {
                println!("Sender died!!");
                return;
            }
        };
        println!("Thread 2 received {}", res);
        for res in r {
            println!("Thread 2 received {}", res);
        }
    });
    for i in 0..x {
        s.send(format!("Msg {} from main Thread", i)).unwrap();
        thread::sleep(Duration::from_nanos(1));
    }
    drop(s);
    t2.join().unwrap();
    t1.join().unwrap();
}

fn shared_state() {
    // Mutex<T> 是一个线程安全版本的 RefCell<T>
    use std::sync::Mutex;
    {
        let m = Mutex::new(20);
        print!("{}", m.lock().unwrap());
        {
            let mut x = m.lock().unwrap();
            print!(" + 55 = ");
            *x += 55;
        }
        println!("{}", m.lock().unwrap());
    }
    {
        use threadpool::ThreadPool;
        use std::sync::Arc;
        let counter = Arc::new(Mutex::new(0 as u128));
        let pool = ThreadPool::new(num_cpus::get());
        let timer = std::time::Instant::now();
        use crate::packages_crates_and_modules::modules::fibonacci::fib;
        for i in 0..50000 {
            let ref_counter = counter.clone();
            pool.execute(move || {
                let mut tmp = 0;
                for i in 0..1000 {
                    tmp += fib(i / 10 as u128);
                }
                tmp += i * i;
                let mut m = ref_counter.lock().unwrap();
                *m += tmp;
            });
        }
        pool.join();
        println!("pool\t{}\t{}", counter.lock().unwrap(), timer.elapsed().as_nanos());
    }
}

fn the_rayon_library() {
    use crate::packages_crates_and_modules::modules::fibonacci::fib;
    let x = 0..50000;
    let op = |x: u128| {
        let mut res = 0;
        for i in 0..1000 {
            res += fib(i / 10 as u128);
        }
        res += x * x;
        res
    };
    {
        use rayon::prelude::*;
        let timer = std::time::Instant::now();
        let res: u128 = x.clone().into_par_iter().map(op).sum();
        println!("rayon\t{}\t{}", res, timer.elapsed().as_nanos());
    }
    {
        let timer = std::time::Instant::now();
        let res: u128 = x.clone().map(op).sum();
        println!("simple\t{}\t{}", res, timer.elapsed().as_nanos());
    }
}

#[allow(dead_code)]
pub fn main() {
    threads(5);
    message_passing(10);
    shared_state();
    the_rayon_library();
}