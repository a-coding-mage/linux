// SPDX-License-Identifier: GPL-2.0

// C dependencies: <vmlinux.h>, "bpf_experimental.h", "bpf_qdisc_common.h"

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_void};
use core::ptr;

type u32 = u32;

const NET_XMIT_SUCCESS: i32 = 0;
const NET_XMIT_DROP: i32 = 1;

#[repr(C)]
pub struct sk_buff {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_sk_buff_ptr {
    _private: [u8; 0],
}

#[repr(C)]
pub struct nlattr {
    _private: [u8; 0],
}

#[repr(C)]
pub struct netlink_ext_ack {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_spin_lock {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_list_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_list_head {
    _private: [u8; 0],
}

#[repr(C)]
pub struct skb_node {
    pub skb: *mut sk_buff,
    pub node: bpf_list_node,
}

#[repr(C)]
pub struct Qdisc {
    pub q: Qdisc_q,
    pub limit: u32,
    pub qstats: Qdisc_qstats,
}

#[repr(C)]
pub struct Qdisc_q {
    pub qlen: u32,
}

#[repr(C)]
pub struct Qdisc_qstats {
    pub backlog: u32,
}

#[repr(C)]
pub struct Qdisc_ops {
    pub enqueue: *mut c_void,
    pub dequeue: *mut c_void,
    pub init: *mut c_void,
    pub reset: *mut c_void,
    pub destroy: *mut c_void,
    pub id: *const c_char,
}

unsafe extern "C" {
    fn qdisc_pkt_len(skb: *mut sk_buff) -> u32;
    fn bpf_obj_new(size: usize) -> *mut c_void;
    fn bpf_kptr_xchg(map_value: *mut *mut sk_buff, ptr: *mut sk_buff) -> *mut sk_buff;
    fn bpf_qdisc_skb_drop(skb: *mut sk_buff, to_free: *mut bpf_sk_buff_ptr);
    fn bpf_spin_lock(lock: *mut bpf_spin_lock);
    fn bpf_spin_unlock(lock: *mut bpf_spin_lock);
    fn bpf_list_push_back(head: *mut bpf_list_head, node: *mut bpf_list_node);
    fn bpf_list_pop_front(head: *mut bpf_list_head) -> *mut bpf_list_node;
    fn bpf_obj_drop(ptr: *mut skb_node);
    fn bpf_qdisc_bstats_update(sch: *mut Qdisc, skb: *mut sk_buff);
    fn bpf_kfree_skb(skb: *mut sk_buff);
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [c_char; 4] = [b'G' as c_char, b'P' as c_char, b'L' as c_char, 0];

// private(A) struct bpf_spin_lock q_fifo_lock;
#[unsafe(no_mangle)]
pub static mut q_fifo_lock: bpf_spin_lock = bpf_spin_lock { _private: [] };

// private(A) struct bpf_list_head q_fifo __contains(skb_node, node);
#[unsafe(no_mangle)]
pub static mut q_fifo: bpf_list_head = bpf_list_head { _private: [] };

#[unsafe(no_mangle)]
pub static mut init_called: bool = false;

#[inline]
unsafe fn container_of_skb_node_node(node: *mut bpf_list_node) -> *mut skb_node {
    node as *mut skb_node
}

#[unsafe(link_section = "struct_ops/bpf_fifo_enqueue")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bpf_fifo_enqueue(
    mut skb: *mut sk_buff,
    sch: *mut Qdisc,
    to_free: *mut bpf_sk_buff_ptr,
) -> i32 {
    let mut skbn: *mut skb_node;
    let pkt_len: u32;

    if (*sch).q.qlen == (*sch).limit {
        bpf_qdisc_skb_drop(skb, to_free);
        return NET_XMIT_DROP;
    }

    skbn = bpf_obj_new(core::mem::size_of::<skb_node>()) as *mut skb_node;
    if skbn.is_null() {
        bpf_qdisc_skb_drop(skb, to_free);
        return NET_XMIT_DROP;
    }

    pkt_len = qdisc_pkt_len(skb);

    (*sch).q.qlen = (*sch).q.qlen.wrapping_add(1);
    skb = bpf_kptr_xchg(ptr::addr_of_mut!((*skbn).skb), skb);
    if !skb.is_null() {
        bpf_qdisc_skb_drop(skb, to_free);
    }

    bpf_spin_lock(ptr::addr_of_mut!(q_fifo_lock));
    bpf_list_push_back(ptr::addr_of_mut!(q_fifo), ptr::addr_of_mut!((*skbn).node));
    bpf_spin_unlock(ptr::addr_of_mut!(q_fifo_lock));

    (*sch).qstats.backlog = (*sch).qstats.backlog.wrapping_add(pkt_len);
    NET_XMIT_SUCCESS
}

#[unsafe(link_section = "struct_ops/bpf_fifo_dequeue")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bpf_fifo_dequeue(sch: *mut Qdisc) -> *mut sk_buff {
    let node: *mut bpf_list_node;
    let mut skb: *mut sk_buff = ptr::null_mut();
    let skbn: *mut skb_node;

