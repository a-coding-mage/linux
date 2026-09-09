/* SPDX-License-Identifier: GPL-2.0-only OR MIT */
/*
 * Apple RTKit IPC Library
 * Copyright (C) The Asahi Linux Contributors
 *
 * Apple's SoCs come with various co-processors running their RTKit operating
 * system. This protocol library is used by client drivers to use the
 * features provided by them.
 */

use core::ffi::c_void;

/*
 * Struct to represent implementation-specific RTKit operations.
 *
 * @buffer:    Shared memory buffer allocated inside normal RAM.
 * @iomem:     Shared memory buffer controlled by the co-processors.
 * @size:      Size of the shared memory buffer.
 * @iova:      Device VA of shared memory buffer.
 * @is_mapped: Shared memory buffer is managed by the co-processor.
 * @private:   Private data pointer for the parent driver.
 */
#[repr(C)]
pub struct apple_rtkit_shmem {
    pub buffer: *mut c_void,
    pub iomem: *mut c_void,
    pub size: usize,
    pub iova: usize,
    pub is_mapped: bool,
    pub private: *mut c_void,
}

/*
 * Struct to represent implementation-specific RTKit operations.
 *
 * @crashed:       Called when the co-processor has crashed. Runs in process
 *                 context.
 * @recv_message:  Function called when a message from RTKit is received
 *                 on a non-system endpoint. Called from a worker thread.
 * @recv_message_early:
 *                 Like recv_message, but called from atomic context. It
 *                 should return true if it handled the message. If it
 *                 returns false, the message will be passed on to the
 *                 worker thread.
 * @shmem_setup:   Setup shared memory buffer. If bfr.is_iomem is true the
 *                 buffer is managed by the co-processor and needs to be mapped.
 *                 Otherwise the buffer is managed by Linux and needs to be
 *                 allocated. If not specified dma_alloc_coherent is used.
 *                 Called in process context.
 * @shmem_destroy: Undo the shared memory buffer setup in shmem_setup. If not
 *                 specified dma_free_coherent is used. Called in process
 *                 context.
 */
#[repr(C)]
pub struct apple_rtkit_ops {
    pub crashed: Option<unsafe extern "C" fn(*mut c_void, *const c_void, usize)>,
    pub recv_message: Option<unsafe extern "C" fn(*mut c_void, u8, u64)>,
    pub recv_message_early: Option<unsafe extern "C" fn(*mut c_void, u8, u64) -> bool>,
    pub shmem_setup:
        Option<unsafe extern "C" fn(*mut c_void, *mut apple_rtkit_shmem) -> i32>,
    pub shmem_destroy: Option<unsafe extern "C" fn(*mut c_void, *mut apple_rtkit_shmem)>,
}

pub struct apple_rtkit;

extern "C" {
    pub fn devm_apple_rtkit_init(
        dev: *mut c_void,
        cookie: *mut c_void,
        mbox_name: *const i8,
        mbox_idx: i32,
        ops: *const apple_rtkit_ops,
    ) -> *mut apple_rtkit;

    pub fn apple_rtkit_init(
        dev: *mut c_void,
        cookie: *mut c_void,
        mbox_name: *const i8,
        mbox_idx: i32,
        ops: *const apple_rtkit_ops,
    ) -> *mut apple_rtkit;

    pub fn apple_rtkit_free(rtk: *mut apple_rtkit);
    pub fn apple_rtkit_reinit(rtk: *mut apple_rtkit) -> i32;
    pub fn apple_rtkit_boot(rtk: *mut apple_rtkit) -> i32;
    pub fn apple_rtkit_quiesce(rtk: *mut apple_rtkit) -> i32;
    pub fn apple_rtkit_wake(rtk: *mut apple_rtkit) -> i32;
    pub fn apple_rtkit_shutdown(rtk: *mut apple_rtkit) -> i32;
    pub fn apple_rtkit_poweroff(rtk: *mut apple_rtkit) -> i32;
    pub fn apple_rtkit_idle(rtk: *mut apple_rtkit) -> i32;
    pub fn apple_rtkit_is_running(rtk: *mut apple_rtkit) -> bool;
    pub fn apple_rtkit_is_crashed(rtk: *mut apple_rtkit) -> bool;
    pub fn apple_rtkit_start_ep(rtk: *mut apple_rtkit, endpoint: u8) -> i32;
    pub fn apple_rtkit_send_message(
        rtk: *mut apple_rtkit,
        ep: u8,
        message: u64,
        completion: *mut c_void,
        atomic: bool,
    ) -> i32;
    pub fn apple_rtkit_poll(rtk: *mut apple_rtkit) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
