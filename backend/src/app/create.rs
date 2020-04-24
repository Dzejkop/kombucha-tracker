use super::App;
use data_types::KombuchaId;
use sqlx::prelude::*;

impl App {
    pub async fn create_new_kombucha(
        &self,
    ) -> Result<KombuchaId, anyhow::Error> {
        let (id,) = sqlx::query_as::<_, (KombuchaId,)>(
            "INSERT INTO kombucha (name, added) VALUES ('', NOW()) RETURNING id",
        )
        .fetch_one(&self.db)
        .await?;

        Ok(id)
    }

    pub async fn create_new_kombucha_entry(
        &self,
        kombucha_id: KombuchaId,
    ) -> Result<KombuchaId, anyhow::Error> {
        let (id,) = sqlx::query_as::<_, (KombuchaId,)>(
            "INSERT INTO kombucha_entry (kombucha_id, content, added) VALUES ($1, '', NOW()) RETURNING id",
        )
        .bind(kombucha_id)
        .fetch_one(&self.db)
        .await?;

        Ok(id)
    }
}
