/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Copyright (C) 2005 Oracle. All rights reserved. */

pub const O2NET_MSG_MAGIC: u16 = 0xfa55;
pub const O2NET_MSG_STATUS_MAGIC: u16 = 0xfa56;
pub const O2NET_MSG_KEEP_REQ_MAGIC: u16 = 0xfa57;
pub const O2NET_MSG_KEEP_RESP_MAGIC: u16 = 0xfa58;

/* We're delaying our quorum decision so heartbeat will have timed out truly
 * dead nodes by the time we come around to making decisions on their number. */
pub const O2NET_QUORUM_DELAY_MS: u64 = (o2hb_dead_threshold + 2) * O2HB_REGION_TIMEOUT_MS;

/* This version covers both the raw network message protocol and filesystem
 * locking semantics using that protocol. Version 11 separates filesystem
 * locking negotiation into the filesystem's major.minor version. */
pub const O2NET_PROTOCOL_VERSION: u64 = 11;

#[repr(C)]
pub struct o2net_handshake {
    pub protocol_version: __be64,
    pub connector_id: __be64,
    pub o2hb_heartbeat_timeout_ms: __be32,
    pub o2net_idle_timeout_ms: __be32,
    pub o2net_keepalive_delay_ms: __be32,
    pub o2net_reconnect_delay_ms: __be32,
}

#[repr(C)]
pub struct o2net_node {
    /* this is never called from int/bh */
    pub nn_lock: spinlock_t,
    /* set the moment an sc is allocated and a connect is started */
    pub nn_sc: *mut o2net_sock_container,
    /* _valid is only set after the handshake passes and tx can happen */
    pub nn_sc_valid: u8,
    /* if this is set tx just returns it */
    pub nn_persistent_error: core::ffi::c_int,
    /* It is only set to 1 after the idle time out. */
    pub nn_timeout: atomic_t,
    pub nn_sc_wq: wait_queue_head_t,
    pub nn_status_idr: idr,
    pub nn_status_list: list_head,
    pub nn_connect_work: delayed_work,
    pub nn_last_connect_attempt: c_ulong,
    pub nn_connect_expired: delayed_work,
    pub nn_still_up: delayed_work,
}

#[repr(C)]
pub struct o2net_sock_container {
    pub sc_kref: kref,
    pub sc_sock: *mut socket,
    pub sc_node: *mut o2nm_node,
    pub sc_rx_work: work_struct,
    pub sc_connect_work: work_struct,
    pub sc_shutdown_work: work_struct,
    pub sc_idle_timeout: timer_list,
    pub sc_keepalive_work: delayed_work,
    pub sc_handshake_ok: u8,
    pub sc_page: *mut page,
    pub sc_page_off: usize,
    pub sc_state_change: Option<unsafe extern "C" fn(sk: *mut sock)>,
    pub sc_data_ready: Option<unsafe extern "C" fn(sk: *mut sock)>,
    pub sc_msg_key: u32,
    pub sc_msg_type: u16,
    #[cfg(CONFIG_DEBUG_FS)]
    pub sc_net_debug_item: list_head,
    #[cfg(CONFIG_DEBUG_FS)]
    pub sc_tv_timer: ktime_t,
    #[cfg(CONFIG_DEBUG_FS)]
    pub sc_tv_data_ready: ktime_t,
    #[cfg(CONFIG_DEBUG_FS)]
    pub sc_tv_advance_start: ktime_t,
    #[cfg(CONFIG_DEBUG_FS)]
    pub sc_tv_advance_stop: ktime_t,
    #[cfg(CONFIG_DEBUG_FS)]
    pub sc_tv_func_start: ktime_t,
    #[cfg(CONFIG_DEBUG_FS)]
    pub sc_tv_func_stop: ktime_t,
    #[cfg(CONFIG_OCFS2_FS_STATS)]
    pub sc_tv_acquiry_total: ktime_t,
    #[cfg(CONFIG_OCFS2_FS_STATS)]
    pub sc_tv_send_total: ktime_t,
    #[cfg(CONFIG_OCFS2_FS_STATS)]
    pub sc_tv_status_total: ktime_t,
    #[cfg(CONFIG_OCFS2_FS_STATS)]
    pub sc_send_count: u32,
    #[cfg(CONFIG_OCFS2_FS_STATS)]
    pub sc_recv_count: u32,
    #[cfg(CONFIG_OCFS2_FS_STATS)]
    pub sc_tv_process_total: ktime_t,
    pub sc_send_lock: mutex,
}

#[repr(C)]
pub struct o2net_msg_handler {
    pub nh_node: rb_node,
    pub nh_max_len: u32,
    pub nh_msg_type: u32,
    pub nh_key: u32,
    pub nh_func: *mut o2net_msg_handler_func,
    pub nh_func_data: *mut core::ffi::c_void,
    pub nh_post_func: *mut o2net_post_msg_handler_func,
    pub nh_kref: kref,
    pub nh_unregister_item: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum o2net_system_error {
    O2NET_ERR_NONE = 0,
    O2NET_ERR_NO_HNDLR,
    O2NET_ERR_OVERFLOW,
    O2NET_ERR_DIED,
    O2NET_ERR_MAX,
}

#[repr(C)]
pub struct o2net_status_wait {
    pub ns_sys_status: o2net_system_error,
    pub ns_status: s32,
    pub ns_id: core::ffi::c_int,
    pub ns_wq: wait_queue_head_t,
    pub ns_node_item: list_head,
}

#[cfg(CONFIG_DEBUG_FS)]
#[repr(C)]
pub struct o2net_send_tracking {
    pub st_net_debug_item: list_head,
    pub st_task: *mut task_struct,
    pub st_sc: *mut o2net_sock_container,
    pub st_id: u32,
    pub st_msg_type: u32,
    pub st_msg_key: u32,
    pub st_node: u8,
    pub st_sock_time: ktime_t,
    pub st_send_time: ktime_t,
    pub st_status_time: ktime_t,
}

#[cfg(not(CONFIG_DEBUG_FS))]
#[repr(C)]
pub struct o2net_send_tracking {
    pub dummy: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
