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

use crate::types::{Formatted, WatchListItem, space_out};

pub struct PulseTradeApp {
    command: State<InputState>,
    watch_list: State<Vec<WatchListItem>>,
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
            trend: 0.41,
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
                LayoutItem::Widget(Size::Flex(1)),
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
                        space_out(self.watch_list.lock().await.get_formatted()).join("\n")
                    )),
                ),
                (
                    LayoutItem::Widget(Size::Flex(1)),
                    Box::new(
                        vec![
                            " ACTIVE POSITIONS",
                            " BTC  +4,120.40  20,000.00",
                            " SOL  +2,435.40  20,000.00",
                        ]
                        .join("\n"),
                    ),
                ),
                (
                    LayoutItem::Widget(Size::Flex(1)),
                    Box::new(
                        vec![
                            " ACCOUNT",
                            " Equity:      $25,483.21",
                            " Liquid:      $11,928.43",
                            " Unreal:      +$483.12",
                            " Realized:    +$2,182.49",
                            " Margin:      0.00%",
                        ]
                        .join("\n"),
                    ),
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
    })
    .await;
}
