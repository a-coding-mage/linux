/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright © 2006, Intel Corporation. */

/* Translated from async_tx.h.  Dependencies are supplied by the surrounding kernel bindings. */

/* On architectures without dma-mapping capabilities, __async_inline is __always_inline. */

#[repr(C)]
pub struct dma_chan_ref {
    pub chan: *mut dma_chan,
    pub node: list_head,
    pub rcu: rcu_head,
    pub count: atomic_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum async_tx_flags {
    ASYNC_TX_XOR_ZERO_DST = 1 << 0,
    ASYNC_TX_XOR_DROP_DST = 1 << 1,
    ASYNC_TX_ACK = 1 << 2,
    ASYNC_TX_FENCE = 1 << 3,
    ASYNC_TX_PQ_XOR_DST = 1 << 4,
}

#[repr(C)]
pub struct async_submit_ctl {
    pub flags: async_tx_flags,
    pub depend_tx: *mut dma_async_tx_descriptor,
    pub cb_fn: dma_async_tx_callback,
    pub cb_param: *mut core::ffi::c_void,
    pub scribble: *mut core::ffi::c_void,
}

#[cfg(all(CONFIG_DMA_ENGINE, not(CONFIG_ASYNC_TX_CHANNEL_SWITCH)))]
pub use dma_issue_pending_all as async_tx_issue_pending_all;

#[cfg(all(CONFIG_DMA_ENGINE, not(CONFIG_ASYNC_TX_CHANNEL_SWITCH)))]
#[inline]
pub unsafe fn async_tx_issue_pending(tx: *mut dma_async_tx_descriptor) {
    if !tx.is_null() {
        let chan = (*tx).chan;
        let dma = (*chan).device;
        ((*dma).device_issue_pending)(chan);
    }
}

/* CONFIG_ARCH_HAS_ASYNC_TX_FIND_CHANNEL supplies the architecture-specific form. */
#[cfg(all(CONFIG_DMA_ENGINE, not(CONFIG_ASYNC_TX_CHANNEL_SWITCH), not(CONFIG_ARCH_HAS_ASYNC_TX_FIND_CHANNEL)))]
pub unsafe fn __async_tx_find_channel(
    submit: *mut async_submit_ctl,
    tx_type: dma_transaction_type,
) -> *mut dma_chan;

#[cfg(all(CONFIG_DMA_ENGINE, not(CONFIG_ASYNC_TX_CHANNEL_SWITCH), not(CONFIG_ARCH_HAS_ASYNC_TX_FIND_CHANNEL)))]
#[inline]
pub unsafe fn async_tx_find_channel(
    dep: *mut async_submit_ctl,
    tx_type: dma_transaction_type,
    _dst: *mut *mut page,
    _dst_count: i32,
    _src: *mut *mut page,
    _src_count: i32,
    _len: usize,
) -> *mut dma_chan {
    __async_tx_find_channel(dep, tx_type)
}

#[cfg(any(not(CONFIG_DMA_ENGINE), CONFIG_ASYNC_TX_CHANNEL_SWITCH))]
#[inline]
pub unsafe fn async_tx_issue_pending_all() {}

#[cfg(any(not(CONFIG_DMA_ENGINE), CONFIG_ASYNC_TX_CHANNEL_SWITCH))]
#[inline]
pub unsafe fn async_tx_issue_pending(_tx: *mut dma_async_tx_descriptor) {}

#[cfg(any(not(CONFIG_DMA_ENGINE), CONFIG_ASYNC_TX_CHANNEL_SWITCH))]
#[inline]
pub unsafe fn async_tx_find_channel(
    _submit: *mut async_submit_ctl,
    _tx_type: dma_transaction_type,
    _dst: *mut *mut page,
    _dst_count: i32,
    _src: *mut *mut page,
    _src_count: i32,
    _len: usize,
) -> *mut dma_chan {
    core::ptr::null_mut()
}

#[inline]
pub unsafe fn async_tx_sync_epilog(submit: *mut async_submit_ctl) {
    if !(*submit).cb_fn.is_none() {
        ((*submit).cb_fn.unwrap())((*submit).cb_param);
    }
}

#[repr(C)]
pub union addr_conv_t {
    pub addr: libc::c_ulong,
    pub page: *mut page,
    pub dma: dma_addr_t,
}

#[inline]
pub unsafe fn init_async_submit(
    args: *mut async_submit_ctl,
    flags: async_tx_flags,
    tx: *mut dma_async_tx_descriptor,
    cb_fn: dma_async_tx_callback,
    cb_param: *mut core::ffi::c_void,
    scribble: *mut addr_conv_t,
) {
    (*args).flags = flags;
    (*args).depend_tx = tx;
    (*args).cb_fn = cb_fn;
    (*args).cb_param = cb_param;
    (*args).scribble = scribble.cast();
}

extern "C" {
    pub fn async_tx_submit(chan: *mut dma_chan, tx: *mut dma_async_tx_descriptor, submit: *mut async_submit_ctl);
    pub fn async_xor(dest: *mut page, src_list: *mut *mut page, offset: u32, src_cnt: i32, len: usize, submit: *mut async_submit_ctl) -> *mut dma_async_tx_descriptor;
    pub fn async_xor_offs(dest: *mut page, offset: u32, src_list: *mut *mut page, src_offset: *mut u32, src_cnt: i32, len: usize, submit: *mut async_submit_ctl) -> *mut dma_async_tx_descriptor;
    pub fn async_xor_val_offs(dest: *mut page, offset: u32, src_list: *mut *mut page, src_offset: *mut u32, src_cnt: i32, len: usize, result: *mut sum_check_flags, submit: *mut async_submit_ctl) -> *mut dma_async_tx_descriptor;
    pub fn async_memcpy(dest: *mut page, src: *mut page, dest_offset: u32, src_offset: u32, len: usize, submit: *mut async_submit_ctl) -> *mut dma_async_tx_descriptor;
    pub fn async_trigger_callback(submit: *mut async_submit_ctl) -> *mut dma_async_tx_descriptor;
    pub fn async_gen_syndrome(blocks: *mut *mut page, offsets: *mut u32, src_cnt: i32, len: usize, submit: *mut async_submit_ctl) -> *mut dma_async_tx_descriptor;
    pub fn async_syndrome_val(blocks: *mut *mut page, offsets: *mut u32, src_cnt: i32, len: usize, pqres: *mut sum_check_flags, spare: *mut page, s_off: u32, submit: *mut async_submit_ctl) -> *mut dma_async_tx_descriptor;
    pub fn async_raid6_2data_recov(src_num: i32, bytes: usize, faila: i32, failb: i32, ptrs: *mut *mut page, offs: *mut u32, submit: *mut async_submit_ctl) -> *mut dma_async_tx_descriptor;
    pub fn async_raid6_datap_recov(src_num: i32, bytes: usize, faila: i32, ptrs: *mut *mut page, offs: *mut u32, submit: *mut async_submit_ctl) -> *mut dma_async_tx_descriptor;
    pub fn async_tx_quiesce(tx: *mut *mut dma_async_tx_descriptor);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
