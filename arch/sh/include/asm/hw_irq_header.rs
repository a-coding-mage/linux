/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation.
use core::ffi::{c_int, c_uint, c_ulong};

extern "C" {
    pub static mut irq_err_count: atomic_t;
}

#[repr(C)]
pub struct ipr_data {
    pub irq: u8,
    pub ipr_idx: u8, // Index for the IPR registered
    pub shift: u8, // Number of bits to shift the data
    pub priority: u8, // The priority
}

#[repr(C)]
pub struct ipr_desc {
    pub ipr_offsets: *mut c_ulong,
    pub nr_offsets: c_uint,
    pub ipr_data: *mut ipr_data,
    pub nr_irqs: c_uint,
    pub chip: irq_chip,
}

extern "C" {
    pub fn register_ipr_controller(desc: *mut ipr_desc);

    // __init
    pub fn plat_irq_setup();
    // __init
    pub fn plat_irq_setup_sh3();
    // __init
    pub fn plat_irq_setup_pins(mode: c_int);
}

pub const IRQ_MODE_IRQ: c_int = 0;
pub const IRQ_MODE_IRQ7654: c_int = 1;
pub const IRQ_MODE_IRQ3210: c_int = 2;
pub const IRQ_MODE_IRL7654_MASK: c_int = 3;
pub const IRQ_MODE_IRL3210_MASK: c_int = 4;
pub const IRQ_MODE_IRL7654: c_int = 5;
pub const IRQ_MODE_IRL3210: c_int = 6;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
