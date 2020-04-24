use super::App;
use data_types::{EntryId, KombuchaId};
use sqlx::prelude::*;

impl App {
    pub async fn delete_kombucha_entry(
        &self,
        kombucha_id: KombuchaId,
        entry_id: EntryId,
    ) -> Result<(), anyhow::Error> {
        let query = sqlx::query(
            "DELETE FROM kombucha_entry WHERE kombucha_id = $1 and id = $2",
        )
        .bind(kombucha_id)
        .bind(entry_id);

        self.db.acquire().await?.execute(query).await?;

        Ok(())
    }

    pub async fn delete_kombucha(
        &self,
        kombucha_id: KombuchaId,
    ) -> Result<(), anyhow::Error> {
        let mut transaction = self.db.begin().await?;

        let delete_entries_query =
            sqlx::query("DELETE FROM kombucha_entry WHERE kombucha_id = $1")
                .bind(kombucha_id);

        let delete_kombucha_query =
            sqlx::query("DELETE FROM kombucha WHERE id = $1").bind(kombucha_id);

        transaction.execute(delete_entries_query).await?;
        transaction.execute(delete_kombucha_query).await?;

        transaction.commit().await?;

        Ok(())
    }
}
