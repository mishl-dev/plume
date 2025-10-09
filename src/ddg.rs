use chrono::NaiveDate;
use reqwest::Client;
use serde::Serialize;
use scraper::{Html, Selector};
use std::collections::HashMap;
use std::error::Error;

#[derive(Debug, Serialize)]
pub struct DuckDuckGoResult {
    pub title: String,
    pub link: String,
    pub snippet: String,
    pub favicon: Option<String>,
    pub date: Option<NaiveDate>,
}

pub struct DuckDuckGoSearch {
    client: Client,
    base_url: String,
}

impl DuckDuckGoSearch {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            base_url: "https://html.duckduckgo.com/html/".to_string(),
        }
    }

    async fn get_html(&self, query: &str, start: usize) -> Result<String, Box<dyn Error>> {
        let mut params: HashMap<&'static str, &str> = HashMap::new();
        let start_str: String = start.to_string();
        params.insert("q", query);
        params.insert("kl", "us-en");
        params.insert("s", &start_str);


        let res: String = self
            .client
            .post(&self.base_url)
            .form(&params)
            .header(
                "User-Agent",
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64)",
            )
            .send()
            .await?
            .text()
            .await?;
        Ok(res)
    }

    fn parse_date(&self, date_str: &str) -> Option<NaiveDate> {
        let formats: [&'static str; 3] = ["%b %d, %Y", "%d %b %Y", "%Y-%m-%d"];
        for fmt in &formats {
            if let Ok(d) = NaiveDate::parse_from_str(date_str, fmt) {
                return Some(d);
            }
        }
        None
    }

    fn parse_html(&self, html: &str) -> Vec<DuckDuckGoResult> {
        let mut results: Vec<DuckDuckGoResult> = Vec::new();
        let document = Html::parse_document(html);
        let result_selector = Selector::parse("div.result").unwrap();
        let title_selector = Selector::parse("a.result__a").unwrap();
        let snippet_selector = Selector::parse("a.result__snippet").unwrap();
        let favicon_selector = Selector::parse("img.result__icon__img").unwrap();
        let extras_selector = Selector::parse("div.result__extras__url span").unwrap();

        for element in document.select(&result_selector) {
            let title_elem = element.select(&title_selector).next();
            let snippet_elem = element.select(&snippet_selector).next();
            if let (Some(title_el), Some(snippet_el)) = (title_elem, snippet_elem) {
                let title = title_el.text().collect::<String>();
                let mut link = title_el.value().attr("href").unwrap_or("").to_string();
                if !link.starts_with("http") {
                    link = format!("https:{}", link);
                }
                let snippet = snippet_el.text().collect::<String>();

                let favicon = element
                    .select(&favicon_selector)
                    .next()
                    .and_then(|img| img.value().attr("src"))
                    .map(|s| if s.starts_with("http") { s.to_string() } else { format!("https:{}", s) });

                let date = element
                    .select(&extras_selector)
                    .next()
                    .map(|s| self.parse_date(&s.text().collect::<String>()))
                    .flatten();

                results.push(DuckDuckGoResult {
                    title,
                    link,
                    snippet,
                    favicon,
                    date,
                });
            }
        }
        results
    }

    pub async fn get_results(&self, query: &str, pages: usize) -> Result<Vec<DuckDuckGoResult>, Box<dyn Error>> {
        let mut all_results: Vec<DuckDuckGoResult> = Vec::new();
        let mut start: usize = 0;

        for _ in 0..pages {
            let html: String = self.get_html(query, start).await?;
            let results: Vec<DuckDuckGoResult> = self.parse_html(&html);
            if results.is_empty() {
                break; // no more results
            }
            start += results.len();
            all_results.extend(results);
        }

        Ok(all_results)
    }
}
