
// https://github.com/wildonion/cs-concepts
// https://connectivity.libp2p.io/

// a realtime node monitoring and packet sniffing tools
// using zmq to manage the load of each instance 
// in realtime, in our proxy zmq subscribers are node 
// instances that must be balanced 

// we can build zmq using tokio socket actors 
// and build libp2p and rpc system 
// using zmq pub/sub sockets  


// proxy, firewall, vpns, packet sniffer and load balancer like pingora, nginx, HAproxy, v2ray and wireshark for all layers
// • tokio channels + worker green threadpool + event loop, zmq and riker actor concepts
// • a p2p based vpn like v2ray and tor using noise protocol, gossipsub, kademlia quic and p2p websocket 
// • rpc capnp to communicate between each balancer
// • decompress encoded packet 
// • cpu task scheduling, 
// • weighted round robin dns, 
// • vector clock, 
// • event loop
// • iptables
// • zmq pub/sub with borsh serialization 
// • simd divide and conquer based vectorization
// • language binding
// • reverse proxy for NAT traversal implemented in Rust
// • implement DNS Server in Rust    
// • google Search Crawler implemented in Rust (scalable and secure)
// • caching server implemented in Rust like redis
// • scalable and Secure Firewall implemented in Rust


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
    pub weights: Option<Vec<Weight>>,
}


#[derive(Clone, Debug, Serialize, Deserialize)]

pub struct Container{
    pub id: String,
    pub nodes: Vec<Node>,
}



//// TODO - 
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Pod{ //// a pod is a load balancer which can have one or more containers 
    pub id: String,
    pub containers: Vec<Container>,
}
