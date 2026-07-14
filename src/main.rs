use pulse_ui::{
    App,
    layout::{LayoutItem, layout},
    state::State,
    unit::Size,
};

pub struct PulseTradeApp {
    count: State<i32>,
}

impl App for PulseTradeApp {
    async fn init(&mut self, ctx: pulse_ui::state::Context) {}

    async fn update(&mut self, event: Box<dyn std::any::Any>) {}

    async fn layout(&self) -> pulse_ui::layout::LayoutItem {
        layout(vec![
            LayoutItem::Widget(Size::Flex(16)),
            LayoutItem::Widget(Size::Flex(9)),
        ])
    }

    async fn render(&mut self, layout: pulse_ui::layout::Allocation) {
        println!("{:?}", layout);
        println!("{:?}", *self.count.lock().await);
    }
}

#[tokio::main]
async fn main() {
    pulse_ui::run(|ctx| PulseTradeApp {
        count: ctx.use_state(0),
    })
    .await;
}
