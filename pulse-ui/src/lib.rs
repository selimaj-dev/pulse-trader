use std::any::Any;

use tokio::sync::mpsc;

pub mod layout;
pub mod render;
pub mod state;
pub mod unit;
pub mod widget;

#[allow(async_fn_in_trait)]
pub trait App {
    // State
    async fn init(&mut self, ctx: state::Context);
    async fn update(&mut self, event: Box<dyn Any>);

    // Rendering
    async fn layout(&self) -> layout::LayoutItem;
    async fn render(&mut self, allocation: layout::Allocation);

    async fn refresh(&mut self) {
        crossterm::execute!(
            std::io::stdout(),
            crossterm::terminal::Clear(crossterm::terminal::ClearType::All)
        )
        .unwrap();

        crossterm::execute!(
            std::io::stdout(),
            crossterm::terminal::Clear(crossterm::terminal::ClearType::Purge)
        )
        .unwrap();

        crossterm::execute!(std::io::stdout(), crossterm::cursor::Hide).unwrap();

        self.render(get_screen().allocate(&self.layout().await))
            .await;
    }
}

pub async fn run<A: App>(create_app: impl FnOnce(&state::Context) -> A) {
    let (tx, mut rx) = mpsc::channel(100);

    let ctx = state::Context { tx };

    let mut app = create_app(&ctx);

    app.init(ctx).await;

    app.refresh().await;

    while let Some(event) = rx.recv().await {
        app.update(event).await;
        app.refresh().await;
    }
}

pub fn get_screen() -> unit::Rect {
    let (width, height) = crossterm::terminal::size().unwrap();

    unit::Rect {
        x: 0,
        y: 0,
        width,
        height,
    }
}
