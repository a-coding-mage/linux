/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Rust translation of inet_timewait_sock.h. */

// C dependencies supplied by other translation units/headers.

#[repr(C)]
pub struct inet_timewait_sock {
    /* Must remain the first member, as in the C definition. */
    pub __tw_common: sock_common,
    pub tw_mark: __u32,
    pub tw_substate: ::core::ffi::c_uchar,
    pub tw_rcv_wscale: ::core::ffi::c_uchar,
    pub tw_sport: __be16,
    /* C bit-fields; represented as their containing word. */
    pub tw_transparent_flowlabel_usec_ts_connect_bind_pad_tos: ::core::ffi::c_uint,
    pub tw_txhash: u32,
    pub tw_priority: u32,
    /// Time of entry into TCP_TIME_WAIT state in msec.
    pub tw_entry_stamp: u32,
    pub tw_timer: timer_list,
    pub tw_tb: *mut inet_bind_bucket,
    pub tw_tb2: *mut inet_bind2_bucket,
}

// The following C preprocessor aliases refer to fields of __tw_common.
// Rust callers can access them as `obj.__tw_common.skc_*`.
// tw_family = __tw_common.skc_family
// tw_state = __tw_common.skc_state
// tw_reuse = __tw_common.skc_reuse
// tw_reuseport = __tw_common.skc_reuseport
// tw_ipv6only = __tw_common.skc_ipv6only
// tw_bound_dev_if = __tw_common.skc_bound_dev_if
// tw_node = __tw_common.skc_nulls_node
// tw_bind_node = __tw_common.skc_bind_node
// tw_refcnt = __tw_common.skc_refcnt
// tw_tx_queue_mapping = __tw_common.skc_tx_queue_mapping
// tw_rx_queue_mapping = __tw_common.skc_rx_queue_mapping
// tw_hash = __tw_common.skc_hash
// tw_prot = __tw_common.skc_prot
// tw_net = __tw_common.skc_net
// tw_daddr = __tw_common.skc_daddr
// tw_v6_daddr = __tw_common.skc_v6_daddr
// tw_rcv_saddr = __tw_common.skc_rcv_saddr
// tw_v6_rcv_saddr = __tw_common.skc_v6_rcv_saddr
// tw_dport = __tw_common.skc_dport
// tw_num = __tw_common.skc_num
// tw_cookie = __tw_common.skc_cookie
// tw_dr = __tw_common.skc_tw_dr

// #if IS_ENABLED(CONFIG_INET_PSP)
// pub psp_assoc: *mut psp_assoc,
// #endif
// #ifdef CONFIG_SOCK_VALIDATE_XMIT
// pub tw_validate_xmit_skb: Option<unsafe extern "C" fn(
//     sk: *mut sock, dev: *mut net_device, skb: *mut sk_buff,
// ) -> *mut sk_buff>,
// #endif

extern "C" {
    pub fn inet_twsk_free(tw: *mut inet_timewait_sock);
    pub fn inet_twsk_put(tw: *mut inet_timewait_sock);
    pub fn inet_twsk_bind_unhash(tw: *mut inet_timewait_sock, hashinfo: *mut inet_hashinfo);
    pub fn inet_twsk_alloc(
        sk: *const sock,
        dr: *mut inet_timewait_death_row,
        state: ::core::ffi::c_int,
    ) -> *mut inet_timewait_sock;
    pub fn inet_twsk_hashdance_schedule(
        tw: *mut inet_timewait_sock,
        sk: *mut sock,
        hashinfo: *mut inet_hashinfo,
        timeo: ::core::ffi::c_int,
    );
    pub fn __inet_twsk_schedule(
        tw: *mut inet_timewait_sock,
        timeo: ::core::ffi::c_int,
        rearm: bool,
    );
    pub fn inet_twsk_deschedule_put(tw: *mut inet_timewait_sock);
    pub fn inet_twsk_purge(hashinfo: *mut inet_hashinfo);
    pub fn read_pnet(pnet: *const *mut net) -> *mut net;
    pub fn write_pnet(pnet: *mut *mut net, net: *mut net);
}

#[inline]
pub unsafe fn inet_twsk(sk: *const sock) -> *mut inet_timewait_sock {
    sk as *mut inet_timewait_sock
}

#[inline]
pub unsafe fn inet_twsk_reschedule(tw: *mut inet_timewait_sock, timeo: ::core::ffi::c_int) {
    __inet_twsk_schedule(tw, timeo, true);
}

#[inline]
pub unsafe fn twsk_net(twsk: *const inet_timewait_sock) -> *mut net {
    read_pnet(&(*twsk).__tw_common.skc_net)
}

#[inline]
pub unsafe fn twsk_net_set(twsk: *mut inet_timewait_sock, net_ptr: *mut net) {
    write_pnet(&mut (*twsk).__tw_common.skc_net, net_ptr);
}

// #define tw_tclass tw_tos

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
