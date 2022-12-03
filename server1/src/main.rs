use std::thread;
use std::time::Duration;
use std::net::UdpSocket;
use local_ip_address::local_ip;
use std::collections::HashMap;
use std::io::BufReader;
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};

fn main() {
    //get local ip

    let local_ip = local_ip().unwrap();

    let agent_listen_port = std::env::args().nth(1).unwrap();

    let agent_send_port = std::env::args().nth(2).unwrap(); // send my own server address to client

    let server_num = std::env::args().nth(3).unwrap();

    //IMPPPPP modify local_ip to work on different physical machines and send to two agents


    // create a socket to send address to agent

    // let socket_send_address = UdpSocket::bind(format!("{}:{}", local_ip, agent_send_port)).unwrap();

    


    

    // create a socket to listen to messages from the agent

    let socket_listen = UdpSocket::bind(format!("{}:{}", local_ip, agent_listen_port)).unwrap();

    //create a socket to send response to the agent

    let socket_send = UdpSocket::bind(format!("{}:{}", local_ip, agent_send_port)).unwrap();

    socket_listen.send_to(server_num.as_bytes(), format!("{}:{}", local_ip, "7878")).unwrap(); //sending server number to agent1
    socket_listen.send_to(server_num.as_bytes(), format!("{}:{}", local_ip, "7879")).unwrap(); //sending server number to agent2



    //sleep for 2 seconds
    thread::sleep(Duration::from_secs(2));
    // socket_listen.send_to("Alive".as_bytes(), format!("{}:{}", local_ip, "7878")).unwrap(); //sending server number to agent1
    // socket_listen.send_to("Alive".as_bytes(), format!("{}:{}", local_ip, "7879")).unwrap(); //sending server number to agent2
    //clone listen socket

    let socket_listen_clone = socket_listen.try_clone().unwrap();


    let mut buf = [0; 1024];


    let (tx_message, rx_message) : (Sender<String>, Receiver<String>) = mpsc::channel();
    let (tx_address, rx_address) : (Sender<String>, Receiver<String>) = mpsc::channel();

    // create a thread to listen to messages from the agent 
    let handle = thread::spawn(move || {
        loop {
            match socket_listen.recv_from(&mut buf) {
                Ok((amt, src)) => {
                    //print received message from agent
                    println!("Received: ({})", String::from_utf8_lossy(&buf[..amt]));
                    //received message "send me data(i)"
                    //parse message and get i
                    let message = String::from_utf8_lossy(&buf[..amt]);
                    let message_vec: Vec<&str> = message.split("(").collect();
                    let message_vec2: Vec<&str> = message_vec[1].split(")").collect();
                    let request = message_vec2[0].parse::<String>().unwrap();

                    //send message to the other thread
                    tx_message.send(request).unwrap();
                    tx_address.send(src.to_string()).unwrap();
                    //send address to the other thread
                    // tx_address.send(src.to_string()).unwrap();
                    //print src
                    // println!("Address: {}", src);


                }
                Err(e) => {
                    println!("recv_from function failed: {:?}", e);
                }
            }
            thread::sleep(Duration::from_millis(100));
        }
    });

    // create a thread to send messages to the agent
    let handle2 = thread::spawn(move || {
        for i in 0..i32::MAX {

            


            if server_num == "1" {
            

                let message = format!("Dead");
                // socket_listen_clone.send_to(message.as_bytes(), format!("{}:{}",local_ip,"7882")).unwrap(); // send to agent 2
                socket_listen_clone.send_to(message.as_bytes(), format!("{}:{}",local_ip,"7884")).unwrap(); // send to agent 1
                socket_listen_clone.send_to(message.as_bytes(), format!("{}:{}",local_ip,"7882")).unwrap(); // send to agent 2
                println!("Sending: {} then I will sleep for 10 mins", message);

                thread::sleep(Duration::from_secs(600));


            } else {
                let message = format!("Alive");
                // // socket_listen_clone.send_to(message.as_bytes(), format!("{}:{}",local_ip,"7882")).unwrap(); // send to agent 2
                // socket_listen_clone.send_to(message.as_bytes(), format!("{}:{}",local_ip,"7884")).unwrap(); // send to agent 1
                // socket_listen_clone.send_to(message.as_bytes(), format!("{}:{}",local_ip,"7882")).unwrap(); // send to agent 2
                let req = rx_message.recv().unwrap();
                
                let message = format!("{}", req);

                let address = rx_address.recv().unwrap();

                println!("address: {}", address);
                // print sending to address
                // println!("Sending response: {} ", message);
                // socket_listen_clone.send_to(message.as_bytes(), format!("{}:{}",local_ip,"7882")).unwrap();
                socket_listen_clone.send_to(message.as_bytes(), address).unwrap(); // send to agent 1
            }

            


            //sleep for 2 seconds
            thread::sleep(Duration::from_millis(1000));
        }
    });

    handle.join().unwrap();
    handle2.join().unwrap();



}