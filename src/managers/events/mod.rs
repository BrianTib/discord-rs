use std::collections::HashMap;
use serde_json::Value;
use std::any::Any;

use crate::client::DispatchEvent;

macro_rules! into_system {
    // This macro takes an argument of designator `ident` and
    // creates a function named `$func_name`.
    // The `ident` designator is used for variable/function names.
    ($func_name:ident) => {
        fn $func_name() {
            // The `stringify!` macro converts an `ident` into a string.
            println!("You called {:?}()",
                     stringify!($func_name));
        }
    };
}

pub struct EventManager {
    _events: HashMap<DispatchEvent, fn(dyn Any)>
}

impl EventManager {
    pub fn new() -> Self {
        Self { _events: HashMap::new() }
    }

    // pub fn receive(&self, event_type: DispatchEvent, data: Value) {
    //     if let Some(handler) = self._events.get(&event_type) {
    //         handler(data.into())
    //     }
    // }
}