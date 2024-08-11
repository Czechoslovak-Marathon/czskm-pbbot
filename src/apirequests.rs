use crate::apitypes::*;
use anyhow::Result;
use reqwest::header::{self};
use std::collections::HashMap;

// Speedrun.com API

pub async fn get_latest_run(runner: &str) -> Result<Option<Run>> {
    let request_url = format!(
        "https://www.speedrun.com/api/v1/users/{runner}/personal-bests",
        runner = runner
    );
    let response = reqwest::get(&request_url).await?.text().await?;
    let data: RunResponse = serde_json::from_str(&response)?;
    match data.data {
        Some(runs) => {
            let latest_run = runs.into_iter().fold(None, |max, x| match max {
                None => Some(x),
                Some(y) => Some(
                    if y.run.status.verify_date.is_none()
                        || x.run.status.verify_date > y.run.status.verify_date
                    {
                        x
                    } else {
                        y
                    },
                ),
            });
            Ok(latest_run)
        }
        None => Ok(None),
    }
}

pub async fn get_game_data(game: &str) -> Result<Option<Game>> {
    let request_url = format!("https://www.speedrun.com/api/v1/games/{game}", game = game);
    let response = reqwest::get(&request_url).await?.text().await?;
    let data: GameResponse = serde_json::from_str(&response)?;
    Ok(Some(data.data))
}

pub async fn get_category_data(category: &str) -> Result<Option<Category>> {
    let request_url = format!(
        "https://www.speedrun.com/api/v1/categories/{category}",
        category = category
    );
    let response = reqwest::get(&request_url).await?.text().await?;
    let data: CategoryResponse = serde_json::from_str(&response)?;
    Ok(Some(data.data))
}

pub async fn get_level_data(level: &str) -> Result<Option<Level>> {
    let request_url = format!(
        "https://www.speedrun.com/api/v1/levels/{level}",
        level = level
    );
    let response = reqwest::get(&request_url).await?.text().await?;
    let data: LevelResponse = serde_json::from_str(&response)?;
    Ok(Some(data.data))
}

pub async fn get_variables(values: HashMap<String, String>) -> Result<Option<String>> {
    let mut variables: Vec<String> = Vec::new();
    for (key, value) in values {
        let request_url = format!(
            "https://www.speedrun.com/api/v1/variables/{variable}",
            variable = key
        );
        let response = reqwest::get(&request_url).await?.text().await?;
        let data: VariableResponse = serde_json::from_str(&response)?;
        variables.push(data.data.values.values[&value].label.clone());
    }
    if variables.is_empty() {
        return Ok(None);
    }
    Ok(Some(variables.join(", ")))
}

// Twitch API

pub async fn get_twitch_user_id(user_name: &str) -> Result<Option<TwitchUser>> {
    let request_url = format!("https://api.twitch.tv/helix/users?login={user_name}", user_name = user_name);
    let mut headers = header::HeaderMap::new();
    headers.insert(header::AUTHORIZATION, header::HeaderValue::from_static("Bearer {OAUTH_KEY_HERE}"));
    headers.insert(
        "Client-Id",
        header::HeaderValue::from_static("{CLIENT_ID_HERE}"),
    );

    let client = reqwest::Client::new();

    let response = client
        .get(request_url)
        .headers(headers)
        .send()
        .await?
        .text()
        .await?;

    let data: TwitchUserResponse = serde_json::from_str(&response)?;

    if let Some(twitch_user) = data.data.first().cloned() {
        Ok(Some(twitch_user))
    } else {
        Ok(None)
    }
}

pub async fn get_twitch_stream(user_id: &str) -> Result<Option<TwitchStream>> {
    let request_url = format!("https://api.twitch.tv/helix/streams?user_id={user_id}", user_id = user_id);
    
    let mut headers = header::HeaderMap::new();
    headers.insert(header::AUTHORIZATION, header::HeaderValue::from_static("Bearer {OAUTH_KEY_HERE}"));
    headers.insert(
        "Client-Id",
        header::HeaderValue::from_static("{CLIENT_ID_HERE}"),
    );

    let client = reqwest::Client::new();

    let response = client
        .get(request_url)
        .headers(headers)
        .send()
        .await?
        .text()
        .await?;

    let data: TwitchStreamResponse = serde_json::from_str(&response)?;

    if let Some(twitch_stream) = data.data.first().cloned() {
        Ok(Some(twitch_stream))
    } else {
        Ok(None)
    }
}