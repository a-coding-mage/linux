/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * linux/can/rx-offload.h
 *
 * Copyright (c) 2014 David Jander, Protonic Holland
 * Copyright (c) 2014-2017, 2023 Pengutronix, Marc Kleine-Budde <kernel@pengutronix.de>
 */

// Dependencies supplied by the surrounding kernel translation:
// linux/netdevice.h and linux/can.h

#[repr(C)]
pub struct can_rx_offload {
    pub dev: *mut net_device,

    pub mailbox_read: Option<
        unsafe extern "C" fn(
            offload: *mut can_rx_offload,
            mb: ::core::ffi::c_uint,
            timestamp: *mut u32,
            drop: bool,
        ) -> *mut sk_buff,
    >,

    pub skb_queue: sk_buff_head,
    pub skb_irq_queue: sk_buff_head,
    pub skb_queue_len_max: u32,

    pub mb_first: ::core::ffi::c_uint,
    pub mb_last: ::core::ffi::c_uint,

    pub napi: napi_struct,

    pub inc: bool,
}

unsafe extern "C" {
    pub fn can_rx_offload_add_timestamp(
        dev: *mut net_device,
        offload: *mut can_rx_offload,
    ) -> ::core::ffi::c_int;
    pub fn can_rx_offload_add_fifo(
        dev: *mut net_device,
        offload: *mut can_rx_offload,
        weight: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
    pub fn can_rx_offload_add_manual(
        dev: *mut net_device,
        offload: *mut can_rx_offload,
        weight: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
    pub fn can_rx_offload_irq_offload_timestamp(
        offload: *mut can_rx_offload,
        reg: u64,
    ) -> ::core::ffi::c_int;
    pub fn can_rx_offload_irq_offload_fifo(
        offload: *mut can_rx_offload,
    ) -> ::core::ffi::c_int;
    pub fn can_rx_offload_queue_timestamp(
        offload: *mut can_rx_offload,
        skb: *mut sk_buff,
        timestamp: u32,
    ) -> ::core::ffi::c_int;
    pub fn can_rx_offload_get_echo_skb_queue_timestamp(
        offload: *mut can_rx_offload,
        idx: ::core::ffi::c_uint,
        timestamp: u32,
        frame_len_ptr: *mut ::core::ffi::c_uint,
    ) -> ::core::ffi::c_uint;
    pub fn can_rx_offload_queue_tail(
        offload: *mut can_rx_offload,
        skb: *mut sk_buff,
    ) -> ::core::ffi::c_int;
    pub fn can_rx_offload_get_echo_skb_queue_tail(
        offload: *mut can_rx_offload,
        idx: ::core::ffi::c_uint,
        frame_len_ptr: *mut ::core::ffi::c_uint,
    ) -> ::core::ffi::c_uint;
    pub fn can_rx_offload_irq_finish(offload: *mut can_rx_offload);
    pub fn can_rx_offload_threaded_irq_finish(offload: *mut can_rx_offload);
    pub fn can_rx_offload_del(offload: *mut can_rx_offload);
    pub fn can_rx_offload_enable(offload: *mut can_rx_offload);
}

#[inline]
pub unsafe fn can_rx_offload_disable(offload: *mut can_rx_offload) {
    napi_disable(&mut (*offload).napi);
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
