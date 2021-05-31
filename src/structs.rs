// Here we bring the Diesel types into scope
use crate::schema::user_login;
use diesel::sql_types::*;

// Notice that `fullname` is not a column on our users table,
// but we're are going to return it by
// concatenating the first and last name together.
// QueryableByName will use the users table for
// all other column types.

#[derive(Debug, QueryableByName)]
#[table_name = "user_login"]
pub struct User {
    pub id: i64,
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    #[sql_type = "Text"]
    pub fullname: String,
}
