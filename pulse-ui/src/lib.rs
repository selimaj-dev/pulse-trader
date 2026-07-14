use std::any::Any;

use tokio::sync::mpsc;

pub mod layout;
pub mod render;
pub mod unit;
pub mod widget;

#[derive(Debug, Clone)]
pub struct Context {
    tx: mpsc::Sender<Box<dyn Any>>,
}

pub struct Refresh;

pub trait App {
    // State
    #[allow(async_fn_in_trait)]
    async fn init(&mut self, ctx: Context);
    #[allow(async_fn_in_trait)]
    async fn update(&mut self, event: Box<dyn Any>);

    // Rendering
    fn layout(&self) -> layout::Allocation;
    fn render(&mut self, layout: layout::Allocation);
}

pub struct AppHandler<A: App> {
    pub app: A,
}

impl<A: App> AppHandler<A> {
    pub async fn run(&mut self) {
        let (tx, mut rx) = mpsc::channel(100);

        self.app.init(Context { tx }).await;

        self.app.render(self.app.layout());

        while let Some(event) = rx.recv().await {
            self.app.update(event).await;
            self.app.render(self.app.layout());
        }
    }
}
