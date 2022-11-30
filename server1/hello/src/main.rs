// server code using udp
use std::net::UdpSocket;
use std::io::prelude::*;
use std::io::BufReader;
use local_ip_address::local_ip;





//receives server port num as first argument and agent port num as second argument and server num as third argument

fn main() {
    //get current ip address
    let ip = local_ip().unwrap().to_string();
    // get port number from terminal
    
    let server_port = std::env::args().nth(1).unwrap();
    // let agent_port = std::env::args().nth(2).unwrap();

    let agent1_port = std::env::args().nth(2).unwrap();
    let agent2_port = std::env::args().nth(3).unwrap();

    let server_num = std::env::args().nth(4).unwrap();

    // create array of agent ports
    let agent_ports = [agent1_port, agent2_port];

    let ip_and_port = format!("{}:{}", ip, server_port);

    println!("Server: : {}", ip_and_port);


    

    //create a socket
    let mut server_socket = UdpSocket::bind(ip_and_port).unwrap();

    if server_num == "1"{
        //send to agents that message "Dead"
        for agent in agent_ports{
            let ip_and_port_agent = format!("{}:{}", ip, agent);
            server_socket.send_to(b"Dead", ip_and_port_agent).unwrap();
        }

    }

    


    //create a buffer
    let mut buf = [0; 1024];
    for _ in 0..1000000 {
        //read from the socket
        // let agent_ip_port = format!("{}:{}", ip, agent_port);
        let (amt, src) = server_socket.recv_from(&mut buf).unwrap();
        //print the message
        println!("{} bytes received from {}", amt, src);
        let agent_ip_port = src.to_string();

        println!("message: {}", String::from_utf8_lossy(&buf[..amt]));
        buf = [0; 1024];
        //send a message back to the agent
        let message = format!("Hello from the server: {}", server_num);
        
        
        server_socket.send_to(message.as_bytes(), agent_ip_port).unwrap();



    }
    //close the socket
    drop(server_socket);

}


