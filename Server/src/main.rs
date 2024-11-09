mod tcp_server;

use crate::tcp_server::*;
fn main()
{
    let _ = TcpServer::new("127.0.0.1:8080").start();
}
