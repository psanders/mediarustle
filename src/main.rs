use tokio::net::UdpSocket;

struct MediaServer {
  listener: UdpSocket,
}

impl MediaServer {
  async fn new(addr: &str) -> Result<Self, std::io::Error> {
    let listener = UdpSocket::bind(addr).await?;
    Ok(Self { listener })
  }

  async fn run(self) -> Result<(), std::io::Error> {
    let mut buf = [0; 1024];
    loop {
      let (len, addr) = self.listener.recv_from(&mut buf).await?;
      println!("{:?} bytes received from {:?}", len, addr);
    }
  }
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
  let server = MediaServer::new("0.0.0.0:5000").await?;
  server.run().await?;
  Ok(())
}