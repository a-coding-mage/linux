/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Interrupt flow handler typedefs are defined here to avoid circular
 * include dependencies.
 */

#[repr(C)]
pub struct irq_desc {
    _private: [u8; 0],
}

pub type irq_flow_handler_t = Option<unsafe extern "C" fn(desc: *mut irq_desc)>;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
