/* SPDX-License-Identifier: GPL-2.0-or-later */
/* SCTP kernel implementation
 * (C) Copyright IBM Corp. 2001, 2004
 * Copyright (c) 1999-2000 Cisco, Inc.
 * Copyright (c) 1999-2001 Motorola, Inc.
 * Copyright (c) 2001 Intel Corp.
 * Copyright (c) 2001 Nokia, Inc.
 * Copyright (c) 2001 La Monte H.P. Yarroll
 *
 * These are the definitions needed for the sctp_ulpevent type.  The
 * sctp_ulpevent type is used to carry information from the state machine
 * upwards to the ULP.
 */

/* A structure to carry information to the ULP (e.g. Sockets API).
 * This sits inside an skb.cb[] area and must remain packed. */
#[repr(C, packed)]
pub struct sctp_ulpevent {
    pub asoc: *mut sctp_association,
    pub chunk: *mut sctp_chunk,
    pub rmem_len: ::core::ffi::c_uint,
    pub mid_or_ssn: sctp_ulpevent_mid_or_ssn,
    pub ppid_or_fsn: sctp_ulpevent_ppid_or_fsn,
    pub tsn: __u32,
    pub cumtsn: __u32,
    pub stream: __u16,
    pub flags: __u16,
    pub msg_flags: __u16,
}

#[repr(C)]
pub union sctp_ulpevent_mid_or_ssn {
    pub mid: __u32,
    pub ssn: __u16,
}

#[repr(C)]
pub union sctp_ulpevent_ppid_or_fsn {
    pub ppid: __u32,
    pub fsn: __u32,
}

/* Retrieve the skb this event sits inside of.  The containing-object
 * calculation depends on the external sk_buff layout. */
#[inline]
pub unsafe fn sctp_event2skb(ev: *const sctp_ulpevent) -> *mut sk_buff {
    container_of_sctp_event(ev)
}

/* Retrieve and cast the event sitting inside the skb. */
#[inline]
pub unsafe fn sctp_skb2event(skb: *mut sk_buff) -> *mut sctp_ulpevent {
    (*(skb as *mut sk_buff_cb_access)).cb.as_mut_ptr() as *mut sctp_ulpevent
}

extern "C" {
    pub fn sctp_ulpevent_free(event: *mut sctp_ulpevent);
    pub fn sctp_ulpevent_is_notification(event: *const sctp_ulpevent) -> bool;
    pub fn sctp_queue_purge_ulpevents(list: *mut sk_buff_head) -> ::core::ffi::c_uint;
    pub fn sctp_ulpevent_make_assoc_change(asoc: *const sctp_association, flags: __u16, state: __u16, error: __u16, outbound: __u16, inbound: __u16, chunk: *mut sctp_chunk, gfp: gfp_t) -> *mut sctp_ulpevent;
    pub fn sctp_ulpevent_notify_peer_addr_change(transport: *mut sctp_transport, state: ::core::ffi::c_int, error: ::core::ffi::c_int);
    pub fn sctp_ulpevent_make_remote_error(asoc: *const sctp_association, chunk: *mut sctp_chunk, flags: __u16, gfp: gfp_t) -> *mut sctp_ulpevent;
    pub fn sctp_ulpevent_make_send_failed(asoc: *const sctp_association, chunk: *mut sctp_chunk, flags: __u16, error: __u32, gfp: gfp_t) -> *mut sctp_ulpevent;
    pub fn sctp_ulpevent_make_send_failed_event(asoc: *const sctp_association, chunk: *mut sctp_chunk, flags: __u16, error: __u32, gfp: gfp_t) -> *mut sctp_ulpevent;
    pub fn sctp_ulpevent_make_shutdown_event(asoc: *const sctp_association, flags: __u16, gfp: gfp_t) -> *mut sctp_ulpevent;
    pub fn sctp_ulpevent_make_pdapi(asoc: *const sctp_association, indication: __u32, sid: __u32, seq: __u32, flags: __u32, gfp: gfp_t) -> *mut sctp_ulpevent;
    pub fn sctp_ulpevent_make_adaptation_indication(asoc: *const sctp_association, gfp: gfp_t) -> *mut sctp_ulpevent;
    pub fn sctp_ulpevent_make_rcvmsg(asoc: *mut sctp_association, chunk: *mut sctp_chunk, gfp: gfp_t) -> *mut sctp_ulpevent;
    pub fn sctp_ulpevent_make_authkey(asoc: *const sctp_association, key_id: __u16, indication: __u32, gfp: gfp_t) -> *mut sctp_ulpevent;
    pub fn sctp_ulpevent_make_sender_dry_event(asoc: *const sctp_association, gfp: gfp_t) -> *mut sctp_ulpevent;
    pub fn sctp_ulpevent_make_stream_reset_event(asoc: *const sctp_association, flags: __u16, stream_num: __u16, stream_list: *mut __be16, gfp: gfp_t) -> *mut sctp_ulpevent;
    pub fn sctp_ulpevent_make_assoc_reset_event(asoc: *const sctp_association, flags: __u16, local_tsn: __u32, remote_tsn: __u32, gfp: gfp_t) -> *mut sctp_ulpevent;
    pub fn sctp_ulpevent_make_stream_change_event(asoc: *const sctp_association, flags: __u16, strchange_instrms: __u32, strchange_outstrms: __u32, gfp: gfp_t) -> *mut sctp_ulpevent;
    pub fn sctp_make_reassembled_event(net: *mut net, queue: *mut sk_buff_head, f_frag: *mut sk_buff, l_frag: *mut sk_buff) -> *mut sctp_ulpevent;
    pub fn sctp_ulpevent_read_sndrcvinfo(event: *const sctp_ulpevent, msg: *mut msghdr);
    pub fn sctp_ulpevent_read_rcvinfo(event: *const sctp_ulpevent, msg: *mut msghdr);
    pub fn sctp_ulpevent_read_nxtinfo(event: *const sctp_ulpevent, msg: *mut msghdr, sk: *mut sock);
    pub fn sctp_ulpevent_get_notification_type(event: *const sctp_ulpevent) -> __u16;
}

#[inline]
pub unsafe fn sctp_ulpevent_type_set(subscribe: *mut __u16, sn_type: __u16, on: __u8) {
    if sn_type > SCTP_SN_TYPE_MAX { return; }
    let bit = 1u16 << (sn_type - SCTP_SN_TYPE_BASE);
    if on != 0 { *subscribe |= bit; } else { *subscribe &= !bit; }
}

#[inline]
pub unsafe fn sctp_ulpevent_type_enabled(subscribe: __u16, sn_type: __u16) -> bool {
    if sn_type > SCTP_SN_TYPE_MAX { return false; }
    (subscribe & (1u16 << (sn_type - SCTP_SN_TYPE_BASE))) != 0
}

#[inline]
pub unsafe fn sctp_ulpevent_is_enabled(event: *const sctp_ulpevent, subscribe: __u16) -> bool {
    if !sctp_ulpevent_is_notification(event) { return true; }
    sctp_ulpevent_type_enabled(subscribe, sctp_ulpevent_get_notification_type(event))
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
