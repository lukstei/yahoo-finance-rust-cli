use crate::{Config, InfluxDB, Measurement, Portfolio, Quote, QuoteMessage, QuoteMeta, Yahoo};
use chrono::Utc;
use itertools::Itertools;
use tokio::sync::mpsc::Receiver;
use tracing::{info, instrument};

pub struct QuoteReceiver {
    pub config: Config,
    pub rx: Receiver<QuoteMessage>,
    db: Option<InfluxDB>,
}

impl QuoteReceiver {
    pub fn new(config: Config, rx: Receiver<QuoteMessage>) -> Self {
        let db = config.db.as_ref().map(|x| InfluxDB::new(x.clone()));
        QuoteReceiver { config, rx, db }
    }

    fn report(&self, portfolio: &Portfolio, s: &str) {
        if self.config.print_portfolio {
            crate::cls();
            println!("{}", portfolio);

            let pl = portfolio.daily_pl();
            let nw = portfolio.market_value();
            println!(
                "Daily P/L: {} ({}%), Market Value: {} ({})",
                pl.map(|pl| format!("{:+.0}", pl))
                    .unwrap_or_else(|| "?".into()),
                pl.zip(nw)
                    .map(|(pl, nw)| format!("{:+.2}", pl / nw * 100.))
                    .unwrap_or_else(|| "?".into()),
                nw.map(|pl| format!("{:.0}", pl))
                    .unwrap_or_else(|| "?".into()),
                s
            );
        }
    }

    async fn initialize_portfolio(&self) -> anyhow::Result<Portfolio> {
        let (quotes, metas): (Vec<Quote>, Vec<QuoteMeta>) = Yahoo::get_quotes(
            self.config
                .portfolio
                .iter()
                .map(|(symbol, _pos)| symbol.as_str())
                .collect_vec(),
        )
        .await?
        .into_iter()
        .unzip();

        let (fx_quotes, fx_metas): (Vec<Quote>, Vec<QuoteMeta>) =
            Yahoo::get_quotes(vec!["EURUSD=X"])
                .await?
                .into_iter()
                .unzip();
        let mut portfolio = Portfolio::new(self.config.home_currency, fx_metas);

        for (quote_meta, (symbol, position)) in metas.iter().zip_eq(self.config.portfolio.iter()) {
            assert_eq!(quote_meta.symbol, *symbol);
            portfolio.add_position(quote_meta.clone(), *position);
        }

        for quote in fx_quotes.iter().chain(quotes.iter()) {
            portfolio.update(quote);
        }

        self.push_measurements(quotes.iter().map_into().collect_vec(), &portfolio)
            .await?;

        Ok(portfolio)
    }

    async fn push_measurements(
        &self,
        mut measurements: Vec<Measurement>,
        portfolio: &Portfolio,
    ) -> anyhow::Result<()> {
        if let Some(db) = &self.db {
            let now = Utc::now();
            if let Some(x) = portfolio.market_value() {
                measurements.push(Measurement {
                    value: x,
                    symbol: "portfolio:market_value".into(),
                    time: now,
                })
            }
            if let Some(x) = portfolio.daily_pl() {
                measurements.push(Measurement {
                    value: x,
                    symbol: "portfolio:daily_pl".into(),
                    time: now,
                })
            }

            db.push(measurements).await?
        }

        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn start(&mut self) -> anyhow::Result<()> {
        let mut portfolio = self.initialize_portfolio().await?;

        self.report(&portfolio, "Quotes received");

        while let Some(quotes) = self.rx.recv().await {
            async {
                for quote in &quotes {
                    portfolio.update(quote);
                    info!(?quote, "Quote updated");
                }

                self.report(
                    &portfolio,
                    format!(
                        "Quote {} updated",
                        quotes.iter().map(|x| x.symbol.as_str()).join(", ")
                    )
                    .as_str(),
                );
                self.push_measurements(quotes.iter().map_into().collect_vec(), &portfolio)
                    .await
            }
            .await?
        }

        Ok::<(), anyhow::Error>(())
    }
}
