use crate::{
    action::Action,
    messages::{InspectorMessageIn, InspectorMessageOut},
    state::State,
};
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use tilepad_plugin_sdk::{
    Inspector, Plugin, PluginSessionHandle, TileInteractionContext,
    tracing::{self},
};
use tokio::task::spawn_local;

/// Properties for the plugin itself
#[derive(Debug, Deserialize, Serialize)]
pub struct Properties {
    #[serde(default = "default_counter")]
    pub counter: u64,
}

fn default_counter() -> u64 {
    0
}

#[derive(Default)]
pub struct ExamplePlugin {
    state: Rc<State>,
}

impl ExamplePlugin {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Plugin for ExamplePlugin {
    fn on_properties(&mut self, _session: &PluginSessionHandle, properties: serde_json::Value) {
        let state = self.state.clone();
        let properties: Properties = match serde_json::from_value(properties) {
            Ok(value) => value,
            Err(cause) => {
                tracing::error!(?cause, "failed to parse properties");
                return;
            }
        };

        // Update the counter from state
        spawn_local(async move {
            state.set_counter(properties.counter).await;
        });
    }

    fn on_inspector_open(&mut self, _session: &PluginSessionHandle, inspector: Inspector) {
        self.state.set_inspector(Some(inspector));
    }

    fn on_inspector_close(&mut self, _session: &PluginSessionHandle, _inspector: Inspector) {
        self.state.set_inspector(None);
    }

    fn on_inspector_message(
        &mut self,
        _session: &PluginSessionHandle,
        inspector: Inspector,
        message: serde_json::Value,
    ) {
        let message: InspectorMessageIn = match serde_json::from_value(message) {
            Ok(value) => value,
            Err(_) => return,
        };

        match message {
            InspectorMessageIn::GetCounter => {
                let state = self.state.clone();
                spawn_local(async move {
                    let counter = state.get_counter().await;
                    inspector.send(InspectorMessageOut::Counter { value: counter })
                });
            }
        }
    }

    fn on_tile_clicked(
        &mut self,
        _session: &PluginSessionHandle,
        ctx: TileInteractionContext,
        properties: serde_json::Value,
    ) {
        let action_id = ctx.action_id.as_str();
        let action = match Action::from_action(action_id, properties) {
            Some(Ok(value)) => value,
            Some(Err(cause)) => {
                tracing::error!(?cause, ?action_id, "failed to deserialize action");
                return;
            }
            None => {
                tracing::debug!(?action_id, "unknown tile action requested");
                return;
            }
        };

        let state = self.state.clone();

        match action {
            Action::Increase(properties) => {
                spawn_local(async move {
                    state.increase(properties.amount.unwrap_or(1)).await;
                });
            }
            Action::Decrease(properties) => {
                spawn_local(async move {
                    state.decrease(properties.amount.unwrap_or(1)).await;
                });
            }
        }
    }
}
