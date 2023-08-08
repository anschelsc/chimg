use anyhow::{anyhow, bail, Result};
use eggbug::Attachment;
use serde::Deserialize;
use std::path::Path;
use std::env;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        eprintln!("Usage: {} FILENAME", args[0]);
        return;
    }
    match upload(&args[1]) {
        Ok(url) => println!("{}", url),
        Err(e) => eprintln!("{}", e),
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
async fn upload<P: AsRef<Path>>(path: P) -> Result<String> {
    let info = PageInfo::load(".cohost.json")?;
    let session = eggbug::Session::login(&info.email, &info.password).await?;
    let attachment = build_attachment(path).await?;
    let mut post = eggbug::Post {
        markdown: "this draft can be deleted if you've used the image in another post".into(),
        draft: true,
        attachments: vec![attachment],
        ..Default::default()
    };
    session.create_post(&info.page, &mut post).await?;
    match post.attachments[0].url() {
        None => Err(anyhow!("Failed to upload attachment")),
        Some(url) => Ok(url.into()),
    }
}

fn get_content_type<P: AsRef<Path>>(path: P) -> Option<String> {
    Some(
        match path.as_ref().extension()?.to_str()? {
            "jpg" | "jpeg" => "image/jpeg",
            "gif" => "image/gif",
            "png" => "image/png",
            "webp" => "image/webp",
            "tiff" => "image/tiff",
            _ => {
                return None;
            }
        }
        .into(),
    )
}

async fn build_attachment<P: AsRef<Path>>(path: P) -> Result<Attachment> {
    let content_type = match get_content_type(&path) {
        Some(ct) => ct,
        None => bail!("Unknown file extension"),
    };
    Ok(Attachment::new_from_file(path, content_type, None).await?)
}
