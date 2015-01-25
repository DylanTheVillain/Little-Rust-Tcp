use std::string::String;
use std::vec::Vec;
use std::io::TcpStream;

pub struct Clients
{
    destinations: Vec<String>,
}

pub trait ClientFunction
{
    fn new() -> Self;
    fn add_client(&mut self, ip: String, port: String);
    fn remove_client(&mut self, index: usize);
    fn send_message(&mut self, message: String);
}

impl ClientFunction for Clients
{
    fn new() -> Clients
    {
        return Clients{destinations: Vec::new()};
    }

    fn add_client(&mut self, ip: String, port: String)
    {
        let mut port_ip = String::new();
        port_ip.push_str(ip.as_slice());
        port_ip.push_str(":");
        port_ip.push_str(port.as_slice());
        self.destinations.push(port_ip);
    }

    fn remove_client(&mut self, index: usize)
    {
        self.destinations.remove(index);
    }

    fn send_message(&mut self, message: String)
    {
        for dest in self.destinations.iter()
        {
            let mut tcp_stream = TcpStream::connect(dest.as_slice()).unwrap();
            let _ = tcp_stream.write(message.as_bytes());
        }
    }
}
