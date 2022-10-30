use std::time::Duration;
use tokio::time::sleep;

// async is forbidden does not work without a runtime
// we need a async runtime! C# and javascript have built-in runtimes but Rust does not
/**
 * - in order to communicate through between we can either use a mutex (shared state) or through channels
 * - async code uses "co-ooperative scheduling" instead of "pre-emptive scheduling"
 */
#[tokio::main(flavor = "current_thread")]
async fn main() {
    // store tokio task handles
    let mut handles = vec![];

    // spawn a tokio task - we will run tasks concurrently now
    for i in 0..2 {
        // async closures are unstable so we use a block instead
        // since we want to lexically capture ownership of vars in the block we use move
        let handle = tokio::spawn(async move {
            my_function(i).await;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }
}

/**
 * -------------
 *  ASYNC AWAIT
 * -------------
 *  Async functions are syntactic sugar
 *  - async functions looks synchronous
 *  - allows the thread to stop execution and free the runtime for other resources
 *  - async functions returns a generic that implements the Future trait!
 *  - we dont want to use CPU intensive operations on an async function
 *  
 *  Ex:
 *    fn my_function() -> impl Future<Output = ()> {
//     println!("I'm an async function")
//   }
 */
async fn my_function(i: i32) {
    println!("I'm an async function!");

    let s1 = read_from_db().await;
    println!("[{i}] First Result: {}", s1);
    let s2 = read_from_db().await;
    println!("[{i}] Second Result: {}", s1);
}

/** Imagine Simulating IO  */
async fn read_from_db() -> String {
    // stops the Future from executing for a certain duration - does NOT stop the thread
    sleep(Duration::from_millis(50)).await;
    "DB Result".to_owned()
}

/**
 *  -------------
 *  FUTURES
 *  -------------
 *  - Behind the scenes Future is like a simple state machine
 *  - "Zero Cost Abstraction" you will never incur a runtime cost unless the futures are executed
 *  - Futures are "lazy", easy to cancel
 */
trait Future {
    type Output;
    // wake is a callback that will execute when Poll is ready! When the future is ready it will notify the execturor fn to continue
    fn poll(&mut self, wake: fn()) -> Poll<Self::Output>;
}

// Poll is either Ready or Pending at any given time
enum Poll<T> {
    Ready(T),
    Pending,
}
