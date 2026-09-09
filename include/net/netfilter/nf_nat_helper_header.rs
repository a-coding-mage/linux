/* SPDX-License-Identifier: GPL-2.0 */
/* NAT protocol helper routines. */

use core::ffi::c_char;

/* Dependencies supplied by the surrounding translation unit. */

/* These return true or false. */
extern "C" {
    pub fn __nf_nat_mangle_tcp_packet(
        skb: *mut sk_buff,
        ct: *mut nf_conn,
        ctinfo: ip_conntrack_info,
        protoff: u32,
        match_offset: u32,
        match_len: u32,
        rep_buffer: *const c_char,
        rep_len: u32,
        adjust: bool,
    ) -> bool;

    pub fn nf_nat_mangle_udp_packet(
        skb: *mut sk_buff,
        ct: *mut nf_conn,
        ctinfo: ip_conntrack_info,
        protoff: u32,
        match_offset: u32,
        match_len: u32,
        rep_buffer: *const c_char,
        rep_len: u32,
    ) -> bool;

    /* Setup NAT on this expected conntrack so it follows master, but goes
     * to port ct->master->saved_proto. */
    pub fn nf_nat_follow_master(ct: *mut nf_conn, this: *mut nf_conntrack_expect);

    pub fn nf_nat_exp_find_port(exp: *mut nf_conntrack_expect, port: u16) -> u16;
}

#[inline]
pub unsafe fn nf_nat_mangle_tcp_packet(
    skb: *mut sk_buff,
    ct: *mut nf_conn,
    ctinfo: ip_conntrack_info,
    protoff: u32,
    match_offset: u32,
    match_len: u32,
    rep_buffer: *const c_char,
    rep_len: u32,
) -> bool {
    __nf_nat_mangle_tcp_packet(
        skb,
        ct,
        ctinfo,
        protoff,
        match_offset,
        match_len,
        rep_buffer,
        rep_len,
        true,
    )
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
