/* SPDX-License-Identifier: GPL-2.0 */

// Translated from the C header `asm/ebus_dma.h`.

#[repr(C)]
pub struct ebus_dma_info {
    pub lock: spinlock_t,
    pub regs: *mut core::ffi::c_void,

    pub flags: core::ffi::c_uint,

    // These are only valid if EBUS_DMA_FLAG_USE_EBDMA_HANDLER is set.
    pub callback:
        Option<unsafe extern "C" fn(p: *mut ebus_dma_info, event: core::ffi::c_int, cookie: *mut core::ffi::c_void)>,
    pub client_cookie: *mut core::ffi::c_void,
    pub irq: core::ffi::c_uint,

    pub name: [core::ffi::c_uchar; 64],
}

pub const EBUS_DMA_FLAG_USE_EBDMA_HANDLER: core::ffi::c_uint = 0x0000_0001;
pub const EBUS_DMA_FLAG_TCI_DISABLE: core::ffi::c_uint = 0x0000_0002;

pub const EBUS_DMA_EVENT_ERROR: core::ffi::c_uint = 1;
pub const EBUS_DMA_EVENT_DMA: core::ffi::c_uint = 2;
pub const EBUS_DMA_EVENT_DEVICE: core::ffi::c_uint = 4;

unsafe extern "C" {
    pub fn ebus_dma_register(p: *mut ebus_dma_info) -> core::ffi::c_int;
    pub fn ebus_dma_irq_enable(p: *mut ebus_dma_info, on: core::ffi::c_int) -> core::ffi::c_int;
    pub fn ebus_dma_unregister(p: *mut ebus_dma_info);
    pub fn ebus_dma_request(
        p: *mut ebus_dma_info,
        bus_addr: dma_addr_t,
        len: usize,
    ) -> core::ffi::c_int;
    pub fn ebus_dma_prepare(p: *mut ebus_dma_info, write: core::ffi::c_int);
    pub fn ebus_dma_residue(p: *mut ebus_dma_info) -> core::ffi::c_uint;
    pub fn ebus_dma_addr(p: *mut ebus_dma_info) -> core::ffi::c_uint;
    pub fn ebus_dma_enable(p: *mut ebus_dma_info, on: core::ffi::c_int);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
