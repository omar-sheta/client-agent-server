use std::thread;
use std::time::Duration;
use std::net::UdpSocket;
use local_ip_address::local_ip;
use std::collections::HashMap;
use std::io::BufReader;


fn main() {
    
    // get local ip
    let local_ip = local_ip().unwrap();
    println!("Local IP: {}", local_ip);
    
    let client_port_listen = std::env::args().nth(1).unwrap();

    let client_port_send = std::env::args().nth(2).unwrap();

    let agent_port = std::env::args().nth(3).unwrap();

    // create a socket to listen to messages from the agent


    //create a socket to listen to messages from the agent

    let socket_listen = UdpSocket::bind(format!("{}:{}", local_ip, client_port_listen)).unwrap();

    let mut buf = [0; 1024];
    // let mut buf2 = [0; 1024];

    // create a socket to send messages to the agent
    let socket_send = UdpSocket::bind(format!("{}:{}", local_ip, client_port_send)).unwrap();


    // create a thread to send messages to the agent
    let handle = thread::spawn(move || {
        for i in 0..i32::MAX {
            println!("Sending message to agent {}",agent_port);

            let message = format!("Send me data({})", i);
            socket_send.send_to(message.as_bytes(), format!("{}:{}", local_ip, agent_port)).unwrap();
            //sleep for 1 second
            thread::sleep(Duration::from_secs(1));
        }
    });

    // create a thread to listen to messages from the agent
    let handle2 = thread::spawn(move || {
        loop {
            match socket_listen.recv_from(&mut buf) {
                Ok((amt, src)) => {
                    //print received message from agent
                    println!("Received: ({})", String::from_utf8_lossy(&buf[..amt]));
                }
                Err(e) => {
                    println!("recv_from function failed: {:?}", e);
                }
            }
            thread::sleep(Duration::from_millis(100));
        }
    });

    handle.join().unwrap();
    handle2.join().unwrap();






}

