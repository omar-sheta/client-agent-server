// server code using udp
use std::net::UdpSocket;
use std::io::prelude::*;
use std::io::BufReader;
use local_ip_address::local_ip;


// port num of client = 7878
// port num of agent = 7879
// port num of server = 7880


//receives server port num as first argument and agent port num as second argument and server num as third argument

fn main() {
    //get current ip address
    let ip = local_ip().unwrap().to_string();
    // get port number from terminal
    
    let server_port = std::env::args().nth(1).unwrap();
    let agent_port = std::env::args().nth(2).unwrap();
    let server_num = std::env::args().nth(3).unwrap();

    // send a message to the agent

    let ip_and_port = format!("{}:{}", ip, server_port);

    println!("Server: : {}", ip_and_port);

    //create a socket
    let mut socket = UdpSocket::bind(ip_and_port).unwrap();

    //create a buffer
    let mut buf = [0; 1024];
for _ in 0..1000000 {
    //read from the socket
    let agent_ip_port = format!("{}:{}", ip, agent_port);
    let (amt, src) = socket.recv_from(&mut buf).unwrap();
    //print the message
    println!("{} bytes received from {}", amt, src);
    println!("message: {}", String::from_utf8_lossy(&buf[..amt]));
    buf = [0; 1024];
    //send a message back to the agent
    let message = format!("Hello from the server: {}", server_num);;
    
    
    socket.send_to(message.as_bytes(), agent_ip_port).unwrap();



}
    //close the socket
    drop(socket);

}


