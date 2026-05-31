#[macro_export]
macro_rules! send_telemetry_sync_from_ctx {
    ($event:expr, $ctx:expr) => {
        if false {
            let _ = &$event;
        }
        let _ = &$ctx;
    };
}

#[macro_export]
macro_rules! send_telemetry_sync_from_app_ctx {
    ($event:expr, $app_ctx:expr) => {
        if false {
            let _ = &$event;
        }
        let _ = &$app_ctx;
    };
}

#[macro_export]
macro_rules! send_telemetry_on_executor {
    ($auth_state:expr, $event:expr, $executor:expr) => {
        if false {
            let _ = &$event;
        }
        let _ = &$auth_state;
        let _ = &$executor;
    };
}
