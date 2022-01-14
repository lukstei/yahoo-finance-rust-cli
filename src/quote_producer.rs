use crate::{Config, Quote, YahooFinanceQuote};
use crate::{Message as YahooProtobufMessage, QuoteMessage};
use anyhow::anyhow;
use futures_util::{SinkExt, StreamExt, TryStreamExt};
use itertools::Itertools;
use protobuf::Message;
use std::time::Instant;
use tokio::sync::mpsc::Sender;
use tokio_tungstenite::connect_async;
use tracing::{debug, info, instrument, warn};

pub struct QuoteProducer {
    pub config: Config,
    pub tx: Sender<QuoteMessage>,
}

impl QuoteProducer {
    pub fn new(config: Config, tx: Sender<QuoteMessage>) -> Self {
        QuoteProducer { config, tx }
    }

    async fn create_websocket(&self) -> anyhow::Result<()> {
        let (ws_stream, _) = connect_async("wss://streamer.finance.yahoo.com/").await?;
        debug!("WebSocket handshake has been successfully completed");

        let (mut write, read) = ws_stream.split();

        let symbols = self
            .config
            .portfolio
            .iter()
            .map(|(symbol, _p)| symbol.as_str())
            .chain(["EURUSD=X"])
            .map(|symbol| format!("\"{}\"", symbol))
            .join(", ");
        let subscription_message = format!("{{\"subscribe\":[{}]}}", symbols);
        write
            .send(YahooProtobufMessage::from(subscription_message))
            .await?;

        let tx = self.tx.clone();
        read.map_err(|e| anyhow!(e))
            .try_for_each(|message| async {
                async {
                    async {
                        let data = message.into_text()?;
                        let result = base64::decode(data)?;
                        let finance_quote = YahooFinanceQuote::parse_from_bytes(&result)?;
                        if !finance_quote.get_id().is_empty() {
                            let quote = Quote::from(finance_quote);
                            info!(?quote, "Got quote");
                            tx.send(vec![quote]).await?;
                        }
                        Ok::<(), anyhow::Error>(())
                    }
                    .await
                }
                .await
            })
            .await
    }

    #[instrument(skip(self))]
    pub async fn start(&self) -> anyhow::Result<()> {
        let mut last_refresh = Instant::now();

        loop {
            debug!("WebSocket connecting...");
            match self.create_websocket().await {
                Err(e) => warn!("WS Error: {}", e),
                _ => warn!("WS Finished"),
            }

            if last_refresh.elapsed().as_secs() > 5 {
                last_refresh = Instant::now();
            } else {
                break;
            }
        }

        Err(anyhow!("WebSocket: Last refresh < 5 seconds"))
    }
}
