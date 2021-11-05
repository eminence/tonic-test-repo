use tokio_stream::wrappers::TcpListenerStream;
use tonic::transport::{Identity, Server, ServerTlsConfig};
use tonic_test::services::{ping_service_server::PingServiceServer, MyPingService};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let use_tls = std::env::args().any(|arg| arg == "--tls");
    let use_incoming = std::env::args().any(|arg| arg == "--incoming");

    // println!("Listening on port {}", port);

    let builder = Server::builder();

    let mut builder = if use_tls {
        let id = Identity::from_pem(
            std::fs::read_to_string("ssl/server.crt")?,
            std::fs::read_to_string("ssl/server.key")?,
        );

        println!("Using TLS");
        builder.tls_config(ServerTlsConfig::new().identity(id))?
    } else {
        println!("NOT using TLS");
        builder
    };

    let server = builder.add_service(PingServiceServer::new(MyPingService {}));

    if use_incoming {
        println!("Starting server using serve_with_incoming() ...");
        let listener = std::net::TcpListener::bind("0.0.0.0:50051")?;
        let listener = tokio::net::TcpListener::from_std(listener)?;

        server
            .serve_with_incoming(TcpListenerStream::new(listener))
            .await?;
    } else {
        println!("Starting server using serve() ...");
        server.serve("0.0.0.0:50051".parse().unwrap()).await?;
    }

    println!("Server is done");

    Ok(())
}
