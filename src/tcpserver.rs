use std::io::{TcpListener, TcpStream};
use std::io::{Acceptor, Listener};
use std::string::String;

pub struct Server
{
    port: String,
    ip: String
}

pub trait ServerFunction
{
    fn new(new_port: String) -> Server;
    fn start_server(&self);
    fn format_ip(&self) -> String;
    fn get_port(&self) -> String;
    fn get_ip(&self) -> String;
}

impl ServerFunction for Server
{
    fn new(new_port: String) -> Server
    {
        return Server{port: new_port, ip: String::from_str("localhost")};
    }

    fn start_server(&self)
    {
        let ip = self.format_ip();
        let listener = TcpListener::bind(ip.as_slice());
        let mut acceptor = listener.listen().unwrap();
        for opt_stream in acceptor.incoming()
        {
            spawn(proc(){
                let mut stream = opt_stream.unwrap();
                stream.write(b"Hello World\r\n").unwrap();
                stream.write(b"Hello World\r\n").unwrap();
            })
        }
    }

    fn format_ip(&self) -> String
    {
        let mut complete_ip = String::new();
        complete_ip.push_str(self.ip.as_slice());
        complete_ip.push_str(":");
        complete_ip.push_str(self.port.as_slice());
        return complete_ip;
    }

    fn get_port(&self) -> String
    {
        let mut port = String::new();
        port.push_str(self.port.as_slice());
        return port;
    }

    fn get_ip(&self) -> String
    {
        let mut ip = String::new();
        ip.push_str(self.ip.as_slice());
        return ip;
    }
}
