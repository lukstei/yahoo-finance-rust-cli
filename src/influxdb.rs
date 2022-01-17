use chrono::DateTime;
use hyper::{Body, Client, Uri};
use serde::Deserialize;
use std::io::Read;
use std::str::FromStr;

use hyper::body::Buf;
use hyper_tls::HttpsConnector;
use itertools::Itertools;

use crate::Quote;
use anyhow::anyhow;
use hyper::client::HttpConnector;
use tokio_tungstenite::tungstenite::http::Request;
use tracing::instrument;

#[derive(Clone, Deserialize)]
pub struct Config {
    pub base_url: String,
    pub bucket: String,
    pub org: String,
    pub token: String,
}

pub struct InfluxDB {
    config: Config,
    client: Client<HttpsConnector<HttpConnector>>,
}

#[derive(Debug)]
pub struct Measurement {
    pub(crate) value: f64,
    pub(crate) symbol: String,
    pub(crate) time: DateTime<chrono::Utc>,
}

impl From<&Quote> for Measurement {
    fn from(x: &Quote) -> Self {
        Measurement::new(x.price, x.symbol.clone(), x.time)
    }
}

impl Measurement {
    pub fn new(value: f64, symbol: String, time: DateTime<chrono::Utc>) -> Self {
        Measurement {
            value,
            symbol,
            time,
        }
    }
}

impl InfluxDB {
    pub fn new(config: Config) -> Self {
        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, hyper::Body>(https);

        InfluxDB { config, client }
    }

    #[instrument(skip(self))]
    pub async fn push(&self, measurements: Vec<Measurement>) -> anyhow::Result<()> {
        let url = Uri::from_str(&format!(
            "{}/api/v2/write?bucket={}&org={}&token={}&precision=ms",
            self.config.base_url, self.config.bucket, self.config.org, self.config.token
        ))?;

        let s = measurements
            .iter()
            .map(|s| {
                format!(
                    "quote,symbol={} value={} {}",
                    s.symbol.replace("=", "-"),
                    s.value,
                    s.time.timestamp_millis()
                )
            })
            .join("\n");

        let response = self
            .client
            .request(
                Request::builder()
                    .method("POST")
                    .uri(url)
                    .header("Authorization", format!("Token {}", self.config.token))
                    .body(Body::from(s))?,
            )
            .await?;

        if response.status().is_success() {
            Ok(())
        } else {
            let mut s = String::new();
            let reason = response.status().canonical_reason().unwrap();
            let _ = hyper::body::aggregate(response)
                .await
                .map(|x| x.reader().read_to_string(&mut s));

            Err(anyhow!("InfluxDB Request failed: {} - {}", reason, s))
        }
    }
}
