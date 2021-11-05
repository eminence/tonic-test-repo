use tonic::transport::{Certificate, Channel, ClientTlsConfig};
use tonic_test::services::PingRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let use_tls = std::env::args().any(|arg| arg == "--tls");

    let ca_cert = Certificate::from_pem(std::fs::read_to_string("ssl/ca.crt")?);

    let tls = ClientTlsConfig::new()
        .ca_certificate(ca_cert)
        .domain_name("localhost");

    let endpoint = Channel::from_static("http://localhost:50051");
    let channel = if use_tls {
        println!("Connecting with TLS");
        endpoint.tls_config(tls)?.connect().await?
    } else {
        println!("Connecting without TLS");
        endpoint.connect().await?
    };
    println!("Connection established, making RPC call...");

    let mut client = tonic_test::services::ping_service_client::PingServiceClient::new(channel);
    let resp = client.ping(PingRequest {}).await?;
    println!("Response: {:?}", resp);

    Ok(())
}
