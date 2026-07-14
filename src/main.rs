use std::any::Any;

use pulse_ui::{
    App,
    layout::{LayoutItem, layout},
    state::{Refresh, State},
    unit::Size,
    widget::{outline::Outline, spaced::SpacedRows},
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
    }

    async fn layout(&self) -> pulse_ui::layout::LayoutItem {
        layout(vec![LayoutItem::Frame {
            padding: 1,
            item: Box::new(LayoutItem::Widget(Size::Flex(1))),
        }])
    }

    async fn render(&mut self, layout: pulse_ui::layout::Allocation) {
        layout.draw_frame(0, Outline);

        layout.draw(
            0,
            SpacedRows(vec![
                (LayoutItem::Widget(Size::Flex(1)), Box::new("one")),
                (LayoutItem::Widget(Size::Flex(1)), Box::new("two")),
                (LayoutItem::Widget(Size::Flex(1)), Box::new("three")),
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
