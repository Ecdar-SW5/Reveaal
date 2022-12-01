use crossbeam_channel::{unbounded, Receiver, Sender};
use futures::Future;
use num_cpus;
use std::sync::{Arc, Mutex};
use std::task::{Poll, Waker};
use std::thread::{self, JoinHandle};

type ThreadPoolFunction = Box<dyn FnOnce() + Send + 'static>;

/// A construct that uses a fixed amount of threads to do work in parallel.
#[derive(Debug)]
pub struct ThreadPool {
    sender: Option<Sender<ThreadPoolFunction>>,
    threads: Vec<JoinHandle<()>>,
}

impl ThreadPool {
    /// Create a new thread pool.
    ///
    /// # Arguments
    ///
    /// * `num_threads` - The amount of threads in the thread pool.
    pub fn new(num_threads: usize) -> Self {
        let (sender, receiver): (Sender<ThreadPoolFunction>, Receiver<ThreadPoolFunction>) =
            unbounded();

        let threads = (0..num_threads)
            .map(|_| {
                let thread_receiver = receiver.clone();
                thread::spawn(move || {
                    for func in thread_receiver {
                        func();
                    }
                })
            })
            .collect();

        ThreadPool {
            sender: Some(sender),
            threads,
        }
    }

    /// Enqueue a function.
    /// The function will be executed on the threadpool and the returned value from the function
    /// will be available in the future this function returns.
    ///
    /// # Arguments
    ///
    /// * `function` - The function to execute on the threadpool.
    pub fn enqueue<T: Send + 'static>(
        self: &Arc<Self>,
        function: impl FnOnce(Arc<ThreadPool>) -> T + Send + 'static,
    ) -> ThreadPoolFuture<T> {
        let mut thread_future = ThreadPoolFuture::default();
        let return_future = thread_future.clone();
        let threadpool = self.clone();
        self.sender
            .as_ref()
            .unwrap()
            .send(Box::new(move || {
                thread_future.complete(function(threadpool))
            }))
            .unwrap();
        return_future
    }
}

impl Default for ThreadPool {
    fn default() -> Self {
        Self::new(num_cpus::get())
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        // Drop the sender to signal the threads to quit
        self.sender = None;

        loop {
            match self.threads.pop() {
                None => break,
                Some(thread) => thread.join().unwrap(),
            }
        }
    }
}

/// A generic future that can be completed from another thread.
#[derive(Debug)]
pub struct ThreadPoolFuture<T: Send> {
    result: Arc<Mutex<Option<T>>>,
    waker: Arc<Mutex<Option<Waker>>>,
}

impl<T: Send> ThreadPoolFuture<T> {
    fn complete(&mut self, return_type: T) {
        *self.result.lock().unwrap() = Some(return_type);
        let waker = self.waker.lock().unwrap();

        if let Some(waker) = waker.as_ref() {
            waker.wake_by_ref()
        };
    }
}

impl<T: Send> Future for ThreadPoolFuture<T> {
    type Output = T;

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        let mut waker = self.waker.lock().unwrap();
        *waker = Some(cx.waker().clone());
        let mut result = self.result.lock().unwrap();
        let result = result.take();
        result.map_or(Poll::Pending, Poll::Ready)
    }
}

impl<T: Send> Default for ThreadPoolFuture<T> {
    fn default() -> Self {
        ThreadPoolFuture {
            result: Arc::new(Mutex::new(None)),
            waker: Arc::new(Mutex::new(None)),
        }
    }
}

impl<T: Send> Clone for ThreadPoolFuture<T> {
    fn clone(&self) -> Self {
        ThreadPoolFuture {
            result: Arc::clone(&self.result),
            waker: Arc::clone(&self.waker),
        }
    }
}
