

/*


https://github.com/wildonion/cs-concepts
https://connectivity.libp2p.io/
https://blog.cloudflare.com/rust-nginx-module/
https://github.com/wildonion/uniXerr/blob/master/infra/valhalla/coiniXerr/src/tlps/p2p.pubsub.rs
https://github.com/foniod/build-images


blockchain distributed algorithms and scheduling tlps:
  multithreaded node/agent/bot1
          |
          |
           ---actix-wss/tokio mutex,select,jobq,spawn,tcp,udp)/rpc-capnp/actix-https
                libp2p quic,gossipsub,kademlia,noise/redis pubsub strams
		noise,tokio-rustl,wallexerr,web3
                                |
                                |
                                 --- multithreaded node/agent/bot2


a realtime node monitoring and packet sniffing tools
using zmq to manage the load of each instance 
in realtime, in our proxy, zmq subscribers are server app 
node instances that must be balanced by subscribing 
on the incoming topic from the balancer publishers, like
spread requests between node server instances 
using different balancing algorithms and pubsub pattern 
to manage the total load of the VPS also we can build zmq 
using tokio socket actors and build libp2p and rpc system 
using zmq pub/sub sockets  


codec like serde, borsh and capnp also send notif (publish backonline topic) 
to other pods if another one gets back online or finding online pods 
using following flow:
    - actix ws actor event and stream handler/loop using tokio spawn, 
        select, mpsc, mutex and tcp with redis and libp2p pubsub streams
    - event and stream handler to handle the incoming async task like ws 
        messages packets using actix StreamHandler and tokio tcp 
    - message handler to handle the message type which is going to 
        be sent between other actors and other parts of the app
    - ws actor stream and event handlers are like:
        streaming over incoming bytes through the tokio tcp socket 
        to send them as the async task to tokio green threadpool using
        tokio spawn to handle them as an event using tokio select event 
        loop handler

	>>>> look start_tcp_server api in gem <<<< 
	streaming over incoming encoded io future object of utf8 bytes 
	to decode them into structs to mutate them concurrently by moving
	them between tokio threads using jobq channels and mutex 
			    or 
	event of async task handler, streamer, loop 
	inside std::thread::scope and tokio::spawn based 
	tokio tcp stream or mmq streaming over future 
	bytes using tokio and ws actor and redis pubsub 
	and streams by streaming over incoming bytes 
	inside the tokio gread threadpool and pass them 
	to other threads using tokio::sync::mpsc, actor, 
	select, spawn, mutex, pubsub, tcp stream, hex, serding 
	to_string vs from utf8


sha256, sha3, Keccak256 and argon2, multipart, base64, rustls to load trusted ssl certs from /etc/ssl/certs/ca-certificates.crt 
and ssh RSA ECC curves keypair with simple-hyper-server-tls, openssl, tokio-rustls and noise-protocol we can create a secured communication 
streaming channel between our hyper, ws, tcp or udp servers and clients based on the created certificate 
and the key by implementing the tls protocols for the raw underlying 
of tcp and udp socket stream of io future objects


➙ we can setup exit codes with enum to know which error caused the program to stopped when using Box<dyn Error> which can be implemented for the type that will cause the error at runtime 
➙ public key digital signature ring ed25519 verification for updating app and server verification apis 
➙ bpf based proxy, firewall, vpns, packet sniffer and load balancer like pingora, docker networking, nginx, ngrok, HAproxy, v2ray and wireshark for all layers
   • tokio channels + worker green threadpool + event loopg, hyper, actix actor concepts, rpc capnp, zmq, libp2p stacks, ws, tcp and udp
   • a p2p based vpn like v2ray and tor using noise protocol, gossipsub, kademlia quic and p2p websocket 
   • simple-hyper-server-tls, noise-protocol and tokio-rustls to implement ssl protocols and make a secure channel for the underlying raw socket streams
   • gateway and proxy using hyper: https://github.com/hyperium/hyper/tree/master/examples
   • rpc capnp to communicate between each balancer
   • decompress encoded packet using borsh and serde 
   • cpu task scheduling, 
   • vod streaming
   • weighted round robin dns, 
   • vector clock, 
   • event loop
   • iptables and ssh tunneling
   • zmq pub/sub with borsh serialization 
   • simd divide and conquer based vectorization
   • language binding
   • reverse proxy for NAT traversal implemented in Rust based macros
   • implement DNS Server in Rust    
   • google Search Crawler implemented in Rust (scalable and secure)
   • caching server implemented in Rust like redis
   • scalable and Secure Firewall implemented in Rust
   • ngrok process: [https://docs.rs/ngrok/latest/ngrok/] || [https://ngrok.com/docs/using-ngrok-with/rust/]
 	➙ first it'll open a port on local machine 
 	➙ then it will create a session on that port with a random dns on its servers 
 	➙ finally it forwards all the traffic to that session to the local port it created
	➙ ngrok and ssh vps will starts a server on a random part then forward all the packets 
 	  coming from outside to the localhost it's like: 
	  outside <---packet---> ngrok or ssh vps server act like proxy <---packet---> localhost
   • cloudflare warp vpn
	    • boringtun protocol which is based on wireguard protocol
	    • uses noise protocol with ed25519 encryption
	    • 1111 dns based protocol 
	    • udp and quic for packet sending   
	    • argo routing to send packets to cloudflare gateways
	    • ed25519 digital signature pubkey with chacha20 in noise protocol for making vpn
*/


    
use serde::{Deserialize, Serialize};
use borsh::{BorshDeserialize, BorshSerialize};
mod jobq;
use jobq::*;
mod acter;
use acter::*;
mod workers;
use workers::*;
use uuid::Uuid;


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
    pub balancer: Balancer,
    pub nodes: Vec<Node>,
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Balancer{
    RoundRobin,
    LeastConnection,
    WeightedLeastConnection,
    WeightedResponseTime,
    ResourceBased,
    WeightedRoundRobin,
    IpHash,
}


