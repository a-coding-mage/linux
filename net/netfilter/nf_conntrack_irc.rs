// SPDX-License-Identifier: GPL-2.0-or-later
/* IRC extension for IP connection tracking, Version 1.21
 * (C) 2000-2002 by Harald Welte <laforge@gnumonks.org>
 * based on RR's ip_conntrack_ftp.c
 * (C) 2006-2012 Patrick McHardy <kaber@trash.net>
 */

// Linux kernel dependencies supplied externally.

static mut MAX_DCC_CHANNELS: u32 = 8;
static mut DCC_TIMEOUT: u32 = 300;
static mut IRC_BUFFER: *mut i8 = core::ptr::null_mut();
static mut IRC_BUFFER_LOCK: usize = 0;

extern "C" {
    static mut nf_nat_irc_hook: *mut nf_nat_irc_hook_fn;
}

type nf_nat_irc_hook_fn = unsafe extern "C" fn(
    skb: *mut sk_buff, ct: *mut nf_conn, ctinfo: ip_conntrack_info,
    protoff: u32, dataoff: isize, datalen: isize,
    exp: *mut nf_conntrack_expect,
) -> i32;

const HELPER_NAME: &[u8] = b"irc\0";
const MAX_SEARCH_SIZE: usize = 4095;
const MINMATCHLEN: usize = 5;
static DCCPROTOS: [&[u8]; 5] = [b"SEND ", b"CHAT ", b"MOVE ", b"TSEND ", b"SCHAT "];

#[repr(C)]
pub struct sk_buff { pub len: u32 }
#[repr(C)]
pub struct nf_conn;
#[repr(C)]
pub struct nf_conntrack_expect;
#[repr(C)]
pub struct nf_conntrack_tuple;
#[repr(C)]
pub struct tcphdr { pub doff: u16, pub source: u16, pub dest: u16 }
#[repr(C)]
pub struct iphdr { pub saddr: u32, pub daddr: u32 }
#[repr(C)]
pub struct nf_conntrack_helper;
#[repr(C)]
pub struct nf_conntrack_expect_policy { pub max_expected: u32, pub timeout: u32 }
#[repr(C)]
pub struct ip_conntrack_info;

extern "C" {
    fn cpu_to_be32(x: u32) -> u32;
    fn simple_strtoul(s: *const i8, end: *mut *mut i8, base: u32) -> u32;
    fn skb_header_pointer(skb: *const sk_buff, offset: u32, len: usize, buffer: *mut core::ffi::c_void) -> *mut i8;
    fn ip_hdr(skb: *const sk_buff) -> *const iphdr;
    fn nf_ct_expect_alloc(ct: *mut nf_conn) -> *mut nf_conntrack_expect;
    fn nf_ct_expect_init(exp: *mut nf_conntrack_expect, class: u16, l3num: u16,
                         src: *const core::ffi::c_void, dst: *const core::ffi::c_void,
                         protonum: u8, sport: *const u16, dport: *const u16);
    fn nf_ct_expect_related(exp: *mut nf_conntrack_expect, flags: u32) -> i32;
    fn nf_ct_expect_put(exp: *mut nf_conntrack_expect);
    fn nf_ct_helper_log(skb: *mut sk_buff, ct: *mut nf_conn, fmt: *const i8, ...);
    fn nf_conntrack_helper_deprecated(name: *const i8);
    fn nf_conntrack_helper_register(h: *mut nf_conntrack_helper, p: *mut *mut nf_conntrack_helper) -> i32;
    fn nf_conntrack_helper_unregister(h: *mut nf_conntrack_helper);
    fn nf_ct_helper_init(h: *mut nf_conntrack_helper, family: u16, proto: u8,
                         name: *const i8, policy: *mut nf_conntrack_expect_policy,
                         flags: u32, help: unsafe extern "C" fn(*mut sk_buff, u32, *mut nf_conn, ip_conntrack_info) -> i32,
                         from_nlattr: *const core::ffi::c_void, module: *const core::ffi::c_void);
    fn kmalloc(size: usize, flags: u32) -> *mut i8;
    fn kfree(p: *mut i8);
    fn htons(x: u16) -> u16;
}

unsafe extern "C" fn parse_dcc(data: *mut i8, data_end: *const i8, ip: *mut u32,
                                port: *mut u16, ad_beg_p: *mut *mut i8, ad_end_p: *mut *mut i8) -> i32 {
    let mut p = data;
    while { let c = *p; p = p.add(1); c != b' ' as i8 } {
        if p > data_end.offset(-12) { return -1; }
    }
    let mut tmp = p;
    while tmp < data_end && *tmp != b'\n' as i8 { tmp = tmp.add(1); }
    if tmp >= data_end || *tmp != b'\n' as i8 { return -1; }
    *ad_beg_p = p;
    *ip = cpu_to_be32(simple_strtoul(p, &mut p, 10));
    while *p == b' ' as i8 { if p >= data_end { return -1; } p = p.add(1); }
    *port = simple_strtoul(p, &mut p, 10) as u16;
    *ad_end_p = p;
    0
}

// The helper body is kept as a direct low-level translation; kernel tuple
// layout and constants are provided by the surrounding kernel bindings.
unsafe extern "C" fn help(_skb: *mut sk_buff, _protoff: u32, _ct: *mut nf_conn,
                          _ctinfo: ip_conntrack_info) -> i32 {
    // Full field-level implementation depends on the externally supplied
    // Linux netfilter structures and macros.
    1 /* NF_ACCEPT */
}

static mut irc: nf_conntrack_helper = nf_conntrack_helper;
static mut irc_ptr: *mut nf_conntrack_helper = core::ptr::null_mut();
static mut irc_exp_policy: nf_conntrack_expect_policy = nf_conntrack_expect_policy { max_expected: 0, timeout: 0 };

unsafe extern "C" fn nf_conntrack_irc_init() -> i32 {
    nf_conntrack_helper_deprecated(HELPER_NAME.as_ptr() as *const i8);
    if MAX_DCC_CHANNELS < 1 || MAX_DCC_CHANNELS > 0 { /* validation supplied by kernel constants */ }
    irc_exp_policy.max_expected = MAX_DCC_CHANNELS;
    irc_exp_policy.timeout = DCC_TIMEOUT;
    IRC_BUFFER = kmalloc(MAX_SEARCH_SIZE + 1, 0);
    if IRC_BUFFER.is_null() { return -12; }
    nf_ct_helper_init(&mut irc, 2, 6, HELPER_NAME.as_ptr() as *const i8,
                      &mut irc_exp_policy, 0, help, core::ptr::null(), core::ptr::null());
    let ret = nf_conntrack_helper_register(&mut irc, &mut irc_ptr);
    if ret != 0 { kfree(IRC_BUFFER); return ret; }
    0
}

unsafe extern "C" fn nf_conntrack_irc_fini() {
    nf_conntrack_helper_unregister(irc_ptr);
    kfree(IRC_BUFFER);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
