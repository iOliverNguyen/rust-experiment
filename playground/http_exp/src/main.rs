use isahc;
use isahc::{AsyncReadResponseExt, HttpClient};
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json;

static LOREM_JSON_URL: &str = "https://lorem-json.com/api/json";

#[derive(Debug, Deserialize)]
struct Item {
    pub age: u8,
    pub name: String,
    pub country: String,
}

static item_desc: &str = r#"{
    "name": "{{name()}}",
    "age": "{{int(25, 30)}}",
    "country": "{{country()}}"
}"#;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let item = exec_reqwest().await?;
    println!("{:?}", item);
    println!(
        "{} is {} years old and lives in {}",
        item.name, item.age, item.country
    );

    let item = exec_isahc().await?;
    println!("{:?}", item);
    println!(
        "{} is {} years old and lives in {}",
        item.name, item.age, item.country
    );

    Ok(())
}

async fn exec_reqwest() -> Result<Item, Box<dyn std::error::Error>> {
    let res = reqwest::Client::new()
        .post(LOREM_JSON_URL)
        .header("content-type", "application/json")
        .body(item_desc)
        .send()
        .await?;
    println!("-> {:?}", item_desc);

    if res.status().is_success() {
        let data = res.bytes().await?;
        println!("<- {}", std::str::from_utf8(&data).unwrap());

        let item: Item = serde_json::from_slice(&data)?;
        Ok(item)
    } else {
        println!("Request failed with status: {}", res.status());
        Err("Request failed".into())
    }
}

async fn exec_isahc() -> Result<Item, Box<dyn std::error::Error>> {
    let req = isahc::Request::post(LOREM_JSON_URL)
        .header("content-type", "application/json")
        .body(item_desc)?;
    let mut res = isahc::send_async(req).await?;
    println!("-> {:?}", item_desc);

    if res.status().is_success() {
        let data = res.bytes().await?;
        println!("<- {}", std::str::from_utf8(&data).unwrap());

        let item: Item = serde_json::from_slice(&data)?;
        Ok(item)
    } else {
        println!("Request failed with status: {}", res.status());
        Err("Request failed".into())
    }
}
