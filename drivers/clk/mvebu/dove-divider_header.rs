/* SPDX-License-Identifier: GPL-2.0 */

// C dependency: `struct device_node` is supplied by the surrounding codebase.
#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

// C `__init` annotation preserved as declaration intent; implementation is external.
unsafe extern "C" {
    pub fn dove_divider_clk_init(np: *mut device_node);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
