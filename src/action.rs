use serde::Deserialize;

pub enum Action {
    Increase(IncreaseProperties),
    Decrease(DecreaseProperties),
}

impl Action {
    pub fn from_action(
        action_id: &str,
        properties: serde_json::Value,
    ) -> Option<Result<Action, serde_json::Error>> {
        Some(match action_id {
            "increase" => serde_json::from_value(properties).map(Action::Increase),
            "decrease" => serde_json::from_value(properties).map(Action::Decrease),
            _ => return None,
        })
    }
}

#[derive(Deserialize)]
pub struct IncreaseProperties {
    pub amount: Option<u64>,
}

#[derive(Deserialize)]
pub struct DecreaseProperties {
    pub amount: Option<u64>,
}
