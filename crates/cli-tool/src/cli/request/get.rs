pub async fn get(url: &String) -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get(url).await?;
    println!("@@ {:#?}", resp);
    Ok(())
}
