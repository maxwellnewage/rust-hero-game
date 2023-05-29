use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Owner {
    pub id: i32,
    pub is_author: bool,
    pub username: String,
}

#[derive(Debug, Deserialize)]
pub struct Player {
    pub hp: i32,
    pub id: i32,
    pub money: i32,
    pub name: String,
    pub score: i32,
    pub owner: Owner
}