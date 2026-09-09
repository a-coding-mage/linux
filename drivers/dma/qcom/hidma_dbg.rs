// SPDX-License-Identifier: GPL-2.0-only
/*
 * Qualcomm Technologies HIDMA debug file
 *
 * Copyright (c) 2015-2016, The Linux Foundation. All rights reserved.
 */

use crate::hidma::*;
use core::ffi::{c_char, c_int, c_uint, c_void};

// Linux-kernel and HIDMA declarations supplied by the surrounding translation.
extern "C" {
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn seq_printf(s: *mut seq_file, fmt: *const c_char, ...);
    fn seq_puts(s: *mut seq_file, text: *const c_char);
    fn atomic_read(v: *const atomic_t) -> c_int;
    fn pm_runtime_get_sync(dev: *mut device) -> c_int;
    fn pm_runtime_mark_last_busy(dev: *mut device);
    fn pm_runtime_put_autosuspend(dev: *mut device) -> c_int;
    fn debugfs_create_dir(name: *const c_char, parent: *mut dentry) -> *mut dentry;
    fn debugfs_create_file(name: *const c_char, mode: c_uint, parent: *mut dentry,
                           data: *mut c_void, fops: *const file_operations) -> *mut dentry;
    fn debugfs_remove_recursive(dentry: *mut dentry);
    fn dev_name(dev: *mut device) -> *const c_char;
    fn resource_size(resource: *const resource) -> resource_size_t;
    fn sprintf(buf: *mut c_char, fmt: *const c_char, ... ) -> c_int;
}

extern "C" {
    static hidma_chan_fops: file_operations;
    static hidma_dma_fops: file_operations;
}

unsafe fn hidma_ll_chstats(s: *mut seq_file, llhndl: *mut c_void, tre_ch: u32) {
    let lldev = llhndl as *mut hidma_lldev;
    let tre: *mut hidma_tre;
    let mut length: u32;
    let mut src_start: dma_addr_t;
    let mut dest_start: dma_addr_t;
    let tre_local: *mut u32;

    if tre_ch >= (*lldev).nr_tres {
        dev_err((*lldev).dev, b"invalid TRE number in chstats:%d\0".as_ptr() as *const c_char, tre_ch);
        return;
    }
    tre = (*lldev).trepool.add(tre_ch as usize);
    seq_printf(s, b"------Channel %d -----\n\0".as_ptr() as *const c_char, tre_ch);
    seq_printf(s, b"allocated=%d\n\0".as_ptr() as *const c_char, atomic_read(&(*tre).allocated));
    seq_printf(s, b"queued = 0x%x\n\0".as_ptr() as *const c_char, (*tre).queued);
    seq_printf(s, b"err_info = 0x%x\n\0".as_ptr() as *const c_char, (*tre).err_info);
    seq_printf(s, b"err_code = 0x%x\n\0".as_ptr() as *const c_char, (*tre).err_code);
    seq_printf(s, b"status = 0x%x\n\0".as_ptr() as *const c_char, (*tre).status);
    seq_printf(s, b"idx = 0x%x\n\0".as_ptr() as *const c_char, (*tre).idx);
    seq_printf(s, b"dma_sig = 0x%x\n\0".as_ptr() as *const c_char, (*tre).dma_sig);
    seq_printf(s, b"dev_name=%s\n\0".as_ptr() as *const c_char, (*tre).dev_name);
    seq_printf(s, b"callback=%p\n\0".as_ptr() as *const c_char, (*tre).callback);
    seq_printf(s, b"data=%p\n\0".as_ptr() as *const c_char, (*tre).data);
    seq_printf(s, b"tre_index = 0x%x\n\0".as_ptr() as *const c_char, (*tre).tre_index);

    tre_local = (*tre).tre_local;
    src_start = *tre_local.add(HIDMA_TRE_SRC_LOW_IDX as usize) as dma_addr_t;
    src_start = ((*tre_local.add(HIDMA_TRE_SRC_HI_IDX as usize) as u64) << 32) + src_start;
    dest_start = *tre_local.add(HIDMA_TRE_DEST_LOW_IDX as usize) as dma_addr_t;
    dest_start += (*tre_local.add(HIDMA_TRE_DEST_HI_IDX as usize) as u64) << 32;
    length = *tre_local.add(HIDMA_TRE_LEN_IDX as usize);
    seq_printf(s, b"src=%pap\n\0".as_ptr() as *const c_char, &src_start);
    seq_printf(s, b"dest=%pap\n\0".as_ptr() as *const c_char, &dest_start);
    seq_printf(s, b"length = 0x%x\n\0".as_ptr() as *const c_char, length);
}

