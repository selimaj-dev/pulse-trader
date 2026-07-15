use std::{any::Any, time::Duration};

use tokio::sync::mpsc;

pub mod layout;
pub mod render;
pub mod state;
pub mod unit;
pub mod widget;

#[allow(async_fn_in_trait)]
pub trait App {
    // State
    async fn init(&mut self, ctx: &state::Context);
    async fn update(&mut self, ctx: &state::Context, event: Box<dyn Any + Send + Sync>);

    // Rendering
    async fn layout(&self) -> layout::LayoutItem;
    async fn render(&mut self, allocation: layout::Allocation);

    async fn refresh(&mut self) {
        clear();

        crossterm::execute!(std::io::stdout(), crossterm::cursor::Hide).unwrap();

        self.render(get_screen().allocate(&self.layout().await))
            .await;

        crossterm::execute!(std::io::stdout()).unwrap();
    }
}

pub async fn run<A: App>(create_app: impl FnOnce(&state::Context) -> A) {
    let (tx, mut rx) = mpsc::channel(100);

    let ctx = state::Context { tx: tx.clone() };

    crossterm::terminal::enable_raw_mode().unwrap();

    tokio::spawn(async move {
        while !tx.is_closed() {
            if crossterm::event::poll(Duration::from_millis(100)).unwrap() {
                tx.send(Box::new(crossterm::event::read().unwrap()))
                    .await
                    .unwrap();
            }
        }
    });

    let mut app = create_app(&ctx);

    app.init(&ctx).await;

    app.refresh().await;

    while let Some(event) = rx.recv().await {
        if let Some(state::Close) = event.downcast_ref() {
            break;
        }

        app.update(&ctx, event).await;
        app.refresh().await;
    }

    crossterm::terminal::disable_raw_mode().unwrap();
    crossterm::execute!(std::io::stdout(), crossterm::cursor::Show).unwrap();

    clear();
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

pub fn clear() {
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

    crossterm::execute!(std::io::stdout(), crossterm::cursor::MoveTo(0, 0)).unwrap();
}
