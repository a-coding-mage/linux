/* SPDX-License-Identifier: GPL-2.0 */

#[repr(C)]
pub struct rv_monitor {
    _private: [u8; 0],
}

extern "C" {
    pub static mut rv_rtapp: rv_monitor;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
