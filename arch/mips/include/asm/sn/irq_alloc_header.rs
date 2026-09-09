/* SPDX-License-Identifier: GPL-2.0 */
// __ASM_SN_IRQ_ALLOC_H

#[repr(C)]
pub struct irq_alloc_info {
    pub ctrl: *mut core::ffi::c_void,
    pub nasid: nasid_t,
    pub pin: i32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
