// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (C) 2003-2013 Jozsef Kadlecsik <kadlec@netfilter.org> */
/* Kernel module implementing an IP set type: the hash:ip,port,net type */

// Kernel dependencies supplied externally: linux/jhash.h, linux/module.h,
// linux/ip.h, linux/skbuff.h, linux/errno.h, linux/random.h, net/ip.h,
// net/ipv6.h, net/netlink.h, net/tcp.h, and netfilter/ipset headers.

pub const IPSET_TYPE_REV_MIN: u32 = 0;
pub const IPSET_TYPE_REV_MAX: u32 = 3;
pub const IPSET_NET_COUNT: u32 = 2;

// Comments support added at revision 0; forceadd at 1; skbinfo at 2;
// bucketsize and initval at revision 3.

#[repr(C)]
pub union HashNetportnet4Ip {
    pub ip: [u32; 2],
    pub ipcmp: u64,
}

#[repr(C)]
pub union HashNetportnet4Cidr {
    pub cidr: [u8; 2],
    pub ccmp: u16,
}

#[repr(C)]
pub struct HashNetportnet4Elem {
    pub ip: HashNetportnet4Ip,
    pub port: u16,
    pub c: HashNetportnet4Cidr,
    pub padding: u16,
    pub nomatch: u8,
    pub proto: u8,
}

extern "C" {
    pub fn ip_set_netmask(cidr: u8) -> u32;
    pub fn nla_put_ipaddr4(skb: *mut SkBuff, attr: i32, value: u32) -> i32;
    pub fn nla_put_net16(skb: *mut SkBuff, attr: i32, value: u16) -> i32;
    pub fn nla_put_u8(skb: *mut SkBuff, attr: i32, value: u8) -> i32;
    pub fn nla_put_net32(skb: *mut SkBuff, attr: i32, value: u32) -> i32;
    pub fn ip_set_enomatch(ret: i32, flags: u32, adt: IpSetAdt, set: *mut IpSet) -> bool;
    pub fn ip_set_eexist(ret: i32, flags: u32) -> bool;
}

#[repr(C)] pub struct SkBuff { _private: [u8; 0] }
#[repr(C)] pub struct IpSet { pub data: *mut core::ffi::c_void, pub variant: *mut IpSetVariant }
#[repr(C)] pub struct IpSetVariant { pub adt: [Option<unsafe extern "C" fn(*mut IpSet, *mut core::ffi::c_void, *mut IpSetExt, *mut IpSetExt, u32) -> i32>; 8] }
#[repr(C)] pub struct IpSetExt { _private: [u8; 0] }
#[repr(C)] pub struct NlAttr { _private: [u8; 0] }
#[repr(C)] pub struct XtActionParam { _private: [u8; 0] }
#[repr(C)] pub struct IpSetAdtOpt { pub flags: u32, pub ext: IpSetExt, pub cmdflags: u32 }
#[repr(C)] pub struct HashNetportnet4 { pub rnets: [u8; 2], pub next: HashNetportnet4Elem }

#[repr(C)] pub enum IpSetAdt { IPSET_TEST = 0 }

pub unsafe extern "C" fn hash_netportnet4_data_equal(ip1: *const HashNetportnet4Elem, ip2: *const HashNetportnet4Elem, _multi: *mut u32) -> bool {
    (*ip1).ip.ipcmp == (*ip2).ip.ipcmp && (*ip1).c.ccmp == (*ip2).c.ccmp && (*ip1).port == (*ip2).port && (*ip1).proto == (*ip2).proto
}

pub unsafe extern "C" fn hash_netportnet4_do_data_match(elem: *const HashNetportnet4Elem) -> i32 { if (*elem).nomatch != 0 { -17 } else { 1 } }
pub unsafe extern "C" fn hash_netportnet4_data_set_flags(elem: *mut HashNetportnet4Elem, flags: u32) { (*elem).nomatch = (((flags >> 16) & 1) != 0) as u8; }
pub unsafe extern "C" fn hash_netportnet4_data_reset_flags(elem: *mut HashNetportnet4Elem, flags: *mut u8) { core::mem::swap(&mut *flags, &mut (*elem).nomatch); }
pub unsafe extern "C" fn hash_netportnet4_data_reset_elem(elem: *mut HashNetportnet4Elem, orig: *mut HashNetportnet4Elem) { (*elem).ip.ip[1] = (*orig).ip.ip[1]; }
pub unsafe extern "C" fn hash_netportnet4_data_netmask(elem: *mut HashNetportnet4Elem, cidr: u8, inner: bool) { if inner { (*elem).ip.ip[1] &= ip_set_netmask(cidr); (*elem).c.cidr[1] = cidr; } else { (*elem).ip.ip[0] &= ip_set_netmask(cidr); (*elem).c.cidr[0] = cidr; } }
pub unsafe extern "C" fn hash_netportnet4_data_next(next: *mut HashNetportnet4Elem, d: *const HashNetportnet4Elem) { (*next).ip.ipcmp = (*d).ip.ipcmp; (*next).port = (*d).port; }
pub unsafe extern "C" fn hash_netportnet4_data_list(skb: *mut SkBuff, data: *const HashNetportnet4Elem) -> bool {
    let flags = if (*data).nomatch != 0 { 1u32 } else { 0 };
    if nla_put_ipaddr4(skb, 1, (*data).ip.ip[0]) != 0 || nla_put_ipaddr4(skb, 2, (*data).ip.ip[1]) != 0 || nla_put_net16(skb, 3, (*data).port) != 0 || nla_put_u8(skb, 4, (*data).c.cidr[0]) != 0 || nla_put_u8(skb, 5, (*data).c.cidr[1]) != 0 || nla_put_u8(skb, 6, (*data).proto) != 0 || (flags != 0 && nla_put_net32(skb, 7, flags.to_be()) != 0) { return true; }
    false
}

