/* SPDX-License-Identifier: GPL-2.0 */

// Opaque declaration corresponding to `struct device_node`.
#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn gef_pic_get_irq() -> u32;
    pub fn gef_pic_init(node: *mut device_node);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
