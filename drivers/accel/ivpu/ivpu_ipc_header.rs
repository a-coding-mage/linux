/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2020-2024 Intel Corporation
 */

// Dependency declarations from the C header are supplied by other translated files.

pub const IVPU_IPC_CHAN_BOOT_MSG: u32 = 0x3ff;
pub const IVPU_IPC_BOOT_MSG_DATA_ADDR: u32 = 0x424f4f54;

// The alignment to be used for IPC Buffers and IPC Data.
pub const IVPU_IPC_ALIGNMENT: usize = 64;

pub const IVPU_IPC_HDR_FREE: u8 = 0;
pub const IVPU_IPC_HDR_ALLOCATED: u8 = 1;

#[repr(C, align(64))]
pub struct ivpu_ipc_hdr {
    pub data_addr: u32,
    pub data_size: u32,
    pub channel: u16,
    pub src_node: u8,
    pub dst_node: u8,
    pub status: u8,
}

pub type ivpu_ipc_rx_callback_t = Option<unsafe extern "C" fn(
    vdev: *mut ivpu_device,
    ipc_hdr: *mut ivpu_ipc_hdr,
    jsm_msg: *mut vpu_jsm_msg,
)>;

#[repr(C)]
pub struct ivpu_ipc_rx_msg {
    pub link: list_head,
    pub ipc_hdr: *mut ivpu_ipc_hdr,
    pub jsm_msg: *mut vpu_jsm_msg,
    pub callback: ivpu_ipc_rx_callback_t,
}

#[repr(C)]
pub struct ivpu_ipc_consumer {
    pub link: list_head,
    pub channel: u32,
    pub tx_vpu_addr: u32,
    pub request_id: u32,
    pub aborted: bool,
    pub rx_callback: ivpu_ipc_rx_callback_t,

    // Protects rx_msg_list and aborted
    pub rx_lock: spinlock_t,
    pub rx_msg_list: list_head,
    pub rx_msg_wq: wait_queue_head_t,
}

#[repr(C)]
pub struct ivpu_ipc_info {
    pub mm_tx: *mut gen_pool,
    pub mem_tx: *mut ivpu_bo,
    pub mem_rx: *mut ivpu_bo,
    pub rx_msg_cache: *mut kmem_cache,

    pub rx_msg_count: atomic_t,

    // Protects cons_list and cb_msg_list
    pub cons_lock: spinlock_t,
    pub cons_list: list_head,
    pub cb_msg_list: list_head,

    pub request_id: atomic_t,
    // Lock on status
    pub lock: mutex,
    pub on: bool,
}

extern "C" {
    pub fn ivpu_ipc_init(vdev: *mut ivpu_device) -> i32;
    pub fn ivpu_ipc_fini(vdev: *mut ivpu_device);

    pub fn ivpu_ipc_enable(vdev: *mut ivpu_device);
    pub fn ivpu_ipc_disable(vdev: *mut ivpu_device);
    pub fn ivpu_ipc_reset(vdev: *mut ivpu_device);

    pub fn ivpu_ipc_irq_handler(vdev: *mut ivpu_device);
    pub fn ivpu_ipc_irq_thread_handler(irq: i32, ptr: *mut core::ffi::c_void) -> irqreturn_t;

    pub fn ivpu_ipc_consumer_add(
        vdev: *mut ivpu_device,
        cons: *mut ivpu_ipc_consumer,
        channel: u32,
        callback: ivpu_ipc_rx_callback_t,
    );
    pub fn ivpu_ipc_consumer_del(vdev: *mut ivpu_device, cons: *mut ivpu_ipc_consumer);

    pub fn ivpu_ipc_send(
        vdev: *mut ivpu_device,
        cons: *mut ivpu_ipc_consumer,
        req: *mut vpu_jsm_msg,
    ) -> i32;
    pub fn ivpu_ipc_receive(
        vdev: *mut ivpu_device,
        cons: *mut ivpu_ipc_consumer,
        ipc_buf: *mut ivpu_ipc_hdr,
        jsm_msg: *mut vpu_jsm_msg,
        timeout_ms: c_ulong,
    ) -> i32;
    pub fn ivpu_ipc_send_receive_internal(
        vdev: *mut ivpu_device,
        req: *mut vpu_jsm_msg,
        expected_resp_type: vpu_ipc_msg_type,
        resp: *mut vpu_jsm_msg,
        channel: u32,
        timeout_ms: c_ulong,
    ) -> i32;
    pub fn ivpu_ipc_send_receive(
        vdev: *mut ivpu_device,
        req: *mut vpu_jsm_msg,
        expected_resp: vpu_ipc_msg_type,
        resp: *mut vpu_jsm_msg,
        channel: u32,
        timeout_ms: c_ulong,
    ) -> i32;
    pub fn ivpu_ipc_send_and_wait(
        vdev: *mut ivpu_device,
        req: *mut vpu_jsm_msg,
        channel: u32,
        timeout_ms: c_ulong,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
