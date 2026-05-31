mod events;
mod macros;

pub use events::*;

pub struct TelemetryContextValue;

impl TelemetryContextValue {
    pub fn as_value(&self) -> serde_json::Value {
        serde_json::Value::Null
    }
}

pub fn telemetry_context() -> TelemetryContextValue {
    TelemetryContextValue
}
