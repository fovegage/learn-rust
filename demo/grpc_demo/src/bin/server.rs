use std::thread;

use grpc_demo::hello_grpc::*;
use grpc_demo::hello::*;

struct HelloServer;

impl HelloService for HelloServer {
    fn say(&self,
           _o: grpc::ServerHandlerContext,
           req: grpc::ServerRequestSingle<SayRequest>,
           resp: grpc::ServerResponseUnarySink<SayResponse>) -> grpc::Result<()> {
        let mut r = SayResponse::new();

        println!("Say {}", req.message.get_name());

        r.set_msg("Success".to_string());
        resp.finish(r)
    }
}

fn main() {
    let mut server = grpc::ServerBuilder::new_plain();
    server.http.set_port(10086);
    server.add_service(HelloServiceServer::new_service_def(HelloServer));
    let _server = server.build().expect("Could not start server");
    loop {
        thread::park();
    }
}
