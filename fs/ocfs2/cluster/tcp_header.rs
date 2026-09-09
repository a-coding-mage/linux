/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * tcp.h
 *
 * Function prototypes
 *
 * Copyright (C) 2004 Oracle.  All rights reserved.
 */

/* C header dependencies are supplied by the surrounding kernel translation. */

#[repr(C)]
pub struct o2net_msg {
    pub magic: __be16,
    pub data_len: __be16,
    pub msg_type: __be16,
    pub pad1: __be16,
    pub sys_status: __be32,
    pub status: __be32,
    pub key: __be32,
    pub msg_num: __be32,
    pub buf: [__u8; 0],
}

pub type o2net_msg_handler_func = unsafe extern "C" fn(
    msg: *mut o2net_msg,
    len: u32,
    data: *mut core::ffi::c_void,
    ret_data: *mut *mut core::ffi::c_void,
) -> i32;

pub type o2net_post_msg_handler_func = unsafe extern "C" fn(
    status: i32,
    data: *mut core::ffi::c_void,
    ret_data: *mut core::ffi::c_void,
);

pub const O2NET_MAX_PAYLOAD_BYTES: usize = 4096 - core::mem::size_of::<o2net_msg>();

/* same as hb delay, we're waiting for another node to recognize our hb */
pub const O2NET_RECONNECT_DELAY_MS_DEFAULT: u32 = 2000;

pub const O2NET_KEEPALIVE_DELAY_MS_DEFAULT: u32 = 2000;
pub const O2NET_IDLE_TIMEOUT_MS_DEFAULT: u32 = 30000;

pub const O2NET_TCP_USER_TIMEOUT: i32 = 0x7fffffff;

/* TODO: figure this out.... */
pub unsafe fn o2net_link_down(err: i32, sock: *mut socket) -> i32 {
    if !sock.is_null() {
        /* The `socket`/`sock` layout and TCP state constants come from kernel headers. */
        // if (*sock).sk.sk_state != TCP_ESTABLISHED && (*sock).sk.sk_state != TCP_CLOSE_WAIT {
        //     return 1;
        // }
    }

    if err >= 0 {
        return 0;
    }
    match err {
        /* ????????????????????????? */
        -ERESTARTSYS | -EBADF |
        /* When the server has died, an ICMP port unreachable
         * message prompts ECONNREFUSED. */
        -ECONNREFUSED | -ENOTCONN | -ECONNRESET | -EPIPE => 1,
        _ => 0,
    }
}

pub const O2NET_DRIVER_UNINITED: i32 = 0;
pub const O2NET_DRIVER_READY: i32 = 1;

extern "C" {
    pub fn o2net_send_message(
        msg_type: u32, key: u32, data: *mut core::ffi::c_void, len: u32,
        target_node: __u8, status: *mut i32,
    ) -> i32;
    pub fn o2net_send_message_vec(
        msg_type: u32, key: u32, vec: *mut kvec, veclen: usize,
        target_node: __u8, status: *mut i32,
    ) -> i32;

    pub fn o2net_register_handler(
        msg_type: u32, key: u32, max_len: u32, func: o2net_msg_handler_func,
        data: *mut core::ffi::c_void, post_func: o2net_post_msg_handler_func,
        unreg_list: *mut list_head,
    ) -> i32;
    pub fn o2net_unregister_handler_list(list: *mut list_head);
    pub fn o2net_unregister_and_flush_handler_list(list: *mut list_head);

    pub fn o2net_fill_node_map(map: *mut c_ulong, bytes: c_uint);

    pub fn o2net_register_hb_callbacks() -> i32;
    pub fn o2net_unregister_hb_callbacks();
    pub fn o2net_start_listening(node: *mut o2nm_node) -> i32;
    pub fn o2net_complete_start_listening(node: *mut o2nm_node);
    pub fn o2net_stop_listening(node: *mut o2nm_node);
    pub fn o2net_disconnect_node(node: *mut o2nm_node);
    pub fn o2net_num_connected_peers() -> i32;

    pub fn o2net_init() -> i32;
    pub fn o2net_exit();
}

pub enum o2nm_node {}
pub enum o2net_send_tracking {}
pub enum o2net_sock_container {}

/* CONFIG_DEBUG_FS declarations are selected by the surrounding build configuration. */
#[cfg(feature = "CONFIG_DEBUG_FS")]
extern "C" {
    pub fn o2net_debugfs_init();
    pub fn o2net_debugfs_exit();
    pub fn o2net_debug_add_nst(nst: *mut o2net_send_tracking);
    pub fn o2net_debug_del_nst(nst: *mut o2net_send_tracking);
    pub fn o2net_debug_add_sc(sc: *mut o2net_sock_container);
    pub fn o2net_debug_del_sc(sc: *mut o2net_sock_container);
}

#[cfg(not(feature = "CONFIG_DEBUG_FS"))]
pub unsafe fn o2net_debugfs_init() {}
#[cfg(not(feature = "CONFIG_DEBUG_FS"))]
pub unsafe fn o2net_debugfs_exit() {}
#[cfg(not(feature = "CONFIG_DEBUG_FS"))]
pub unsafe fn o2net_debug_add_nst(_nst: *mut o2net_send_tracking) {}
#[cfg(not(feature = "CONFIG_DEBUG_FS"))]
pub unsafe fn o2net_debug_del_nst(_nst: *mut o2net_send_tracking) {}
#[cfg(not(feature = "CONFIG_DEBUG_FS"))]
pub unsafe fn o2net_debug_add_sc(_sc: *mut o2net_sock_container) {}
#[cfg(not(feature = "CONFIG_DEBUG_FS"))]
pub unsafe fn o2net_debug_del_sc(_sc: *mut o2net_sock_container) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
