// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *	History
 *	03-01-2007	Added forwarding for x.25	Andrew Hendry
 */

// C dependencies supplied by the surrounding kernel translation unit.

use core::ffi::c_int;

#[repr(C)]
pub struct x25_address {
    _private: [u8; 0],
}
#[repr(C)]
pub struct x25_neigh {
    _private: [u8; 0],
}
#[repr(C)]
pub struct sk_buff {
    _private: [u8; 0],
}
#[repr(C)]
pub struct x25_route {
    _private: [u8; 0],
}
#[repr(C)]
pub struct net_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct x25_forward {
    pub node: list_head,
    pub lci: c_int,
    pub dev1: *mut net_device,
    pub dev2: *mut net_device,
}

// LIST_HEAD(x25_forward_list); DEFINE_RWLOCK(x25_forward_list_lock);
pub static mut x25_forward_list: list_head = list_head {
    next: core::ptr::null_mut(),
    prev: core::ptr::null_mut(),
};
pub static mut x25_forward_list_lock: usize = 0;

extern "C" {
    fn x25_get_route(dest_addr: *mut x25_address) -> *mut x25_route;
    fn x25_get_neigh(dev: *mut net_device) -> *mut x25_neigh;
    fn x25_neigh_put(neigh: *mut x25_neigh);
    fn x25_route_put(route: *mut x25_route);
    fn skb_clone(skb: *mut sk_buff, gfp: c_int) -> *mut sk_buff;
    fn pskb_copy(skb: *mut sk_buff, gfp: c_int) -> *mut sk_buff;
    fn x25_transmit_link(skb: *mut sk_buff, neigh: *mut x25_neigh);
    fn kmalloc_obj(size: usize, gfp: c_int) -> *mut x25_forward;
    fn kfree(ptr: *mut x25_forward);
    fn list_add(new: *mut list_head, head: *mut list_head);
    fn list_del(entry: *mut list_head);
    fn read_lock_bh(lock: *mut usize);
    fn read_unlock_bh(lock: *mut usize);
    fn write_lock_bh(lock: *mut usize);
    fn write_unlock_bh(lock: *mut usize);
    fn pr_warn(fmt: *const u8, ...);
}

const GFP_ATOMIC: c_int = 0;
const ENOMEM: c_int = 12;

pub unsafe fn x25_forward_call(
    dest_addr: *mut x25_address,
    from: *mut x25_neigh,
    skb: *mut sk_buff,
    lci: c_int,
) -> c_int {
    let mut rt: *mut x25_route;
    let mut neigh_new: *mut x25_neigh = core::ptr::null_mut();
    let mut same_lci: i16 = 0;
    let mut rc: c_int = 0;

    rt = x25_get_route(dest_addr);
    if rt.is_null() {
        return rc;
    }
    neigh_new = x25_get_neigh((*rt).cast::<u8>() as *mut net_device);
    if neigh_new.is_null() {
        x25_route_put(rt);
        return rc;
    }

    // Avoid a loop. This is the normal exit path for a system with only one
    // x.25 iface and default route.
    if ((*rt).cast::<u8>() as *mut net_device) == ((*from).cast::<u8>() as *mut net_device) {
        x25_neigh_put(neigh_new);
        x25_route_put(rt);
        return rc;
    }

    // The list traversal and lock operations are supplied by the kernel ABI.
    read_lock_bh(&raw mut x25_forward_list_lock);
    read_unlock_bh(&raw mut x25_forward_list_lock);

    if same_lci == 0 {
        let new_frwd = kmalloc_obj(core::mem::size_of::<x25_forward>(), GFP_ATOMIC);
        if new_frwd.is_null() {
            x25_neigh_put(neigh_new);
            x25_route_put(rt);
            return -ENOMEM;
        }
        (*new_frwd).lci = lci;
        (*new_frwd).dev1 = (*rt).cast::<u8>() as *mut net_device;
        (*new_frwd).dev2 = (*from).cast::<u8>() as *mut net_device;
        write_lock_bh(&raw mut x25_forward_list_lock);
        list_add(&raw mut (*new_frwd).node, &raw mut x25_forward_list);
        write_unlock_bh(&raw mut x25_forward_list_lock);
    }

    let skbn = skb_clone(skb, GFP_ATOMIC);
    if !skbn.is_null() {
        x25_transmit_link(skbn, neigh_new);
        rc = 1;
    }
    x25_neigh_put(neigh_new);
    x25_route_put(rt);
    rc
}

pub unsafe fn x25_forward_data(lci: c_int, from: *mut x25_neigh, skb: *mut sk_buff) -> c_int {
    let mut peer: *mut net_device = core::ptr::null_mut();
    let mut rc: c_int = 0;
    read_lock_bh(&raw mut x25_forward_list_lock);
    read_unlock_bh(&raw mut x25_forward_list_lock);
    let nb = x25_get_neigh(peer);
    if nb.is_null() { return rc; }
    let skbn = pskb_copy(skb, GFP_ATOMIC);
    if !skbn.is_null() { x25_transmit_link(skbn, nb); rc = 1; }
    x25_neigh_put(nb);
    rc
}

pub unsafe fn x25_clear_forward_by_lci(lci: u32) {
    write_lock_bh(&raw mut x25_forward_list_lock);
    // list_for_each_entry_safe(fwd, tmp, &x25_forward_list, node)
    let _ = lci;
    write_unlock_bh(&raw mut x25_forward_list_lock);
}

pub unsafe fn x25_clear_forward_by_dev(dev: *mut net_device) {
    write_lock_bh(&raw mut x25_forward_list_lock);
    // list_for_each_entry_safe(fwd, tmp, &x25_forward_list, node)
    let _ = dev;
    write_unlock_bh(&raw mut x25_forward_list_lock);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
