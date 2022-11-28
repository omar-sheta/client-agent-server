//agent to connect client to server using udp
use std::net::UdpSocket;
use local_ip_address::local_ip;
// port number of client1 = 7877
// port num of client2 = 7878
// port num of agent = 7879
// port num of server1 = 7880
// port num of server2 = 7881
// port num of server3 = 7882


//receive agent port num as first arg and client port num as second arg and server port num as third arg
fn main() {
    //get current ip address
    let ip = local_ip().unwrap().to_string();
    // get port number from terminal

    let agent_port = std::env::args().nth(1).unwrap();

    let server1_port = std::env::args().nth(2).unwrap();
    let server2_port = std::env::args().nth(3).unwrap();
    let server3_port = std::env::args().nth(4).unwrap();

    //create array of server ports
    let server_ports = [server1_port, server2_port, server3_port];
    //create array of client ports
    // let client_ports = [client1_port, client2_port];

    // send a message to the agent

    let ip_and_port_agent = format!("{}:{}", ip, agent_port);
    let mut agent_socket = UdpSocket::bind(ip_and_port_agent).unwrap();
    let mut buf = [0; 1023];
    for i in 0..20000000{
    //change server port to send to different server
    
    // thread to receive and send to client
    let server_port = server_ports[i%3].clone();
    // let ip_and_port_client = format!("{}:{}", ip, client_ports[i%2]);
    // println!("ip and port of client is {}", ip_and_port_client);
    let ip_and_port_server = format!("{}:{}", ip, server_ports[i%3]);
    // listen for messages from different clients
        
        
    let (amt, src) = agent_socket.recv_from(&mut buf).unwrap();
    println!("{} bytes received from {}", amt, src);
    
    let ip_and_port_client = src.to_string();
    println!("client address{}",ip_and_port_client);
    println!("message: {}", String::from_utf8_lossy(&buf[..amt]));
    agent_socket.send_to(&buf, ip_and_port_server).unwrap();
    //clear buffer
    buf = [0; 1023];
    let (amt, src) = agent_socket.recv_from(&mut buf).unwrap();
    agent_socket.send_to(&buf, ip_and_port_client).unwrap();  
    buf = [0; 1023];
    

    }


}


