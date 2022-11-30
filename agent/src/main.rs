use std::net::UdpSocket;
use local_ip_address::local_ip;
use std::collections::HashMap;
use std::io::BufReader;



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


    //create array of server ports that can be used to send to
    let server_ports = [server1_port, server2_port, server3_port];

    let server_ports1 = server_ports.clone();


    // //map between server address and boolean true if server is available and false if not
    // let mut server_map = HashMap::new();



    //initialize server map of string and bool
    let mut server_map = HashMap::new();
    
    for server in server_ports {
        server_map.insert(server, true);
    }

    
    //create array of client ports
    // let client_ports = [client1_port, client2_port];

    // send a message to the agent

    let ip_and_port_agent = format!("{}:{}", ip, agent_port);
    let agent_socket = UdpSocket::bind(ip_and_port_agent).unwrap();


    //wait for a message from server
    let mut buf = [0; 10];
    let (amt, src) = agent_socket.recv_from(&mut buf).unwrap();

    //let dead_server = src.to_string();
    let dead_server = src.port().to_string();
    
    
    println!("Server:{} is Dead", src);

    //if message is "Dead" then set server to false
    if buf[0] == 68 && buf[1] == 101 && buf[2] == 97 && buf[3] == 100 {
        server_map.insert(dead_server, false);
    }


    
    

    let mut buf = [0; 1023];
    for i in 0..20000000{
   

    let mut server_port = server_ports1[i%3].clone();

    if server_map.get(&server_port) == Some(&true) {
        let ip_and_port_server = format!("{}:{}", ip, server_port);
    }
    else{

        let dead_server = server_port.clone();
        // println!("Server {}:{} is dead",ip, dead_server);
        //skip this iteration
        continue;
    }
    

    


    let ip_and_port_server = format!("{}:{}", ip, server_port);
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


