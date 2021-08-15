use crate::SETTINGS;

pub async fn init_config() -> anyhow::Result<()> {
    SETTINGS
        .write()
        .await
        .merge(config::File::with_name("config"))
        .unwrap();
    Ok(())
}
