/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Definitions for PCI support.
 */

pub const FL_BASE_MASK: u32 = 0x0007;
pub const FL_BASE0: u32 = 0x0000;
pub const FL_BASE1: u32 = 0x0001;
pub const FL_BASE2: u32 = 0x0002;
pub const FL_BASE3: u32 = 0x0003;
pub const FL_BASE4: u32 = 0x0004;

#[inline]
pub const fn FL_GET_BASE(x: u32) -> u32 {
    x & FL_BASE_MASK
}

/* Use successive BARs (PCI base address registers),
   else use offset into some specified BAR */
pub const FL_BASE_BARS: u32 = 0x0008;

/* do not assign an irq */
pub const FL_NOIRQ: u32 = 0x0080;

/* Use the Base address register size to cap number of ports */
pub const FL_REGION_SZ_CAP: u32 = 0x0100;

#[repr(C)]
pub struct pciserial_board {
    pub flags: core::ffi::c_uint,
    pub num_ports: core::ffi::c_uint,
    pub base_baud: core::ffi::c_uint,
    pub uart_offset: core::ffi::c_uint,
    pub reg_shift: core::ffi::c_uint,
    pub first_offset: core::ffi::c_uint,
}

pub enum serial_private {}

pub enum pci_dev {}

unsafe extern "C" {
    pub fn pciserial_init_ports(
        dev: *mut pci_dev,
        board: *const pciserial_board,
    ) -> *mut serial_private;
    pub fn pciserial_remove_ports(priv_: *mut serial_private);
    pub fn pciserial_suspend_ports(priv_: *mut serial_private);
    pub fn pciserial_resume_ports(priv_: *mut serial_private);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
