//! Basic xkcd API wrapper with [`serde`] and [`reqwest`].
//!
//! See https://xkcd.com/json.html.

use anyhow::Context;
use rand::Rng;
use reqwest::StatusCode;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct XkcdComic {
    #[serde(rename = "num")]
    pub number: u32,
    pub title: String,
    pub year: String,
    pub month: String,
    pub day: String,
    #[serde(rename = "img")]
    pub image_url: String,
}

impl XkcdComic {
    /// Get a specific xkcd comic by number.
    pub async fn get_number(number: u32) -> anyhow::Result<Option<XkcdComic>> {
        let url = format!("https://xkcd.com/{number}/info.0.json");
        let response = reqwest::get(url).await?;

        if response.status() == StatusCode::NOT_FOUND {
            return Ok(None);
        }

        let comic = response.error_for_status()?.json().await?;

        Ok(Some(comic))
    }

    /// Get the latest xkcd comic.
    pub async fn get_latest() -> anyhow::Result<XkcdComic> {
        let response = reqwest::get("https://xkcd.com/info.0.json").await?;

        Ok(response.error_for_status()?.json().await?)
    }

    pub async fn get_random() -> anyhow::Result<XkcdComic> {
        let latest_response = reqwest::get("https://xkcd.com/info.0.json").await?;

        let latest_number = latest_response
            .error_for_status()?
            .json::<XkcdComic>()
            .await?
            .number;

        let rand_num = rand::thread_rng().gen_range(0..=latest_number);
        let rand_comic = Self::get_number(rand_num).await?;
        rand_comic.context("failed to get random xkcd comic")
    }

    /// Return the comic URL.
    pub fn url(&self) -> String {
        format!("https://xkcd.com/{}", self.number)
    }
}
