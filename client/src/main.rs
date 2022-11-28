// use udp to create a connection to an agent and send a message using threads

use std::net::UdpSocket;
use std::io::prelude::*;
use std::io::BufReader;
use std::env;
use std::thread;
use local_ip_address::local_ip;



// take port num of client as first argument and port num of agent as second argument

fn main() {
    

    //get current ip address
    let ip = local_ip().unwrap().to_string();
    // get port number from terminal
    
    let port = std::env::args().nth(1).unwrap();


    // send a message to the agent

    let ip_and_port = format!("{}:{}", ip, port);

    //create a socket
    let mut socket = UdpSocket::bind(ip_and_port).unwrap();

    //create a buffer to read from
    let mut buf = [0; 1024];




    //create a thread to write to the socket
    let agent_port = env::args().nth(2).unwrap();
    
        let handle = thread::spawn(move || {
        
    

        // let mut input = String::new();
        // let mut reader = BufReader::new(std::io::stdin());
        // send messages
        loop {
            let agent_ip_port = format!("{}:{}", ip, agent_port);
            buf = [0; 1024];
            
            // send a message to the agent
            let message = format!("{}" ,port); // if load balancing is tested
            socket.send_to(message.as_bytes(), agent_ip_port).unwrap();
            //sleep for 100 ms
            std::thread::sleep(std::time::Duration::from_millis(2000));
            
            //take input from user
            // reader.read_line(&mut input).unwrap();
            //send input to agent


            // socket.send_to(input.as_bytes(), agent_ip_port).unwrap();
            // input.clear();
            let (amt, src) = socket.recv_from(&mut buf).unwrap();
            println!("{} bytes received from {}", amt, src);
            println!("message: {}", String::from_utf8_lossy(&buf[..amt]));

        }


    });

    handle.join().unwrap();

    //wait for threads to finish
    loop {}
    

}



