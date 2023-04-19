use std::{net::TcpListener, time::Duration};

use reqwest::ClientBuilder;

#[test]
fn server_running_should_be_able_to_respond_to_request() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener
        .local_addr()
        .expect("Unable to get the address of the test server");
    let client = ClientBuilder::new().timeout(Duration::from_secs(3)).build();
}
