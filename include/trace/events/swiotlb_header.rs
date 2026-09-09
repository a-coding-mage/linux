/* SPDX-License-Identifier: GPL-2.0 */

// TRACE_SYSTEM swiotlb
//
// The C tracepoint machinery and trace/define_trace.h are supplied by the
// surrounding kernel translation.  This file preserves the event payload,
// assignment order, and print format of TRACE_EVENT(swiotlb_bounced).

use core::ffi::{c_char, c_void};

#[repr(C)]
pub struct Device {
    pub dma_mask: *const u64,
}

extern "C" {
    pub fn dev_name(dev: *const Device) -> *const c_char;
    pub fn is_swiotlb_force_bounce(dev: *const Device) -> bool;
}

#[repr(C)]
pub struct SwiOTLBouncedEntry {
    // __string(dev_name, dev_name(dev))
    pub dev_name: *const c_char,
    pub dma_mask: u64,
    pub dev_addr: usize,
    pub size: usize,
    pub force: bool,
}

impl SwiOTLBouncedEntry {
    #[inline]
    pub unsafe fn assign(
        &mut self,
        dev: *const Device,
        dev_addr: usize,
        size: usize,
    ) {
        // __assign_str(dev_name)
        self.dev_name = dev_name(dev);
        self.dma_mask = if !(*dev).dma_mask.is_null() {
            *(*dev).dma_mask
        } else {
            0
        };
        self.dev_addr = dev_addr;
        self.size = size;
        self.force = is_swiotlb_force_bounce(dev);
    }
}

// TP_PROTO(struct device *dev, dma_addr_t dev_addr, size_t size)
// TP_ARGS(dev, dev_addr, size)
// TP_printk("dev_name: %s dma_mask=%llx dev_addr=%llx size=%zu %s",
//     __get_str(dev_name), __entry->dma_mask,
//     (unsigned long long)__entry->dev_addr, __entry->size,
//     __entry->force ? "FORCE" : "NORMAL")
#[allow(dead_code)]
pub const SWIOTLB_BOUNCED_PRINTK_FORMAT: &[u8] =
    b"dev_name: %s dma_mask=%llx dev_addr=%llx size=%zu %s\0";

// The generated tracepoint declaration/registration is intentionally left to
// the external tracepoint implementation, corresponding to TRACE_EVENT.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
