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
    bpm: u16,
    key: String,
}

impl BPM<'_> {
    async fn ensure_bpm_and_key(&mut self, query: String) -> Result<()> {

        if let Some(last) = &self.last_searched {
            if last.query == query {
                return Ok(());
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

        let bpm_selector = Selector::parse("span.text-2xl.font-bold.text-gray-700.sm\\:text-3xl.sm\\:font-normal").unwrap();
        let key_selector = Selector::parse("span.text-2xl.text-gray-700.sm\\:text-3xl").unwrap();

        let doc = Html::parse_document(page.as_str());
        let found_bpm = doc.select(&bpm_selector).next().unwrap();
        let found_key = doc.select(&key_selector).next().unwrap();

        let bpm = found_bpm.inner_html().parse::<u16>()
            .or_else(|er| Err(Error::Failure(format!("Error number {}", er))))?;
        let key = found_key.inner_html();

        let ls = LastSearched { query, bpm, key };
        self.last_searched = Some(ls);

        return Ok(());
    }
}

#[async_trait]
impl Section<'_> for BPM<'_> {
    async fn update(&mut self, _click_event: &Option<ClickEvent>) -> Result<String> {
        let metadata = self.proxy.metadata().await?;

        let query = format!("{} - {}", metadata.title, metadata.artist.get(0).unwrap());

        self.ensure_bpm_and_key(query).await.unwrap();

        let data = self.last_searched.as_ref().unwrap();

        Ok(format!("BPM: {} | KEY: {}", data.bpm, data.key))
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
