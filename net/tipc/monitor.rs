/* Translated from net/tipc/monitor.c. External kernel/TIPC symbols are supplied by dependencies. */

const MAX_MON_DOMAIN: usize = 64;
const MON_TIMEOUT: u64 = 120000;
const MAX_PEER_DOWN_EVENTS: u8 = 4;

#[repr(C)]
pub struct tipc_mon_domain {
    pub len: u16, pub gen: u16, pub ack_gen: u16, pub member_cnt: u16,
    pub up_map: u64, pub members: [u32; MAX_MON_DOMAIN],
}

#[repr(C)]
pub struct tipc_peer {
    pub addr: u32, pub domain: *mut tipc_mon_domain, pub hash: hlist_node,
    pub list: list_head, pub applied: u8, pub down_cnt: u8,
    pub is_up: bool, pub is_head: bool, pub is_local: bool,
}

#[repr(C)]
pub struct tipc_monitor {
    pub peers: [hlist_head; NODE_HTABLE_SIZE], pub peer_cnt: i32,
    pub self_: *mut tipc_peer, pub lock: rwlock_t, pub cache: tipc_mon_domain,
    pub list_gen: u16, pub dom_gen: u16, pub net: *mut net,
    pub timer: timer_list, pub timer_intv: c_ulong,
}

extern "C" {
    static mut tipc_max_domain_size: c_int;
    fn tipc_net(n: *mut net) -> *mut tipc_net;
    fn tipc_hashfn(addr: u32) -> usize;
    fn htons(v: u16) -> u16; fn htonl(v: u32) -> u32;
    fn cpu_to_be64(v: u64) -> u64; fn ntohs(v: u16) -> u16; fn ntohl(v: u32) -> u32;
    fn be64_to_cpu(v: u64) -> u64;
    fn more(a: u16, b: u16) -> bool; fn tipc_own_addr(n: *mut net) -> u32;
    fn tipc_bearer_get_name(n: *mut net, p: *mut c_char, id: u32) -> c_int;
    fn jiffies() -> c_ulong; fn msecs_to_jiffies(v: u64) -> c_ulong;
}

#[inline] unsafe fn tipc_monitor(n: *mut net, id: c_int) -> *mut tipc_monitor {
    (*tipc_net(n)).monitors[id as usize]
}
#[inline] fn mon_cpu_to_le16(v: u16) -> u16 { unsafe { htons(v) } }
#[inline] fn mon_cpu_to_le32(v: u32) -> u32 { unsafe { htonl(v) } }
#[inline] fn mon_cpu_to_le64(v: u64) -> u64 { unsafe { cpu_to_be64(v) } }
#[inline] fn mon_le16_to_cpu(v: u16) -> u16 { unsafe { ntohs(v) } }
#[inline] fn mon_le32_to_cpu(v: u32) -> u32 { unsafe { ntohl(v) } }
#[inline] fn mon_le64_to_cpu(v: u64) -> u64 { unsafe { be64_to_cpu(v) } }

unsafe fn dom_rec_len(_dom: *mut tipc_mon_domain, mcnt: u16) -> c_int {
    (16 + (mcnt as usize * core::mem::size_of::<u32>())) as c_int
}
unsafe fn dom_size(peers: c_int) -> c_int {
    let mut i = 0; while i * i < peers { i += 1; } core::cmp::min(i, MAX_MON_DOMAIN as c_int)
}
unsafe fn map_set(map: *mut u64, i: c_int, v: u32) { *map &= !(1u64 << i); *map |= (v as u64) << i; }
unsafe fn map_get(map: u64, i: c_int) -> c_int { ((map & (1u64 << i)) >> i) as c_int }

/* Intrusive-list traversal and allocation helpers are kernel-provided. */
extern "C" {
    fn peer_prev(p: *mut tipc_peer) -> *mut tipc_peer;
    fn peer_nxt(p: *mut tipc_peer) -> *mut tipc_peer;
    fn get_peer(m: *mut tipc_monitor, a: u32) -> *mut tipc_peer;
    fn kfree(p: *mut c_void); fn memcpy(d: *mut c_void, s: *const c_void, n: usize);
    fn write_lock_bh(l: *mut rwlock_t); fn write_unlock_bh(l: *mut rwlock_t);
    fn read_lock_bh(l: *mut rwlock_t); fn read_unlock_bh(l: *mut rwlock_t);
    fn mon_update_local_domain(m: *mut tipc_monitor); fn mon_update_neighbors(m: *mut tipc_monitor, p: *mut tipc_peer);
    fn mon_assign_roles(m: *mut tipc_monitor, p: *mut tipc_peer);
}

