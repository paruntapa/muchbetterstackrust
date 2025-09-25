use crate::{ store::Store};
use chrono::Utc;
use diesel::{prelude::*};
use uuid::Uuid;

#[derive(Queryable, Insertable, Selectable)]
#[diesel(table_name = crate::schema::website)]
#[diesel(check_for_backend(diesel::pg::Pg))]

pub struct Website {
    pub id: String,
    pub url: String,
    pub user_id: Option<String>,
    pub time_added: chrono::NaiveDateTime
}

impl Store{
    pub fn create_website(
        &mut self, 
        user_id: String, 
        url: String
    ) -> Result<Website, diesel::result::Error> {
        let id = Uuid::new_v4();
        let website = Website {
            id: id.to_string(),
            url: url,
            user_id: Option::Some(user_id),
            time_added: Utc::now().naive_utc()
        };

        let website = diesel::insert_into(crate::schema::website::table)
        .values(&website)
        .returning(Website::as_returning())
        .get_result(&mut self.conn)?;

    Ok(website)
    }

    pub fn get_website(
        &mut self, 
        input_id: String
    ) -> Result<Website, diesel::result::Error> {
        use crate::schema::website::dsl::*;

        let resulting_website = website
        .filter(id.eq(input_id))
        .select(Website::as_select())
        .first(&mut self.conn)?;

    Ok(resulting_website)
    }
}