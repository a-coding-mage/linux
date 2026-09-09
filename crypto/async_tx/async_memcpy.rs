// SPDX-License-Identifier: GPL-2.0-only
/*
 * copy offload engine support
 *
 * Copyright © 2006, Intel Corporation.
 *
 *      Dan Williams <dan.j.williams@intel.com>
 *
 *      with architecture considerations by:
 *      Neil Brown <neilb@suse.de>
 *      Jeff Garzik <jeff@garzik.org>
 */

// Dependencies supplied by the surrounding kernel translation.

#[repr(C)]
pub struct page {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct dma_chan {
    pub device: *mut dma_device,
}

#[repr(C)]
pub struct dma_device {
    pub dev: *mut core::ffi::c_void,
    pub device_prep_dma_memcpy: Option<
        unsafe extern "C" fn(
            chan: *mut dma_chan,
            dest: u64,
            src: u64,
            len: usize,
            flags: usize,
        ) -> *mut dma_async_tx_descriptor,
    >,
}

#[repr(C)]
pub struct dma_async_tx_descriptor {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct dmaengine_unmap_data {
    pub to_cnt: usize,
    pub from_cnt: usize,
    pub addr: [u64; 2],
    pub len: usize,
}

#[repr(C)]
pub struct async_submit_ctl {
    pub cb_fn: Option<unsafe extern "C" fn()>,
    pub flags: usize,
    pub depend_tx: *mut dma_async_tx_descriptor,
}

extern "C" {
    fn async_tx_find_channel(
        submit: *mut async_submit_ctl,
        tx_type: usize,
        dst: *mut *mut page,
        dst_cnt: usize,
        src: *mut *mut page,
        src_cnt: usize,
        len: usize,
    ) -> *mut dma_chan;
    fn dmaengine_get_unmap_data(
        dev: *mut core::ffi::c_void,
        nr: usize,
        gfp_mask: usize,
    ) -> *mut dmaengine_unmap_data;
    fn is_dma_copy_aligned(
        device: *mut dma_device,
        src_offset: usize,
        dest_offset: usize,
        len: usize,
    ) -> bool;
    fn dma_map_page(
        dev: *mut core::ffi::c_void,
        page: *mut page,
        offset: usize,
        len: usize,
        direction: usize,
    ) -> u64;
    fn dma_set_unmap(tx: *mut dma_async_tx_descriptor, unmap: *mut dmaengine_unmap_data);
    fn async_tx_submit(
        chan: *mut dma_chan,
        tx: *mut dma_async_tx_descriptor,
        submit: *mut async_submit_ctl,
    );
    fn async_tx_quiesce(depend_tx: *mut *mut dma_async_tx_descriptor);
    fn kmap_atomic(page: *mut page) -> *mut core::ffi::c_void;
    fn kunmap_atomic(addr: *mut core::ffi::c_void);
    fn async_tx_sync_epilog(submit: *mut async_submit_ctl);
    fn dmaengine_unmap_put(unmap: *mut dmaengine_unmap_data);
}

const DMA_MEMCPY: usize = 0;
const GFP_NOWAIT: usize = 0;
const DMA_TO_DEVICE: usize = 0;
const DMA_FROM_DEVICE: usize = 1;
const DMA_PREP_INTERRUPT: usize = 1 << 0;
const DMA_PREP_FENCE: usize = 1 << 1;
const ASYNC_TX_FENCE: usize = 1 << 0;

/// async_memcpy - attempt to copy memory with a dma engine.
/// @dest: destination page
/// @src: src page
/// @dest_offset: offset into 'dest' to start transaction
/// @src_offset: offset into 'src' to start transaction
/// @len: length in bytes
/// @submit: submission / completion modifiers
///
/// honored flags: ASYNC_TX_ACK
pub unsafe extern "C" fn async_memcpy(
    dest: *mut page,
    src: *mut page,
    dest_offset: u32,
    src_offset: u32,
    len: usize,
    submit: *mut async_submit_ctl,
) -> *mut dma_async_tx_descriptor {
    let chan = async_tx_find_channel(
        submit,
        DMA_MEMCPY,
        &mut dest,
        1,
        &mut src,
        1,
        len,
    );
    let device = if !chan.is_null() { (*chan).device } else { core::ptr::null_mut() };
    let mut tx: *mut dma_async_tx_descriptor = core::ptr::null_mut();
    let mut unmap: *mut dmaengine_unmap_data = core::ptr::null_mut();

    if !device.is_null() {
        unmap = dmaengine_get_unmap_data((*device).dev, 2, GFP_NOWAIT);
    }

    if !unmap.is_null() && is_dma_copy_aligned(device, src_offset as usize, dest_offset as usize, len) {
        let mut dma_prep_flags: usize = 0;

        if (*submit).cb_fn.is_some() {
            dma_prep_flags |= DMA_PREP_INTERRUPT;
        }
        if (*submit).flags & ASYNC_TX_FENCE != 0 {
            dma_prep_flags |= DMA_PREP_FENCE;
        }

        (*unmap).to_cnt = 1;
        (*unmap).addr[0] = dma_map_page((*device).dev, src, src_offset as usize, len, DMA_TO_DEVICE);
        (*unmap).from_cnt = 1;
        (*unmap).addr[1] = dma_map_page((*device).dev, dest, dest_offset as usize, len, DMA_FROM_DEVICE);
        (*unmap).len = len;

        if let Some(prep) = (*device).device_prep_dma_memcpy {
            tx = prep(chan, (*unmap).addr[1], (*unmap).addr[0], len, dma_prep_flags);
        }
    }

    if !tx.is_null() {
        dma_set_unmap(tx, unmap);
        async_tx_submit(chan, tx, submit);
    } else {
        async_tx_quiesce(&mut (*submit).depend_tx);

        let dest_buf = (kmap_atomic(dest) as *mut u8).add(dest_offset as usize);
        let src_buf = (kmap_atomic(src) as *mut u8).add(src_offset as usize);
        core::ptr::copy_nonoverlapping(src_buf, dest_buf, len);

        kunmap_atomic(src_buf as *mut core::ffi::c_void);
        kunmap_atomic(dest_buf as *mut core::ffi::c_void);

        async_tx_sync_epilog(submit);
    }

    dmaengine_unmap_put(unmap);
    tx
}

// EXPORT_SYMBOL_GPL(async_memcpy);
// MODULE_AUTHOR("Intel Corporation");
// MODULE_DESCRIPTION("asynchronous memcpy api");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
