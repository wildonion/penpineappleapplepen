



/* 


https://en.wikipedia.org/wiki/Head-of-line_blocking -> fix the head of line blocking issue
https://github.com/wildonion/cs-concepts
https://github.com/codepr/tasq
https://dev.to/zeroassumptions/build-a-job-queue-with-rust-using-aide-de-camp-part-1-4g5m
https://poor.dev/blog/what-job-queue/
https://cetra3.github.io/blog/implementing-a-jobq/
https://rodent.club/queue-manager/
https://cetra3.github.io/blog/implementing-a-jobq-with-tokio/
https://tokio.rs/tokio/tutorial/channels
https://rust-lang.github.io/async-book/01_getting_started/01_chapter.html
https://www.fpcomplete.com/blog/http-status-codes-async-rust/


actors have:
    - task scheduling algos
    - worker threadpool like tokio::spawn()
    - pub sub channels for broadcasting messages and tasks scheduling
    - jobq like celery and the one inside the rabbitmq and zmq or tokio jobq algos
and actix actors be like:
they are smart objects have jobq controller like in workers.rs which have a threadpool 
in their schema that can handle async tasks inside their own spawned free thread also 
they can communicate with each other through message sending pattern to send signals to 
other actors between different parts of the app also in actix multithreaded based web server:
every api is an async task which gets solved inside a free thread of actix server
thus all the params passed to that api method must be send sync and static and be 
shreable between threads so we can move them between different parts of the app or 
threads safely like storage param, every api needs to have a path specified using 
actix post or get macro and gets registered using config.service() method so actix 
know what method needs to be executed inside a free thread based on what path or 
route the design pattern of actix it's like:

fn api_method(req: HttpRequest, shared_storage_data: Storage){} 
let mut apis = vec![];
apis.push(api_method);

let server = actix.server::new(10); // start server with 10 workers
server.run(apis);

    - spawn 10 workers which will be waiting to receive the data constantly from the sender
    - send each api job through the mpsc channel
    - recieve the api in execute() method
    - execute the api in a spawned free thread in execute method


...


⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ 
      task handler or worker threadpool implementations from scratch  
⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ 
 in worker threadpool we'll use tokio jobq channels 
 to share and schedule the Arc<Mutex<T>>: Send + Sync + 'static 
 to be executed safely and without any race conditions or deadlocks
 between threads.


    for worker in 0..10{ //// spawning tokio green threads for 10 workers
        tokio::spawn(async move{ //// spawning tokio worker green threadpool to solve async task
            
            //// any heavy logic here that must be shared using tokio channels inside a threadpool  
            //// ...
            
        });
    }



    let mut tokio_async_worker = AsyncWorker::new();
    let mut native_sync_worker = NativeSyncWorker::spawn(n_workers);
    let mut rayon_sync_worker  = RayonSyncWorker::new();
    let (sender, receiver) = std_mpsc::channel::<u8>();
    let cloned_sender = sender.clone();
    
    native_sync_worker.execute(move ||{
        let async_heavy_method = ||{
            // mining();
            let big_end_bytes = number.to_be_bytes();
            let index = 0;
            let new_chunk = cloned_ops(big_end_bytes[index]);
            cloned_sender.send(new_chunk).unwrap();
        }
        block_on(async_heavy_method());
    });
    
    rayon_sync_worker.spawn(move ||{
        block_on(async_heavy_method()); 
    });

    tokio_async_worker.spawn(async move{
        async_heavy_method().await;
        Ok(())
    })
    tokio_async_worker.execute().await // wait for all the workers of this worker to complete if there were any

    
    let bytes: Vec<u8> = receiver.iter().take(n_workers).collect() // collecting data from all workers
        
*/





use std::thread;
use std::sync::mpsc as std_mpsc;
use tokio::sync::mpsc;
use futures_util::Future;
use is_type::Is;
use uuid::Uuid;
use std::sync::{Arc, Mutex};
use log::info;
use crate::*;



type ErrType = Box<dyn std::error::Error + Send + Sync + 'static>;
pub struct TaskStruct<J, T> where J: FnMut(fn() -> T) -> Result<(), ErrType>{ //// generic `J` is a closure type that accept a function as its argument 
    job: J, //// the job itself
    res: T, //// job response
}



pub type Task = Job; //// the type of the Task is of type Job structure


pub struct Job{ // the job that must be received by the receiver
    pub id: Uuid,
    pub task: Box<dyn FnOnce() + Send + Sync + 'static>, //// the task that can be shared between worker threadpool for solving
} 

