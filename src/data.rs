use std::io::TcpStream;

pub trait ServerData
{
    fn process_request_data(&mut self, request: TcpStream);
}
