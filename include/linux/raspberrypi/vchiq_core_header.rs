/* SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause */
/* Copyright (c) 2010-2012 Broadcom. All rights reserved. */

// Linux dependencies and build-time configuration are supplied by the containing crate.
// When CONFIG_RASPBERRYPI_FIRMWARE is disabled, `dsb` is intentionally a no-op.

pub const VCHIQ_SERVICE_HANDLE_INVALID: u32 = 0;
pub const VCHIQ_SLOT_SIZE: usize = 4096;
pub const VCHIQ_MAX_MSG_SIZE: usize = VCHIQ_SLOT_SIZE - core::mem::size_of::<vchiq_header>();
pub const VCHIQ_SLOT_MASK: usize = VCHIQ_SLOT_SIZE - 1;

#[inline]
pub const fn vchiq_slot_queue_mask() -> usize { VCHIQ_MAX_SLOTS_PER_SIDE - 1 }
#[inline]
pub const fn vchiq_slot_zero_slots() -> usize {
    (core::mem::size_of::<vchiq_slot_zero>() + VCHIQ_SLOT_SIZE - 1) / VCHIQ_SLOT_SIZE
}
#[inline] pub const fn bitset_size(b: usize) -> usize { (b + 31) >> 5 }
#[inline] pub const fn bitset_word(b: usize) -> usize { b >> 5 }
#[inline] pub const fn bitset_bit(b: usize) -> i32 { 1 << (b & 31) }

#[repr(i32)]
#[derive(Copy, Clone)]
pub enum DebugEntry { DEBUG_ENTRIES = 0, DEBUG_MAX }

#[repr(i32)]
#[derive(Copy, Clone)]
pub enum VchiqConnstate {
    VCHIQ_CONNSTATE_DISCONNECTED,
    VCHIQ_CONNSTATE_CONNECTING,
    VCHIQ_CONNSTATE_CONNECTED,
    VCHIQ_CONNSTATE_PAUSING,
    VCHIQ_CONNSTATE_PAUSE_SENT,
    VCHIQ_CONNSTATE_PAUSED,
    VCHIQ_CONNSTATE_RESUMING,
    VCHIQ_CONNSTATE_PAUSE_TIMEOUT,
    VCHIQ_CONNSTATE_RESUME_TIMEOUT,
}

#[repr(i32)]
#[derive(Copy, Clone)]
pub enum VchiqSrvstate {
    VCHIQ_SRVSTATE_FREE,
    VCHIQ_SRVSTATE_HIDDEN,
    VCHIQ_SRVSTATE_LISTENING,
    VCHIQ_SRVSTATE_OPENING,
    VCHIQ_SRVSTATE_OPEN,
    VCHIQ_SRVSTATE_OPENSYNC,
    VCHIQ_SRVSTATE_CLOSESENT,
    VCHIQ_SRVSTATE_CLOSERECVD,
    VCHIQ_SRVSTATE_CLOSEWAIT,
    VCHIQ_SRVSTATE_CLOSED,
}

#[repr(i32)]
#[derive(Copy, Clone)]
pub enum VchiqBulkDir { VCHIQ_BULK_TRANSMIT, VCHIQ_BULK_RECEIVE }

#[repr(C)]
pub struct vchiq_bulk {
    pub mode: i16, pub dir: i16, pub cb_data: *mut core::ffi::c_void,
    pub cb_userdata: *mut core::ffi::c_void, pub waiter: *mut bulk_waiter,
    pub dma_addr: dma_addr_t, pub size: i32, pub remote_data: *mut core::ffi::c_void,
    pub remote_size: i32, pub actual: i32, pub offset: *mut core::ffi::c_void,
    pub uoffset: *mut core::ffi::c_void,
}
#[repr(C)]
pub struct vchiq_bulk_queue {
    pub local_insert: i32, pub remote_insert: i32, pub process: i32,
    pub remote_notify: i32, pub remove: i32,
    pub bulks: [vchiq_bulk; VCHIQ_NUM_SERVICE_BULKS],
}

