pub async fn post(data: &Option<String>, url: &String) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let res = client.post(url).send().await?;
    // "http://httpbin.org/post"

    println!("{:#?}", res);
    Ok(())
}
