use std::{any::Any, sync::Arc};

use tokio::sync::{Mutex, MutexGuard, mpsc};

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

impl Context {
    pub fn use_state<T>(&self, v: T) -> State<T> {
        State {
            value: Arc::new(Mutex::new(v)),
            tx: self.tx.clone(),
        }
    }
}

pub struct State<T> {
    pub value: Arc<Mutex<T>>,
    tx: mpsc::Sender<Box<dyn Any>>,
}

pub struct StateGuard<'a, T> {
    is_mutated: bool,
    value: MutexGuard<'a, T>,
    tx: &'a mpsc::Sender<Box<dyn Any>>,
}

impl<T> State<T> {
    pub async fn lock<'a>(&'a self) -> StateGuard<'a, T> {
        StateGuard {
            value: self.value.lock().await,
            is_mutated: false,
            tx: &self.tx,
        }
    }
}

impl<'a, T> std::ops::Deref for StateGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<'a, T> std::ops::DerefMut for StateGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.is_mutated = true;

        &mut self.value
    }
}

impl<'a, T> Drop for StateGuard<'a, T> {
    fn drop(&mut self) {
        if self.is_mutated {
            self.tx.blocking_send(Box::new(Refresh)).unwrap()
        }
    }
}
