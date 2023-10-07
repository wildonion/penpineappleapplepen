



/*


⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ 
  jobq, mutex and rwlock from scratch using Dining_philosophers_problem
⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ 

building something jobq like mpsc, mutex and rwlock from scratch by solving the Dining_philosophers_problem 
algo using semaphores reference counting to avoid deadlocks and race conditions

https://en.wikipedia.org/wiki/Dining_philosophers_problem
https://google.github.io/comprehensive-rust/exercises/concurrency/dining-philosophers.html

  TOKIO MULTITHREADING JOBQ CHANNLE ALGORITHMS
  
    mpsc: multi-producer, single-consumer channel. Many values can be sent from different parts of the app thus the sender is cloneable
    oneshot: single-producer, single consumer channel. A single value can be sent.
    broadcast: multi-producer, multi-consumer. Many values can be sent. Each receiver sees every value.
    watch: single-producer, multi-consumer. Many values can be sent, but no history is kept. Receivers only see the most recent value.


Mutex (Mutual Exclusion): Rust's std::sync::Mutex provides mutual exclusion, meaning that at any given time, 
at most one thread can access the shared data inside the mutex. Relating to the dining philosophers, if each 
fork is represented as a mutex, only one philosopher can hold a fork at a time, ensuring mutual exclusion. 
Using mutexes requires careful design to avoid deadlocks.

RwLock (Read-Write Lock): std::sync::RwLock allows multiple readers or one writer to access the shared data 
simultaneously. If philosophers were allowed to either "read" (observe) or "write" (use) a fork, then multiple 
philosophers could simultaneously "read" a fork, but only one could "write" to it. In this analogy, the RwLock 
provides more flexible access than a mutex, but still ensures that data is not concurrently modified 
by multiple entities.

mpsc (Multiple Producer, Single Consumer) jobq: Rust's std::sync::mpsc provides channels for sending messages from 
multiple producers to a single consumer. While this might not have a direct mapping to the traditional dining 
philosophers problem, you can imagine an extension where philosophers send requests for forks (producers) to a 
mediator (consumer), who decides who gets which forks. The mediator ensures that no two philosophers hold the 
same fork, avoiding deadlock.



*/

use crate::*;
