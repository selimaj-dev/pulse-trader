use std::collections::HashMap;

use pulse_wire::terminal::WatchListItem;
use serde_json::Value;

use crate::terminal::TerminalServer;

const WATCH_LIST_SYMBOLS: &[&str] = &["BTC", "ETH", "SOL", "XRP"];

pub struct Engine {
    pub terminal_server: TerminalServer,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            terminal_server: TerminalServer::new(),
        }
    }

    pub fn spawn_terminal_server(&self) {
        let terminal_server = self.terminal_server.clone();

        tokio::spawn(async move {
            terminal_server
                .run()
                .await
                .expect("Failed to run terminal server");
        });
    }

    pub fn spawn_broadcaster(&mut self) {
        let terminal_server = self.terminal_server.clone();

        tokio::spawn(async move {
            let mut refresh = tokio::time::interval(tokio::time::Duration::from_secs(5));

            loop {
                refresh.tick().await;

                match fetch_watch_list(WATCH_LIST_SYMBOLS).await {
                    Ok(watch_list) => {
                        if let Err(error) = terminal_server
                            .broadcast(
                                pulse_wire::terminal::TerminalServerMessage::WatchListUpdated(
                                    watch_list,
                                ),
                            )
                            .await
                        {
                            eprintln!("Failed to broadcast Hyperliquid watch list: {error}");
                        }
                    }
                    Err(error) => eprintln!("Failed to refresh Hyperliquid watch list: {error}"),
                }
            }
        });
    }

    pub async fn run_engine(&mut self) -> tokio::io::Result<()> {
        loop {
            tokio::time::sleep(tokio::time::Duration::from_millis(5000)).await;
        }

        Ok(())
    }
}

async fn fetch_watch_list(symbols: &[&str]) -> Result<Vec<WatchListItem>, String> {
    let response = hypersdk::hypercore::mainnet()
        .meta_and_asset_ctxs(None)
        .await
        .map_err(|error| format!("Hyperliquid metaAndAssetCtxs request failed: {error}"))?;

    normalize_watch_list(&response, symbols)
}

fn normalize_watch_list(response: &Value, symbols: &[&str]) -> Result<Vec<WatchListItem>, String> {
    let response = response
        .as_array()
        .ok_or("metaAndAssetCtxs response must be an array")?;

    if response.len() != 2 {
        return Err(format!(
            "metaAndAssetCtxs response must contain meta and contexts, got {} entries",
            response.len()
        ));
    }

    let universe = response[0]["universe"]
        .as_array()
        .ok_or("metaAndAssetCtxs response is missing meta.universe")?;
    let contexts = response[1]
        .as_array()
        .ok_or("metaAndAssetCtxs response contexts must be an array")?;

    if universe.len() != contexts.len() {
        return Err(format!(
            "Hyperliquid returned {} instruments but {} contexts",
            universe.len(),
            contexts.len()
        ));
    }

    let mut by_symbol = HashMap::with_capacity(universe.len());

    for (meta, context) in universe.iter().zip(contexts) {
        let symbol = meta["name"]
            .as_str()
            .ok_or("instrument metadata is missing a name")?;
        let price = number(context, "markPx")?;
        let previous_day_price = number(context, "prevDayPx")?;
        let volume_24h = number(context, "dayNtlVlm")?;

        if previous_day_price <= 0.0 {
            return Err(format!("{symbol} has an invalid previous-day price"));
        }

        by_symbol.insert(
            symbol,
            WatchListItem {
                symbol: symbol.to_owned(),
                price,
                trend: ((price / previous_day_price) - 1.0) * 100.0,
                volume_24h,
            },
        );
    }

    symbols
        .iter()
        .map(|symbol| {
            by_symbol
                .remove(*symbol)
                .ok_or_else(|| format!("{symbol} is not in the Hyperliquid perpetual universe"))
        })
        .collect()
}

fn number(value: &Value, field: &str) -> Result<f64, String> {
    let raw = value[field]
        .as_str()
        .ok_or_else(|| format!("asset context field {field} must be a string"))?;

    raw.parse::<f64>()
        .map_err(|error| format!("could not parse asset context field {field} ({raw}): {error}"))
}
