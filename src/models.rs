use diesel::{Queryable, Insertable};
use serde::{Serialize, Deserialize};
use crate::schema::users;

#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub password: String,
    pub nom: String,
    pub prenom: String,
    pub email: String,
    pub role: String,
    pub created_at: chrono::NaiveDateTime,
}


#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser<'a> {
    pub password: &'a str,
    pub nom: &'a str,
    pub prenom: &'a str,
    pub email: &'a str,
    pub role: &'a str,
}