//// TODO - 
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Pod{ //// a pod is a load balancer which can have one or more containers 
    pub id: String,
    pub containers: Vec<Container>,
}


pub async fn start_tcp_listener(){

// https://github.com/wildonion/redis4
// more info in start_tcp_listener() api in gem admin access
	
#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct TcpServerData{
    pub data: String,
}
let tcp_server_data = TcpServerData::default();
let (tcp_msg_sender, mut tcp_msg_receiver) = 
	tokio::sync::mpsc::channel::<bool>(1024);
    
    /* ----------------------------------------- */
    /* starting a tcp listener in the background */
    /* ----------------------------------------- */

    let bind_address = format!("0.0.0.0:2323");
    let mut api_listener = tokio::net::TcpListener::bind(bind_address.as_str()).await;
    
    if api_listener.is_err(){
	resp!{
	    &[u8], // response data
	    &[], // response message
	    TCP_SERVER_ERROR,
	    StatusCode::EXPECTATION_FAILED, // status code
	    None::<Cookie<'_>>, // cookie
	}
    }

    let api_listener = api_listener.unwrap();
    info!("➔ 🚀 tcp listener is started at [{}]", bind_address);

    tokio::spawn(async move{

	while let Ok((mut api_streamer, addr)) = api_listener.accept().await{
	    info!("🍐 new peer connection: [{}]", addr);

	    let tcp_server_data = tcp_server_data.clone();

	    tokio::spawn(async move {

		let mut buffer = vec![0; 1024];

		while match api_streamer.read(&mut buffer).await {
		    Ok(rcvd_bytes) if rcvd_bytes == 0 => return,
		    Ok(rcvd_bytes) => {
    
			let string_data = std::str::from_utf8(&buffer[..rcvd_bytes]).unwrap();
			info!("📺 received data from peer: {}", string_data);
    
			let send_tcp_server_data = tcp_server_data.data.clone();
			if let Err(why) = api_streamer.write_all(&send_tcp_server_data.as_bytes()).await{
			    error!("❌ failed to write to api_streamer; {}", why);
			    return;
			} else{
			    info!("🗃️ sent {}, wrote {} bytes to api_streamer", tcp_server_data.data.clone(), send_tcp_server_data.len());
			    return;
			}
		    
		    },
		    Err(e) => {
			error!("❌ failed to read from api_streamer; {:?}", e);
			return;
		    }
		    
		}{}
    
	    });
	}{}
	
    });
}