pub struct Queue{ // a queue which contains all the incoming jobs from the sender 
    pub tasks: Vec<Task>,   
}

pub struct JobHandler; // a threadpool structure to handle the poped-out job from the queue



pub mod _async{



    // async worker pool scheduler using tokio based on mpsc jobq channel protocol
    // this scheduler is used for asynchronous IO by not blocking the thread using tokio green threads


    use super::*;
    




    pub struct AsyncWorker<E>{
        count: usize, // number of workers
        sender: mpsc::UnboundedSender<Result<(), E>>, // sender async side with no byte limitation
        receiver: mpsc::UnboundedReceiver<Result<(), E>>, // receiver async side with no byte limitation
    }


    impl<E: Send + 'static> AsyncWorker<E>{ // E can be shared between threads

        pub fn new() -> Self{
            let (sender, 
                receiver) = mpsc::unbounded_channel(); // async mpsc jobq channel channel with no byte limitation to avoid deadlocks and race conditions
            AsyncWorker{
                count: 0, // will be equaled to the number of workers by solving all the jobs which are comming to the downside of the mpsc jobq channel
                sender,
                receiver
            }
        }

        pub fn spawn<T>(&mut self, task: T)
            where 
                T: Future<Output=Result<(), E>> + Send + 'static, // T can be shared between threads
                T::Output: Is<Type = Result<(), E>>, // T is a future and now we can access the Output type to make sure that is of type Result<(), E> - T::Output is the GAT of the Future trait
                {
                    let sender = self.sender.clone();
                    tokio::spawn(async move{ // spawn the task inside tokio green threads
                        let res = task.await;
                        match sender.send(res.into_val()){
                            Ok(()) => (),
                            Err(_) => panic!("Impossible Panic for Sender"),
                        }
                    });
                    self.count += 1;
                }


        pub async fn execute(mut self) -> Result<(), E>{

            std::mem::drop(self.sender); // make sure that the sender is dead since we want to receive all the messages and avoid deadlocks and race condition
            let mut index = 0;

            loop{ // we can use while let Some() syntax
                match self.receiver.recv().await{
                    Some(Ok(())) => {
                        assert!(index < self.count);
                    }
                    Some(Err(e)) => {
                        assert!(index < self.count);
                        return Err(e);
                    }
                    None => {
                        assert_eq!(index, self.count);
                        break Ok(()); // return this to the main
                    }
                }
                index+=1;
            }

        }

    }


}















pub mod sync{



    // a sync task scheduler (worker pool) with mpsc as the jobq channel protocol
    // this scheduler is used for synchronous IO by blocking the thread using rust native std thread - alternative to this is rayon


    use super::*;





    type Job = Box<dyn FnOnce() + Send + 'static>; // a job is of type closure which must be Send and static across all threads inside a Box on the heap


    //// there is no guaranteed order of execution for spawns, given that other threads 
    //// may steal tasks at any time, however, they are generally prioritized in a LIFO order 
    //// on the thread from which they were spawned, other threads always steal from the 
    //// other end of the deque, like FIFO order, the idea is that recent tasks are most 
    //// likely to be fresh in the local CPU's cache, while other threads can steal older stale tasks.
    pub struct RayonSyncWorker{
        count: usize, // number of workers
        sender: mpsc::UnboundedSender<Job>, // sender async side with no byte limitation
        receiver: mpsc::UnboundedReceiver<Job>, // receiver async side with no byte limitation
    }


    impl RayonSyncWorker{

        pub fn new() -> Self{
            let (sender, 
                receiver) = mpsc::unbounded_channel(); // async mpsc jobq channel channel with no byte limitation to avoid deadlocks and race conditions
            RayonSyncWorker{
                count: 0, // will be equaled to the number of workers by solving all the jobs which are comming to the downside of the mpsc jobq channel
                sender,
                receiver
            }
        }

        pub fn spawn(&mut self, task: Job)
            where 
                {
                    let sender = self.sender.clone();
                    rayon::spawn(move || { // firing off a task into the rayon threadpool in the 'static or global scope
                        match sender.send(task){
                            Ok(()) => (),
                            Err(_) => panic!("Impossible Panic for Sender"),
                        }
                    });
                    self.count += 1;
                }


        pub async fn execute(mut self) -> Result<(), Box<dyn std::error::Error + Send +'static>>{

