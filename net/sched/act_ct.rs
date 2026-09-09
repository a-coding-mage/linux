// SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB
/*
 * Rust translation of net/sched/act_ct.c.
 *
 * This implementation intentionally retains the Linux kernel ABI names,
 * layout assumptions, pointer operations, and external helper dependencies.
 * The referenced kernel declarations are supplied by the surrounding crate.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::mem::{self, MaybeUninit};
use core::ptr;

// Kernel declarations imported from the surrounding translation unit.
extern "C" {
    static mut act_ct_wq: *mut workqueue_struct;
    static mut zones_ht: rhashtable;
    static zones_mutex: mutex;
}

#[repr(C)]
pub struct zones_ht_key { pub net: *mut net, pub zone: u16 }

#[repr(C)]
pub struct tcf_ct_flow_table {
    pub node: rhash_head,
    pub rwork: rcu_work,
    pub nf_ft: nf_flowtable,
    pub ref_: refcount_t,
    pub key: zones_ht_key,
    pub dying: bool,
}

// The following declarations and definitions are a literal low-level Rust
// rendering of the C implementation. Kernel-provided types/functions remain
// external dependencies, as they are in the original source.

unsafe fn tcf_ct_flow_table_flow_action_get_next(flow_action: *mut flow_action) -> *mut flow_action_entry {
    let i = (*flow_action).num_entries;
    (*flow_action).num_entries = i + 1;
    (*flow_action).entries.add(i as usize)
}

unsafe fn tcf_ct_add_mangle_action(action: *mut flow_action, htype: flow_action_mangle_base,
                                   offset: u32, mask: u32, val: u32) {
    let entry = tcf_ct_flow_table_flow_action_get_next(action);
    (*entry).id = FLOW_ACTION_MANGLE;
    (*entry).mangle.htype = htype;
    (*entry).mangle.mask = !mask;
    (*entry).mangle.offset = offset;
    (*entry).mangle.val = val;
}

unsafe fn tcf_ct_flow_table_add_action_nat_ipv4(tuple: *const nf_conntrack_tuple,
                                                target: nf_conntrack_tuple,
                                                action: *mut flow_action) {
    if libc::memcmp(&(*(&target.src.u3 as *const _ as *const u8)), &(*(&(*tuple).src.u3 as *const _ as *const u8)), mem::size_of_val(&target.src.u3)) != 0 {
        tcf_ct_add_mangle_action(action, FLOW_ACT_MANGLE_HDR_TYPE_IP4,
            mem::offset_of!(iphdr, saddr) as u32, 0xffff_ffff, be32_to_cpu(target.src.u3.ip));
    }
    if libc::memcmp(&(*(&target.dst.u3 as *const _ as *const u8)), &(*(&(*tuple).dst.u3 as *const _ as *const u8)), mem::size_of_val(&target.dst.u3)) != 0 {
        tcf_ct_add_mangle_action(action, FLOW_ACT_MANGLE_HDR_TYPE_IP4,
            mem::offset_of!(iphdr, daddr) as u32, 0xffff_ffff, be32_to_cpu(target.dst.u3.ip));
    }
}

unsafe fn tcf_ct_flow_table_add_action_nat_tcp(tuple: *const nf_conntrack_tuple,
                                               target: nf_conntrack_tuple,
                                               action: *mut flow_action) {
    let src = target.src.u.tcp.port;
    let dst = target.dst.u.tcp.port;
    if src != (*tuple).src.u.tcp.port { tcf_ct_add_mangle_action(action, FLOW_ACT_MANGLE_HDR_TYPE_TCP, mem::offset_of!(tcphdr, source) as u32, 0xffff, be16_to_cpu(src) as u32); }
    if dst != (*tuple).dst.u.tcp.port { tcf_ct_add_mangle_action(action, FLOW_ACT_MANGLE_HDR_TYPE_TCP, mem::offset_of!(tcphdr, dest) as u32, 0xffff, be16_to_cpu(dst) as u32); }
}

unsafe fn tcf_ct_flow_table_add_action_nat_udp(tuple: *const nf_conntrack_tuple,
                                               target: nf_conntrack_tuple,
                                               action: *mut flow_action) {
    let src = target.src.u.udp.port;
    let dst = target.dst.u.udp.port;
    if src != (*tuple).src.u.udp.port { tcf_ct_add_mangle_action(action, FLOW_ACT_MANGLE_HDR_TYPE_UDP, mem::offset_of!(udphdr, source) as u32, 0xffff, be16_to_cpu(src) as u32); }
    if dst != (*tuple).dst.u.udp.port { tcf_ct_add_mangle_action(action, FLOW_ACT_MANGLE_HDR_TYPE_UDP, mem::offset_of!(udphdr, dest) as u32, 0xffff, be16_to_cpu(dst) as u32); }
}

// Remaining kernel-facing routines are kept as ABI declarations until the
// corresponding translated Linux networking definitions are available.
extern "C" {
    fn tcf_ct_act(skb: *mut sk_buff, a: *const tc_action, res: *mut tcf_result) -> i32;
    fn tcf_ct_init(net: *mut net, nla: *mut nlattr, est: *mut nlattr,
                   a: *mut *mut tc_action, tp: *mut tcf_proto, flags: u32,
                   extack: *mut netlink_ext_ack) -> i32;
    fn tcf_ct_cleanup(a: *mut tc_action);
    fn ct_init_module() -> i32;
    fn ct_cleanup_module();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
