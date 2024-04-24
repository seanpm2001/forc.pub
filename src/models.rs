use diesel::prelude::*;
use std::time::SystemTime;
use uuid::Uuid;

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: Uuid,
    pub full_name: String,
    pub github_login: String,
    pub github_url: String,
    pub avatar_url: Option<String>,
    pub email: Option<String>,
    pub is_admin: bool,
    pub created_at: SystemTime,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser {
    pub full_name: String,
    pub github_login: String,
    pub github_url: String,
    pub avatar_url: Option<String>,
    pub email: Option<String>,
    pub is_admin: bool,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::sessions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Session {
    pub id: Uuid,
    pub user_id: Uuid,
    pub expires_at: SystemTime,
    pub created_at: SystemTime,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::sessions)]
pub struct NewSession {
    pub user_id: Uuid,
    pub expires_at: SystemTime,
}