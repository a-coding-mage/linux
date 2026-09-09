/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Support for the configuration register space at port I/O locations
 * 0x22 and 0x23 variously used by PC architectures, e.g. the MP Spec,
 * Cyrix CPUs, numerous chipsets.
 */

// Dependencies supplied by the surrounding kernel translation unit:
// linux/io.h, linux/spinlock.h, and linux/types.h.

pub const PC_CONF_INDEX: u16 = 0x22;
pub const PC_CONF_DATA: u16 = 0x23;

pub const PC_CONF_MPS_IMCR: u8 = 0x70;

extern "C" {
    pub static mut pc_conf_lock: raw_spinlock_t;

    fn outb(value: u8, port: u16);
    fn inb(port: u16) -> u8;
}

#[inline]
pub unsafe fn pc_conf_get(reg: u8) -> u8 {
    outb(reg, PC_CONF_INDEX);
    inb(PC_CONF_DATA)
}

#[inline]
pub unsafe fn pc_conf_set(reg: u8, data: u8) {
    outb(reg, PC_CONF_INDEX);
    outb(data, PC_CONF_DATA);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
