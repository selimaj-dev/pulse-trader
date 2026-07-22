use std::{any::Any, sync::Arc};

use tokio::sync::{Mutex, MutexGuard, mpsc::Sender};

pub struct Refresh;
pub struct Close;

#[derive(Debug, Clone)]
pub struct Context {
    pub(crate) tx: Sender<Box<dyn Any + Send + Sync>>,
}

#[derive(Debug)]
pub struct State<T> {
    pub value: Arc<Mutex<T>>,
    tx: Sender<Box<dyn Any + Send + Sync>>,
}

pub struct StateGuard<'a, T> {
    is_mutated: bool,
    value: MutexGuard<'a, T>,
    tx: &'a Sender<Box<dyn Any + Send + Sync>>,
}

impl Context {
    pub fn use_state<T: Send + Sync>(&self, v: T) -> State<T> {
        State {
            value: Arc::new(Mutex::new(v)),
            tx: self.tx.clone(),
        }
    }
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

impl<T: std::fmt::Display> State<T> {
    pub async fn display(&self) -> String {
        self.lock().await.to_string()
    }
}

impl<T> Clone for State<T> {
    fn clone(&self) -> Self {
        State {
            value: self.value.clone(),
            tx: self.tx.clone(),
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
            let tx = self.tx.clone();

            tokio::spawn(async move { tx.send(Box::new(Refresh)).await.unwrap() });
        }
    }
}

impl Context {
    pub async fn event<E: Any + Send + Sync>(&self, event: E) {
        self.tx.send(Box::new(event)).await.unwrap()
    }

    pub async fn close(&self) {
        self.event(Close).await
    }

    pub async fn refresh(&self) {
        self.event(Refresh).await
    }
}
