/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Qualcomm Technologies HIDMA data structures
 *
 * Copyright (c) 2014-2016, The Linux Foundation. All rights reserved.
 */

// Dependencies supplied by the surrounding kernel/Rust environment:
// kfifo, interrupt, dmaengine, atomic_t, spinlock_t, tasklet_struct, and
// the other referenced kernel types are intentionally not defined here.

pub const HIDMA_TRE_SIZE: usize = 32; /* each TRE is 32 bytes */
pub const HIDMA_TRE_CFG_IDX: usize = 0;
pub const HIDMA_TRE_LEN_IDX: usize = 1;
pub const HIDMA_TRE_SRC_LOW_IDX: usize = 2;
pub const HIDMA_TRE_SRC_HI_IDX: usize = 3;
pub const HIDMA_TRE_DEST_LOW_IDX: usize = 4;
pub const HIDMA_TRE_DEST_HI_IDX: usize = 5;

#[repr(C)]
pub enum tre_type {
    HIDMA_TRE_MEMCPY = 3,
    HIDMA_TRE_MEMSET = 4,
}

#[repr(C)]
pub struct hidma_tre {
    pub allocated: atomic_t, /* if this channel is allocated */
    pub queued: bool, /* flag whether this is pending */
    pub status: u16, /* status */
    pub idx: u32, /* index of the tre */
    pub dma_sig: u32, /* signature of the tre */
    pub dev_name: *const core::ffi::c_char, /* name of the device */
    pub callback: Option<unsafe extern "C" fn(*mut core::ffi::c_void)>, /* requester callback */
    pub data: *mut core::ffi::c_void, /* Data associated with this channel */
    pub lldev: *mut hidma_lldev, /* lldma device pointer */
    pub tre_local: [u32; HIDMA_TRE_SIZE / core::mem::size_of::<u32>() + 1], /* TRE local copy */
    pub tre_index: u32, /* the offset where this was written */
    pub int_flags: u32, /* interrupt flags */
    pub err_info: u8, /* error record in this transfer */
    pub err_code: u8, /* completion code */
}

#[repr(C)]
pub struct hidma_lldev {
    pub msi_support: bool,
    pub initialized: bool,
    pub trch_state: u8,
    pub evch_state: u8,
    pub chidx: u8,
    pub nr_tres: u32,
    pub lock: spinlock_t,
    pub trepool: *mut hidma_tre,
    pub dev: *mut device,
    pub trca: *mut core::ffi::c_void,
    pub evca: *mut core::ffi::c_void,
    pub pending_tre_list: *mut *mut hidma_tre,
    pub pending_tre_count: atomic_t,
    pub tre_ring: *mut core::ffi::c_void,
    pub tre_dma: dma_addr_t,
    pub tre_ring_size: u32,
    pub tre_processed_off: u32,
    pub evre_ring: *mut core::ffi::c_void,
    pub evre_dma: dma_addr_t,
    pub evre_ring_size: u32,
    pub evre_processed_off: u32,
    pub tre_write_offset: u32,
    pub task: tasklet_struct,
    // DECLARE_KFIFO_PTR(handoff_fifo, struct hidma_tre *)
    pub handoff_fifo: kfifo_ptr<*mut hidma_tre>,
}

#[repr(C)]
pub struct hidma_desc {
    pub desc: dma_async_tx_descriptor,
    /* link list node for this channel */
    pub node: list_head,
    pub tre_ch: u32,
}

#[repr(C)]
pub struct hidma_chan {
    pub paused: bool,
    pub allocated: bool,
    pub dbg_name: [core::ffi::c_char; 16],
    pub dma_sig: u32,
    pub last_success: dma_cookie_t,
    /*
     * active descriptor on this channel
     * It is used by the DMA complete notification to
     * locate the descriptor that initiated the transfer.
     */
    pub dmadev: *mut hidma_dev,
    pub running: *mut hidma_desc,
    pub chan: dma_chan,
    pub free: list_head,
    pub prepared: list_head,
    pub queued: list_head,
    pub active: list_head,
    pub completed: list_head,
    /* Lock for this structure */
    pub lock: spinlock_t,
}

#[repr(C)]
pub struct hidma_dev {
    pub irq: core::ffi::c_int,
    pub chidx: core::ffi::c_int,
    pub nr_descriptors: u32,
    pub msi_virqbase: core::ffi::c_int,
    pub lldev: *mut hidma_lldev,
    pub dev_trca: *mut core::ffi::c_void,
    pub trca_resource: *mut resource,
    pub dev_evca: *mut core::ffi::c_void,
    pub evca_resource: *mut resource,
    /* used to protect the pending channel list */
    pub lock: spinlock_t,
    pub ddev: dma_device,
    pub debugfs: *mut dentry,
    /* sysfs entry for the channel id */
    pub chid_attrs: *mut device_attribute,
    /* Task delivering issue_pending */
    pub task: tasklet_struct,
}

extern "C" {
    pub fn hidma_ll_request(
        llhndl: *mut hidma_lldev, dev_id: u32, dev_name: *const core::ffi::c_char,
        callback: Option<unsafe extern "C" fn(*mut core::ffi::c_void)>,
        data: *mut core::ffi::c_void, tre_ch: *mut u32,
    ) -> core::ffi::c_int;
    pub fn hidma_ll_free(llhndl: *mut hidma_lldev, tre_ch: u32);
    pub fn hidma_ll_status(llhndl: *mut hidma_lldev, tre_ch: u32) -> dma_status;
    pub fn hidma_ll_isenabled(llhndl: *mut hidma_lldev) -> bool;
    pub fn hidma_ll_queue_request(llhndl: *mut hidma_lldev, tre_ch: u32);
    pub fn hidma_ll_start(llhndl: *mut hidma_lldev);
    pub fn hidma_ll_disable(lldev: *mut hidma_lldev) -> core::ffi::c_int;
    pub fn hidma_ll_enable(llhndl: *mut hidma_lldev) -> core::ffi::c_int;
    pub fn hidma_ll_set_transfer_params(llhndl: *mut hidma_lldev, tre_ch: u32, src: dma_addr_t, dest: dma_addr_t, len: u32, flags: u32, txntype: u32);
    pub fn hidma_ll_setup_irq(lldev: *mut hidma_lldev, msi: bool);
    pub fn hidma_ll_setup(lldev: *mut hidma_lldev) -> core::ffi::c_int;
    pub fn hidma_ll_init(dev: *mut device, max_channels: u32, trca: *mut core::ffi::c_void, evca: *mut core::ffi::c_void, chidx: u8) -> *mut hidma_lldev;
    pub fn hidma_ll_uninit(llhndl: *mut hidma_lldev) -> core::ffi::c_int;
    pub fn hidma_ll_inthandler(irq: core::ffi::c_int, arg: *mut core::ffi::c_void) -> irqreturn_t;
    pub fn hidma_ll_inthandler_msi(irq: core::ffi::c_int, arg: *mut core::ffi::c_void, cause: core::ffi::c_int) -> irqreturn_t;
    pub fn hidma_cleanup_pending_tre(llhndl: *mut hidma_lldev, err_info: u8, err_code: u8);
    pub fn hidma_debug_init(dmadev: *mut hidma_dev);
    pub fn hidma_debug_uninit(dmadev: *mut hidma_dev);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
