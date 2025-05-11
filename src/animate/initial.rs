use leptos::prelude::{GetValue, SetValue, StoredValue};

pub struct Initial(StoredValue<bool>);

impl Initial {
    pub fn new() -> Self {
        Self(StoredValue::new(true))
    }

    pub fn get(&self) -> bool {
        if self.0.get_value() {
            self.0.set_value(false);
            true
        } else {
            false
        }
    }
}
