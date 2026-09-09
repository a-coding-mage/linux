/* SPDX-License-Identifier: GPL-2.0-only */

// __MFD_CS5535_H__

// External dependency: struct software_node is declared elsewhere.
#[repr(C)]
pub struct software_node {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub static cs5535_gpio_swnode: software_node;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
