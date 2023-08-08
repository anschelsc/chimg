use anyhow::Result;
use serde::Deserialize;
use std::path::Path;

fn main() {
    if let Err(e) = test_chost() {
        println!("{}", e);
    }
}

#[derive(Deserialize)]
struct PageInfo {
    email: String,
    password: String,
    page: String,
}

impl PageInfo {
    fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let raw = std::fs::read_to_string(path)?;
        Ok(serde_json::from_str(&raw)?)
    }
}

#[tokio::main(flavor = "current_thread")]
async fn test_chost() -> Result<()> {
    let info = PageInfo::load(".cohost.json")?;
    let session = eggbug::Session::login(&info.email, &info.password).await?;
    let mut post = eggbug::Post {
        headline: "Pest Toast".into(),
        markdown: "plz do not gnore".into(),
        draft: true,
        ..Default::default()
    };
    session.create_post(&info.page, &mut post).await?;
    Ok(())
}