pub async fn race_condition_avoidance(){

    /* ---------------------------------------------------------------------- */
    /* ---------------------- RACE CONDITION AVOIDANCE ---------------------- */
    /*  
        race conditions means that two threads want to mutate the data 
        at the same time, we have to use mutex so tell the other threads
        wait there is a threads that is trying to mutate this type and 
        will update you once the lock gets freed and in order to avoid blockcing 
        issues in the current thread we have to lock inside a separate thread 
        and mutate the type in there like tokio::spawn() then send it through 
        the jobq channel to the other threads for reading and future mutations
    */
    
    pub type ArcedMutexed<'lifetime> = std::sync::Arc<tokio::sync::Mutex<String>>;
    
    #[derive(Clone)]
    pub struct Data<D: Send + Sync + 'static>{
        /* we're using tokio mutex to avoid blocing issues inside the current thread since it locks asycnly */
        pub actual: D
    }
    let mut data_instance = Data::<ArcedMutexed>{
        actual: std::sync::Arc::new(tokio::sync::
            Mutex::new(
                String::from("a mutexed data")
            )
        ),
    };
    
    println!("data instance actual value before getting mutated >>> [{}]", data_instance.actual.lock().await.to_owned());
    
    /* reading from the channel is a mutable process thus receiver must be mutable */
    let (data_sender, mut data_receiver) = 
        tokio::sync::mpsc::channel::<Data<ArcedMutexed>>(1024);
    /*
        since tokio spawn takes a closure which captures the env vars 
        we have to use the cloned form of those types and pass them into
        the closure scopes so we can use them in later scopes 
    */
    let sender = data_sender.clone();
    tokio::spawn(async move{
        
        let new_string = String::from("an updated mutexed");
        /* 
            we're cloning data_instance and data_instance_cloned.actual to create a 
            longer lifetime value to use the cloned form to mutate, since by sending 
            data_instance_cloned to the channel its lifetime will be dropped and its 
            ownership will be moved because we're borroing the actual field by locking 
            on it so we can't move the data_instance_cloned into the mpsc channel using 
            the sender, in other words we can't move out of the type if it's behind a 
            shared reference we have to either pass a reference or clone the type and 
            work on the cloned form like the followings which we're cloning the actual 
            field to lock on its mutex and send the data_instance_cloned into 
            the downside of the channel
        */
        let data_instance_cloned = data_instance.clone();
        let data_instance_cloned_actual = data_instance_cloned.actual.clone();
        let mut data_string = data_instance_cloned_actual.lock().await; /* lock the mutex to mutate it */
        
        /* 
            mutating the locked mutex is done by dereferencing the guard 
            we're mutating data string inside the actual field in data_instance_cloned
            this will mutate the actual field inside data_instance_cloned 
        */
        *data_string = new_string; /* the actual field of the data_instance_cloned will be mutated too */

        if let Err(why) = sender.send(data_instance_cloned).await{
            println!("can't send because {:?}", why.to_string());
        }

    });

    /* receiving asyncly inside other threads to avoid blocking issues on heavy computations */
    tokio::spawn(async move{
        /* receving data asyncly while they're comming to the end of mpsc jobq channle */
        while let Some(data) = data_receiver.recv().await{
            
            let new_data_string = data.actual.lock().await.to_owned();
            println!("data instance actual value after getting mutated >>> [{}]", new_data_string);
    
        }
    });

}


pub mod bpf(){


    /* 
    
        with BPF VM we can compile the whole node 
        into an .elf or .so which contains the 
        BPF bytecode that can be executed from 
        the linux kernel. LLVM13 is needed 
        to compile BPF bytecode for Rust version
    
        https://blog.redsift.com/labs/writing-bpf-code-in-rust/
        binding using .so and https://crates.io/crates/pyo3

    */
    
    // bpf loader
    // ... 
    
}



