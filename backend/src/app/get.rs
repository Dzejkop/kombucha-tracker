use super::App;
use data_types::db::{
    Entry as DbEntry, Fermentation as DbFermentation, Kombucha as DbKombucha,
};
use data_types::{Entry, EntryId, Kombucha, KombuchaId};
use sqlx::prelude::*;

impl App {
    async fn get_all_db_kombuchas(
        &self,
    ) -> Result<Vec<DbKombucha>, anyhow::Error> {
        let row = sqlx::query_as::<_, DbKombucha>(
            "SELECT id, name, added FROM kombucha ORDER BY id",
        )
        .fetch_all(&self.db)
        .await?;

        Ok(row)
    }

    pub async fn get_all_kombuchas(
        &self,
    ) -> Result<Vec<Kombucha>, anyhow::Error> {
        let db_kombuchas = self.get_all_db_kombuchas().await?;

        let mut kombuchas = Vec::with_capacity(db_kombuchas.len());

        for db_kombucha in db_kombuchas.into_iter() {
            let db_entries =
                self.get_db_entries_for_kombucha(db_kombucha.id).await?;

            let DbKombucha {
                id, name, added, ..
            } = db_kombucha;

            let entries = db_entries.into_iter().map(Entry::from).collect();

            kombuchas.push(Kombucha {
                id,
                name,
                added,
                entries,
                fermentations: vec![],
            });
        }

        Ok(kombuchas)
    }

    async fn get_db_kombucha(
        &self,
        id: KombuchaId,
    ) -> Result<Option<DbKombucha>, anyhow::Error> {
        let row = sqlx::query_as::<_, DbKombucha>(
            "SELECT id, name, added FROM kombucha WHERE id  = $1",
        )
        .bind(id)
        .fetch_optional(&self.db)
        .await?;

        Ok(row)
    }

    pub async fn get_kombucha(
        &self,
        id: KombuchaId,
    ) -> Result<Option<Kombucha>, anyhow::Error> {
        if let Some(DbKombucha {
            id, name, added, ..
        }) = self.get_db_kombucha(id).await?
        {
            let entries = self.get_db_entries_for_kombucha(id).await?;
            let entries = entries.into_iter().map(Entry::from).collect();

            Ok(Some(Kombucha {
                id,
                name,
                added,
                entries,
                fermentations: vec![],
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn get_kombucha_entry(
        &self,
        kombucha_id: KombuchaId,
        entry_id: EntryId,
    ) -> Result<Option<Entry>, anyhow::Error> {
        let maybe_entry = sqlx::query_as::<_, DbEntry>(
            "SELECT id, kombucha_id, content, added FROM kombucha_entry WHERE id = $1 AND kombucha_id = $2"
        )
        .bind(entry_id)
        .bind(kombucha_id)
        .fetch_optional(&self.db)
        .await?;

        Ok(maybe_entry.map(Entry::from))
    }

    pub async fn get_kombucha_entries(
        &self,
        kombucha_id: KombuchaId,
    ) -> Result<Vec<Entry>, anyhow::Error> {
        let entries = sqlx::query_as::<_, DbEntry>(
            "SELECT id, kombucha_id, content, added FROM kombucha_entry WHERE kombucha_id = $1"
        )
        .bind(kombucha_id)
        .fetch_all(&self.db)
        .await?
        .into_iter()
        .map(Entry::from)
        .collect();

        Ok(entries)
    }

    async fn get_db_entries_for_kombucha(
        &self,
        kombucha_id: KombuchaId,
    ) -> Result<Vec<DbEntry>, anyhow::Error> {
        let row = sqlx::query_as::<_, DbEntry>(
            "SELECT id, kombucha_id, content, added FROM kombucha_entry WHERE kombucha_id = $1",
        )
        .bind(kombucha_id)
        .fetch_all(&self.db)
        .await?;

        Ok(row)
    }

    async fn get_db_fermentations_for_kombucha(
        &self,
        kombucha_id: KombuchaId,
    ) -> Result<Vec<DbFermentation>, anyhow::Error> {
        let rows = sqlx::query_as::<_, DbFermentation>(
            "SELECT id, kombucha_id, start_date, end_date, est_end_date, status FROM kombucha_fermentation WHERE kombucha_id = $1"
        )
        .bind(kombucha_id)
        .fetch_all(&self.db)
        .await?;

        Ok(rows)
    }
}
