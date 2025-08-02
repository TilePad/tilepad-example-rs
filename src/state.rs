use std::cell::RefCell;

use tilepad_plugin_sdk::Inspector;
use tokio::sync::Mutex;

use crate::messages::InspectorMessageOut;

#[derive(Default)]
pub struct State {
    counter: Mutex<u64>,
    inspector: RefCell<Option<Inspector>>,
}

impl State {
    pub fn set_inspector(&self, inspector: Option<Inspector>) {
        *self.inspector.borrow_mut() = inspector;
    }

    pub async fn get_counter(&self) -> u64 {
        *self.counter.lock().await
    }

    pub async fn set_counter(&self, value: u64) {
        *self.counter.lock().await = value;
    }

    pub async fn increase(&self, amount: u64) {
        {
            let counter = &mut *self.counter.lock().await;
            *counter = counter.saturating_add(amount);
        }

        self.update_inspector().await;
    }

    pub async fn decrease(&self, amount: u64) {
        {
            let counter = &mut *self.counter.lock().await;
            *counter = counter.saturating_sub(amount);
        }

        self.update_inspector().await;
    }

    async fn update_inspector(&self) {
        let counter = self.get_counter().await;

        if let Some(inspector) = self.inspector.borrow().as_ref() {
            _ = inspector.send(InspectorMessageOut::Counter { value: counter });
        }
    }
}
