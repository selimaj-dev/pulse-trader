use std::any::Any;

use pulse_ui::{
    App, layout::{LayoutItem, layout}, state::State, unit::Size, widget::outline::Outline,
};

pub struct PulseTradeApp {
    count: State<i32>,
}

impl App for PulseTradeApp {
    async fn init(&mut self, ctx: &pulse_ui::state::Context) {}

    async fn update(&mut self, ctx: &pulse_ui::state::Context, event: Box<dyn Any + Send + Sync>) {
        ctx.close().await
    }

    async fn layout(&self) -> pulse_ui::layout::LayoutItem {
        layout(vec![
            LayoutItem::Widget(Size::Flex(1)),
            LayoutItem::Widget(Size::Flex(1)),
        ])
    }

    async fn render(&mut self, layout: pulse_ui::layout::Allocation) {
        layout.draw(1, "Yo");
        layout.draw_frame(0, Outline);
    }
}

#[tokio::main]
async fn main() {
    pulse_ui::run(|ctx| PulseTradeApp {
        count: ctx.use_state(0),
    })
    .await;
}
