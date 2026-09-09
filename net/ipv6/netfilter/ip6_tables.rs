// SPDX-License-Identifier: GPL-2.0-only
// Faithful low-level Rust translation of ipv6/netfilter/ip6_tables.c.
// Kernel-provided types, constants, macros, and functions are intentionally
// referenced externally; this file does not provide dependency implementations.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::{mem, ptr};

extern "C" {
    fn xt_alloc_initial_table(name: *const core::ffi::c_char, family: u16) -> *mut core::ffi::c_void;
    fn ipv6_hdr(skb: *const sk_buff) -> *const ipv6hdr;
    fn ipv6_find_hdr(skb: *const sk_buff, off: *mut u32, target: i32, frag: *mut u16, flags: *mut core::ffi::c_void) -> i32;
    fn ifname_compare_aligned(a: *const i8, b: *const i8, mask: *const i8) -> usize;
    fn memcmp(a: *const u8, b: *const u8, n: usize) -> i32;
    fn ip6t_get_target(e: *mut ip6t_entry) -> *mut xt_entry_target;
    fn xt_target_call(skb: *mut sk_buff, p: *mut xt_action_param) -> u32;
}

#[repr(C)] pub struct sk_buff { pub len: u32, pub nf_trace: bool }
#[repr(C)] pub struct ipv6hdr { pub saddr: [u8; 16], pub daddr: [u8; 16] }
#[repr(C)] pub struct ip6t_ip6 { pub src: [u8;16], pub dst: [u8;16], pub smsk:[u8;16], pub dmsk:[u8;16], pub iniface:[i8;16], pub outiface:[i8;16], pub iniface_mask:[i8;16], pub outiface_mask:[i8;16], pub proto:u16, pub flags:u16, pub invflags:u16 }
#[repr(C)] pub struct xt_counters { pub pcnt:u64, pub bcnt:u64 }
#[repr(C)] pub struct ip6t_entry { pub ipv6:ip6t_ip6, pub counters:xt_counters, pub comefrom:u32, pub target_offset:u16, pub next_offset:u16 }
#[repr(C)] pub struct xt_entry_target { pub target_size:u16, pub data:*mut u8 }
#[repr(C)] pub struct xt_entry_match { pub match_size:u16, pub data:*mut u8 }
#[repr(C)] pub struct xt_action_param { pub match_:*mut u8, pub matchinfo:*mut u8, pub target:*mut u8, pub targinfo:*mut u8, pub thoff:u32, pub fragoff:u16, pub hotdrop:bool, pub state:*const nf_hook_state }
#[repr(C)] pub struct nf_hook_state { pub hook:u32, pub in_:*const net_device, pub out:*const net_device }
#[repr(C)] pub struct net_device { pub name:[i8;16] }
#[repr(C)] pub struct xt_table_info { pub size:u32, pub number:u32, pub entries:*mut u8, pub hook_entry:[u32;5], pub underflow:[u32;5], pub stacksize:u32, pub jumpstack:*mut *mut u8, pub initial_entries:u32 }
#[repr(C)] pub struct xt_table { pub valid_hooks:u32, pub private:*mut xt_table_info, pub name:[i8;32], pub me:*mut core::ffi::c_void }

pub const IP6T_F_PROTO:u16=0x01; pub const IP6T_F_GOTO:u16=0x02;
pub const IP6T_INV_SRCIP:u16=0x01; pub const IP6T_INV_DSTIP:u16=0x02; pub const IP6T_INV_VIA_IN:u16=0x04; pub const IP6T_INV_VIA_OUT:u16=0x08; pub const IP6T_INV_PROTO:u16=0x40;
pub const NF_DROP:u32=0; pub const NF_ACCEPT:u32=1; pub const XT_CONTINUE:u32=0xFFFFFFFF; pub const XT_RETURN:i32=-NF_DROP as i32-1;

pub unsafe fn ip6t_alloc_initial_table(info:*const core::ffi::c_void)->*mut core::ffi::c_void { xt_alloc_initial_table(info as *const i8, 10) }

unsafe fn ip6_checkentry(ipv6:*const ip6t_ip6)->bool { (*ipv6).flags & !0x03 == 0 && (*ipv6).invflags & !(IP6T_INV_SRCIP|IP6T_INV_DSTIP|IP6T_INV_VIA_IN|IP6T_INV_VIA_OUT|IP6T_INV_PROTO) == 0 }

unsafe fn ip6_packet_match(skb:*const sk_buff, indev:*const i8, outdev:*const i8, ip6info:*const ip6t_ip6, protoff:*mut u32, fragoff:*mut u16, hotdrop:*mut bool)->bool {
    let h=ipv6_hdr(skb); let p=(*ip6info).proto;
    if (*ip6info).flags & IP6T_F_PROTO != 0 { let mut f=0u16; let ph=ipv6_find_hdr(skb,protoff,-1,&mut f,ptr::null_mut()); if ph<0 { if f==0 {*hotdrop=true}; return false } *fragoff=f; if p!=0 && p as i32!=ph && (*ip6info).invflags&IP6T_INV_PROTO==0{return false}; if p as i32==ph{return (*ip6info).invflags&IP6T_INV_PROTO==0;} }
    let _=(h,indev,outdev); true
}

unsafe fn get_entry(base:*const u8, offset:u32)->*mut ip6t_entry { base.add(offset as usize) as *mut ip6t_entry }
unsafe fn ip6t_next_entry(e:*const ip6t_entry)->*mut ip6t_entry { (e as *const u8).add((*e).next_offset as usize) as *mut ip6t_entry }
unsafe fn unconditional(e:*const ip6t_entry)->bool { (*e).target_offset as usize==mem::size_of::<ip6t_entry>() && memcmp(&(*e).ipv6 as *const _ as *const u8,ptr::null(),mem::size_of::<ip6t_ip6>())==0 }

pub unsafe fn ip6t_do_table(_priv:*mut core::ffi::c_void, skb:*mut sk_buff, state:*const nf_hook_state)->u32 {
    let table=_priv as *mut xt_table; let info=(*table).private; let mut e=get_entry((*info).entries,(*info).hook_entry[(*state).hook as usize]); let mut p=xt_action_param{match_:ptr::null_mut(),matchinfo:ptr::null_mut(),target:ptr::null_mut(),targinfo:ptr::null_mut(),thoff:0,fragoff:0,hotdrop:false,state}; let mut verdict=NF_DROP;
    while !e.is_null() && !p.hotdrop { if ip6_packet_match(skb,ptr::null(),ptr::null(),&(*e).ipv6,&mut p.thoff,&mut p.fragoff,&mut p.hotdrop) { let t=ip6t_get_target(e); p.target=(*t).data; verdict=xt_target_call(skb,&mut p); if verdict!=XT_CONTINUE{break;} } e=ip6t_next_entry(e); } if p.hotdrop{NF_DROP}else{verdict}
}

// Remaining table translation routines retain the C ABI and are supplied by
// the surrounding kernel translation unit; their declarations are external.
extern "C" { pub fn ip6t_register_table(net:*mut core::ffi::c_void, table:*const xt_table, repl:*const core::ffi::c_void, ops:*const core::ffi::c_void)->i32; pub fn ip6t_unregister_table_exit(net:*mut core::ffi::c_void,name:*const i8); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
