#[cfg(test)]
mod tests {
    use kanwar_server::approach_1_server::Approach1Server;
    use kanwar_server::common::ServerTrait;
    use reqwest::ClientBuilder;
    use std::{net::TcpListener, thread, time::Duration};
    #[tokio::test]
    async fn server_running_should_be_able_to_respond_to_get_request() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener
            .local_addr()
            .expect("Unable to get the address of the test server");

        let approach_1_server = Approach1Server::new(4);
        thread::spawn(move || {
            approach_1_server
                .start_listening(listener)
                .expect("failed listening to client calls ");
        });

        let client = ClientBuilder::new()
            .timeout(Duration::from_secs(3))
            .build()
            .expect("client malformed");
        let x = client
            .get(format!("http://{}:{}", addr.ip(), addr.port()))
            .build()
            .expect("unable to form a proper request");
        let resp = client
            .execute(x)
            .await
            .expect("Got an error calling the get endpoint");
        assert_eq!(resp.status(), 200);
        assert_eq!(
            String::from_utf8(resp.bytes().await.unwrap().to_vec()).unwrap(),
            "Got a simple get request"
        );
    }
    #[tokio::test]
    async fn server_running_should_be_able_to_respond_to_post_request() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener
            .local_addr()
            .expect("Unable to get the address of the test server");

        let approach_1_server = Approach1Server::new(4);
        thread::spawn(move || {
            approach_1_server
                .start_listening(listener)
                .expect("failed listening to client calls ");
        });

        let client = ClientBuilder::new()
            .timeout(Duration::from_secs(3))
            .build()
            .expect("client malformed");
        let x = client
            .post(format!("http://{}:{}", addr.ip(), addr.port()))
            .build()
            .expect("unable to form a proper request");
        let resp = client
            .execute(x)
            .await
            .expect("Got an error calling the get endpoint");
        assert_eq!(resp.status(), 200);
        assert_eq!(
            String::from_utf8(resp.bytes().await.unwrap().to_vec()).unwrap(),
            "Oh Noo post req"
        );
    }
}
