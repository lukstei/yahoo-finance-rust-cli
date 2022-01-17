use crate::YahooFinanceQuote;
use chrono::{DateTime, Local, NaiveDateTime, NaiveTime, Utc};
use comfy_table::{Cell, CellAlignment, Color, Table};
use itertools::Itertools;
use serde::Deserialize;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
#[allow(clippy::upper_case_acronyms)]
pub enum Currency {
    USD,
    EUR,
}

impl Display for Currency {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Currency::USD => "$",
                Currency::EUR => "€",
            }
        )
    }
}

#[derive(Debug, Clone)]
pub struct Quote {
    pub symbol: String,
    pub time: DateTime<Utc>,
    pub price: f64,
    pub change: f64,
}

#[derive(Debug, Clone)]
pub struct QuoteMeta {
    pub symbol: String,
    pub description: String,
    pub currency: Currency,
    pub currency_pair: Option<(Currency, Currency)>, // set when fx quote
}

impl Quote {
    pub fn old_price(&self) -> f64 {
        self.price / (self.change + 1.)
    }
}

impl From<YahooFinanceQuote> for Quote {
    fn from(q: YahooFinanceQuote) -> Self {
        Quote {
            symbol: q.get_id().into(),
            time: DateTime::from_utc(
                NaiveDateTime::from_timestamp(
                    q.get_time() / 1000,
                    (q.get_time() % 1000 * 1000 * 1000) as u32,
                ),
                Utc,
            ),
            price: q.get_price() as f64,
            change: q.get_changePercent() as f64 * 0.01,
        }
    }
}

pub struct PortfolioItem {
    pub quote: Option<Quote>,
    pub position: f64,
    pub meta: QuoteMeta,
}

impl PortfolioItem {
    pub fn daily_pl(&self) -> Option<f64> {
        self.quote
            .as_ref()
            .filter(|x| {
                x.time
                    >= Local::today()
                        .and_time(NaiveTime::from_hms(0, 0, 0))
                        .unwrap()
            })
            .map(|x| (x.price - x.old_price()) * self.position)
    }
}

pub struct Portfolio {
    positions: HashMap<String, PortfolioItem>,
    fx_rates: HashMap<String, PortfolioItem>,
    home_currency: Currency,
}

impl Display for Portfolio {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut table = Table::new();
        table
            .load_preset(comfy_table::presets::UTF8_BORDERS_ONLY)
            .apply_modifier(comfy_table::modifiers::UTF8_ROUND_CORNERS)
            .set_header(vec![
                "Symbol",
                "Position",
                "Daily P/L",
                "Price",
                "Market Value",
                "Quote Date",
                "Description",
            ]);

        let mut symbols: Vec<(&String, &PortfolioItem)> = self.positions.iter().collect();
        symbols.sort_by_key(|(symbol, _position)| *symbol);

        for (symbol, item) in symbols {
            table.add_row(match &item.quote {
                None => {
                    vec![
                        Cell::new(symbol),
                        Cell::new(format!("{:.0}", item.position)),
                        Cell::new("?"),
                        Cell::new("?"),
                        Cell::new("?"),
                        Cell::new("?"),
                        Cell::new(&item.meta.description),
                    ]
                }
                Some(quote) => {
                    let total_mv = self.market_value();
                    let pl = item.daily_pl().unwrap_or(0.0);
                    vec![
                        Cell::new(symbol),
                        Cell::new(format!("{:.0}", item.position))
                            .set_alignment(CellAlignment::Right),
                        Cell::new(
                            self.convert(pl, item.meta.currency, self.home_currency)
                                .map(|pl| {
                                    format!(
                                        "{} {:+.2} ({:+.2}%)",
                                        self.home_currency,
                                        pl,
                                        pl / (quote.price * item.position) * 100.
                                    )
                                })
                                .unwrap_or_else(|| "?".into()),
                        )
                        .fg(if pl >= 0. { Color::Green } else { Color::Red })
                        .set_alignment(CellAlignment::Right),
                        Cell::new(format!("{} {:.2}", item.meta.currency, quote.price))
                            .set_alignment(CellAlignment::Right),
                        Cell::new(
                            self.convert(
                                quote.price * item.position,
                                item.meta.currency,
                                self.home_currency,
                            )
                            .map(|mv| {
                                format!(
                                    "{} {:.2} ({}%)",
                                    self.home_currency,
                                    mv,
                                    total_mv
                                        .map(|total| format!("{:>2.0}", mv / total * 100.))
                                        .unwrap_or_else(|| "?".into())
                                )
                            })
                            .unwrap_or_else(|| "?".into()),
                        )
                        .set_alignment(CellAlignment::Right),
                        Cell::new(quote.time),
                        Cell::new(&item.meta.description),
                    ]
                }
            });
        }
        write!(f, "{}", table)
    }
}

