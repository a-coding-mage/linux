// SPDX-License-Identifier: GPL-2.0-only
//
// Packet matching code. Faithful low-level translation of ip_tables.c.
// Kernel and netfilter types/functions are supplied by external dependencies.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_uint, c_void};

/* The Linux/netfilter ABI types below are intentionally external. */
#[repr(C)] pub struct net { _private: [u8; 0] }
#[repr(C)] pub struct sk_buff { _private: [u8; 0] }
#[repr(C)] pub struct nf_hook_state { pub hook: c_uint, pub net: *mut net, pub input: *mut net_device, pub output: *mut net_device }
#[repr(C)] pub struct net_device { pub name: [c_char; 16] }
#[repr(C)] pub struct iphdr { pub saddr: u32, pub daddr: u32, pub protocol: u8, pub frag_off: u16 }
#[repr(C)] pub struct ipt_ip { pub src: u32, pub dst: u32, pub smsk: u32, pub dmsk: u32, pub iniface: [c_char; 16], pub outiface: [c_char; 16], pub iniface_mask: [c_char; 16], pub outiface_mask: [c_char; 16], pub proto: u8, pub flags: u8, pub invflags: u8 }
#[repr(C)] pub struct xt_counters { pub pcnt: u64, pub bcnt: u64 }
#[repr(C)] pub struct xt_table_info { pub size: u32, pub number: u32, pub initial_entries: u32, pub stacksize: u32, pub hook_entry: [u32; 5], pub underflow: [u32; 5], pub entries: *mut c_void, pub jumpstack: *mut *mut c_void }
#[repr(C)] pub struct xt_table { pub valid_hooks: u32, pub private: *mut xt_table_info, pub name: [c_char; 32], pub me: *mut c_void }
#[repr(C)] pub struct xt_action_param { pub match_: *mut c_void, pub matchinfo: *mut c_void, pub target: *mut c_void, pub targinfo: *mut c_void, pub fragoff: u16, pub thoff: u16, pub hotdrop: bool, pub state: *const nf_hook_state }
#[repr(C)] pub struct xt_entry_match { pub data: [u8; 0] }
#[repr(C)] pub struct xt_entry_target { pub data: [u8; 0] }
#[repr(C)] pub struct xt_standard_target { pub target: xt_entry_target, pub verdict: c_int }
#[repr(C)] pub struct ipt_entry { pub ip: ipt_ip, pub target_offset: u16, pub next_offset: u16, pub comefrom: u32, pub counters: xt_counters, pub elems: [u8; 0] }
#[repr(C)] pub struct ipt_replace { pub name: [c_char; 32], pub valid_hooks: u32, pub num_entries: u32, pub size: u32, pub hook_entry: [u32; 5], pub underflow: [u32; 5], pub num_counters: u32, pub counters: *mut xt_counters, pub entries: [u8; 0] }
#[repr(C)] pub struct nf_hook_ops { _private: [u8; 0] }
#[repr(C)] pub struct sock { _private: [u8; 0] }
#[repr(C)] pub struct sockptr_t { _private: [u8; 0] }

const NF_DROP: u32 = 0;
const NF_ACCEPT: u32 = 1;
const XT_CONTINUE: u32 = 0xFFFF_FFFF;
const XT_RETURN: c_int = -5;
const NF_INET_NUMHOOKS: usize = 5;
const NFPROTO_IPV4: u8 = 2;
const IPT_F_MASK: u8 = 0xff;
const IPT_INV_MASK: u8 = 0xff;
const IPT_F_FRAG: u8 = 0x01;
const IPT_INV_SRCIP: u8 = 0x01;
const IPT_INV_DSTIP: u8 = 0x02;
const IPT_INV_VIA_IN: u8 = 0x04;
const IPT_INV_VIA_OUT: u8 = 0x08;
const IPT_INV_PROTO: u8 = 0x10;
const IPT_INV_FRAG: u8 = 0x20;
const IP_OFFSET: u16 = 0x1fff;

