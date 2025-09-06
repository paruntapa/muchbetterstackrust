use diesel::prelude::*;
use diesel::Connection;
use crate::config::Config;

pub struct Store {
    conn: PgConnection
 }
 
 impl Store {
     pub fn default() -> Result<Self, ConnectionError> {
         let config = Config::default();
         let conn = PgConnection::establish(&config.db_url)?;
 
         Ok(Self {
             conn
         })
     }
 }