use std::thread;
use std::time::Duration;
use std::net::UdpSocket;
use local_ip_address::local_ip;
use std::collections::HashMap;
use std::io::BufReader;
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};
// library to generate random numbers
use rand::Rng;
use std::time::Instant;


fn main() {
    //get local ip

    let local_ip = local_ip().unwrap();

    let agent_listen_port = std::env::args().nth(1).unwrap();

    let agent_send_port = std::env::args().nth(2).unwrap(); // send my own server address to client

    let server_num = std::env::args().nth(3).unwrap();

    let election_port = std::env::args().nth(4).unwrap(); // address of server 1 election port

    let server1_port = std::env::args().nth(5).unwrap(); // address of server 2 election port

    let server2_port = std::env::args().nth(6).unwrap(); // address of server 3 election port


    let election_socket = UdpSocket::bind(format!("{}:{}", local_ip, election_port)).unwrap(); // election socket




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
    // start a timer

    let socket_listen_clone = socket_listen.try_clone().unwrap();
    let socket_listen_clone_1 = socket_listen.try_clone().unwrap();


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

        }
    });



    // channel to receive message from the other election thread
    let (tx_election, rx_election) : (Sender<bool>, Receiver<bool>) = mpsc::channel();
    
    // create a thread to send messages to the agent
    let handle2 = thread::spawn(move || {
        loop {

                
               




                
                // let message = format!("Alive");
                                
                //     // socket_listen_clone.send_to(message.as_bytes(), format!("{}:{}",local_ip,"7882")).unwrap(); // send to agent 2
                // socket_listen_clone.send_to(message.as_bytes(), format!("{}:{}",local_ip,"7884")).unwrap(); // send to agent 1
                // socket_listen_clone.send_to(message.as_bytes(), format!("{}:{}",local_ip,"7882")).unwrap(); // send to agent 2

                

                    // declare request and address
                    let mut req = String::new();
                    let mut address = String::new();
                    let mut message = String::new();
            
                
                    req = rx_message.try_recv().unwrap_or("No req".to_string());
                    address = rx_address.try_recv().unwrap_or("No add".to_string());
                    message = format!("{}", req);


               




                // let req = rx_message.recv_timeout(Duration::from_secs(1)).unwrap();
                
                // let message = format!("{}", req);

                // let address = rx_address.recv_timeout(Duration::from_secs(1)).unwrap();

                

                // println!("address: {}", address);
                // print sending to address
                // println!("Sending response: {} ", message);
                // socket_listen_clone.send_to(message.as_bytes(), format!("{}:{}",local_ip,"7882")).unwrap();
                if(message != "No req".to_string() && address != "No add".to_string()) {
                    socket_listen_clone.send_to(message.as_bytes(), address).unwrap(); // send to agent
                }else{
                    continue;
                }
                

                // }
                // else {
                //     let message = format!("Dead");
                //     // print sending Dead message from server number
                //     // println!("Sending: {} from server number {}", message, server_num);
                                
                //             // socket_listen_clone.send_to(message.as_bytes(), format!("{}:{}",local_ip,"7882")).unwrap(); // send to agent 2
                //     socket_listen_clone.send_to(message.as_bytes(), format!("{}:{}",local_ip,"7884")).unwrap(); // send to agent 1
                //     socket_listen_clone.send_to(message.as_bytes(), format!("{}:{}",local_ip,"7882")).unwrap(); // send to agent 2
                //     // print received request will be dropped
                //     // println!("Received request will be dropped");
                // }



            // }

        }
    });

    

    // start a timer
    // thread to have election between 3 servers
    let handle3 = thread::spawn(move || {

       // print in election thread
        println!("Election thread started");
        // sleep for 5 seconds
        thread::sleep(Duration::from_secs(5));

        




        

        loop {


             // generate random number between 1 and 1000000 to decide who is the leader
             let mut random_number = rand::thread_rng().gen_range(0..10000);


             //send random number to the other servers

            
            
            // if start.elapsed().as_secs()%30 < 10 {
   
            election_socket.send_to(random_number.to_string().as_bytes(), format!("{}:{}",local_ip,server1_port)).unwrap();
            election_socket.send_to(random_number.to_string().as_bytes(), format!("{}:{}",local_ip,server2_port)).unwrap();

            
            // }
            

            let mut numbers = vec![random_number, 0, 0];
                


                for i in 1..3 {


                    // send random number to the other servers
                // receive numbers from the 2 severs from other servers on election socket
                    let mut buf = [0; 1024];
                    match election_socket.recv_from(&mut buf) {
                        Ok((amt, src)) => {
                            //print received message from agent
                            println!("Received: ({})", String::from_utf8_lossy(&buf[..amt]));
                            // change the number in the vector to the received number
                            let message = String::from_utf8_lossy(&buf[..amt]);
                            // let message be int
                            let request = message.parse::<i32>().unwrap();
                            

                            numbers[i] = request;

                            if i == 2 {
                                
                                // find the max number
                                let max = numbers.iter().max().unwrap();
                                // if the max number is the random number, then this server is the leader
                                let start = Instant::now();
                                if max == &random_number {
                                    
                                    println!("I am the leader");

                                    println!("sending Dead");
                                    // send true to the other thread

                                    // tx_election.send(false).unwrap();
                                    socket_listen_clone_1.send_to("Dead".as_bytes(), format!("{}:{}",local_ip,"7884")).unwrap(); // send to agent 1
                                    socket_listen_clone_1.send_to("Dead".as_bytes(), format!("{}:{}",local_ip,"7882")).unwrap(); // send to agent 2
                                    // sleep for 30 seconds
                                    // thread::sleep(Duration::from_secs(30));
                                    while(start.elapsed().as_secs()%31 < 30) {
                                        
                                        // send true to the other thread
                                    //     socket_listen_clone_1.send_to("Dead".as_bytes(), format!("{}:{}",local_ip,"7884")).unwrap(); // send to agent 1
                                    // socket_listen_clone_1.send_to("Dead".as_bytes(), format!("{}:{}",local_ip,"7882")).unwrap(); // send to agent 2

                                        // tx_election.send(false).unwrap();
                                    }
                                } else {
                                    println!("I am not the leader");
                                    println!("sending Alive");

                                    socket_listen_clone_1.send_to("Alive".as_bytes(), format!("{}:{}",local_ip,"7884")).unwrap(); // send to agent 1
                                    socket_listen_clone_1.send_to("Alive".as_bytes(), format!("{}:{}",local_ip,"7882")).unwrap(); // send to agent 2



                                    // send false to the other thread 
                                    // tx_election.send(true).unwrap();
                                    // tx_election.send(true).unwrap();
                                    while(start.elapsed().as_secs()%31 < 30) {
                                    //     socket_listen_clone_1.send_to("Alive".as_bytes(), format!("{}:{}",local_ip,"7884")).unwrap(); // send to agent 1
                                    // socket_listen_clone_1.send_to("Alive".as_bytes(), format!("{}:{}",local_ip,"7882")).unwrap(); // send to agent 2
                                        // send true to the other thread
                                        // tx_election.send(true).unwrap();

                                    }
                                }
                                
                                
                            }

                            
                                
                            }
                        Err(e) => {
                            println!("recv_from function failed: {:?}", e);
                        }






                    }
                }
            }

      



       


    });


    handle.join().unwrap();
    handle2.join().unwrap();
    handle3.join().unwrap();



}