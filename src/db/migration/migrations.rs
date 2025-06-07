use super::Migration;

pub struct Migration0001 {}

impl Migration for Migration0001 {
    fn up(&self, db: &rusqlite::Connection) -> Result<(), anyhow::Error> {
        db.execute(
            "
            CREATE TABLE files (
                id              INTEGER NOT NULL UNIQUE,
                path            TEXT    NOT NULL,
                timestamp       TEXT    NOT NULL,
                description     TEXT    NOT NULL,
                format_group    TEXT    NOT NULL,
                format          TEXT    NOT NULL,

                PRIMARY KEY(id)
            );
            ",
            rusqlite::params![],
        )?;

        db.execute(
            "
                CREATE TABLE tags (
                    id      INTEGER NOT NULL UNIQUE,
                    name    TEXT    NOT NULL UNIQUE,

                    PRIMARY KEY(id)
                );
                ",
            rusqlite::params![],
        )?;

        db.execute(
            "
                CREATE TABLE taggings (
                    file INTEGER NOT NULL,
                    tag INTEGER NOT NULL,

                    FOREIGN KEY(file) REFERENCES files(id),
                    FOREIGN KEY(tag) REFERENCES tags(id)
                );
                ",
            rusqlite::params![],
        )?;

        Ok(())
    }
}
