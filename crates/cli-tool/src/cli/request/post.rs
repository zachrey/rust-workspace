use std::collections::HashMap;

use serde_json::Value;

// demo url => "http://httpbin.org/post"
pub async fn post(data: &Option<String>, url: &String) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let res = client.post(url);

    let res = match data {
        Some(data) => {
            let json_data: Option<HashMap<String, Value>> =
                serde_json::from_str(data.as_str()).unwrap_or(None);

            res.json(&json_data)
        }
        None => res,
    };

    let res = res.send().await;

    println!("{:#?}", res);
    Ok(())
}
