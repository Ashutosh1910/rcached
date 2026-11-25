use std::{
    cell::RefCell, collections::HashMap, format, io::{BufReader, prelude::*}, net::{TcpListener, TcpStream}, unimplemented,
};
use log::{info,};
use env_logger;
use clap::Parser;

 
struct Value{
    bytes:Vec<u8>,
}

struct LRU<K,V>{
    capacity:u128,
    size:u128,
    map:HashMap<K,V>,
    head:Option<DoublyLinkedList<K,V>>
}
struct DoublyLinkedList<K,V>{
    key:K,
    value:V,
    previous:Option<Box<RefCell<DoublyLinkedList<K,V>>>>,
    next:Option<Box<RefCell<DoublyLinkedList<K,V>>>>
}

impl LRU<String,Value> {
    fn new(cap:u128)->Self{
        Self { capacity: cap, size: 0, map: HashMap::new(), head: None }
    }

    fn insert(){
        unimplemented!()
    }

    fn get(){
        unimplemented!()
    }
}

//Server CLI for rCached, a simple object cache implementation in Rust.
#[derive(Parser,Debug)]
 struct Cli{
     
     #[arg(short,long,default_value_t = 8000)]
     port: u16,
     //Cache node size in MB
     #[arg(short,long,default_value_t = 512)]
     size: u128
 }



fn server_loop(port:u16){
    let address=format!("0.0.0.0:{port}");
    let listener = TcpListener::bind(&address).expect("Failed to start tcp server");
    info!("Started tcp server at {address}");
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {http_request:#?}");
    stream.write(b"hello\n").unwrap();
}

fn main() {
    env_logger::init();
    
    let args=Cli::parse();
    
    server_loop(args.port);
    
    
}