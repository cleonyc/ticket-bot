use std::io;
use std::path::Path;
use time::{format_description, macros::format_description, OffsetDateTime};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::{fs, io::BufWriter};

pub async fn write_messages(messages: Vec<String>) -> anyhow::Result<String> {
    match fs::create_dir(format!("ticketarchive/")).await {
        Ok(_) => {}
        Err(e) => {
            if e.kind() != io::ErrorKind::AlreadyExists {
                return Err(anyhow::Error::from(e));
            }
        }
    };
    // format_description!("[year]-[month]-[day]T[hour repr:24]:[minute]:[second]");
    let desc = format_description::parse("[year]-[month]-[day]T[hour repr:24]:[minute]:[second]")?;
    let path = format!(
        "ticketarchive/{}.txt",
        OffsetDateTime::now_utc().format(&desc)?
    );
    let file = fs::File::create(path.clone()).await?;
    let mut writer = BufWriter::new(file);
    for msg in messages {
        let msg = msg.replace("\n", "\\n");
        writer.write(format!("{}\n", msg).as_bytes()).await?;
    }
    writer.flush().await?;
    Ok(path)
}
