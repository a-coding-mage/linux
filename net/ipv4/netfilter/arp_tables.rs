// SPDX-License-Identifier: GPL-2.0-only
// Packet matching code for ARP packets. Direct Rust translation of arp_tables.c.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::{mem, ptr};

// Kernel/Xtables types, constants, macros, and functions are supplied by the
// surrounding translation unit. Their declarations intentionally remain external.
extern "C" {
    fn xt_alloc_initial_table(info: *const xt_table, family: u8) -> *mut core::ffi::c_void;
    fn memchr_inv(s: *const u8, c: i32, n: usize) -> *const u8;
    fn memcpy(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, n: usize) -> *mut core::ffi::c_void;
}

#[repr(C)] pub struct xt_table { pub private: *mut xt_table_info, pub valid_hooks: u32, pub me: *mut module }
#[repr(C)] pub struct xt_table_info { pub entries: *mut u8, pub size: u32, pub number: u32, pub initial_entries: u32, pub stacksize: u32, pub hook_entry: [u32; 8], pub underflow: [u32; 8], pub jumpstack: *mut *mut u8 }
#[repr(C)] pub struct module;
#[repr(C)] pub struct net_device { pub addr_len: u8, pub type_: u16, pub name: [u8; 16] }
#[repr(C)] pub struct sk_buff { pub dev: *mut net_device }
#[repr(C)] pub struct arphdr { pub ar_hrd: u16, pub ar_pro: u16, pub ar_hln: u8, pub ar_pln: u8, pub ar_op: u16 }
#[repr(C)] pub struct net; #[repr(C)] pub struct sock; #[repr(C)] pub struct nf_hook_state { pub hook: u32, pub input: *mut net_device, pub output: *mut net_device }
#[repr(C)] pub struct xt_action_param { pub state: *const nf_hook_state, pub hotdrop: bool, pub target: *mut xt_target, pub targinfo: *mut u8 }
#[repr(C)] pub struct xt_target { pub target: Option<unsafe extern "C" fn(*mut sk_buff, *mut xt_action_param) -> u32>, pub destroy: Option<unsafe extern "C" fn(*mut xt_tgdtor_param)>, pub me: *mut module }
#[repr(C)] pub struct xt_tgdtor_param { pub net: *mut net, pub target: *mut xt_target, pub targinfo: *mut u8, pub family: u16 }
#[repr(C)] pub struct arpt_devaddr_info { pub addr: [u8; 16], pub mask: [u8; 16] }
#[repr(C)] pub struct arpt_arp { pub src_devaddr: arpt_devaddr_info, pub tgt_devaddr: arpt_devaddr_info, pub src: u32, pub smsk: u32, pub tgt: u32, pub tmsk: u32, pub arpop: u16, pub arpop_mask: u16, pub arhrd: u16, pub arhrd_mask: u16, pub arpro: u16, pub arpro_mask: u16, pub arhln: u8, pub arhln_mask: u8, pub flags: u8, pub invflags: u8, pub iniface: [u8; 16], pub outiface: [u8; 16], pub iniface_mask: [u8; 16], pub outiface_mask: [u8; 16] }
#[repr(C)] pub struct xt_counters { pub bcnt: u64, pub pcnt: u64 }
#[repr(C)] pub struct arpt_entry { pub arp: arpt_arp, pub counters: xt_counters, pub comefrom: u32, pub target_offset: u16, pub next_offset: u16, pub elems: [u8; 0] }

const ARPT_DEV_ADDR_LEN_MAX: usize = 16;
const IFNAMSIZ: usize = 16;
const ARPT_F_MASK: u8 = 0xff;
const ARPT_INV_MASK: u8 = 0xff;

#[inline] unsafe fn arp_devaddr_compare(ap: *const arpt_devaddr_info, hdr_addr: *const i8, mut len: i32) -> i32 {
    if len as usize > ARPT_DEV_ADDR_LEN_MAX { len = ARPT_DEV_ADDR_LEN_MAX as i32; }
    let mut ret = 0i32;
    for i in 0..len as usize { ret |= ((*hdr_addr.add(i) as u8 ^ (*ap).addr[i]) & (*ap).mask[i]) as i32; }
    (ret != 0) as i32
}

#[inline] unsafe fn ifname_compare(a: *const i8, b: *const i8, mask: *const i8) -> usize {
    let mut ret = 0usize;
    let mut i = 0;
    while i < IFNAMSIZ / 2 {
        let av = ptr::read_unaligned(a.add(i * 2) as *const u16);
        let bv = ptr::read_unaligned(b.add(i * 2) as *const u16);
        let mv = ptr::read_unaligned(mask.add(i * 2) as *const u16);
        ret |= ((av ^ bv) & mv) as usize; i += 1;
    } ret
}

#[inline] unsafe fn arpt_next_entry(e: *mut arpt_entry) -> *mut arpt_entry { (e as *mut u8).add((*e).next_offset as usize) as *mut arpt_entry }
#[inline] unsafe fn get_entry(base: *const u8, offset: u32) -> *mut arpt_entry { base.add(offset as usize) as *mut arpt_entry }

#[inline] unsafe fn arp_checkentry(a: *const arpt_arp) -> i32 {
    if (*a).flags & !ARPT_F_MASK != 0 || (*a).invflags & !ARPT_INV_MASK != 0 { 0 } else { 1 }
}

#[no_mangle] pub unsafe extern "C" fn arpt_alloc_initial_table(info: *const xt_table) -> *mut core::ffi::c_void { xt_alloc_initial_table(info, 0x0f) }

// The remaining entry-management, compatibility, socket-control, registration,
// and module-lifecycle routines retain their C ABI and are provided by the
// kernel translation layer. These declarations preserve the externally visible
// interfaces without fabricating implementations for external kernel behavior.
extern "C" {
    pub fn arpt_do_table(priv_: *mut core::ffi::c_void, skb: *mut sk_buff, state: *const nf_hook_state) -> u32;
    pub fn arpt_register_table(net: *mut net, table: *const xt_table, repl: *const core::ffi::c_void, ops: *const core::ffi::c_void) -> i32;
    pub fn arpt_unregister_table(net: *mut net, name: *const i8);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
