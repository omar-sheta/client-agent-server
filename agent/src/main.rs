use std::thread;
use std::time::Duration;
use std::net::UdpSocket;
use local_ip_address::local_ip;
use std::collections::HashMap;
use std::io::BufReader;
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};
use std::string::String;

//check if you need 4 threads or 2 threads


//receive Address from 3 servers -> to port "7878"
//create a port to listen to messages from the server

fn main(){


    //get local ip
    let local_ip = local_ip().unwrap();

    let agent_listen_port = std::env::args().nth(1).unwrap();

    let agent_send_port = std::env::args().nth(2).unwrap();

    let client_port = std::env::args().nth(3).unwrap();

    let port_listen_server = std::env::args().nth(4).unwrap() ; // listen for addresses from 3 servers



    println!("Local IP: {}", local_ip);

    let socket_listen_server = UdpSocket::bind(format!("{}:{}", local_ip, port_listen_server)).unwrap();

    //vector of 3 addresses
    let mut addresses = Vec::new();


    let mut buf = [0; 1024];


    //blocking receive to get 3 addresses from 3 servers in the beginning

    
    for i in 0..=2{
        //receive address from servers
        match socket_listen_server.recv_from(&mut buf) {
            Ok((amt, src)) => {
                //print received message from server
                println!("Received from server: ({})", String::from_utf8_lossy(&buf[..amt]));
                //print src
                println!("Address: {}", src);
                addresses.push((src));
            }
            Err(e) => {
                println!("recv_from function failed: {:?}", e);
            }
        }


    }



    

    // get server 1,2,3 addresses 
    //change to be address of servers
    let server1 = addresses[0].to_string();
    let server2 =  addresses[1].to_string();
    let server3 =  addresses[2].to_string();

    let server1_clone = server1.clone();
    let server2_clone = server2.clone();
    let server3_clone = server3.clone();

    // vector of servers
    let mut servers = Vec::new();
    servers.push(server1);
    servers.push(server2);
    servers.push(server3);

    //create a hashmap to store server addresses and their status (true , false)
    let mut server_status = HashMap::new();
    server_status.insert(server1_clone, true);
    server_status.insert(server2_clone, true);
    server_status.insert(server3_clone, true);





    //create a vector to store server ports and local ip
    



    //create a socket dedicated to client

    let cleint_socket = UdpSocket::bind(format!("{}:{}", local_ip, agent_listen_port)).unwrap();

    //clone client socket
    let cleint_socket_clone = cleint_socket.try_clone().unwrap();
   
   
    //create a socket dedicated to server
    let server_socket = UdpSocket::bind(format!("{}:{}", local_ip, agent_send_port)).unwrap();
    let server_socket_clone = server_socket.try_clone().unwrap();

    let mut buf = [0; 1024];
    let mut buf2 = [0; 1024];

    //create a channel
    let (tx, rx) : (Sender<HashMap<String,bool>>, Receiver<HashMap<String,bool>>) = mpsc::channel();


    //channel to forward messages from server to client
    let (tx2, rx2) : (Sender<String>, Receiver<String>) = mpsc::channel();


    



    //create a thread to listen to messages from the server
    let handle = thread::spawn(move || {
        loop {

            println!("Listening to server");
            match server_socket.recv_from(&mut buf) {
                Ok((amt, src)) => {
                    //print received message from server
                    
                    //print received message from server
                    println!("Received from server: {}", String::from_utf8_lossy(&buf[..amt]));
                    let received = String::from_utf8_lossy(&buf[..amt]);
                    //check if the message is "Dead"
                    //clone the server_status hashmap
                    let mut server_status_clone = server_status.clone();
                    
                    if received == "Dead" {
                        //if the message is "Dead", change the status of the server to false
                        server_status.insert(src.to_string(), false);
                        //print the status of the server
                        println!("Server {} is dead", src.to_string());

                        //print the status of the servers
                    }

                    //print sendig server status
                    println!("Server status: {:?}", server_status);
                    //send the server_status hashmap to the client thread
                    tx.send(server_status_clone);
                    //send the message to the client thread
                    // tx2.send(received.to_string());




                }
                Err(e) => {
                    println!("recv_from function failed: {:?}", e);
                }
            }
            thread::sleep(Duration::from_millis(100));
        }
    });

    // //how to receive the hashmap from the server thread
    // let handle2 = thread::spawn(move || {
    //     loop {
    //         let received = rx.recv().unwrap();
    //         println!("Received: {:?}", received);
    //     }
    // });


    //create a thread to listen to messages from the client
    
    
    let handle2 = thread::spawn(move || {
       let mut i = 0;
        loop {

            //print in client thread
            println!("Client thread");

            let received = rx.recv().unwrap();
            println!("Received from channel: {:?}", received);
            let received_2 = received.clone();


            match cleint_socket.recv_from(&mut buf) {
                Ok((amt, src)) => {
                    //print received message from client
                    println!("Received: ({})", String::from_utf8_lossy(&buf[..amt]));
                    
                    
                    //select a server to send the message to
                    let mut server = servers[i%3].clone();

                    //check if the server is alive
                    if received_2.get(&server).unwrap() == &false {
                        //if the server is dead, select the next server
                        i = i + 1;
                        server = servers[(i)%3].clone();

                    }

                    
                    // if received_2.get(&server).unwrap() == &false {
                    //     i = i+1;
                    //     server = servers[(i)%3].clone();


                        
                    // }

                    //print selected server
                    // println!("Selected server: {}", server);
                    // if received_2.get(&server).unwrap() == &false {
                    //     server = servers[(i+2)%3].clone();
                    // }
                    //send the message to the server
                    server_socket_clone.send_to(&buf[..amt], server).unwrap();


                    
                }
                Err(e) => {
                    println!("recv_from function failed: {:?}", e);
                }
                
            
            }

            
            
            i = i+1;

            // println!("i: {}", i);

                // //print the status of the servers
                // println!("Server1: {}", server_status.get(&server1).unwrap());
                // println!("Server2: {}", server_status.get(&server2).unwrap());
                // println!("Server3: {}", server_status.get(&server3).unwrap());
            
            thread::sleep(Duration::from_millis(100));
        }
    });
   
    //thread to send response to client
    let handle3 = thread::spawn(move || {
        loop {

            let received = rx2.recv().unwrap();

            //send the message to the client

            //client address is localaddress:clientport
            let client_address = format!("{}:{}", local_ip, client_port);

            cleint_socket_clone.send_to(received.as_bytes(),client_address ).unwrap();
        }


            thread::sleep(Duration::from_millis(100));
        
    });

    handle.join().unwrap();
    handle2.join().unwrap();
    handle3.join().unwrap();

}



