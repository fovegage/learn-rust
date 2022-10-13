use futures::executor;
use grpc::ClientStubExt;

use grpc_demo::hello::*;
use grpc_demo::hello_grpc::*;

fn main() {
    let client = HelloServiceClient::new_plain("127.0.0.1", 10086, Default::default()).unwrap();
    let mut req = SayRequest::new();
    req.set_name("FraynJo".to_string());

    let resp = client.say(grpc::RequestOptions::new(), req).join_metadata_result();
    let resp = executor::block_on(resp);
    match resp {
        Err(e) => panic!("{:?}", e),
        Ok((_, r, _)) => println!("{:?}", r),
    }
}
