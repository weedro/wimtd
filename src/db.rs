use crate::err::Error;

pub struct Record {
    pub start_time: String,
    pub title: String,
    pub process_name: String,
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
                start_time DATETIME DEFAULT CURRENT_TIMESTAMP,
                title TEXT NOT NULL,
                process_name TEXT NOT NULL,
                active_seconds INTEGER DEFAULT 1
            );
            "
        )?;

        Ok(Storage { path })
    }

    pub fn last_record(&self) -> Result<Option<Record>, Error> {
        let connection = sqlite::open(self.path)?;

        let mut stmt = connection.prepare(
            "SELECT * FROM records ORDER BY start_time DESC LIMIT 1"
        )?;

        match stmt.next()? {
            sqlite::State::Row => {
                let start_time = stmt.read::<String>(0)?;
                let title = stmt.read::<String>(1)?;
                let process_name = stmt.read::<String>(2)?;

                Ok(Some(Record { start_time, title, process_name }))
            },
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
}