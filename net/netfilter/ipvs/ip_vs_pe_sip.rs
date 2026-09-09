// SPDX-License-Identifier: GPL-2.0-only
// pr_fmt(fmt) = "IPVS: " fmt

use core::ffi::{c_char, c_int, c_uint, c_void};

// Kernel and netfilter declarations supplied by the surrounding translation.
type U32 = u32;
type Be16 = u16;

#[repr(C)]
pub struct ip_vs_conn_param {
    pub af: c_int,
    pub caddr: *const c_void,
    pub vaddr: *const c_void,
    pub vport: Be16,
    pub protocol: c_uint,
    pub pe_data: *mut c_void,
    pub pe_data_len: c_uint,
}
#[repr(C)] pub struct sk_buff { pub len: c_uint, pub data: *mut u8 }
#[repr(C)] pub struct ip_vs_iphdr { pub protocol: u8, pub len: c_uint }
#[repr(C)] pub struct ip_vs_conn {
    pub af: c_int, pub caddr: *const c_void, pub vaddr: *const c_void,
    pub vport: Be16, pub flags: c_uint, pub protocol: c_uint,
    pub pe_data: *mut c_void, pub pe_data_len: c_uint,
}
#[repr(C)] pub struct ip_vs_service;
#[repr(C)] pub struct ip_vs_dest;
#[repr(C)] pub struct ip_vs_rht { pub hash_key: ip_vs_rht_key }
#[repr(C)] pub struct ip_vs_rht_key { pub key: [u64; 1] }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct atomic_t { pub counter: c_int }
#[repr(C)] pub struct ip_vs_pe {
    pub name: *const c_char,
    pub refcnt: atomic_t,
    pub module: *mut c_void,
    pub n_list: list_head,
    pub fill_param: Option<unsafe extern "C" fn(*mut ip_vs_conn_param, *mut sk_buff) -> c_int>,
    pub ct_match: Option<unsafe extern "C" fn(*const ip_vs_conn_param, *mut ip_vs_conn) -> bool>,
    pub hashkey_raw: Option<unsafe extern "C" fn(*const ip_vs_conn_param, *mut ip_vs_rht, bool) -> U32>,
    pub show_pe_data: Option<unsafe extern "C" fn(*const ip_vs_conn, *mut c_char) -> c_int>,
    pub conn_out: Option<unsafe extern "C" fn(*mut ip_vs_service, *mut ip_vs_dest, *mut sk_buff, *const ip_vs_iphdr, Be16, Be16) -> *mut ip_vs_conn>,
}

extern "C" {
    fn ct_sip_get_header(_: *mut c_void, dptr: *const c_char, dataoff: c_uint, datalen: c_uint,
                         header: c_int, matchoff: *mut c_uint, matchlen: *mut c_uint) -> c_int;
    fn ip_vs_fill_iph_skb(af: c_int, skb: *mut sk_buff, inverse: bool, iph: *mut ip_vs_iphdr) -> c_int;
    fn skb_linearize(skb: *mut sk_buff) -> c_int;
    fn kmemdup(src: *const c_void, len: usize, flags: c_uint) -> *mut c_void;
    fn ip_vs_addr_equal(af: c_int, a: *const c_void, b: *const c_void) -> bool;
    fn jhash(data: *const c_void, len: c_uint, initval: U32) -> U32;
    fn ip_vs_new_conn_out(svc: *mut ip_vs_service, dest: *mut ip_vs_dest, skb: *mut sk_buff,
                          iph: *const ip_vs_iphdr, dport: Be16, cport: Be16) -> *mut ip_vs_conn;
    fn register_ip_vs_pe(pe: *mut ip_vs_pe) -> c_int;
    fn unregister_ip_vs_pe(pe: *mut ip_vs_pe);
    fn synchronize_rcu();
    fn ip_vs_proto_name(protocol: c_uint) -> *const c_char;
}

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const IP_VS_PEDATA_MAXLEN: c_uint = 4096;
const SIP_HDR_CALL_ID: c_int = 0;
const IPPROTO_UDP: u8 = 17;
const IPPROTO_IP: c_uint = 0;
const AF_UNSPEC: c_int = 0;
const IP_VS_CONN_F_TEMPLATE: c_uint = 1 << 0;
const GFP_ATOMIC: c_uint = 0;

#[cfg(CONFIG_IP_VS_DEBUG)]
unsafe fn ip_vs_dbg_callid(buf: *mut c_char, buf_len: usize, callid: *const c_char,
                           callid_len: usize, idx: *mut usize) -> *const c_char {
    let max_len = 64usize;
    let len = core::cmp::min(core::cmp::min(max_len, callid_len), buf_len - *idx - 1);
    core::ptr::copy_nonoverlapping(callid, buf.add(*idx), len);
    *buf.add(*idx + len) = 0;
    *idx += len + 1;
    buf.add(*idx - len)
}

