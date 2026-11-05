use rsx::prelude::*;

use crate::ui::radio_group::{RadioGroup, RadioGroupItem};

pub fn demo() -> Element {
    <RadioGroup
        default_value="comfortable"
        items={vec![
            RadioGroupItem::new("default", "Default"),
            RadioGroupItem::new("comfortable", "Comfortable"),
            RadioGroupItem::new("compact", "Compact"),
        ]}
    />
}
