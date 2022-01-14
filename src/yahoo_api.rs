use crate::portfolio::Currency::{EUR, USD};
use crate::portfolio::{Currency, QuoteMeta};
use crate::Quote;
use anyhow::anyhow;
use chrono::{DateTime, NaiveDateTime};
use hyper::body::Buf;
use hyper::{Client, Uri};
use hyper_tls::HttpsConnector;
use itertools::Itertools;
use serde_json::Value;
use std::str::FromStr;
use tracing::instrument;

pub struct Yahoo {}
impl Yahoo {
    fn map_currency(s: &str) -> Currency {
        match s {
            "EUR" => EUR,
            "USD" => USD,
            _ => panic!("Unexpected currency {}", s),
        }
    }

    #[instrument]
    pub async fn get_quotes(symbols: Vec<&str>) -> anyhow::Result<Vec<(Quote, QuoteMeta)>> {
        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, hyper::Body>(https);

        let url = Uri::from_str(&format!("https://query1.finance.yahoo.com/v7/finance/quote?symbols={}&fields=&region=US&lang=en-US", symbols.iter().join(",")))?;
        let res = client.get(url).await?;

        // asynchronously aggregate the chunks of the body
        let body = hyper::body::aggregate(res).await?;
        let v: Value = serde_json::from_reader(body.reader())?;

        fn map_result(x: &Value) -> Option<(Quote, QuoteMeta)> {
            let time = x.get("regularMarketTime")?.as_i64()?;

            let currency_pair = match x.get("quoteType") {
                Some(q) if q == "CURRENCY" => {
                    let (x1, x2) = x.get("shortName")?.as_str()?.split_once("/")?;
                    Some((Yahoo::map_currency(x1), Yahoo::map_currency(x2)))
                }
                _ => None,
            };

            Some((
                Quote {
                    symbol: x.get("symbol")?.as_str()?.into(),
                    time: DateTime::from_utc(NaiveDateTime::from_timestamp(time, 0), chrono::Utc),
                    price: x.get("regularMarketPrice")?.as_f64()?,
                    change: x.get("regularMarketChangePercent")?.as_f64()? * 0.01,
                },
                QuoteMeta {
                    symbol: x.get("symbol")?.as_str()?.into(),
                    description: x.get("shortName")?.as_str()?.into(),
                    currency: match x.get("currency")?.as_str()? {
                        "EUR" => EUR,
                        "USD" => USD,
                        o => {
                            println!("Unknown currency {}", o);
                            None?
                        }
                    },
                    currency_pair,
                },
            ))
        }

        v.get("quoteResponse")
            .and_then(|x| x.get("result"))
            .and_then(|y| {
                y.as_array().and_then(|y| {
                    let mut quotes = y
                        .iter()
                        .filter_map(|el| match map_result(el) {
                            None => {
                                println!("Cannot parse {}", el);
                                None
                            }
                            Some(q) => Some(q),
                        })
                        .collect_vec();
                    quotes.sort_by_key(|(q, _)| {
                        symbols.iter().position(|symbol| *symbol == q.symbol)
                    });
                    if quotes.len() == symbols.len() {
                        Some(quotes)
                    } else {
                        None
                    }
                })
            })
            .ok_or(anyhow!("Error retrieving the quotes"))
    }
}
