use async_trait::async_trait;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use zbus::{Result, Error};

use crate::{dbus::SpotifyMediaPlayerProxy, input::ClickEvent};
use super::Section;

pub struct BPM<'a> {
    pub proxy: &'a SpotifyMediaPlayerProxy<'a>,

    pub last_searched: Option<LastSearched>,
}

pub struct LastSearched {
    query: String,
    bpm: u16
}

impl BPM<'_> {
    async fn get_bpm(&mut self, query: String) -> Result<u16> {

        if let Some(last) = &self.last_searched {
            if last.query == query {
                return Ok(last.bpm);
            }
        }

        let request = serde_json::to_string(&JsonRequest { query: query.clone() })
            .or_else(|er| Err(Error::Failure(format!("Unable to convery query to string {}", er))))?;

        let client = reqwest::Client::new();

        dbg!(&request);

        let response = client.post("https://songbpm.com/api/searches")
            .header("Content-Type", "application/json")
            .body(request)
            .send()
            .await
            .or_else(|er| Err(Error::Failure(format!("Error getting response {}", er))))?
            .json::<JsonResponse>()
            .await
            .or_else(|er| Err(Error::Failure(format!("Error parsing to json {}", er))))?;

        let url = format!("https://songbpm.com/{}", response.data.href);
        let page = reqwest::get(url)
            .await
            .or_else(|er| Err(Error::Failure(format!("Error getting response {}", er))))?
            .text()
            .await
            .or_else(|er| Err(Error::Failure(format!("Error parsing to json {}", er))))?;

        let selector = Selector::parse("span.text-2xl.font-bold.text-gray-700.sm\\:text-3xl.sm\\:font-normal").unwrap();

        let doc = Html::parse_document(page.as_str());
        let found = doc.select(&selector).next().unwrap();

        let bpm = found.inner_html().parse::<u16>()
            .or_else(|er| Err(Error::Failure(format!("Error number {}", er))))?;

        self.last_searched = Some(LastSearched { query, bpm });

        return Ok(bpm);
    }
}

#[async_trait]
impl Section<'_> for BPM<'_> {
    async fn update(&mut self, _click_event: &Option<ClickEvent>) -> Result<String> {
        let metadata = self.proxy.metadata().await?;

        let query = format!("{} - {}", metadata.title, metadata.artist.get(0).unwrap());

        let bpm = self.get_bpm(query).await.unwrap();

        Ok(format!("BPM: {}", bpm))
    }

}

#[derive(Debug, Serialize)]
struct JsonRequest {
    query: String,
}

#[derive(Debug, Deserialize)]
struct JsonResponse {
    status: String,
    data: JsonResponseData
}

#[derive(Debug, Deserialize)]
struct JsonResponseData {
    id: String,
    href: String,
}
