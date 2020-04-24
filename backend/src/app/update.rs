use super::App;
use data_types::Kombucha;
use sqlx::prelude::*;

impl App {
    pub async fn update_kombucha(
        &self,
        kombucha: &Kombucha,
    ) -> Result<(), anyhow::Error> {
        let mut transaction = self.db.begin().await?;

        let query = sqlx::query(
            "UPDATE kombucha SET (name, added) = ($1, $2) WHERE id = $3",
        )
        .bind(&kombucha.name)
        .bind(&kombucha.added)
        .bind(kombucha.id);

        transaction.execute(query).await?;

        for entry in &kombucha.entries {
            let query = sqlx::query("UPDATE kombucha_entry SET (added, content) = ($1, $2) WHERE id = $3")
                .bind(&entry.added)
                .bind(&entry.content)
                .bind(entry.id);

            transaction.execute(query).await?;
        }

        transaction.commit().await?;

        Ok(())
    }
}
