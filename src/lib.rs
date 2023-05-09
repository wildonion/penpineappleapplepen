
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



// gql subs ws client 
//     |
//     |
//     ------riker and tokio server (select!{}, spawn(), jobq channels) -------
//                                                                             |
//                                                     sharded tlps over noise-protocol and tokio-rustls
//                                                                             |
//                                                                             ----- sharded instances -----
//                                                                                         hyper
//                                                                                         p2p stacks
//                                                                                             - kademlia
//                                                                                             - gossipsub over tcp and quic
//                                                                                             - noise protocol
//                                                                                             - ws and webrtc
//                                                                                             - muxer
//                                                                                         quic and udp
//                                                                                         tcp 
//                                                                                         rpc capnp pubsub 
//                                                                                         zmq pubsub
//                                                                                         gql subs
//                                                                                         ws (push notif on data changes, chatapp, realtime monit, webhook setups, mmq and order matching engine)
//                                                                                         connections that implement AsyncWrite and AsyncRead traits for reading/writing IO future objects 
//                                                                                         redis client pubsub + mongodb

// → an eventloop or event listener server can be one of the above sharded tlps which contains an event handler trait 
//  (like riker and senerity EventHanlder traits, tokio channels and tokio::select!{} or ws, zmq and rpc pubsub server) 
//  to handle the incoming published topics over zmq and rpc server, emitted events over ws server or webhooks over http


// → event driven means we must have an event handler or listener on client side to subs to fired or emitted events on the 
//  server side, these handlers can be predefined traits or an event loop like tokio::select!{} which listen to the events 
//  coming from the server over ws, zmq or rpc here is the flow of realtiming:
//                     ws, gql, rpc and zmq pubs to fired or emitted events <--
//                                                                             |
//                                                         notifs or streaming of future io objects
//                                                                             |
//                                                                             ---> ws, gql, rpc and zmq subs or event handler traits for firing or emit events

//                     gql subs + ws + redis client <------> ws server + redis server
//                     http request to set push notif <------> http hyper server to publish topic in redis server
//                     json/capnp rpc client <------> json/capnp rpc server
//                     zmq subs <------> zmq pub server
//                     tcp, quic client <------> tcp, quic streaming future io objects server

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


// ngrok process: [https://docs.rs/ngrok/latest/ngrok/] || [https://ngrok.com/docs/using-ngrok-with/rust/]
//  ➙ first it'll open a port on local machine 
//  ➙ then it will create a session on that port with a random dns on its servers 
//  ➙ finally it forwars all the traffic to that session to the local port it created


// with simple-hyper-server-tls, tokio-rustls and noise-protocol we can create a secured communication 
// streaming channel between our hyper, ws, tcp or udp servers and clients based on the created certificate 
// and the key by implementing the tls protocols for the raw underlying 
// of tcp and udp socket stream of io future objects


// ➙ we can setup exit codes with enum to know which error caused the program to stopped when using Box<dyn Error> which can be implemented for the type that will cause the error at runtime 
// ➙ public key digital signature ring ed25519 verification for updating app and server verification apis 
// ➙ proxy, firewall, vpns, packet sniffer and load balancer like pingora, docker networking, nginx, ngrok, HAproxy, v2ray and wireshark for all layers
//    • tokio channels + worker green threadpool + event loopg, hyper, riker actor concepts, rpc capnp, zmq, libp2p stacks, ws, tcp and udp
//    • a p2p based vpn like v2ray and tor using noise protocol, gossipsub, kademlia quic and p2p websocket 
//    • simple-hyper-server-tls, noise-protocol and tokio-rustls to implement ssl protocols and make a secure channel for the underlying raw socket streams
//    • gateway and proxy using hyper: https://github.com/hyperium/hyper/tree/master/examples
//    • rpc capnp to communicate between each balancer
//    • decompress encoded packet using borsh and serde 
//    • cpu task scheduling, 
//    • vod streaming
//    • weighted round robin dns, 
//    • vector clock, 
//    • event loop
//    • iptables and ssh tunneling
//    • zmq pub/sub with borsh serialization 
//    • simd divide and conquer based vectorization
//    • language binding
//    • reverse proxy for NAT traversal implemented in Rust based macros using ntm repo
//    • implement DNS Server in Rust    
//    • google Search Crawler implemented in Rust (scalable and secure)
//    • caching server implemented in Rust like redis
//    • scalable and Secure Firewall implemented in Rust

/*
cloudflare warp vpn
    • boringtun protocol which is based on wireguard protocol
    • uses noise protocol with ed25519 encryption
    • 1111 dns based protocol 
    • udp and quic for packet sending   
    • argo routing to send packets to cloudflare gateways
*/
    
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
