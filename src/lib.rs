pub mod services {
    use tonic::{Request, Response};

    tonic::include_proto!("services");

    pub struct MyPingService {}

    #[tonic::async_trait]
    impl ping_service_server::PingService for MyPingService {
        async fn ping(
            &self,
            _request: Request<PingRequest>,
        ) -> Result<Response<PingReply>, tonic::Status> {
            println!("Server received a ping!");
            Ok(Response::new(PingReply {}))
        }
    }
}
