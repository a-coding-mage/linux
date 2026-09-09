// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Tap functions for AF_VSOCK sockets.
 *
 * Code based on net/netlink/af_netlink.c tap functions.
 */

// Linux kernel headers and build-time declarations are supplied by the
// surrounding translation unit.

use core::ffi::c_void;

pub const ARPHRD_VSOCKMON: u16 = 826;
pub const EINVAL: i32 = 22;
pub const ENODEV: i32 = 19;
pub const GFP_ATOMIC: u32 = 0;

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct net_device {
    pub type_: u16,
}

#[repr(C)]
pub struct module;

#[repr(C)]
pub struct vsock_tap {
    pub list: list_head,
    pub dev: *mut net_device,
    pub module: *mut module,
}

#[repr(C)]
pub struct sk_buff {
    pub dev: *mut net_device,
}

extern "C" {
    static mut vsock_tap_lock: c_void;
    static mut vsock_tap_all: list_head;

    fn __module_get(module: *mut module);
    fn module_put(module: *mut module);
    fn spin_lock(lock: *mut c_void);
    fn spin_unlock(lock: *mut c_void);
    fn list_add_rcu(new: *mut list_head, head: *mut list_head);
    fn list_del_rcu(entry: *mut list_head);
    fn synchronize_net();
    fn pr_warn(format: *const u8, ...);
    fn skb_clone(skb: *mut sk_buff, gfp_mask: u32) -> *mut sk_buff;
    fn dev_hold(dev: *mut net_device);
    fn dev_queue_xmit(skb: *mut sk_buff) -> i32;
    fn net_xmit_errno(err: i32) -> i32;
    fn dev_put(dev: *mut net_device);
    fn consume_skb(skb: *mut sk_buff);
}

// The kernel's list_for_each_entry/list_for_each_entry_rcu primitives are
// represented here with their corresponding raw-pointer traversal.

#[no_mangle]
pub unsafe extern "C" fn vsock_add_tap(vt: *mut vsock_tap) -> i32 {
    if (*(*vt).dev).type_ != ARPHRD_VSOCKMON {
        return -EINVAL;
    }

    __module_get((*vt).module);

    spin_lock(&mut vsock_tap_lock as *mut c_void);
    list_add_rcu(&mut (*vt).list, &mut vsock_tap_all);
    spin_unlock(&mut vsock_tap_lock as *mut c_void);

    0
}

#[no_mangle]
pub unsafe extern "C" fn vsock_remove_tap(vt: *mut vsock_tap) -> i32 {
    let mut tmp: *mut vsock_tap;
    let mut found = false;

    spin_lock(&mut vsock_tap_lock as *mut c_void);

    let mut pos = vsock_tap_all.next;
    while pos != &mut vsock_tap_all as *mut list_head {
        tmp = (pos as *mut u8).sub(core::mem::offset_of!(vsock_tap, list)) as *mut vsock_tap;
        if vt == tmp {
            list_del_rcu(&mut (*vt).list);
            found = true;
            break;
        }
        pos = (*pos).next;
    }

    if !found {
        pr_warn(b"vsock_remove_tap: %p not found\n\0".as_ptr(), vt);
    }
    spin_unlock(&mut vsock_tap_lock as *mut c_void);

    synchronize_net();

    if found {
        module_put((*vt).module);
    }

    if found { 0 } else { -ENODEV }
}

unsafe fn __vsock_deliver_tap_skb(skb: *mut sk_buff, dev: *mut net_device) -> i32 {
    let mut ret = 0;
    let nskb = skb_clone(skb, GFP_ATOMIC);

    if !nskb.is_null() {
        dev_hold(dev);
        (*nskb).dev = dev;
        ret = dev_queue_xmit(nskb);
        if ret > 0 {
            ret = net_xmit_errno(ret);
        }
        dev_put(dev);
    }

    ret
}

unsafe fn __vsock_deliver_tap(skb: *mut sk_buff) {
    let mut pos = vsock_tap_all.next;
    while pos != &mut vsock_tap_all as *mut list_head {
        let tmp = (pos as *mut u8).sub(core::mem::offset_of!(vsock_tap, list)) as *mut vsock_tap;
        let ret = __vsock_deliver_tap_skb(skb, (*tmp).dev);
        if ret != 0 {
            break;
        }
        pos = (*pos).next;
    }
}

#[no_mangle]
pub unsafe extern "C" fn vsock_deliver_tap(
    build_skb: Option<unsafe extern "C" fn(*mut c_void) -> *mut sk_buff>,
    opaque: *mut c_void,
) {
    extern "C" {
        fn rcu_read_lock();
        fn rcu_read_unlock();
    }

    rcu_read_lock();

    if vsock_tap_all.next == &mut vsock_tap_all as *mut list_head {
        rcu_read_unlock();
        return;
    }

    let skb = build_skb.unwrap()(opaque);
    if !skb.is_null() {
        __vsock_deliver_tap(skb);
        consume_skb(skb);
    }

    rcu_read_unlock();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
