use serde::Serialize;
use sqlite::Statement;

use crate::err::Error;

#[derive(Serialize)]
pub struct Record {
    #[serde(skip_serializing)]
    pub id: usize,

    #[serde(rename = "startTime")]
    pub start_time: String,

    #[serde(rename = "windowName")]
    pub title: String,

    #[serde(rename = "processName")]
    pub process_name: String,

    #[serde(rename = "wastedTime")]
    pub active_seconds: usize,
}

pub struct Storage<'a> {
    path: &'a str
}

impl<'a> Storage<'a> {
    pub fn init(path: &'a str) -> Result<Self, Error> {
        let connection = sqlite::open(path)?;

        connection.execute(
            "
            CREATE TABLE IF NOT EXISTS records (
                id INTEGER PRIMARY KEY,
                start_time DATETIME DEFAULT CURRENT_TIMESTAMP,
                title TEXT NOT NULL,
                process_name TEXT NOT NULL,
                active_seconds INTEGER DEFAULT 1
            );

            CREATE TABLE IF NOT EXISTS syncs (
                id INTEGER PRIMARY KEY,
                datetime DATETIME DEFAULT CURRENT_TIMESTAMP,
                last_record_id INTEGER,
                CONSTRAINT fk_records
                    FOREIGN KEY (last_record_id)
                    REFERENCES records (id)
            );
            "
        )?;

        Ok(Storage { path })
    }

    fn read_record(&self, stmt: &Statement) -> Result<Record, Error> {
        let id = stmt.read::<i64>(0)? as usize;
        let start_time = stmt.read::<String>(1)?;
        let title = stmt.read::<String>(2)?;
        let process_name = stmt.read::<String>(3)?;
        let active_seconds = stmt.read::<i64>(4)? as usize;

        Ok(Record { id, start_time, title, process_name, active_seconds })
    }

    pub fn last_record(&self) -> Result<Option<Record>, Error> {
        let connection = sqlite::open(self.path)?;

        let mut stmt = connection.prepare(
            "SELECT * FROM records ORDER BY start_time DESC LIMIT 1"
        )?;

        match stmt.next()? {
            sqlite::State::Row => Ok(Some(self.read_record(&stmt)?)),
            sqlite::State::Done => Ok(None),
        }
    }

    pub fn insert_record(&self, title: String, process_name: String) -> Result<(), Error> {
        let connection = sqlite::open(self.path)?;

        let mut stmt = connection.prepare(
            "INSERT INTO records (title, process_name) VALUES (?, ?)"
        )?;

        stmt.bind(1, title.as_str())?;
        stmt.bind(2, process_name.as_str())?;
        stmt.next()?;

        Ok(())
    }

    pub fn update_record(&self, start_time: String) -> Result<(), Error> {
        let connection = sqlite::open(self.path)?;

        let mut stmt = connection.prepare(
            "
            UPDATE records 
            SET active_seconds = active_seconds + 1
            WHERE start_time = ?
            "
        )?;
        
        stmt.bind(1, start_time.as_str())?;
        stmt.next()?;

        Ok(())
    }

    pub fn last_synced_record(&self) -> Result<usize, Error> {
        let connection = sqlite::open(self.path)?;

        let mut stmt = connection.prepare(
            "SELECT * FROM syncs ORDER BY id DESC LIMIT 1"
        )?;

        match stmt.next()? {
            sqlite::State::Row => Ok(stmt.read::<i64>(2)? as usize),
            sqlite::State::Done => Ok(0),
        }
    }

    pub fn get_records(&self, offset: usize) -> Result<Vec<Record>, Error> {
        let connection = sqlite::open(self.path)?;

        let mut stmt = connection.prepare(
            "SELECT * FROM records WHERE id > ?"
        )?;
        stmt.bind(1, offset as i64)?;

        let mut records: Vec<Record> = Vec::new();

        while let sqlite::State::Row = stmt.next()? {
            records.push(self.read_record(&stmt)?);
        }

        Ok(records)
    }

    pub fn add_sync_record(&self, last_record_id: usize) -> Result<(), Error> {
        let connection = sqlite::open(self.path)?;

        let mut stmt = connection.prepare(
            "INSERT INTO syncs (last_record_id) VALUES (?)"
        )?;

        stmt.bind(1, last_record_id as i64)?;
        stmt.next()?;

        Ok(())
    }
}