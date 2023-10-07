



/*

jobq using Dining_philosophers_problem algo in rust without using mutex or rwlock with semaphores reference counting
https://en.wikipedia.org/wiki/Dining_philosophers_problem

  TOKIO MULTITHREADING JOBQ CHANNLE ALGORITHMS
  
    mpsc: multi-producer, single-consumer channel. Many values can be sent.
    oneshot: single-producer, single consumer channel. A single value can be sent.
    broadcast: multi-producer, multi-consumer. Many values can be sent. Each receiver sees every value.
    watch: single-producer, multi-consumer. Many values can be sent, but no history is kept. Receivers only see the most recent value.
  
*/

use crate::*;
