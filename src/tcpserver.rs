use std::io::TcpListener;
use std::io::{Acceptor, Listener};
use std::string::String;
use data::ServerData;

pub struct Server
{
    port: String,
    ip: String
}

pub trait ServerFunction
{
    fn new(new_port: String, new_ip: String) -> Self;
    fn start_server<T: ServerData>(&self, mut data_object: T);
    fn format_ip(&self) -> String;
    fn get_port(&self) -> String;
    fn get_ip(&self) -> String;
}

impl ServerFunction for Server
{
    fn new(new_port: String, new_ip: String) -> Server
    {
        return Server{port: new_port, ip: new_ip};
    }

    fn start_server<T: ServerData>(&self, mut data_object: T)
    {
        let ip = self.format_ip();
        let listener = TcpListener::bind(ip.as_slice());
        let mut acceptor = listener.listen().unwrap();
        for opt_stream in acceptor.incoming()
        {
            let stream = opt_stream.unwrap();
            data_object.process_request_data(stream);
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
