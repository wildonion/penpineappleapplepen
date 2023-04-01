
// https://github.com/wildonion/cs-concepts
// https://connectivity.libp2p.io/
// https://blog.cloudflare.com/rust-nginx-module/

// a realtime node monitoring and packet sniffing tools
// using zmq to manage the load of each instance 
// in realtime, in our proxy zmq subscribers are node 
// instances that must be balanced 

// we can build zmq using tokio socket actors 
// and build libp2p and rpc system 
// using zmq pub/sub sockets  



// gql ws client 
//     |
//     |
//     ------riker and tokio server (select!{}, spawn(), job q channels) -------
//                                                                             |
//                                                                tlps over noise and tokio tls
//                                                                             |
//                                                                             -----
//                                                                                 p2p stacks
//                                                                                     - kademlia
//                                                                                     - gossipsub over tcp and quic
//                                                                                     - noise protocol
//                                                                                     - ws and webrtc
//                                                                                     - muxer
//                                                                                 quic and udp
//                                                                                 tcp 
//                                                                                 rpc capnp pubsub 
//                                                                                 zmq pubsub
//                                                                                 gql subs
//                                                                                 ws (push notif, chatapp, realtime monit)
//                                                                                 connections that implement AsyncWrite and AsyncRead traits for reading/writing IO future objects 
//                                                                                 redis client + mongodb



// ➙ tokio tcp, udp streaming future IO objects and select eventloop, spawn, scheduler, channels 
//   (data to share between threads using tokio channels must be Arc<Mutex<Data>> + Send + Sync + 'static
//    means it must be cloned or referenced also must be synced to mutate it and if it's not we can put it 
//    inside Mutext) to build multi worker and thread based proxy and server like nginx and hyper 

// streaming of encoded borsh and serde io future obejcts over 
//  libp2p gossipsub + kademlia, noise, tcp, udp, ws, 
//  redis, zmq pubsub and rpc capnp pubsub for actor message queues using:
//      riker concepts for message
// 	    tokio::spawn → green threadpool
// 	    tokio::channels → message queue channels
// 	    tokio::select → event loop
// also send notif (publish topic) to other pods if another one gets back online or finding online pods like mmq
// code: while let Ok((stream, addr)) = listener.accept().await{
//          tokio::spawn(async move{
//              streaming of IO future objects through redis, hyper, ricker, 
//              tokio tcp and udp and quic and muxer, libp2p stacks, zmq, rpc, ws and gql 
//              for realtiming pubsub streaming like push notif and chatapp
//          });
//      }


// spread requests between node server instances 
// using different balancing algorithms and pubsub pattern 
// to manage the total load of the VPS 


// ngrok process: [https://docs.rs/ngrok/latest/ngrok/]
//  ➙ first it'll open a port on local machine 
//  ➙ then it will create a session on that port with a random dns on its servers 
//  ➙ finally it forwars all the traffic to that session to the local port it created


// ➙ we can setup exit codes with enum to know which error caused the program to stopped when using Box<dyn Error> which can be implemented for the type that will cause the error at runtime 
// ➙ public key digital signature ring ed25519 verification for updating app and server verification apis 
// ➙ proxy, firewall, vpns, packet sniffer and load balancer like pingora, docker networking, nginx, ngrok, HAproxy, v2ray and wireshark for all layers
//    • tokio channels + worker green threadpool + event loopg, hyper, riker actor concepts, rpc capnp, zmq, libp2p stacks, ws, tcp and udp and noise and tokio tls
//    • a p2p based vpn like v2ray and tor using noise protocol, gossipsub, kademlia quic and p2p websocket 
//    • rpc capnp to communicate between each balancer
//    • decompress encoded packet 
//    • cpu task scheduling, 
//    • vod streaming
//    • weighted round robin dns, 
//    • vector clock, 
//    • event loop
//    • iptables and ssh tunneling
//    • zmq pub/sub with borsh serialization 
//    • simd divide and conquer based vectorization
//    • language binding
//    • reverse proxy for NAT traversal implemented in Rust
//    • implement DNS Server in Rust    
//    • google Search Crawler implemented in Rust (scalable and secure)
//    • caching server implemented in Rust like redis
//    • scalable and Secure Firewall implemented in Rust


use serde::{Deserialize, Serialize};
use borsh::{BorshDeserialize, BorshSerialize};


#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct Request; //// it can be Option<Vec<hyper::Request<hyper::Body>>> which all the incoming http hyper requests to this node that must be handled



#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct Weight{
    pub n: u16,
    pub requests: Request,
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Node{ //// this contains server info 
    pub dns: String,
    pub peer_id: String, 
    pub cost_per_api_call: u128, //// this is based on the load of the weights
    pub init_at: i64,
    pub weights: Option<Vec<Weight>>, //// load of requests
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Container{
    pub id: String,
    pub balancer: Balancer::RoundRobin
    pub nodes: Vec<Node>,
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Balancer{
    pub RoundRobin,
    pub LeastConnection,
    pub WeightedLeastConnection,
    pub WeightedResponseTime,
    pub ResourceBased,
    pub WeightedRoundRobin,
    pub IpHash,
}


//// TODO - 
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Pod{ //// a pod is a load balancer which can have one or more containers 
    pub id: String,
    pub containers: Vec<Container>,
}