#[repr(C)] pub struct remote_event { pub armed: i32, pub fired: i32, pub __unused: u32 }
#[repr(C)] pub struct opaque_platform_state { _private: [u8; 0] }
#[repr(C)] pub struct vchiq_slot { pub data: [i8; VCHIQ_SLOT_SIZE] }
#[repr(C)] pub struct vchiq_slot_info { pub use_count: i16, pub release_count: i16 }

#[repr(C)]
pub struct vchiq_service {
    pub base: vchiq_service_base, pub handle: u32, pub ref_count: kref, pub rcu: rcu_head,
    pub srvstate: i32, pub userdata_term: Option<unsafe extern "C" fn(*mut core::ffi::c_void)>,
    pub localport: u32, pub remoteport: u32, pub public_fourcc: i32, pub client_id: i32,
    pub auto_close: i8, pub sync: i8, pub closing: i8, pub trace: i8, pub poll_flags: atomic_t,
    pub version: i16, pub version_min: i16, pub peer_version: i16,
    pub state: *mut vchiq_state, pub instance: *mut vchiq_instance, pub service_use_count: i32,
    pub bulk_tx: vchiq_bulk_queue, pub bulk_rx: vchiq_bulk_queue,
    pub remove_event: completion, pub bulk_remove_event: completion, pub bulk_mutex: mutex,
    pub stats: service_stats_struct, pub msg_queue_read: i32, pub msg_queue_write: i32,
    pub msg_queue_pop: completion, pub msg_queue_push: completion,
    pub msg_queue: [*mut vchiq_header; VCHIQ_MAX_SLOTS],
}
#[repr(C)] pub struct service_stats_struct {
    pub quota_stalls: i32, pub slot_stalls: i32, pub bulk_stalls: i32, pub error_count: i32,
    pub ctrl_tx_count: i32, pub ctrl_rx_count: i32, pub bulk_tx_count: i32, pub bulk_rx_count: i32,
    pub bulk_aborted_count: i32, pub ctrl_tx_bytes: u64, pub ctrl_rx_bytes: u64,
    pub bulk_tx_bytes: u64, pub bulk_rx_bytes: u64,
}
#[repr(C)] pub struct vchiq_service_quota {
    pub slot_quota: u16, pub slot_use_count: u16, pub message_quota: u16,
    pub message_use_count: u16, pub quota_event: completion, pub previous_tx_index: i32,
}
#[repr(C)] pub struct vchiq_shared_state {
    pub initialised: i32, pub slot_first: i32, pub slot_last: i32, pub slot_sync: i32,
    pub trigger: remote_event, pub tx_pos: i32, pub recycle: remote_event,
    pub slot_queue_recycle: i32, pub sync_trigger: remote_event, pub sync_release: remote_event,
    pub slot_queue: [i32; VCHIQ_MAX_SLOTS_PER_SIDE], pub debug: [i32; DEBUG_MAX as usize],
}
#[repr(C)] pub struct vchiq_slot_zero {
    pub magic: i32, pub version: i16, pub version_min: i16, pub slot_zero_size: i32,
    pub slot_size: i32, pub max_slots: i32, pub max_slots_per_side: i32, pub platform_data: [i32; 2],
    pub master: vchiq_shared_state, pub slave: vchiq_shared_state,
    pub slots: [vchiq_slot_info; VCHIQ_MAX_SLOTS],
}
#[repr(C)] pub struct vchiq_state {
    pub dev: *mut device, pub id: i32, pub initialised: i32, pub conn_state: VchiqConnstate,
    pub version_common: i16, pub local: *mut vchiq_shared_state, pub remote: *mut vchiq_shared_state,
    pub slot_data: *mut vchiq_slot, pub default_slot_quota: u16, pub default_message_quota: u16,
    pub connect: completion, pub mutex: mutex, pub instance: *mut *mut vchiq_instance,
    pub slot_handler_thread: *mut task_struct, pub recycle_thread: *mut task_struct,
    pub sync_thread: *mut task_struct, pub trigger_event: wait_queue_head_t,
    pub recycle_event: wait_queue_head_t, pub sync_trigger_event: wait_queue_head_t,
    pub sync_release_event: wait_queue_head_t, pub tx_data: *mut i8, pub rx_data: *mut i8,
    pub rx_info: *mut vchiq_slot_info, pub slot_mutex: mutex, pub recycle_mutex: mutex,
    pub sync_mutex: mutex, pub msg_queue_spinlock: spinlock_t, pub bulk_waiter_spinlock: spinlock_t,
    pub quota_spinlock: spinlock_t, pub rx_pos: i32, pub local_tx_pos: i32,
    pub slot_queue_available: i32, pub poll_needed: i32, pub previous_data_index: i32,
    pub data_use_count: u16, pub data_quota: u16,
    pub poll_services: [atomic_t; bitset_size(VCHIQ_MAX_SERVICES)], pub unused_service: i32,
    pub slot_available_event: completion, pub data_quota_event: completion,
    pub stats: state_stats_struct, pub services: [*mut vchiq_service; VCHIQ_MAX_SERVICES],
    pub service_quotas: [vchiq_service_quota; VCHIQ_MAX_SERVICES],
    pub slot_info: [vchiq_slot_info; VCHIQ_MAX_SLOTS], pub platform_state: *mut opaque_platform_state,
}
#[repr(C)] pub struct state_stats_struct {
    pub slot_stalls: i32, pub data_stalls: i32, pub ctrl_tx_count: i32,
    pub ctrl_rx_count: i32, pub error_count: i32,
}
#[repr(C)] pub struct pagelist { pub length: u32, pub type_: u16, pub offset: u16, pub addrs: [u32; 1] }
#[repr(C)] pub struct vchiq_pagelist_info {
    pub pagelist: *mut pagelist, pub pagelist_buffer_size: usize, pub dma_addr: dma_addr_t,
    pub dma_dir: dma_data_direction, pub num_pages: u32, pub pages_need_release: u32,
    pub pages: *mut *mut page, pub scatterlist: *mut scatterlist, pub scatterlist_mapped: u32,
}
#[inline] pub unsafe fn vchiq_remote_initialised(state: *const vchiq_state) -> bool {
    !(*state).remote.is_null() && (*(*state).remote).initialised != 0
}
#[repr(C)] pub struct bulk_waiter { pub bulk: *mut vchiq_bulk, pub event: completion, pub actual: i32 }
#[repr(C)] pub struct vchiq_config {
    pub max_msg_size: u32, pub bulk_threshold: u32, pub max_outstanding_bulks: u32,
    pub max_services: u32, pub version: i16, pub version_min: i16,
}

