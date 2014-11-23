use std::io::TcpStream;

trait Data
{
    fn process_request_data(&self, mut request: TcpStream);
}
