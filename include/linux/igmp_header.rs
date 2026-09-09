/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Linux NET3: Internet Group Management Protocol [IGMP]
 *
 * Authors:
 *     Alan Cox <alan@lxorguk.ukuu.org.uk>
 *
 * Extended to talk the BSD extended IGMP protocol of mrouted 3.6
 */

// Dependencies supplied by the corresponding Linux headers.

#[inline]
pub unsafe fn igmp_hdr(skb: *const sk_buff) -> *mut igmphdr {
    skb_transport_header(skb) as *mut igmphdr
}

#[inline]
pub unsafe fn igmpv3_report_hdr(skb: *const sk_buff) -> *mut igmpv3_report {
    skb_transport_header(skb) as *mut igmpv3_report
}

#[inline]
pub unsafe fn igmpv3_query_hdr(skb: *const sk_buff) -> *mut igmpv3_query {
    skb_transport_header(skb) as *mut igmpv3_query
}

#[repr(C)]
pub struct ip_sf_socklist {
    pub sl_max: ::core::ffi::c_uint,
    pub sl_count: ::core::ffi::c_uint,
    pub rcu: rcu_head,
    // __be32 sl_addr[] __counted_by(sl_max);
    pub sl_addr: [__be32; 0],
}

pub const IP_SFBLOCK: ::core::ffi::c_uint = 10; // allocate this many at once

/* ip_mc_socklist is real list now. Speed is not argument;
 * this list never used in fast path code
 */
#[repr(C)]
pub struct ip_mc_socklist {
    pub next_rcu: *mut ip_mc_socklist,
    pub multi: ip_mreqn,
    pub sfmode: ::core::ffi::c_uint, // MCAST_{INCLUDE,EXCLUDE}
    pub sflist: *mut ip_sf_socklist,
    pub rcu: rcu_head,
}

#[repr(C)]
pub struct ip_sf_list {
    pub sf_next: *mut ip_sf_list,
    pub sf_count: [::core::ffi::c_ulong; 2], // include/exclude counts
    pub sf_inaddr: __be32,
    pub sf_gsresp: ::core::ffi::c_uchar, // include in g & s response?
    pub sf_oldin: ::core::ffi::c_uchar, // change state
    pub sf_crcount: ::core::ffi::c_uchar, // retrans. left to send
}

#[repr(C)]
pub union ip_mc_list_next {
    pub next: *mut ip_mc_list,
    pub next_rcu: *mut ip_mc_list,
}

#[repr(C)]
pub struct ip_mc_list {
    pub interface: *mut in_device,
    pub multiaddr: __be32,
    pub sfmode: ::core::ffi::c_uint,
    pub sources: *mut ip_sf_list,
    pub tomb: *mut ip_sf_list,
    pub sfcount: [::core::ffi::c_ulong; 2],
    pub next_union: ip_mc_list_next,
    pub next_hash: *mut ip_mc_list,
    pub timer: timer_list,
    pub users: ::core::ffi::c_int,
    pub refcnt: refcount_t,
    pub lock: spinlock_t,
    pub tm_running: ::core::ffi::c_char,
    pub reporter: ::core::ffi::c_char,
    pub unsolicit_count: ::core::ffi::c_char,
    pub loaded: ::core::ffi::c_char,
    pub gsquery: ::core::ffi::c_uchar, // check source marks?
    pub crcount: ::core::ffi::c_uchar,
    pub mca_cstamp: ::core::ffi::c_ulong,
    pub mca_tstamp: ::core::ffi::c_ulong,
    pub rcu: rcu_head,
}

#[inline]
pub const fn IGMPV3_FP_EXP(value: u8) -> u8 { (value >> 4) & 0x07 }
#[inline]
pub const fn IGMPV3_FP_MAN(value: u8) -> u8 { value & 0x0f }
pub const IGMPV3_EXP_MIN_THRESHOLD: ::core::ffi::c_uint = 128;
pub const IGMPV3_EXP_MAX_THRESHOLD: ::core::ffi::c_uint = 31744;

#[inline]
pub unsafe fn igmpv3_exp_field_encode(value: ::core::ffi::c_ulong) -> u8 {
    if value < IGMPV3_EXP_MIN_THRESHOLD as _ { return value as u8; }
    if value >= IGMPV3_EXP_MAX_THRESHOLD as _ { return 0xff; }
    let mc_exp = (fls(value as _) - 8) as u32;
    let mc_man = ((value >> (mc_exp + 3)) & 0x0f) as u8;
    0x80 | ((mc_exp as u8) << 4) | mc_man
}

