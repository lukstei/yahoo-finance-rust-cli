mod influxdb;
mod portfolio;
mod quote_producer;
mod quote_receiver;
mod yahoo_api;
mod yahoo_protobuf;

use anyhow::anyhow;

use crate::yahoo_protobuf::YahooFinanceQuote;

use futures_util::{try_join, TryFutureExt};
use pin_utils::pin_mut;
use tokio::sync::mpsc;

use crate::influxdb::Config as InfluxDBConfig;
use crate::influxdb::{InfluxDB, Measurement};
use crate::portfolio::Currency::EUR;
use crate::portfolio::{Currency, Portfolio, Quote, QuoteMeta};
use crate::yahoo_api::Yahoo;
use quote_producer::QuoteProducer;
use quote_receiver::QuoteReceiver;
use tokio_tungstenite::tungstenite::Message;

#[derive(Clone)]
pub struct Config {
    home_currency: Currency,
    portfolio: Vec<(String, f64)>,
    db: Option<InfluxDBConfig>,
    print_portfolio: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            print_portfolio: true,

            home_currency: EUR,
            db: None,
            // to enable Influx, set:
            /*db: Some(InfluxDBConfig {
                base_url: "http://localhost:8086".into(),
                bucket: "my_bucket".into(),
                org: "primary".into(),
                token: "token".into(),
            }),*/
            portfolio: vec![
                ("AETH-USD.SW".into(), 450.),
                ("AMZN".into(), 1.),
                ("DE000A27Z304.SG".into(), 500.),
                ("CSNDX.SW".into(), 10.),
                ("EXS2.DE".into(), 10.),
                ("IBCL.DE".into(), 10.),
                ("ITEK.MI".into(), 2000.),
                ("IUIT.SW".into(), 2000.),
                ("IUSE.SW".into(), 100.),
                ("MSFT".into(), 25.),
                ("TSM".into(), 100.),
                ("XDWT.DE".into(), 1000.),
            ],
        }
    }
}

fn cls() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

pub type QuoteMessage = Vec<Quote>;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // to enable logging, set RUST_LOG accordingly
    tracing_subscriber::fmt::init();

    let config = Config::default();
    if !matches!(config.home_currency, EUR) {
        panic!("Only EUR supported as home currency for now");
    }

    let (tx, rx) = mpsc::channel::<QuoteMessage>(32);

    let c_config = config.clone();
    let producer = tokio::spawn(async move {
        let quote_producer = QuoteProducer::new(c_config, tx);
        quote_producer.start().await
    })
    .map_err(|e| anyhow!(e))
    .and_then(|x| async { x });

    let c_config = config.clone();
    let receiver = tokio::spawn(async move {
        let mut quote_receiver = QuoteReceiver::new(c_config, rx);
        quote_receiver.start().await
    })
    .map_err(|e| anyhow!(e))
    .and_then(|x| async { x });

    pin_mut!(producer);
    pin_mut!(receiver);

    try_join!(receiver, producer)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::influxdb::InfluxDB;
    use crate::{Config, Measurement, Quote, Yahoo, YahooFinanceQuote};
    use chrono::Utc;
    use protobuf::Message;

    #[test]
    fn test_reading() {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                let db = InfluxDB::new(Config::default().db.unwrap());
                db.push(vec![Measurement {
                    symbol: "test".into(),
                    time: Utc::now(),
                    value: 12.478377342224121,
                }])
                .await
                .unwrap();
            });
    }

    #[test]
    fn test_quote() {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                let vec = Yahoo::get_quotes(vec!["EURUSD=X"]).await.unwrap();
                println!("{:?}", vec);
            })
    }

    #[test]
    fn it_works() {
        let result =
            base64::decode("CgRCRUtFFXE9pEEY4I6A5bJfKgNOWVEwCDgBRd5Oj0BIkr/ICWXAR2E/2AEE").unwrap();
        let quote = YahooFinanceQuote::parse_from_bytes(&result).unwrap();

        assert_eq!(format!("{:?}", Quote::from(quote)),
                   "Quote { symbol: \"BEKE\", time: 2021-12-07T19:12:46Z, price: 20.530000686645508, change_percent: 4.478377342224121 }");
    }
}
