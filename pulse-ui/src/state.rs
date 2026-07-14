use std::{any::Any, sync::Arc};

use tokio::sync::{Mutex, MutexGuard, mpsc::Sender};

pub struct Refresh;

#[derive(Debug, Clone)]
pub struct Context {
    pub(crate) tx: Sender<Box<dyn Any>>,
}

pub struct State<T> {
    pub value: Arc<Mutex<T>>,
    tx: Sender<Box<dyn Any>>,
}

pub struct StateGuard<'a, T> {
    is_mutated: bool,
    value: MutexGuard<'a, T>,
    tx: &'a Sender<Box<dyn Any>>,
}

impl Context {
    pub fn use_state<T>(&self, v: T) -> State<T> {
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
