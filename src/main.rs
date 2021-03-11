#[tokio::main]
async fn main() {
    token().await;
}

async fn token() {
    let con_token = egg_mode::KeyPair::new(
        "gCOpE7s0QP1rfnbNdS7DNEfF1",
        "FYYE8BzAKjNmo51fZjrTMio1Z1pAF67oPVfG07tMQSJHeQmbSo",
    );

    // "oob" is needed for PIN-based auth; see docs for `request_token` for more info
    let request_token = egg_mode::auth::request_token(&con_token, "oob")
        .await
        .unwrap();

    let auth_url = egg_mode::auth::authorize_url(&request_token);
    println!("{}", auth_url);
}
