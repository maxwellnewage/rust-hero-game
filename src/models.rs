use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Owner {
    id: i32,
    is_author: bool,
    username: String,
}

#[derive(Debug, Deserialize)]
pub struct Player {
    hp: i32,
    id: i32,
    money: i32,
    name: String,
    score: i32,
    owner: Owner
}