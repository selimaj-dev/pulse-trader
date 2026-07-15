pub mod formatting;
pub mod types;

use std::any::Any;

use pulse_ui::{
    App,
    layout::{LayoutItem, layout},
    state::{Refresh, State},
    unit::Size,
    widget::{
        align::Center,
        input::{Input, InputState},
        outline::{Outline, VLine},
        spaced::SpacedColumns,
    },
};

use crate::{
    formatting::{Formatted, apply_padding},
    types::{Account, ActivePosition, WatchListItem},
};

pub struct PulseTradeApp {
    command: State<InputState>,
    watch_list: State<Vec<WatchListItem>>,
    active_positions: State<Vec<ActivePosition>>,
    account: State<Account>,
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
        layout.draw(0, format!(" PULSE TRADER v0.1.0"));
        layout.draw(1, Center("LIVE".to_string()));
        layout.draw(2, Center(format!("14:32:51 UTC")));
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
                        " ACCOUNT\n{}",
                        apply_padding(self.account.lock().await.get_formatted()).join("\n")
                    ]),
                ),
            ]),
        );

        layout.draw(
            4,
            SpacedColumns(vec![
                (
                    LayoutItem::Widget(Size::Flex(1)),
                    Box::new(vec![" ACTIVE STRATEGIES"].join("\n")),
                ),
                (
                    LayoutItem::Widget(Size::Flex(1)),
                    Box::new(
                        vec![
                            " SIGNALS",
                            " BUY  BTC  LIM 118,800.12",
                            " BUY  BTC  STL 118,000.00",
                            " BUY  BTC  TAP 120,000.00",
                        ]
                        .join("\n"),
                    ),
                ),
                (
                    LayoutItem::Widget(Size::Flex(1)),
                    Box::new(
                        vec![
                            " SYSTEM",
                            " Feed:      Connected",
                            " Exchange:  Binance",
                            " DEX:       DEX SCREENER",
                            " Latency:   18 ms",
                        ]
                        .join("\n"),
                    ),
                ),
            ]),
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
        account: ctx.use_state(Account {
            equity: 25_483.21,
            liquid: 11_928.43,
            unreal: 483.12,
            realized: 2_182.49,
            margin: 0.0,
        }),
    })
    .await;
}
