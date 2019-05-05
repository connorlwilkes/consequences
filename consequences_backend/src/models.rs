use super::schema::users;

#[derive(Serialize, Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
}

#[derive(Insertable, Debug)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub id: Option<i32>,
    pub username: &'a str,
}
