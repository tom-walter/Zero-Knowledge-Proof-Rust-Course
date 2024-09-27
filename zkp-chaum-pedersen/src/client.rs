use std::io::stdin;

use num_bigint::BigUint;
use tonic::Request;
use zkp_chaum_pedersen::ZKP;

pub mod zkp_auth {
    include!("./zkp_auth.rs");
}

use zkp_auth::{auth_client::AuthClient, RegisterRequest};

#[tokio::main]
async fn main() {
    let addr = "http://127.0.0.1:50051".to_string();
    let mut client = AuthClient::connect(addr)
        .await
        .expect("could not connect to the server");
    println!("☑ Connected client to the server.");

    // read user's input
    let mut buf = String::new();
    println!("Please provide your username:");
    stdin()
        .read_line(&mut buf)
        .expect("could not parse user input from stdin");
    let user_name = buf.trim().to_string();

    println!("Please provide your password:");
    stdin()
        .read_line(&mut buf)
        .expect("could not parse user input from stdin");
    let password = BigUint::from_bytes_be(buf.trim().as_bytes());


    // create ZKP protocol
    let (alpha, beta, prime, order) = ZKP::get_constants();

    // register request
    let y1 = ZKP::exponentiate(
        &alpha,
        &password,
        &prime,
    );
    let y2 = ZKP::exponentiate(
        &beta,
        &password,
        &prime,
    );
    let request= RegisterRequest {
        user: user_name,
        y1: y1.to_bytes_be(),
        y2: y2.to_bytes_be(),
    };
    let _response = client.register(request)
        .await
        .expect("could not register in server");
}