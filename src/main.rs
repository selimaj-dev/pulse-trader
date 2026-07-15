use std::any::Any;

use pulse_ui::{
    App,
    layout::{LayoutItem, layout},
    state::Refresh,
    unit::Size,
    widget::{
        align::Center,
        outline::{Outline, VLine},
        spaced::{SpacedColumns, SpacedRows},
    },
};

pub struct PulseTradeApp {}

impl App for PulseTradeApp {
    async fn init(&mut self, ctx: &pulse_ui::state::Context) {}

    async fn update(&mut self, ctx: &pulse_ui::state::Context, event: Box<dyn Any + Send + Sync>) {
        if let Some(Refresh) = event.downcast_ref() {
            return;
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
                    Box::new(
                        vec![
                            "WATCHLIST",
                            "BTC  118,402.12  ▲ 0.82%",
                            "ETH    3,912.48  ▼ 0.41%",
                            "SOL      182.91  ▲ 2.18%",
                            "XRP        2.84  ▲ 1.22%",
                        ]
                        .join("\n"),
                    ),
                ),
                (
                    LayoutItem::Widget(Size::Flex(1)),
                    Box::new(
                        vec![
                            "ACTIVE POSITIONS",
                            "BTC  +4,120.40  20,000.00",
                            "SOL  +2,435.40  20,000.00",
                        ]
                        .join("\n"),
                    ),
                ),
                (
                    LayoutItem::Widget(Size::Flex(1)),
                    Box::new(
                        vec![
                            "ACCOUNT",
                            "Equity:      $25,483.21",
                            "Liquid:      $11,928.43",
                            "Unreal:      +$483.12",
                            "Realized:    +$2,182.49",
                            "Margin:      0.00%",
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
                    Box::new(SpacedRows(vec![])),
                ),
                (
                    LayoutItem::Widget(Size::Flex(1)),
                    Box::new(SpacedRows(vec![])),
                ),
                (
                    LayoutItem::Widget(Size::Flex(1)),
                    Box::new(SpacedRows(vec![])),
                ),
            ]),
        );
    }
}

#[tokio::main]
async fn main() {
    pulse_ui::run(|_ctx| PulseTradeApp {}).await;
}
