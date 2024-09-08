
#[tokio::main]

async fn main() -> Result<(), reqwest::Error> {
    let todos: String = reqwest::Client::new()
    .get("https://api.jsonbin.io/v3/b/659b464e1f5677401f18f233")
    .send()
    .await?
    .text()
    .await?;

    println!("{:#?}",todos);

    Ok(())
}