unsafe fn get_callid(dptr: *const c_char, mut dataoff: c_uint, datalen: c_uint,
                     matchoff: *mut c_uint, matchlen: *mut c_uint) -> c_int {
    loop {
        let ret = ct_sip_get_header(core::ptr::null_mut(), dptr, dataoff, datalen,
                                    SIP_HDR_CALL_ID, matchoff, matchlen);
        if ret > 0 { break; }
        if ret == 0 { return -EINVAL; }
        dataoff += *matchoff;
    }
    if *matchlen > IP_VS_PEDATA_MAXLEN || *matchoff + *matchlen == datalen { return -EINVAL; }
    let term = *dptr.add((*matchoff + *matchlen) as usize) as u8;
    if term != b'\r' && term != b'\n' { return -EINVAL; }
    0
}

unsafe extern "C" fn ip_vs_sip_fill_param(p: *mut ip_vs_conn_param, skb: *mut sk_buff) -> c_int {
    let mut iph = core::mem::zeroed::<ip_vs_iphdr>();
    let retc = ip_vs_fill_iph_skb((*p).af, skb, false, &mut iph);
    if retc == 0 || iph.protocol != IPPROTO_UDP { return -EINVAL; }
    let dataoff = iph.len + core::mem::size_of::<u16>() as c_uint * 4;
    if dataoff >= (*skb).len { return -EINVAL; }
    let retc = skb_linearize(skb);
    if retc < 0 { return retc; }
    let dptr = (*skb).data.add(dataoff as usize) as *const c_char;
    let datalen = (*skb).len - dataoff;
    let (mut matchoff, mut matchlen) = (0u32, 0u32);
    if get_callid(dptr, 0, datalen, &mut matchoff, &mut matchlen) != 0 { return -EINVAL; }
    (*p).pe_data = kmemdup(dptr.add(matchoff as usize), matchlen as usize, GFP_ATOMIC);
    if (*p).pe_data.is_null() { return -ENOMEM; }
    (*p).pe_data_len = matchlen;
    0
}

unsafe extern "C" fn ip_vs_sip_ct_match(p: *const ip_vs_conn_param, ct: *mut ip_vs_conn) -> bool {
    let ret = (*ct).af == (*p).af &&
        ip_vs_addr_equal((*p).af, (*p).caddr, (*ct).caddr) &&
        ip_vs_addr_equal(if (*p).protocol == IPPROTO_IP { AF_UNSPEC } else { (*p).af }, (*p).vaddr, (*ct).vaddr) &&
        (*ct).vport == (*p).vport && ((*ct).flags & IP_VS_CONN_F_TEMPLATE) != 0 &&
        (*ct).protocol == (*p).protocol && !(*ct).pe_data.is_null() &&
        (*ct).pe_data_len == (*p).pe_data_len &&
        libc_memcmp((*ct).pe_data, (*p).pe_data, (*p).pe_data_len as usize) == 0;
    ret
}

extern "C" { fn libc_memcmp(a: *const c_void, b: *const c_void, n: usize) -> c_int; }

unsafe extern "C" fn ip_vs_sip_hashkey_raw(p: *const ip_vs_conn_param, t: *mut ip_vs_rht, _inverse: bool) -> U32 {
    jhash((*p).pe_data, (*p).pe_data_len, (*t).hash_key.key[0] as U32)
}
unsafe extern "C" fn ip_vs_sip_show_pe_data(cp: *const ip_vs_conn, buf: *mut c_char) -> c_int {
    core::ptr::copy_nonoverlapping((*cp).pe_data as *const u8, buf as *mut u8, (*cp).pe_data_len as usize);
    (*cp).pe_data_len as c_int
}
unsafe extern "C" fn ip_vs_sip_conn_out(svc: *mut ip_vs_service, dest: *mut ip_vs_dest, skb: *mut sk_buff,
                                          iph: *const ip_vs_iphdr, dport: Be16, cport: Be16) -> *mut ip_vs_conn {
    if (*iph).protocol == IPPROTO_UDP { ip_vs_new_conn_out(svc, dest, skb, iph, dport, cport) } else { core::ptr::null_mut() }
}

#[no_mangle] pub static mut ip_vs_sip_pe: ip_vs_pe = ip_vs_pe {
    name: b"sip\0".as_ptr() as *const c_char, refcnt: atomic_t { counter: 0 }, module: core::ptr::null_mut(),
    n_list: list_head { next: core::ptr::null_mut(), prev: core::ptr::null_mut() },
    fill_param: Some(ip_vs_sip_fill_param), ct_match: Some(ip_vs_sip_ct_match), hashkey_raw: Some(ip_vs_sip_hashkey_raw),
    show_pe_data: Some(ip_vs_sip_show_pe_data), conn_out: Some(ip_vs_sip_conn_out),
};

#[no_mangle] pub unsafe extern "C" fn ip_vs_sip_init() -> c_int { register_ip_vs_pe(&mut ip_vs_sip_pe) }
#[no_mangle] pub unsafe extern "C" fn ip_vs_sip_cleanup() { unregister_ip_vs_pe(&mut ip_vs_sip_pe); synchronize_rcu(); }

// module_init(ip_vs_sip_init); module_exit(ip_vs_sip_cleanup);
// MODULE_LICENSE("GPL"); MODULE_DESCRIPTION("ipvs sip helper");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