extern "C" {
    pub static mut bulk_waiter_spinlock: spinlock_t;
    pub fn get_conn_state_name(conn_state: VchiqConnstate) -> *const i8;
    pub fn vchiq_init_slots(dev: *mut device, mem_base: *mut core::ffi::c_void, mem_size: i32) -> *mut vchiq_slot_zero;
    pub fn vchiq_init_state(state: *mut vchiq_state, slot_zero: *mut vchiq_slot_zero, dev: *mut device) -> i32;
    pub fn vchiq_connect_internal(state: *mut vchiq_state, instance: *mut vchiq_instance) -> i32;
    pub fn vchiq_add_service_internal(state: *mut vchiq_state, params: *const vchiq_service_params_kernel, srvstate: i32, instance: *mut vchiq_instance, userdata_term: Option<unsafe extern "C" fn(*mut core::ffi::c_void)>) -> *mut vchiq_service;
    pub fn vchiq_open_service_internal(service: *mut vchiq_service, client_id: i32) -> i32;
    pub fn vchiq_close_service_internal(service: *mut vchiq_service, close_recvd: i32) -> i32;
    pub fn vchiq_terminate_service_internal(service: *mut vchiq_service);
    pub fn vchiq_free_service_internal(service: *mut vchiq_service);
    pub fn vchiq_shutdown_internal(state: *mut vchiq_state, instance: *mut vchiq_instance);
    pub fn remote_event_pollall(state: *mut vchiq_state);
    pub fn vchiq_bulk_xfer_waiting(instance: *mut vchiq_instance, handle: u32, userdata: *mut bulk_waiter) -> i32;
    pub fn vchiq_bulk_xfer_blocking(instance: *mut vchiq_instance, handle: u32, bulk: *mut vchiq_bulk) -> i32;
    pub fn vchiq_bulk_xfer_callback(instance: *mut vchiq_instance, handle: u32, bulk: *mut vchiq_bulk) -> i32;
    pub fn vchiq_dump_state(f: *mut seq_file, state: *mut vchiq_state);
    pub fn request_poll(state: *mut vchiq_state, service: *mut vchiq_service, poll_type: i32);
    pub fn handle_to_service(instance: *mut vchiq_instance, handle: u32) -> *mut vchiq_service;
    pub fn find_service_by_handle(instance: *mut vchiq_instance, handle: u32) -> *mut vchiq_service;
    pub fn find_service_by_port(state: *mut vchiq_state, localport: u32) -> *mut vchiq_service;
    pub fn find_service_for_instance(instance: *mut vchiq_instance, handle: u32) -> *mut vchiq_service;
    pub fn find_closed_service_for_instance(instance: *mut vchiq_instance, handle: u32) -> *mut vchiq_service;
    pub fn __next_service_by_instance(state: *mut vchiq_state, instance: *mut vchiq_instance, pidx: *mut i32) -> *mut vchiq_service;
    pub fn next_service_by_instance(state: *mut vchiq_state, instance: *mut vchiq_instance, pidx: *mut i32) -> *mut vchiq_service;
    pub fn vchiq_service_get(service: *mut vchiq_service);
    pub fn vchiq_service_put(service: *mut vchiq_service);
    pub fn vchiq_queue_message(instance: *mut vchiq_instance, handle: u32, copy_callback: Option<unsafe extern "C" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, usize, usize) -> isize>, context: *mut core::ffi::c_void, size: usize) -> i32;
    pub fn vchiq_dump_platform_state(f: *mut seq_file);
    pub fn vchiq_dump_platform_instances(state: *mut vchiq_state, f: *mut seq_file);
    pub fn vchiq_dump_platform_service_state(f: *mut seq_file, service: *mut vchiq_service);
    pub fn vchiq_use_service_internal(service: *mut vchiq_service) -> i32;
    pub fn vchiq_release_service_internal(service: *mut vchiq_service) -> i32;
    pub fn vchiq_on_remote_use(state: *mut vchiq_state);
    pub fn vchiq_on_remote_release(state: *mut vchiq_state);
    pub fn vchiq_platform_init_state(state: *mut vchiq_state) -> i32;
    pub fn vchiq_check_service(service: *mut vchiq_service) -> i32;
    pub fn vchiq_send_remote_use(state: *mut vchiq_state) -> i32;
    pub fn vchiq_send_remote_use_active(state: *mut vchiq_state) -> i32;
    pub fn vchiq_platform_conn_state_changed(state: *mut vchiq_state, oldstate: VchiqConnstate, newstate: VchiqConnstate);
    pub fn vchiq_set_conn_state(state: *mut vchiq_state, newstate: VchiqConnstate);
    pub fn vchiq_log_dump_mem(dev: *mut device, label: *const i8, addr: u32, void_mem: *const core::ffi::c_void, num_bytes: usize);
    pub fn vchiq_remove_service(instance: *mut vchiq_instance, service: u32) -> i32;
    pub fn vchiq_get_client_id(instance: *mut vchiq_instance, service: u32) -> i32;
    pub fn vchiq_get_config(config: *mut vchiq_config);
    pub fn vchiq_set_service_option(instance: *mut vchiq_instance, service: u32, option: vchiq_service_option, value: i32) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
