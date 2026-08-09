pub async fn api() {
    let body = reqwest::get("https://www.rust-lang.org")
        .await.expect("Calling")
        .text()
        .await.expect("Called");
    println!("body = {body:?}");
}
