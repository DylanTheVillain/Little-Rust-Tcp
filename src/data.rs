use std::io::TcpStream;

pub trait Data
{
    fn process_request_data(&mut self, request: TcpStream);
}
