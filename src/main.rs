
use postgres::{Client, NoTls, Error};



fn main() -> Result<(), Error> {
    let mut client = Client::connect("postgres://user-name:strong-password@localhost:5432/library", NoTls)?;
    client.batch_execute("
    CREATE TABLE IF NOT EXISTS author(
        id      SERIAL PRIMARY KEY,
        name    VARCHAR NOT NULL,
        country VARCHAR NOT NULL
        );
    ")?;

    Ok(())
}