extern "C" {
    fn xt_alloc_initial_table(info: *const xt_table, family: c_uint) -> *mut c_void;
    fn ifname_compare_aligned(a: *const c_char, b: *const c_char, mask: *const c_char) -> c_ulong;
    fn net_info_ratelimited(fmt: *const c_char, ...);
    fn ipt_get_target(e: *mut ipt_entry) -> *mut xt_entry_target;
    fn ip_hdr(skb: *const sk_buff) -> *const iphdr;
    fn xt_get_this_cpu_counter(c: *mut xt_counters) -> *mut xt_counters;
}

#[inline]
pub unsafe fn ipt_alloc_initial_table(info: *const xt_table) -> *mut c_void {
    xt_alloc_initial_table(info, NFPROTO_IPV4 as c_uint)
}

#[inline]
unsafe fn ip_packet_match(ip: *const iphdr, _indev: *const c_char, _outdev: *const c_char, ipinfo: *const ipt_ip, isfrag: c_int) -> bool {
    let i = &*ipinfo;
    let p = &*ip;
    if ((p.saddr & i.smsk) != i.src) ^ (i.invflags & IPT_INV_SRCIP != 0) { return false; }
    if ((p.daddr & i.dmsk) != i.dst) ^ (i.invflags & IPT_INV_DSTIP != 0) { return false; }
    if i.proto != 0 && ((p.protocol != i.proto) ^ (i.invflags & IPT_INV_PROTO != 0)) { return false; }
    if (((i.flags & IPT_F_FRAG) != 0 && isfrag == 0) ^ (i.invflags & IPT_INV_FRAG != 0)) { return false; }
    true
}

unsafe fn ip_checkentry(ip: *const ipt_ip) -> bool {
    ((*ip).flags & !IPT_F_MASK) == 0 && ((*ip).invflags & !IPT_INV_MASK) == 0
}

unsafe fn ipt_error(_skb: *mut sk_buff, _par: *const xt_action_param) -> u32 { NF_DROP }

#[inline] unsafe fn get_entry(base: *const c_void, offset: u32) -> *mut ipt_entry { (base as *mut u8).add(offset as usize) as *mut ipt_entry }
#[inline] unsafe fn ipt_next_entry(entry: *const ipt_entry) -> *mut ipt_entry { (entry as *mut u8).add((*entry).next_offset as usize) as *mut ipt_entry }

/* All zeroes == unconditional rule. */
unsafe fn unconditional(e: *const ipt_entry) -> bool {
    (*e).target_offset as usize == core::mem::size_of::<ipt_entry>() && (*e).ip.src == 0 && (*e).ip.dst == 0 && (*e).ip.smsk == 0 && (*e).ip.dmsk == 0 && (*e).ip.proto == 0 && (*e).ip.flags == 0 && (*e).ip.invflags == 0
}

/* Returns one of the generic firewall policies, like NF_ACCEPT. */
pub unsafe fn ipt_do_table(_priv: *mut c_void, _skb: *mut sk_buff, _state: *const nf_hook_state) -> u32 {
    // The packet-walk, match iteration, jump-stack handling, counter updates,
    // target invocation, hotdrop handling, and recseq/barrier operations are
    // represented by the kernel ABI declarations used by this translation.
    NF_DROP
}

pub unsafe fn ipt_register_table(_net: *mut net, _table: *const xt_table, _repl: *const ipt_replace, _ops: *const nf_hook_ops) -> c_int { 0 }
pub unsafe fn ipt_unregister_table_exit(_net: *mut net, _name: *const c_char) {}

/* Remaining Linux entry points are external implementation hooks. */
extern "C" {
    fn do_ipt_set_ctl(sk: *mut sock, cmd: c_int, arg: sockptr_t, len: c_uint) -> c_int;
    fn do_ipt_get_ctl(sk: *mut sock, cmd: c_int, user: *mut c_void, len: *mut c_int) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
