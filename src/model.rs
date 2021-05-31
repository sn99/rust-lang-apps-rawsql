use crate::structs;
use diesel::prelude::*;
use diesel::sql_query;

pub fn getdbconn() -> PgConnection {
    let database_url = "DATABASE_URL";
    PgConnection::establish(&database_url).unwrap()
}

pub fn list_users() -> Vec<structs::User> {
    let conn = getdbconn();

    let results = sql_query(
        "SELECT
    id,
    firstname,
    lastname,
    email,
    CONCAT(firstname, lastname) as fullname
  FROM
    user_login",
    )
    .load::<structs::User>(&conn)
    .unwrap();
    return results;
}
