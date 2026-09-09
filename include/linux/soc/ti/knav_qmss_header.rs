/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Keystone Navigator Queue Management Sub-System header
 *
 * Copyright (C) 2014 Texas Instruments Incorporated - https://www.ti.com
 * Author: Sandeep Nair <sandeep_n@ti.com>
 *         Cyril Chemparathy <cyril@ti.com>
 *         Santosh Shilimkar <santosh.shilimkar@ti.com>
 */

// C dependencies: linux/err.h, linux/time.h, linux/atomic.h,
// linux/device.h, linux/fcntl.h, and linux/dma-mapping.h.

/* queue types */
pub const KNAV_QUEUE_QPEND: u32 = u32::MAX - 1; /* interruptible qpend queue */
pub const KNAV_QUEUE_ACC: u32 = u32::MAX - 2; /* Accumulated queue */
pub const KNAV_QUEUE_GP: u32 = u32::MAX - 3; /* General purpose queue */

/* queue flags */
pub const KNAV_QUEUE_SHARED: u32 = 0x0001; /* Queue can be shared */

/**
 * enum knav_queue_ctrl_cmd - queue operations.
 * @KNAV_QUEUE_GET_ID:       Get the ID number for an open queue
 * @KNAV_QUEUE_FLUSH:        forcibly empty a queue if possible
 * @KNAV_QUEUE_SET_NOTIFIER: Set a notifier callback to a queue handle.
 * @KNAV_QUEUE_ENABLE_NOTIFY: Enable notifier callback for a queue handle.
 * @KNAV_QUEUE_DISABLE_NOTIFY: Disable notifier callback for a queue handle.
 * @KNAV_QUEUE_GET_COUNT:    Get number of queues.
 */
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum knav_queue_ctrl_cmd {
    KNAV_QUEUE_GET_ID,
    KNAV_QUEUE_FLUSH,
    KNAV_QUEUE_SET_NOTIFIER,
    KNAV_QUEUE_ENABLE_NOTIFY,
    KNAV_QUEUE_DISABLE_NOTIFY,
    KNAV_QUEUE_GET_COUNT,
}

/* Queue notifier callback prototype */
pub type knav_queue_notify_fn = unsafe extern "C" fn(arg: *mut core::ffi::c_void);

/**
 * struct knav_queue_notify_config: Notifier configuration
 * @fn:                     Notifier function
 * @fn_arg:                 Notifier function arguments
 */
#[repr(C)]
pub struct knav_queue_notify_config {
    pub fn_: knav_queue_notify_fn,
    pub fn_arg: *mut core::ffi::c_void,
}

extern "C" {
    pub fn knav_queue_open(
        name: *const core::ffi::c_char,
        id: u32,
        flags: u32,
    ) -> *mut core::ffi::c_void;
    pub fn knav_queue_close(qhandle: *mut core::ffi::c_void);
    pub fn knav_queue_device_control(
        qhandle: *mut core::ffi::c_void,
        cmd: knav_queue_ctrl_cmd,
        arg: core::ffi::c_ulong,
    ) -> core::ffi::c_int;
    pub fn knav_queue_pop(
        qhandle: *mut core::ffi::c_void,
        size: *mut u32,
    ) -> dma_addr_t;
    pub fn knav_queue_push(
        qhandle: *mut core::ffi::c_void,
        dma: dma_addr_t,
        size: u32,
        flags: u32,
    ) -> core::ffi::c_int;

    pub fn knav_pool_create(
        name: *const core::ffi::c_char,
        num_desc: core::ffi::c_int,
        region_id: core::ffi::c_int,
    ) -> *mut core::ffi::c_void;
    pub fn knav_pool_destroy(ph: *mut core::ffi::c_void);
    pub fn knav_pool_count(ph: *mut core::ffi::c_void) -> core::ffi::c_int;
    pub fn knav_pool_desc_get(ph: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    pub fn knav_pool_desc_put(ph: *mut core::ffi::c_void, desc: *mut core::ffi::c_void);
    pub fn knav_pool_desc_map(
        ph: *mut core::ffi::c_void,
        desc: *mut core::ffi::c_void,
        size: u32,
        dma: *mut dma_addr_t,
        dma_sz: *mut u32,
    ) -> core::ffi::c_int;
    pub fn knav_pool_desc_unmap(
        ph: *mut core::ffi::c_void,
        dma: dma_addr_t,
        dma_sz: u32,
    ) -> *mut core::ffi::c_void;
    pub fn knav_pool_desc_virt_to_dma(
        ph: *mut core::ffi::c_void,
        virt: *mut core::ffi::c_void,
    ) -> dma_addr_t;
    pub fn knav_pool_desc_dma_to_virt(
        ph: *mut core::ffi::c_void,
        dma: dma_addr_t,
    ) -> *mut core::ffi::c_void;
    pub fn knav_qmss_device_ready() -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