            std::mem::drop(self.sender); // make sure that the sender is dead since we want to receive all the messages and avoid deadlocks and race condition
            let mut index = 0;

            loop{ // we can use while let Some() syntax
                match self.receiver.recv().await{
                    Some(job) => {
                        job();
                        assert!(index < self.count);
                    },
                    None => {
                        assert_eq!(index, self.count);
                        break Ok(()); // return this to the main
                    }
                }
                index+=1;
            }

        }

    }


    //// spawning native threads are too slow since thread handling in rust is depends 
    //// on user base context switching means that based on the load of the IO in the 
    //// app rust might solve the data load inside another cpu core and use multiprocessing approach:
    ////     • https://www.reddit.com/r/rust/comments/az9ogy/help_am_i_doing_something_wrong_or_do_threads/
    ////     • https://www.reddit.com/r/rust/comments/cz4bt8/is_there_a_simple_way_to_create_lightweight/
    struct Worker{
        id: Uuid,
        thread: Option<thread::JoinHandle<()>>, //// thread is of type JoinHandld struct which return nothing or ()
    }

    pub struct NativeSyncWorker {
        workers: Vec<Worker>,
        sender: std_mpsc::Sender<Message>, // all sends will be asynchronous and they never block
    }

    enum Message {
        NewJob(Job),
        Terminate,
    }

    impl NativeSyncWorker{
        
        pub fn spawn(size: usize) -> NativeSyncWorker {
            assert!(size > 0);
            let (sender, receiver) = std_mpsc::channel();
            let receiver = Arc::new(Mutex::new(receiver)); // reading and writing from an IO must be mutable thus the receiver must be inside a Mutex cause data inside Arc can't be borrows as mutable since the receiver read operation is a mutable process
            let mut workers = Vec::with_capacity(size); // capacity is not always equals to the length and the capacity of this vector is same as the maximum size based on the system arch, on 32 bits arch usize is 4 bytes and on 64 bits arch usize is 8 bytes
            for _ in 0..size { // since the receiver is not bounded to trait Clone we must clone it using Arc in each iteration cause we want to share it between multiple threads to get what the sender has sent 
                workers.push(Worker::new(Uuid::new_v4(), Arc::clone(&receiver)));
            }
            NativeSyncWorker{workers, sender}
        }

        pub fn execute<F>(&self, f: F) where F: FnOnce() + Send + 'static { // calling this method means send the incoming task from the process through the mpsc sender to down side of the channel in order to block a free thread using the receiver on locking the mutex
            let job = Box::new(f);
            self.sender.send(Message::NewJob(job)).unwrap(); // by executing the task handler sender will send a job asynchronously and only one receiver at a time can get that job and solve it by locking on the mutex to block the choosen thread since thread safe programming is all about this pattern!
        }
    }

    impl Drop for NativeSyncWorker{ // hitting CTRL + C can drop the lifetime also
        fn drop(&mut self) { // destructor for NativeSyncWorker struct 
            info!("Sending terminate message to all workers.");
            for _ in &self.workers {
                self.sender.send(Message::Terminate).unwrap();
            }
            info!("Shutting down all workers.");
            for worker in &mut self.workers {
                info!("Shutting down worker {}", worker.id);
                if let Some(thread) = worker.thread.take(){ // take() takes the value out of the option, leaving a None in its place
                    thread.join().unwrap(); // joining on thread will block the current thread to get the computation result and stop the thread from being processed in the background
                }
            }
        }
    }

    impl Worker{
        fn new(id: Uuid, receiver: Arc<Mutex<std_mpsc::Receiver<Message>>>) -> Worker {
            let thread = thread::spawn(move || loop { // spawning a thread inside the new() method and waiting for the receiver until a job becomes available to down side of the channel
                while let Ok(message) = receiver.lock().unwrap().recv(){ // iterate through the receiver to get all incoming messages - since other thread shouldn't mutate this message while this thread is waiting for the job we must do a locking on the message received from the sender to acquire the mutex by blocking the current thread to avoid being in dead lock, shared state and race condition situation
                    match message {
                        Message::NewJob(job) => {
                            info!("Worker {} got a job; executing.", id);
                            job(); // this might be an async task or job spawned by the tokio spawner in the background
                        }
                        Message::Terminate => {
                            info!("Worker {} was told to terminate.", id);
                            break;
                        }
                    }
                }
            });
            Worker {
                id,
                thread: Some(thread),
            }
        }
    }


}