#[inline]
pub unsafe fn igmpv3_mrc(mrt: ::core::ffi::c_ulong) -> u8 { igmpv3_exp_field_encode(mrt) }
#[inline]
pub unsafe fn igmpv3_qqic(qi: ::core::ffi::c_ulong) -> u8 { igmpv3_exp_field_encode(qi) }

#[inline]
pub unsafe fn igmpv3_exp_field_decode(code: u8) -> ::core::ffi::c_ulong {
    if code < IGMPV3_EXP_MIN_THRESHOLD as u8 { code as _ }
    else {
        let mc_exp = IGMPV3_FP_EXP(code);
        let mc_man = IGMPV3_FP_MAN(code);
        ((mc_man as ::core::ffi::c_ulong | 0x10) << (mc_exp + 3))
    }
}

#[inline]
pub unsafe fn igmpv3_mrt(ih3: *const igmpv3_query) -> ::core::ffi::c_ulong {
    igmpv3_exp_field_decode((*ih3).code)
}
#[inline]
pub unsafe fn igmpv3_qqi(ih3: *const igmpv3_query) -> ::core::ffi::c_ulong {
    igmpv3_exp_field_decode((*ih3).qqic)
}

#[inline]
pub unsafe fn ip_mc_may_pull(skb: *mut sk_buff, len: ::core::ffi::c_uint) -> ::core::ffi::c_int {
    if skb_transport_offset(skb) + ip_transport_len(skb) < len { 0 } else { pskb_may_pull(skb, len) }
}

extern "C" {
    pub fn ip_check_mc_rcu(dev: *mut in_device, mc_addr: __be32, src_addr: __be32, proto: u8) -> ::core::ffi::c_int;
    pub fn igmp_rcv(skb: *mut sk_buff) -> ::core::ffi::c_int;
    pub fn ip_mc_join_group(sk: *mut sock, imr: *mut ip_mreqn) -> ::core::ffi::c_int;
    pub fn ip_mc_join_group_ssm(sk: *mut sock, imr: *mut ip_mreqn, mode: ::core::ffi::c_uint) -> ::core::ffi::c_int;
    pub fn ip_mc_leave_group(sk: *mut sock, imr: *mut ip_mreqn) -> ::core::ffi::c_int;
    pub fn ip_mc_drop_socket(sk: *mut sock);
    pub fn ip_mc_source(add: ::core::ffi::c_int, omode: ::core::ffi::c_int, sk: *mut sock, mreqs: *mut ip_mreq_source, ifindex: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn ip_mc_msfilter(sk: *mut sock, msf: *mut ip_msfilter, ifindex: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn ip_mc_msfget(sk: *mut sock, msf: *mut ip_msfilter, optval: sockptr_t, optlen: sockptr_t) -> ::core::ffi::c_int;
    pub fn ip_mc_gsfget(sk: *mut sock, gsf: *mut group_filter, optval: sockptr_t, offset: usize) -> ::core::ffi::c_int;
    pub fn ip_mc_sf_allow(sk: *const sock, local: __be32, rmt: __be32, dif: ::core::ffi::c_int, sdif: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn ip_mc_init_dev(dev: *mut in_device);
    pub fn ip_mc_destroy_dev(dev: *mut in_device);
    pub fn ip_mc_up(dev: *mut in_device);
    pub fn ip_mc_down(dev: *mut in_device);
    pub fn ip_mc_unmap(dev: *mut in_device);
    pub fn ip_mc_remap(dev: *mut in_device);
    pub fn __ip_mc_dec_group(in_dev: *mut in_device, addr: __be32, gfp: gfp_t);
    pub fn __ip_mc_inc_group(in_dev: *mut in_device, addr: __be32, gfp: gfp_t);
    pub fn ip_mc_inc_group(in_dev: *mut in_device, addr: __be32);
    pub fn ip_mc_check_igmp(skb: *mut sk_buff) -> ::core::ffi::c_int;
}

#[inline]
pub unsafe fn ip_mc_dec_group(in_dev: *mut in_device, addr: __be32) {
    __ip_mc_dec_group(in_dev, addr, GFP_KERNEL);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
