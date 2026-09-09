// SPDX-License-Identifier: GPL-2.0+
//
// Dependency: declarations from <linux/property.h> are supplied externally.

pub struct software_node;

macro_rules! PROPERTY_ENTRY_GPIO {
    ($name_:expr, $chip_node_:expr, $idx_:expr, $flags_:expr) => {
        PROPERTY_ENTRY_REF!($name_, $chip_node_, $idx_, $flags_)
    };
}

extern "C" {
    pub static swnode_gpio_undefined: software_node;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
