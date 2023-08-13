>Here are the steps to write a simple 'ngrok' like web server using Rust and the 'axum' crate that opens an SSH tunnel to localhost:8080 and exposes it publicly:

1. First, you'll need to install the `axum` and `tokio` crates by adding them to your `Cargo.toml` file:
```toml
[dependencies]
axum = "0.3"
tokio = { version = "1", features = ["full"] }
```

2. Next, you can write a simple web server using `axum` that listens on port 8080:
```rust
use axum::{handler::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));
    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
```

3. To expose the web server publicly, you can use an SSH tunnel to forward traffic from a remote server to your local machine on port 8080. You can do this by running the following command on your local machine:
```sh
ssh -R 8080:localhost:8080 user@remote_server
```
Make sure to replace `user` with your username on the remote server and `remote_server` with the address of the remote server.

4. To redirect traffic from your Azure static website to your Rust web server, you can configure a URL redirect using Azure Front Door or Azure Application Gateway  . You'll need to set up a redirect rule that forwards traffic from your Azure website to the public IP address of your remote server on port 8080.

I hope this helps! Let me know if you have any questions or need further assistance.


---
### 2nd Alternative is to use JS NPM

npm install -g localtunnel
```bash
#run my backend
cargo run 
#expose publicaly through proxy server
lt --port 8080 --subdomain yoursubdomain
```
https://github.com/localtunnel/localtunnel

# 0.0.0.0 Refresher

In networking, `0.0.0.0` means "all IPv4 addresses on the local machine". 
When a service is listening on `0.0.0.0`, it means that the service is listening on all network interfaces, 
including the loopback interface and any other network interfaces that are configured on the machine. 
This allows the service to be accessible from any IP address that the machine can receive traffic on.

In the case of our Rust web server, we bind to `0.0.0.0` so that the server can accept incoming connections from any network interface, 
including from the remote server when we set up the SSH tunnel. 
This allows traffic to be forwarded from the remote server to our local machine and be handled by our web server.
