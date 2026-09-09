/* SPDX-License-Identifier: GPL-2.0 */
/*
 * SH7760 DMABRG (USB/Audio) support
 */

/* IRQ sources */
pub const DMABRGIRQ_USBDMA: u32 = 0;
pub const DMABRGIRQ_USBDMAERR: u32 = 1;
pub const DMABRGIRQ_A0TXF: u32 = 2;
pub const DMABRGIRQ_A0TXH: u32 = 3;
pub const DMABRGIRQ_A0RXF: u32 = 4;
pub const DMABRGIRQ_A0RXH: u32 = 5;
pub const DMABRGIRQ_A1TXF: u32 = 6;
pub const DMABRGIRQ_A1TXH: u32 = 7;
pub const DMABRGIRQ_A1RXF: u32 = 8;
pub const DMABRGIRQ_A1RXH: u32 = 9;

extern "C" {
    pub fn dmabrg_request_irq(
        irq: core::ffi::c_uint,
        handler: Option<unsafe extern "C" fn(*mut core::ffi::c_void)>,
        data: *mut core::ffi::c_void,
    ) -> core::ffi::c_int;
    pub fn dmabrg_free_irq(irq: core::ffi::c_uint);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
