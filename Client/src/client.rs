use std::fs::File;
use std::io::{BufWriter, Read, Write};
use std::net::TcpStream;
use std::num::TryFromIntError;
use ServerLib::header::Header;
use ServerLib::request::{Request, RequestType};
use ServerLib::request::RequestType::Wrong;
use ServerLib::response;
use ServerLib::response::Resposne;

use ServerLib::serializable::Serializable;

pub struct Client
{
     stream: TcpStream,
}

impl Client
{
    pub(crate) fn new(host:&str)->Self
    {
        Self
        {
            stream: TcpStream::connect(host).expect("could connect")
        }
    }
    pub(crate) fn ping(&mut self)
    {
        let request = Request::new(Header::new(RequestType::Ping,0));
        self.write_request(&request);
        let response = match self.read_response()
        {
            None =>
                {
                    println!("could read message !");
                    return;
                }
            Some(response) =>
                {
                    response
                }
        };

        let delta_ms = response.header.time_created - request.header.time_created;

        let delta_seconds = delta_ms as f64 / 1000.0;

        println!("Time difference: {} milliseconds", delta_ms);
        println!("Time difference: {:.3} seconds", delta_seconds);
    }
    pub(crate) fn pwd(&mut self)
    {
        let request = Request::new(Header::new(RequestType::Pwd,0));
        self.write_request(&request);
        let response = match self.read_response()
        {
            None =>
                {
                    println!("could read message !");
                    return;
                }
            Some(response) =>
                {
                    response
                }
        };
        let message = String::from_utf8_lossy(response.body.as_slice()).to_string();
        println!("{}",message);
    }
    pub(crate) fn download(&mut self,file_name: &str)
    {
        let request = Request::new(Header::new(RequestType::Download(file_name.to_string()),file_name.len() as u64));
        self.write_request(&request);
        let response = match self.read_response()
        {
            None =>
                {
                    println!("couldn't read message !");
                    return;
                }
            Some(response) =>
                {
                    response
                }
        };
        let message = String::from_utf8_lossy(response.body.as_slice()).to_string();
        if  response.header.request_type == Wrong
        {
            println!("{}",message);
            return;
        }
        println!("{}",message);

        let mut output_file = File::create(file_name).unwrap();
        let mut buffer = [0;4096];

        while let Ok(bytes_read) = self.stream.read(&mut buffer)
        {
            if bytes_read == 0 || buffer[..bytes_read].ends_with(b"EOF")
            {
                output_file.write(&buffer[..bytes_read-3]).unwrap();
                return;
            }
            else
            {
                output_file.write(&buffer[..bytes_read]).unwrap();
            }
        }
    }
    pub(crate) fn cd(&mut self,path: &str)
    {
       let request = Request::new(Header::new(
           RequestType::Cd(path.to_string()
           ),path.len() as u64));
        self.write_request(&request);
        let response = match self.read_response()
        {
            None =>
                {
                    println!("could read message !");
                    return;
                }
            Some(response) =>
                {
                    response
                }
        };
        let message = String::from_utf8_lossy(response.body.as_slice()).to_string();
        println!("{}",message);
    }
    fn write_request(&mut self ,request: &Request)
    {
        match self.stream.write_all(request.serialize().as_slice())
        {
            Ok(_) =>
                {

                }
            Err(err) =>
                {
                    println!("{}", err);
                }
        }
    }
    fn read_response(&mut self)->Option<Resposne>
    {
        let mut buffer = [0; 4096];

        match self.stream.read(&mut buffer)
        {
            Ok(size) =>
                {
                    let exact_buffer = buffer[0..size].to_vec();
                     Resposne::deserialize(exact_buffer.as_slice())
                }
            Err(err) =>
                {
                    None
                }
        }
    }
}