unsafe fn hidma_ll_devstats(s: *mut seq_file, llhndl: *mut c_void) {
    let lldev = llhndl as *mut hidma_lldev;
    seq_puts(s, b"------Device -----\n\0".as_ptr() as *const c_char);
    seq_printf(s, b"lldev init = 0x%x\n\0".as_ptr() as *const c_char, (*lldev).initialized);
    seq_printf(s, b"trch_state = 0x%x\n\0".as_ptr() as *const c_char, (*lldev).trch_state);
    seq_printf(s, b"evch_state = 0x%x\n\0".as_ptr() as *const c_char, (*lldev).evch_state);
    seq_printf(s, b"chidx = 0x%x\n\0".as_ptr() as *const c_char, (*lldev).chidx);
    seq_printf(s, b"nr_tres = 0x%x\n\0".as_ptr() as *const c_char, (*lldev).nr_tres);
    seq_printf(s, b"trca=%p\n\0".as_ptr() as *const c_char, (*lldev).trca);
    seq_printf(s, b"tre_ring=%p\n\0".as_ptr() as *const c_char, (*lldev).tre_ring);
    seq_printf(s, b"tre_ring_handle=%pap\n\0".as_ptr() as *const c_char, &(*lldev).tre_dma);
    seq_printf(s, b"tre_ring_size = 0x%x\n\0".as_ptr() as *const c_char, (*lldev).tre_ring_size);
    seq_printf(s, b"tre_processed_off = 0x%x\n\0".as_ptr() as *const c_char, (*lldev).tre_processed_off);
    seq_printf(s, b"pending_tre_count=%d\n\0".as_ptr() as *const c_char, atomic_read(&(*lldev).pending_tre_count));
    seq_printf(s, b"evca=%p\n\0".as_ptr() as *const c_char, (*lldev).evca);
    seq_printf(s, b"evre_ring=%p\n\0".as_ptr() as *const c_char, (*lldev).evre_ring);
    seq_printf(s, b"evre_ring_handle=%pap\n\0".as_ptr() as *const c_char, &(*lldev).evre_dma);
    seq_printf(s, b"evre_ring_size = 0x%x\n\0".as_ptr() as *const c_char, (*lldev).evre_ring_size);
    seq_printf(s, b"evre_processed_off = 0x%x\n\0".as_ptr() as *const c_char, (*lldev).evre_processed_off);
    seq_printf(s, b"tre_write_offset = 0x%x\n\0".as_ptr() as *const c_char, (*lldev).tre_write_offset);
}

unsafe fn hidma_chan_show(s: *mut seq_file, _unused: *mut c_void) -> c_int {
    let mchan = (*s).private as *mut hidma_chan;
    let dmadev = (*mchan).dmadev;
    pm_runtime_get_sync((*dmadev).ddev.dev);
    seq_printf(s, b"paused=%u\n\0".as_ptr() as *const c_char, (*mchan).paused);
    seq_printf(s, b"dma_sig=%u\n\0".as_ptr() as *const c_char, (*mchan).dma_sig);
    seq_puts(s, b"prepared\n\0".as_ptr() as *const c_char);
    // list_for_each_entry(mdesc, &mchan->prepared, node)
    //     hidma_ll_chstats(s, mchan->dmadev->lldev, mdesc->tre_ch);
    seq_puts(s, b"active\n\0".as_ptr() as *const c_char);
    // list_for_each_entry(mdesc, &mchan->active, node)
    //     hidma_ll_chstats(s, mchan->dmadev->lldev, mdesc->tre_ch);
    seq_puts(s, b"completed\n\0".as_ptr() as *const c_char);
    // list_for_each_entry(mdesc, &mchan->completed, node)
    //     hidma_ll_chstats(s, mchan->dmadev->lldev, mdesc->tre_ch);
    hidma_ll_devstats(s, (*dmadev).lldev as *mut c_void);
    pm_runtime_mark_last_busy((*dmadev).ddev.dev);
    pm_runtime_put_autosuspend((*dmadev).ddev.dev);
    0
}

unsafe fn hidma_dma_show(s: *mut seq_file, _unused: *mut c_void) -> c_int {
    let dmadev = (*s).private as *mut hidma_dev;
    let mut sz: resource_size_t;
    seq_printf(s, b"nr_descriptors=%d\n\0".as_ptr() as *const c_char, (*dmadev).nr_descriptors);
    seq_printf(s, b"dev_trca=%p\n\0".as_ptr() as *const c_char, &(*dmadev).dev_trca);
    seq_printf(s, b"dev_trca_phys=%pa\n\0".as_ptr() as *const c_char, &(*(*dmadev).trca_resource).start);
    sz = resource_size((*dmadev).trca_resource);
    seq_printf(s, b"dev_trca_size=%pa\n\0".as_ptr() as *const c_char, &sz);
    seq_printf(s, b"dev_evca=%p\n\0".as_ptr() as *const c_char, &(*dmadev).dev_evca);
    seq_printf(s, b"dev_evca_phys=%pa\n\0".as_ptr() as *const c_char, &(*(*dmadev).evca_resource).start);
    sz = resource_size((*dmadev).evca_resource);
    seq_printf(s, b"dev_evca_size=%pa\n\0".as_ptr() as *const c_char, &sz);
    0
}

// DEFINE_SHOW_ATTRIBUTE(hidma_chan);
// DEFINE_SHOW_ATTRIBUTE(hidma_dma);

pub unsafe fn hidma_debug_uninit(dmadev: *mut hidma_dev) {
    debugfs_remove_recursive((*dmadev).debugfs);
}

pub unsafe fn hidma_debug_init(dmadev: *mut hidma_dev) {
    let mut chidx: c_int = 0;
    let mut position: *mut list_head = core::ptr::null_mut();
    let mut dir: *mut dentry;
    (*dmadev).debugfs = debugfs_create_dir(dev_name((*dmadev).ddev.dev), core::ptr::null_mut());

    /* walk through the virtual channel list */
    // list_for_each(position, &dmadev->ddev.channels) {
    //     struct hidma_chan *chan;
    //     chan = list_entry(position, struct hidma_chan, chan.device_node);
    //     sprintf(chan->dbg_name, "chan%d", chidx);
    //     dir = debugfs_create_dir(chan->dbg_name, dmadev->debugfs);
    //     debugfs_create_file("stats", S_IRUGO, dir, chan, &hidma_chan_fops);
    //     chidx++;
    // }
    let _ = (&mut position, &mut dir, &mut chidx);
    debugfs_create_file(b"stats\0".as_ptr() as *const c_char, S_IRUGO, (*dmadev).debugfs,
                        dmadev as *mut c_void, &hidma_dma_fops);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
