use crate::{models::{Player}, errors::ApiError};
use reqwest::Error;


const BASE_URL: &str = "http://127.0.0.1:8001/api/";

async fn make_api_request(url: &str) -> Result<serde_json::Value, Error> {
    let resp = reqwest::get(url)
        .await?
        .json::<serde_json::Value>()
        .await?;

    Ok(resp)
}

pub async fn get_all_players() -> Result<Vec<Player>, ApiError> {
    let url = &format!("{}{}", BASE_URL, "players/");

    match make_api_request(url).await {
        Ok(resp) => {
            match serde_json::from_value::<Vec<Player>>(resp) {
                Ok(players) => Ok(players),
                Err(e) => Err(ApiError::from(e)),
            }
        }
        Err(e) => Err(ApiError::from(e)),
    }
}