use crate::client::*;
use std::io::{self, Write};

const HOST:&str = "127.0.0.1:8080";
enum CommandType
{
    Cd(String),
    Ping,
    Pwd,
    Download(Vec<String>),
    Unknow
}


pub struct Cli
{
    client: Client
}

impl Cli
{
    pub fn new()->Self
    {
            Cli
            {
                client:Client::new(HOST)
            }
    }
    pub fn start(&mut self)
    {
        println!("connected");
        loop
        {
            eprint!("command: ");
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            match parse_line(&input)
            {
                CommandType::Cd(path) =>
                    {
                        self.client.cd(path.as_str());
                    }
                CommandType::Ping =>
                    {
                        self.client.ping();
                    }
                CommandType::Pwd =>
                    {
                        self.client.pwd();
                    }
                CommandType::Download(fielenames) =>
                    {
                        for download in fielenames
                        {
                            self.client.download(download.as_str());
                        }
                    }
                CommandType::Unknow =>
                    {
                        println!("there is no such a commend");
                    }
            }

        }
    }
}
fn parse_line(input: &String)->CommandType
{
    let words: Vec<String> = input.split_whitespace()
    .map(|s| s.to_string())
    .collect();
    match words[0].to_lowercase().as_str()
    {
        "ping"=>
            {
                CommandType::Ping
            }
        "cd"=>
            {
                return if words.len() > 1 && words.len() < 3
                {
                    CommandType::Cd(words[1].clone())
                }
                else
                {
                    CommandType::Unknow
                }
            }
        "pwd"=>
            {
                CommandType::Pwd
            }
        "download"=>
            {


                    let mut vec = Vec::new();
                    for i in 1..words.len()
                    {
                            vec.push(words[i].clone());
                    }
                    CommandType::Download(vec)

            }
        _=>
            {
                CommandType::Unknow
            }
    }
}