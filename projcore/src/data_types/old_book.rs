use super::Meta;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct OldBook {
    pub name: String,
    pub meta: Meta,
    pub data: String,
}

impl OldBook {
    pub fn new(name: String, meta: Meta, data: String) -> Self {
        Self { name, meta, data }
    }

    /*pub async fn to_rsbook(&self, path: impl Into<PathBuf>) -> Result<(), AsyncError> {
        let mut path = path.into();
        path.push(self.name.clone() + ".rsbook");

        let mut f = File::create_new(path).await?;
        f.write_all(b"rsb").await?;

        let meta_compressed = zstd::encode_all(
            bincode::encode_to_vec(&self.meta, config::standard())?.as_slice(),
            22,
        )?;
        f.write_all(&(meta_compressed.len() as u16).to_ne_bytes())
            .await?;
        f.write_all(&meta_compressed).await?;

        f.write_all(&zstd::encode_all(self.data.as_bytes(), 22)?)
            .await?;
        Ok(())
    }

    pub async fn to_server_rsbook(&self, path: impl Into<PathBuf>) -> Result<(), AsyncError> {
        let mut path = path.into();
        let mut meta = path.clone();
        path.push(self.name.clone() + ".srsbook");
        meta.push(self.name.clone() + ".srsmeta");

        let mut f = File::create_new(meta).await?;
        f.write_all(&zstd::encode_all(
            bincode::encode_to_vec(&self.meta, config::standard())?.as_slice(),
            22,
        )?)
        .await?;

        let mut f = File::create_new(path).await?;
        f.write_all(&zstd::encode_all(self.data.as_bytes(), 22)?)
            .await?;

        Ok(())
    }*/
}
