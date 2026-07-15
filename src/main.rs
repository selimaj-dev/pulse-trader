use std::any::Any;

use pulse_ui::{
    App,
    layout::{LayoutItem, layout},
    state::{Refresh, State},
    unit::Size,
    widget::outline::VLine,
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
        layout(vec![
            LayoutItem::Columns {
                unit: Size::Fixed(1),
                items: vec![
                    LayoutItem::Widget(Size::Flex(1)),
                    LayoutItem::Widget(Size::Flex(1)),
                    LayoutItem::Widget(Size::Flex(1)),
                ],
            },
            LayoutItem::Spacing(Size::Fixed(1)),
        ])
    }

    async fn render(&mut self, layout: pulse_ui::layout::Allocation) {
        layout.draw_frame(2, VLine);
        layout.draw(0, "one");
        layout.draw(1, "two");
        layout.draw(2, "three");
    }
}

#[tokio::main]
async fn main() {
    pulse_ui::run(|ctx| PulseTradeApp {
        count: ctx.use_state(0),
    })
    .await;
}