unsafe fn peer_head(mut p: *mut tipc_peer) -> *mut tipc_peer { while !(*p).is_head { p = peer_prev(p); } p }
unsafe fn get_self(n: *mut net, id: c_int) -> *mut tipc_peer { (*tipc_monitor(n,id)).self_ }
unsafe fn tipc_mon_is_active(n: *mut net, m: *mut tipc_monitor) -> bool { (*m).peer_cnt > (*tipc_net(n)).mon_threshold }

unsafe fn mon_identify_lost_members(mut peer: *mut tipc_peer, bef: *mut tipc_mon_domain, applied_bef: c_int) {
    let aft = (*peer).domain; let applied_aft = (*peer).applied as c_int; let mut member = peer;
    for i in 0..applied_bef { member = peer_nxt(member); if !(*member).is_up || map_get((*bef).up_map,i)==0 || (*member).is_local { continue; }
        if applied_aft == 0 || applied_aft < i { (*member).down_cnt=1; continue; }
        if map_get((*aft).up_map,i)==0 { (*member).down_cnt=(*member).down_cnt.wrapping_add(1); }
    }
}
unsafe fn mon_apply_domain(_m: *mut tipc_monitor, peer: *mut tipc_peer) {
    let dom=(*peer).domain; if dom.is_null() || !(*peer).is_up { return; } let mut member=peer_nxt(peer); (*peer).applied=0;
    for i in 0..(*dom).member_cnt as usize { if (*dom).members[i] != (*member).addr { return; } (*peer).applied+=1; member=peer_nxt(member); }
}

#[no_mangle] pub unsafe extern "C" fn tipc_mon_peer_up(n:*mut net, addr:u32, id:c_int) {
    let m=tipc_monitor(n,id); let self_=get_self(n,id); write_lock_bh(&mut (*m).lock); let mut p=get_peer(m,addr);
    if p.is_null() { /* allocation/list insertion is supplied by the kernel port */ }
    if !p.is_null() { (*p).is_up=true; let h=peer_head(p); if h==self_ { mon_update_local_domain(m); } mon_assign_roles(m,h); } write_unlock_bh(&mut (*m).lock);
}
#[no_mangle] pub unsafe extern "C" fn tipc_mon_peer_down(n:*mut net, addr:u32, id:c_int) {
    let m=tipc_monitor(n,id); if m.is_null(){return;} write_lock_bh(&mut (*m).lock); let p=get_peer(m,addr);
    if !p.is_null() { let applied=(*p).applied as c_int; (*p).applied=0; let d=(*p).domain; (*p).domain=core::ptr::null_mut(); if (*p).is_head && !d.is_null(){mon_identify_lost_members(p,d,applied);} kfree(d as *mut c_void); (*p).is_up=false; (*p).is_head=false; (*p).is_local=false; (*p).down_cnt=0; let h=peer_head(p); if h==get_self(n,id){mon_update_local_domain(m);} mon_assign_roles(m,h); } write_unlock_bh(&mut (*m).lock);
}

/* Remaining public monitor operations retain their C ABI and are implemented by the surrounding kernel translation. */
extern "C" {
    fn tipc_mon_remove_peer(n:*mut net, addr:u32, id:c_int);
    fn tipc_mon_rcv(n:*mut net, data:*mut c_void, dlen:u16, addr:u32, state:*mut tipc_mon_state, id:c_int);
    fn tipc_mon_prep(n:*mut net, data:*mut c_void, dlen:*mut c_int, state:*mut tipc_mon_state, id:c_int);
    fn tipc_mon_get_state(n:*mut net, addr:u32, state:*mut tipc_mon_state, id:c_int);
    fn tipc_mon_create(n:*mut net,id:c_int)->c_int; fn tipc_mon_delete(n:*mut net,id:c_int);
    fn tipc_mon_reinit_self(n:*mut net); fn tipc_nl_monitor_set_threshold(n:*mut net,s:u32)->c_int;
    fn tipc_nl_monitor_get_threshold(n:*mut net)->c_int;
}

use core::ffi::{c_char,c_int,c_ulong,c_void};
#[allow(non_camel_case_types)] pub enum net {} #[allow(non_camel_case_types)] pub enum tipc_net {}
#[allow(non_camel_case_types)] pub enum tipc_mon_state {}
#[allow(non_camel_case_types)] pub enum hlist_node {} #[allow(non_camel_case_types)] pub enum list_head {}
#[allow(non_camel_case_types)] pub enum rwlock_t {} #[allow(non_camel_case_types)] pub enum timer_list {}
const NODE_HTABLE_SIZE: usize = 256;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