impl Portfolio {
    pub fn new(home_currency: Currency, fx: Vec<QuoteMeta>) -> Portfolio {
        let position_map = HashMap::new();

        let mut fx_map = HashMap::new();
        for meta in fx {
            fx_map.insert(
                meta.symbol.clone(),
                PortfolioItem {
                    position: 0.0,
                    quote: None,
                    meta,
                },
            );
        }

        Portfolio {
            home_currency,
            positions: position_map,
            fx_rates: fx_map,
        }
    }

    pub fn add_position(&mut self, meta: QuoteMeta, position: f64) {
        match self.positions.get(&meta.symbol) {
            None => {
                self.positions.insert(
                    meta.symbol.clone(),
                    PortfolioItem {
                        position,
                        quote: None,
                        meta,
                    },
                );
            }
            Some(_) => {}
        }
    }

    pub fn update(&mut self, quote: &Quote) -> bool {
        match self.positions.get_mut(&quote.symbol) {
            None => {}
            Some(item) => {
                item.quote = Some(quote.clone());
                return true;
            }
        }
        match self.fx_rates.get_mut(&quote.symbol) {
            None => false,
            Some(item) => {
                item.quote = Some(quote.clone());
                true
            }
        }
    }

    pub fn daily_pl(&self) -> Option<f64> {
        self.positions
            .values()
            .map(|p| {
                self.convert(
                    p.daily_pl().unwrap_or(0.),
                    p.meta.currency,
                    self.home_currency,
                )
            })
            .fold_options(0., |s, x| s + x)
    }

    pub fn market_value(&self) -> Option<f64> {
        self.positions
            .values()
            .map(|p| {
                p.quote.as_ref().and_then(|q| {
                    self.convert(q.price * p.position, p.meta.currency, self.home_currency)
                })
            })
            .fold_options(0., |s, x| s + x)
    }

    pub fn convert(&self, price: f64, from: Currency, to: Currency) -> Option<f64> {
        self.fx_rates
            .values()
            .filter_map(|x| {
                x.quote.as_ref().and_then(|q| {
                    Self::convert_price(price, from, to, q.price, x.meta.currency_pair.unwrap())
                })
            })
            .next()
    }

    pub fn convert_price(
        price: f64,
        from: Currency,
        to: Currency,
        fx_rate: f64,
        fx_pair: (Currency, Currency),
    ) -> Option<f64> {
        if from == to {
            Some(price)
        } else {
            let (c1, c2) = fx_pair;
            if c1 == from && c2 == to {
                Some(price * fx_rate)
            } else if c2 == from && c1 == to {
                Some(price * 1. / fx_rate)
            } else {
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::portfolio::Currency::USD;
    use crate::{Portfolio, EUR};

    #[test]
    #[allow(clippy::float_cmp)]
    fn convert() {
        assert_eq!(
            1.1,
            Portfolio::convert_price(1., EUR, USD, 1.1, (EUR, USD)).unwrap()
        );
        assert_eq!(
            0.9090909090909091,
            Portfolio::convert_price(1., USD, EUR, 1.1, (EUR, USD)).unwrap()
        );
        assert_eq!(
            1.,
            Portfolio::convert_price(1., EUR, EUR, 1.1, (EUR, USD)).unwrap()
        );
    }
}
