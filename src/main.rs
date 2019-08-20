use futures::prelude::*;
use futures::sync::oneshot;

use tokio::runtime::Runtime;
use tokio::timer::Interval;

use std::time::Duration;

fn main() {
    let mut rt = Runtime::new().unwrap();
    rt.spawn(create_task());

    let (tx, rx) = oneshot::channel();

    println!("runtime started, waiting for shutdown signal...");

    std::thread::spawn(|| {
        // shutdown after 5 secs
        std::thread::sleep(Duration::from_secs(5));
        tx.send(()).unwrap();
        println!("shutdown signal sent.");
    });

    rt.block_on(rx.map_err(|_| ())).unwrap();

    println!("shutting down runtime...");
    rt.shutdown_now().wait().unwrap();

    println!("bye.");
}

fn create_task() -> impl Future<Item = (), Error = ()> {
    Interval::new_interval(Duration::from_secs(1))
        .for_each(|_| {
            println!("task running...");
            Ok(())
        })
        .map_err(|_| panic!("interval broken"))
}
