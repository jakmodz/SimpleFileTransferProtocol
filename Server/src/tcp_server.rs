use ServerLib::header::*;
use ServerLib::response::*;
use ServerLib::request::*;
use ServerLib::serializable::*;
use std::io::{BufReader, Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::{fs, thread};
use std::path::{Path, PathBuf};
use std::fs::File;
use ServerLib::request::RequestType::{Download, Wrong};

pub struct TcpServer
{
    listener: TcpListener,
    path: Arc<Mutex<String>>,
}
impl TcpServer
{
    pub fn new(host: &str) -> Self
    {
        TcpServer
        {
            listener: TcpListener::bind(host).unwrap(),
            path: Arc::new(Mutex::new(String::from("C:\\"))),
        }
    }
    pub fn start(&mut self)
    {
        for stream in self.listener.incoming()
        {
            println!("connected");
            let mut stream = stream.unwrap();
            let path = Arc::clone(&self.path);
            thread::spawn(move ||
            {
                Self::handle(path, &mut stream);
            });
        }
    }
    fn handle(path: Arc<Mutex<String>>, stream: &mut TcpStream)
    {
        loop
        {
            let mut buffer = [0; 4096];
            let size = stream.read(&mut buffer).unwrap();
            let exact_buffer = buffer[..size].to_vec();
            match Request::deserialize(exact_buffer.as_slice())
            {
                None =>
                    {
                        println!("cannot read a message");
                        stream.shutdown(Shutdown::Both).unwrap()
                    }
                Some(request) => match request.header.request_type
                {
                    RequestType::Download(file_name) =>
                        {

                            let str_response = "downloading ".to_owned() + &*file_name.clone();
                            let mut  response = Resposne::new(Header::new(Download(file_name.clone()), str_response.len() as u64));
                            response.body = str_response.as_bytes().to_vec();
                            let mut  file_name_final = String::from(path.lock().unwrap().as_str());
                            file_name_final.push_str("\\");
                            file_name_final.push_str(file_name.as_str());
                            println!("{file_name_final}");
                            if !Path::new(&file_name_final).is_file()
                            {
                                let messeage = "there is no such a file";
                                response = Resposne::new(Header::new(Wrong, messeage.len() as u64));
                                response.body = messeage.as_bytes().to_vec();
                                stream.write_all(response.serialize().as_slice()).unwrap();
                                continue;
                            }
                            let input_file =
                            match File::open(file_name_final)
                                {
                                    Ok(file) => {file}
                                        Err(_) =>
                                        {
                                            break;
                                        }
                                };

                            stream.write_all(response.serialize().as_slice()).unwrap();
                            let mut buffer = [0; 4096];
                            let mut reader = BufReader::new(input_file);

                            loop
                            {
                                let bytes_read = match reader.read(&mut buffer) {
                                    Ok(0) => break,
                                    Ok(n) => n,
                                    Err(_) => break,};
                                stream.write(&buffer[..bytes_read]).unwrap();
                            }
                            stream.write_all(b"EOF").unwrap();
                        }
                    RequestType::Ping =>
                        {
                            let response = Resposne::new(Header::new(RequestType::Ping, 0));
                            stream.write_all(&response.serialize()).unwrap();
                        }
                    RequestType::Pwd =>
                        {
                            let path_len = path.lock().unwrap().len() as u64;
                            let mut response = Resposne::new(Header::new(RequestType::Pwd, path_len));
                            response.body.extend(path.lock().unwrap().as_bytes());
                            stream.write_all(&response.serialize()).unwrap();
                        }
                    RequestType::Cd(path_to_add) =>
                        {
                            let mut  cloned_string =path.lock().unwrap();
                            let origin_path = PathBuf::from(cloned_string.clone());
                            let relative_path = Path::new(&path_to_add);
                            let combined_path = origin_path.join(relative_path);
                            let resolved_path =
                                if combined_path.to_str().unwrap().contains(".")
                                    {
                                        fs::canonicalize(&combined_path).unwrap()
                                    }
                                else
                                    {
                                        combined_path
                                    };
                            if  resolved_path.is_dir()
                            {
                                *cloned_string = resolved_path.to_str().unwrap().to_string();
                            }
                            let path_len = cloned_string.len() as u64;
                            let mut response = Resposne::new(Header::new(RequestType::Cd(resolved_path.to_str().unwrap().to_string()), path_len));
                            response.body.extend(&*cloned_string.as_bytes());
                            stream.write_all(&response.serialize()).unwrap();
                        }
                    _ => {}
                }
            }
        }
    }
}