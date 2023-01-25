use argh::FromArgs;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, UdpSocket};

#[derive(FromArgs)]
/// A utility to echo TCP/UDP data.
struct Echo {
    /// listen port
    #[argh(option, short = 'p', default = "8000")]
    port: u16,
}

async fn echo_tcp(port: u16) {
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();
    loop {
        let Ok((mut stream, _)) = listener.accept().await else {
            break;
        };
        tokio::spawn(async move {
            let (mut r, mut w) = stream.split();
            tokio::io::copy(&mut r, &mut w).await;
        });
    }
}

async fn echo_udp(port: u16) {
    let socket = UdpSocket::bind(format!("0.0.0.0:{}", port)).await.unwrap();
    let mut buf = vec![0u8; 2 * 1024];
    loop {
        let Ok((n, addr)) = socket.recv_from(&mut buf).await else {
            break;
        };
        if socket.send_to(&buf[..n], addr).await.is_err() {
            break;
        }
    }
}

#[tokio::main]
async fn main() {
    let args: Echo = argh::from_env();
    println!("Listening :{}", args.port);
    tokio::join!(echo_tcp(args.port), echo_udp(args.port));
}