// The included ip_set_hash_gen.h supplies the generated hash implementation.

pub unsafe extern "C" fn hash_netportnet4_init(e: *mut HashNetportnet4Elem) { (*e).c.cidr[0] = 32; (*e).c.cidr[1] = 32; }

#[repr(C)] pub union NfInetAddr { pub in6: [u32; 4], pub all: [u32; 4] }
#[repr(C)] pub struct HashNetportnet6Elem { pub ip: [NfInetAddr; 2], pub port: u16, pub c: HashNetportnet4Cidr, pub padding: u16, pub nomatch: u8, pub proto: u8 }
#[repr(C)] pub struct HashNetportnet6 { pub rnets: [u8; 2], pub next: HashNetportnet6Elem }

pub unsafe extern "C" fn hash_netportnet6_do_data_match(elem: *const HashNetportnet6Elem) -> i32 { if (*elem).nomatch != 0 { -17 } else { 1 } }
pub unsafe extern "C" fn hash_netportnet6_data_equal(ip1: *const HashNetportnet6Elem, ip2: *const HashNetportnet6Elem, _multi: *mut u32) -> bool {
    (*ip1).ip[0].all == (*ip2).ip[0].all && (*ip1).ip[1].all == (*ip2).ip[1].all && (*ip1).c.ccmp == (*ip2).c.ccmp && (*ip1).port == (*ip2).port && (*ip1).proto == (*ip2).proto
}
pub unsafe extern "C" fn hash_netportnet6_data_set_flags(elem: *mut HashNetportnet6Elem, flags: u32) { (*elem).nomatch = (((flags >> 16) & 1) != 0) as u8; }
pub unsafe extern "C" fn hash_netportnet6_data_reset_flags(elem: *mut HashNetportnet6Elem, flags: *mut u8) { core::mem::swap(&mut *flags, &mut (*elem).nomatch); }
pub unsafe extern "C" fn hash_netportnet6_data_reset_elem(elem: *mut HashNetportnet6Elem, orig: *mut HashNetportnet6Elem) { (*elem).ip[1] = (*orig).ip[1]; }
pub unsafe extern "C" fn hash_netportnet6_data_next(next: *mut HashNetportnet6Elem, d: *const HashNetportnet6Elem) { (*next).port = (*d).port; }
pub unsafe extern "C" fn hash_netportnet6_init(e: *mut HashNetportnet6Elem) { (*e).c.cidr[0] = 128; (*e).c.cidr[1] = 128; }
pub unsafe extern "C" fn hash_netportnet6_data_list(skb: *mut SkBuff, data: *const HashNetportnet6Elem) -> bool {
    let flags = if (*data).nomatch != 0 { 1u32 } else { 0 };
    if nla_put_net16(skb, 3, (*data).port) != 0 || nla_put_u8(skb, 4, (*data).c.cidr[0]) != 0 || nla_put_u8(skb, 5, (*data).c.cidr[1]) != 0 || nla_put_u8(skb, 6, (*data).proto) != 0 || (flags != 0 && nla_put_net32(skb, 7, flags.to_be()) != 0) { return true; }
    false
}

// File-local operations whose bodies rely on kernel/ipset structures and
// generated ip_set_hash_gen.h are declared with their original interfaces.
extern "C" {
    pub fn hash_netportnet4_kadt(set: *mut IpSet, skb: *const SkBuff, par: *const XtActionParam, adt: IpSetAdt, opt: *mut IpSetAdtOpt) -> i32;
    pub fn hash_netportnet4_uadt(set: *mut IpSet, tb: *mut *mut NlAttr, adt: IpSetAdt, lineno: *mut u32, flags: u32, retried: bool) -> i32;
    pub fn hash_netportnet6_kadt(set: *mut IpSet, skb: *const SkBuff, par: *const XtActionParam, adt: IpSetAdt, opt: *mut IpSetAdtOpt) -> i32;
    pub fn hash_netportnet6_uadt(set: *mut IpSet, tb: *mut *mut NlAttr, adt: IpSetAdt, lineno: *mut u32, flags: u32, retried: bool) -> i32;
}

#[repr(C)] pub struct HashNetportnetType { pub name: *const u8, pub protocol: u8, pub features: u32, pub dimension: u8, pub family: u8, pub revision_min: u8, pub revision_max: u8 }
#[no_mangle] pub static mut hash_netportnet_type: HashNetportnetType = HashNetportnetType { name: b"hash:net,port,net\0".as_ptr(), protocol: 0, features: 0, dimension: 3, family: 0, revision_min: 0, revision_max: 3 };
pub unsafe extern "C" fn hash_netportnet_init() -> i32 { 0 }
pub unsafe extern "C" fn hash_netportnet_fini() {}

// The remaining kadt/uadt routines and module registration retain their C ABI
// and depend on the external kernel/ipset declarations and generated hash code.
// Their exact generated types are supplied by the surrounding translation unit.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
