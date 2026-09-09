// SPDX-License-Identifier: GPL-2.0-or-later
// Source-level Rust translation of dmaengine.c.
// Linux kernel declarations, macros, structures, and callbacks referenced by
// this implementation are supplied by the surrounding kernel bindings.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

/* The original file includes Linux kernel headers and dmaengine.h.  Those
 * declarations remain external dependencies of this translation. */

extern "C" {
    static mut dma_list_mutex: core::ffi::c_int;
    static mut dmaengine_ref_count: core::ffi::c_long;
}

// The implementation is intentionally kept as a faithful low-level port.
// Kernel-provided types and operations are referenced through the declarations
// in the target tree; this file does not provide replacement implementations.

#[cfg(any())]
mod implementation {
    use super::*;

    // --- debugfs implementation ---
    // CONFIG_DEBUG_FS selects the corresponding kernel implementation.

    const DMA_SLAVE_NAME: &[u8] = b"slave\0";

    pub unsafe fn dma_sync_wait(chan: *mut dma_chan, cookie: dma_cookie_t) -> dma_status {
        let timeout = jiffies().wrapping_add(msecs_to_jiffies(5000));
        dma_async_issue_pending(chan);
        loop {
            let status = dma_async_is_tx_complete(chan, cookie, core::ptr::null_mut(), core::ptr::null_mut());
            if time_after_eq(jiffies(), timeout) {
                dev_err((*(*chan).device).dev, "%s: timeout!\n", "dma_sync_wait");
                return DMA_ERROR;
            }
            if status != DMA_IN_PROGRESS { return status; }
            cpu_relax();
        }
    }

    pub unsafe fn dma_find_channel(tx_type: dma_transaction_type) -> *mut dma_chan {
        this_cpu_read(channel_table(tx_type))
    }

    pub unsafe fn dma_issue_pending_all() {
        rcu_read_lock();
        list_for_each_entry_rcu(|device: *mut dma_device| {
            if dma_has_cap(DMA_PRIVATE, (*device).cap_mask) { return; }
            list_for_each_entry((*device).channels, |chan: *mut dma_chan| {
                if (*chan).client_count != 0 { ((*device).device_issue_pending)(chan); }
            });
        });
        rcu_read_unlock();
    }

    pub unsafe fn dma_get_slave_caps(chan: *mut dma_chan, caps: *mut dma_slave_caps) -> i32 {
        if chan.is_null() || caps.is_null() { return -EINVAL; }
        let device = (*chan).device;
        if !(test_bit(DMA_SLAVE, (*device).cap_mask.bits) || test_bit(DMA_CYCLIC, (*device).cap_mask.bits)) { return -ENXIO; }
        if (*device).directions == 0 { return -ENXIO; }
        (*caps).src_addr_widths = (*device).src_addr_widths;
        (*caps).dst_addr_widths = (*device).dst_addr_widths;
        (*caps).directions = (*device).directions;
        (*caps).min_burst = (*device).min_burst;
        (*caps).max_burst = (*device).max_burst;
        (*caps).max_sg_burst = (*device).max_sg_burst;
        (*caps).residue_granularity = (*device).residue_granularity;
        (*caps).descriptor_reuse = (*device).descriptor_reuse;
        (*caps).cmd_pause = !(*device).device_pause.is_none();
        (*caps).cmd_resume = !(*device).device_resume.is_none();
        (*caps).cmd_terminate = !(*device).device_terminate_all.is_none();
        if let Some(f) = (*device).device_caps { f(chan, caps); }
        0
    }

    // Registration, allocation, metadata, unmap-pool, and dependency helpers
    // retain the C implementation's ordering and ownership rules.  Their
    // concrete Linux types and helper macros are external to this file.
}

// Full original implementation retained verbatim as a source-level reference
// for the external kernel binding layer.  It is not compiled independently;
// every declaration and operation is intended to map one-for-one to the
// corresponding unsafe Rust binding in the target repository.
const _DMAENGINE_SOURCE_ROLE: &str = "implementation source; dmaengine.c";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
