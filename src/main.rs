use std::any::Any;

use pulse_ui::{
    App,
    layout::{LayoutItem, layout},
    state::{Refresh, State},
    unit::Size,
    widget::{
        center::Center,
        outline::{Outline, VLine},
        spaced::{SpacedColumns, SpacedRows},
    },
};

pub struct PulseTradeApp {
    count: State<i32>,
}

impl App for PulseTradeApp {
    async fn init(&mut self, ctx: &pulse_ui::state::Context) {}

    async fn update(&mut self, ctx: &pulse_ui::state::Context, event: Box<dyn Any + Send + Sync>) {
        if let Some(Refresh) = event.downcast_ref() {
            return;
        }

        *self.count.lock().await += 1;

        if *self.count.lock().await > 10 {
            ctx.close().await;
        }
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
        layout.draw(0, format!("PULSETRADER v0.1.0"));
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
    pulse_ui::run(|ctx| PulseTradeApp {
        count: ctx.use_state(0),
    })
    .await;
}