    bpf_spin_lock(ptr::addr_of_mut!(q_fifo_lock));
    node = bpf_list_pop_front(ptr::addr_of_mut!(q_fifo));
    bpf_spin_unlock(ptr::addr_of_mut!(q_fifo_lock));
    if node.is_null() {
        return ptr::null_mut();
    }

    skbn = container_of_skb_node_node(node);
    skb = bpf_kptr_xchg(ptr::addr_of_mut!((*skbn).skb), skb);
    bpf_obj_drop(skbn);
    if skb.is_null() {
        return ptr::null_mut();
    }

    (*sch).qstats.backlog = (*sch).qstats.backlog.wrapping_sub(qdisc_pkt_len(skb));
    bpf_qdisc_bstats_update(sch, skb);
    (*sch).q.qlen = (*sch).q.qlen.wrapping_sub(1);

    skb
}

#[unsafe(link_section = "struct_ops/bpf_fifo_init")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bpf_fifo_init(
    sch: *mut Qdisc,
    _opt: *mut nlattr,
    _extack: *mut netlink_ext_ack,
) -> i32 {
    (*sch).limit = 1000;
    init_called = true;
    0
}

#[unsafe(link_section = "struct_ops/bpf_fifo_reset")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bpf_fifo_reset(sch: *mut Qdisc) {
    let mut node: *mut bpf_list_node;
    let mut skbn: *mut skb_node;
    let mut i: i32;

    i = 0;
    while i < (*sch).q.qlen as i32 {
        let mut skb: *mut sk_buff = ptr::null_mut();

        bpf_spin_lock(ptr::addr_of_mut!(q_fifo_lock));
        node = bpf_list_pop_front(ptr::addr_of_mut!(q_fifo));
        bpf_spin_unlock(ptr::addr_of_mut!(q_fifo_lock));

        if node.is_null() {
            break;
        }

        skbn = container_of_skb_node_node(node);
        skb = bpf_kptr_xchg(ptr::addr_of_mut!((*skbn).skb), skb);
        if !skb.is_null() {
            bpf_kfree_skb(skb);
        }
        bpf_obj_drop(skbn);

        i += 1;
    }
    (*sch).q.qlen = 0;
}

#[unsafe(link_section = "struct_ops")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bpf_fifo_destroy(_sch: *mut Qdisc) {}

#[unsafe(link_section = ".struct_ops")]
#[unsafe(no_mangle)]
pub static mut fifo: Qdisc_ops = Qdisc_ops {
    enqueue: bpf_fifo_enqueue as *mut c_void,
    dequeue: bpf_fifo_dequeue as *mut c_void,
    init: bpf_fifo_init as *mut c_void,
    reset: bpf_fifo_reset as *mut c_void,
    destroy: bpf_fifo_destroy as *mut c_void,
    id: c"bpf_fifo".as_ptr(),
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
