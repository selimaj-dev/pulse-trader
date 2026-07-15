pub mod formatting;
pub mod types;

use std::any::Any;

use chrono::{Local, Utc};
use pulse_ui::{
    App,
    layout::{LayoutItem, layout},
    state::{Refresh, State},
    unit::Size,
    widget::{
        align::End,
        input::{Input, InputState},
        outline::{Outline, VLine},
        spaced::SpacedColumns,
    },
};

use crate::{
    formatting::{Formatted, apply_padding},
    types::{ActivePosition, EventLog, MarketOverview, Signal, Status, WatchListItem},
};

pub struct PulseTradeApp {
    command: State<InputState>,
    watch_list: State<Vec<WatchListItem>>,
    active_positions: State<Vec<ActivePosition>>,
    logs: State<Vec<EventLog>>,
    signals: State<Vec<Signal>>,
    market_overview: State<MarketOverview>,
    status: State<Status>,
}

impl App for PulseTradeApp {
    async fn init(&mut self, ctx: &pulse_ui::state::Context) {
        let mut watch_list = self.watch_list.lock().await;

        watch_list.push(WatchListItem {
            symbol: "BTC".to_string(),
            price: 118_402.12,
            trend: 0.82,
        });
        watch_list.push(WatchListItem {
            symbol: "ETH".to_string(),
            price: 3_912.48,
            trend: -0.41,
        });
        watch_list.push(WatchListItem {
            symbol: "SOL".to_string(),
            price: 182.91,
            trend: 2.18,
        });
        watch_list.push(WatchListItem {
            symbol: "XRP".to_string(),
            price: 2.84,
            trend: 1.22,
        });

        let mut active_positions = self.active_positions.lock().await;

        active_positions.push(ActivePosition {
            symbol: "BTC".to_string(),
            profit: 125.50,
            amount: 0.25,
        });
        active_positions.push(ActivePosition {
            symbol: "ETH".to_string(),
            profit: -32.75,
            amount: 1.0,
        });
        active_positions.push(ActivePosition {
            symbol: "SOL".to_string(),
            profit: 84.20,
            amount: 5.0,
        });
        active_positions.push(ActivePosition {
            symbol: "XRP".to_string(),
            profit: 12.30,
            amount: 0.75,
        });

        let mut signals = self.signals.lock().await;

        signals.push(Signal {
            kind: types::SignalKind::BUY,
            symbol: "BTC".to_string(),
            param: types::SignalParameter::LIM,
            price: 118_800.0,
        });

        signals.push(Signal {
            kind: types::SignalKind::BUY,
            symbol: "BTC".to_string(),
            param: types::SignalParameter::TAP,
            price: 120_000.0,
        });

        signals.push(Signal {
            kind: types::SignalKind::BUY,
            symbol: "BTC".to_string(),
            param: types::SignalParameter::STL,
            price: 118_000.0,
        });

        let mut logs = self.logs.lock().await;

        logs.push(EventLog {
            kind: types::LogKind::WARN,
            name: "pulse.init",
            message: "We're still not done yet ;)".to_string(),
        });
    }

    async fn update(&mut self, ctx: &pulse_ui::state::Context, event: Box<dyn Any + Send + Sync>) {
        if let Some(Refresh) = event.downcast_ref() {
            return;
        }

        if let Some(event) = event.downcast_ref() {
            if self.command.value.lock().await.handle_event(event) {
                return;
            }
        }

        ctx.close().await;
    }

    async fn layout(&self) -> pulse_ui::layout::LayoutItem {
        LayoutItem::Frame {
            padding: 1,
            item: Box::new(layout(vec![
                LayoutItem::Columns {
                    unit: Size::Fixed(1),
                    items: vec![
                        LayoutItem::Widget(Size::Flex(1)),
                        LayoutItem::Widget(Size::Flex(1)),
                        LayoutItem::Widget(Size::Flex(1)),
                    ],
                },
                // Account status
                LayoutItem::Spacing(Size::Fixed(1)),
                LayoutItem::Widget(Size::Flex(1)),
                // Live status
                LayoutItem::Spacing(Size::Fixed(1)),
                LayoutItem::Widget(Size::Flex(1)),
                // Event logs
                LayoutItem::Spacing(Size::Fixed(1)),
                LayoutItem::Widget(Size::Fill),
                // Input
                LayoutItem::Spacing(Size::Fixed(1)),
                LayoutItem::Widget(Size::Fixed(1)),
            ])),
        }
    }

    async fn render(&mut self, layout: pulse_ui::layout::Allocation) {
        layout.draw_frame(0, Outline);
        layout.draw(
            0,
            format!("   PULSE TRADER v{}", env!("CARGO_PKG_VERSION")),
        );
        // layout.draw(1, Center("LIVE".to_string()));
        layout.draw(
            2,
            End(format!(
                "{} UTC ({} Local)",
                Utc::now().format("%H:%M"),
                Local::now().format("%H:%M")
            )),
        );
        layout.draw_frame(3, VLine);
        layout.draw_frame(4, VLine);
        layout.draw_frame(5, VLine);
        layout.draw_frame(6, VLine);

        layout.draw(
            3,
            SpacedColumns(vec![
                (
                    LayoutItem::Widget(Size::Flex(1)),
                    Box::new(format!(
                        " WATCHLIST\n{}",
                        apply_padding(self.watch_list.lock().await.get_formatted()).join("\n")
                    )),
                ),
                (
                    LayoutItem::Widget(Size::Flex(1)),
                    Box::new(format!(
                        " ACTIVE POSITIONS\n{}",
                        apply_padding(self.active_positions.lock().await.get_formatted())
                            .join("\n")
                    )),
                ),
                (
                    LayoutItem::Widget(Size::Flex(1)),
                    Box::new(format![
                        " MARKET OVERVIEW\n{}",
                        apply_padding(self.market_overview.lock().await.get_formatted()).join("\n")
                    ]),
                ),
            ]),
        );

        layout.draw(
            4,
            SpacedColumns(vec![
                (
                    LayoutItem::Widget(Size::Flex(1)),
                    Box::new(format!(
                        " SIGNALS\n{}",
                        apply_padding(self.signals.lock().await.get_formatted()).join("\n")
                    )),
                ),
                (
                    LayoutItem::Widget(Size::Flex(1)),
                    Box::new(vec![" INSPECTOR"].join("\n")),
                ),
                (
                    LayoutItem::Widget(Size::Flex(1)),
                    Box::new(format![
                        " STATUS\n{}",
                        apply_padding(self.status.lock().await.get_formatted()).join("\n")
                    ]),
                ),
            ]),
        );

        layout.draw(
            5,
            format![
                " EVENT LOGS\n{}",
                apply_padding(self.logs.lock().await.get_formatted()).join("\n")
            ],
        );

        layout.draw(6, Input(" > ", &*self.command.lock().await));
    }
}

#[tokio::main]
async fn main() {
    pulse_ui::run(|ctx| PulseTradeApp {
        command: ctx.use_state(InputState::new()),
        watch_list: ctx.use_state(Vec::new()),
        active_positions: ctx.use_state(Vec::new()),
        signals: ctx.use_state(Vec::new()),
        logs: ctx.use_state(Vec::new()),
        market_overview: ctx.use_state(MarketOverview {}),
        status: ctx.use_state(Status {
            feed: types::Feed::Connected,
            exchange: "Binance".to_string(),
            dex: "DEX SCREENER".to_string(),
            latency: 18,
        }),
    })
    .await;
}
