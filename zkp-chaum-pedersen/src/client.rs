use std::io::stdin;

use num_bigint::BigUint;
use zkp_chaum_pedersen::ZKP;

pub mod zkp_auth {
    include!("./zkp_auth.rs");
}

use zkp_auth::{auth_client::AuthClient, AuthenticationAnswerRequest, AuthenticationChallengeRequest, RegisterRequest};

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

    buf.clear();
    println!("Please provide your password (to register):");
    stdin()
        .read_line(&mut buf)
        .expect("could not parse user input from stdin");
    let password = BigUint::from_bytes_be(buf.trim().as_bytes());


    // create ZKP protocol
    let (alpha, beta, prime, order) = ZKP::get_constants();
    let zkp = ZKP {
        prime: prime.clone(),
        order: order.clone(),
        alpha: alpha.clone(),
        beta: beta.clone(),
    };

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
        user: user_name.clone(),
        y1: y1.to_bytes_be(),
        y2: y2.to_bytes_be(),
    };
    let _response = client.register(request)
        .await
        .expect("could not register in server");

    // authentication challenge request
    let k = ZKP::generate_random_number(&order);
    let r1 = ZKP::exponentiate(
        &alpha,
        &k,
        &prime,
    );
    let r2 = ZKP::exponentiate(
        &beta,
        &k,
        &prime,
    );
    let request= AuthenticationChallengeRequest {
        user: user_name,
        r1: r1.to_bytes_be(),
        r2: r2.to_bytes_be(),
    };

    let _response = client.create_authentication_challenge(request)
        .await
        .expect("could not create challenge")
        .into_inner();

    let auth_id = _response.auth_id;
    let c = BigUint::from_bytes_be(&_response.c);

    // authentication answer request
    buf.clear();
    println!("Please provide your password (to log in):");
    stdin()
        .read_line(&mut buf)
        .expect("could not parse user input from stdin");
    let password = BigUint::from_bytes_be(buf.trim().as_bytes());
    
    let s = zkp.solve(&k, &c, &password);

    let request = AuthenticationAnswerRequest {
        auth_id: auth_id,
        s: s.to_bytes_be(),
    };

    let _response = client.verify_authentication(request)
        .await
        .expect("server could not verify authentication of user")
        .into_inner();

    println!("You logged in. SessionId is {}", _response.session_id);
}