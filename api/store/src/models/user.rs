use crate::store::Store;
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Insertable, Selectable)]
#[diesel(table_name = crate::schema::user)]
#[diesel(check_for_backend(diesel::pg::Pg))]

struct User {
    id: String,
    username: String,
    password: String,
}

impl Store{
    pub fn sign_up(&mut self, username: String, password: String) -> Result<String, diesel::result::Error>{
        let u = User{
            username,
            password,
            id: Uuid::new_v4().to_string(),
        };

        diesel::insert_into(crate::schema::user::table)
        .values(&u)
        .returning(User::as_returning())
        .get_result(&mut self.conn)?;

        Ok(u.id)
    }

    pub fn sign_in(&self, username: String, password: String) -> String{
        String::from("1")
    }

}