// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * X.25 Packet Layer release 002
 *
 * This is ALPHA test software. This code may break your machine,
 * randomly fail to work with new releases, misbehave and/or generally
 * screw up. It might even work.
 *
 * This code REQUIRES 2.1.15 or higher
 *
 * History
 * X.25 001 Jonathan Naylor Started coding.
 */

// C dependencies supplied by the surrounding kernel translation unit.

pub static mut x25_route_list: list_head = LIST_HEAD_INIT();
pub static mut x25_route_list_lock: rwlock_t = __RW_LOCK_UNLOCKED();

/* Add a new route. */
unsafe fn x25_add_route(
    address: *mut x25_address,
    sigdigits: c_uint,
    dev: *mut net_device,
) -> c_int {
    let mut rt: *mut x25_route;
    let mut rc: c_int = -EINVAL;

    write_lock_bh(&raw mut x25_route_list_lock);

    list_for_each_entry!(rt, &raw mut x25_route_list, node) {
        if memcmp(
            &(*rt).address as *const _ as *const c_void,
            address as *const c_void,
            sigdigits as usize,
        ) == 0 && (*rt).sigdigits == sigdigits
        {
            goto_out!();
        }
    }

    rt = kmalloc_obj!(*rt, GFP_ATOMIC);
    rc = -ENOMEM;
    if rt.is_null() {
        goto_out!();
    }

    strcpy((*rt).address.x25_addr.as_mut_ptr(), b"000000000000000\0".as_ptr());
    memcpy(
        (*rt).address.x25_addr.as_mut_ptr() as *mut c_void,
        (*address).x25_addr.as_ptr() as *const c_void,
        sigdigits as usize,
    );

    (*rt).sigdigits = sigdigits;
    (*rt).dev = dev;
    refcount_set(&raw mut (*rt).refcnt, 1);

    list_add(&raw mut (*rt).node, &raw mut x25_route_list);
    rc = 0;
out:
    write_unlock_bh(&raw mut x25_route_list_lock);
    rc
}

/**
 * __x25_remove_route - remove route from x25_route_list
 * @rt: route to remove
 *
 * Remove route from x25_route_list. If it was there.
 * Caller must hold x25_route_list_lock.
 */
unsafe fn __x25_remove_route(rt: *mut x25_route) {
    if !(*rt).node.next.is_null() {
        list_del(&raw mut (*rt).node);
        x25_route_put(rt);
    }
}

unsafe fn x25_del_route(
    address: *mut x25_address,
    sigdigits: c_uint,
    dev: *mut net_device,
) -> c_int {
    let mut rt: *mut x25_route;
    let mut rc: c_int = -EINVAL;

    write_lock_bh(&raw mut x25_route_list_lock);
    list_for_each_entry!(rt, &raw mut x25_route_list, node) {
        if memcmp(
            &(*rt).address as *const _ as *const c_void,
            address as *const c_void,
            sigdigits as usize,
        ) == 0 && (*rt).sigdigits == sigdigits && (*rt).dev == dev
        {
            __x25_remove_route(rt);
            rc = 0;
            break;
        }
    }
    write_unlock_bh(&raw mut x25_route_list_lock);
    rc
}

/* A device has been removed, remove its routes. */
pub unsafe fn x25_route_device_down(dev: *mut net_device) {
    let mut rt: *mut x25_route;
    let mut entry: *mut list_head;
    let mut tmp: *mut list_head;

    write_lock_bh(&raw mut x25_route_list_lock);
    list_for_each_safe!(entry, tmp, &raw mut x25_route_list) {
        rt = list_entry!(entry, x25_route, node);
        if (*rt).dev == dev {
            __x25_remove_route(rt);
        }
    }
    write_unlock_bh(&raw mut x25_route_list_lock);
}

/* Check that the device given is a valid X.25 interface that is "up". */
pub unsafe fn x25_dev_get(devname: *mut c_char) -> *mut net_device {
    let mut dev = dev_get_by_name(&raw mut init_net, devname);
    if !dev.is_null() && ((*dev).flags & IFF_UP == 0 || (*dev).type_ != ARPHRD_X25) {
        dev_put(dev);
        dev = core::ptr::null_mut();
    }
    dev
}

/** Find a route given an X.25 address. */
pub unsafe fn x25_get_route(addr: *mut x25_address) -> *mut x25_route {
    let mut rt: *mut x25_route;
    let mut use_: *mut x25_route = core::ptr::null_mut();

    read_lock_bh(&raw mut x25_route_list_lock);
    list_for_each_entry!(rt, &raw mut x25_route_list, node) {
        if memcmp(
            &(*rt).address as *const _ as *const c_void,
            addr as *const c_void,
            (*rt).sigdigits as usize,
        ) == 0 {
            if use_.is_null() {
                use_ = rt;
            } else if (*rt).sigdigits > (*use_).sigdigits {
                use_ = rt;
            }
        }
    }
    if !use_.is_null() {
        x25_route_hold(use_);
    }
    read_unlock_bh(&raw mut x25_route_list_lock);
    use_
}

/* Handle the ioctls that control the routing functions. */
pub unsafe fn x25_route_ioctl(cmd: c_uint, arg: *mut c_void) -> c_int {
    let mut rt: x25_route_struct = core::mem::zeroed();
    let mut dev: *mut net_device;
    let mut rc: c_int = -EINVAL;

    if cmd != SIOCADDRT && cmd != SIOCDELRT { return rc; }
    rc = -EFAULT;
    if copy_from_user(&raw mut rt as *mut c_void, arg, core::mem::size_of::<x25_route_struct>()) != 0 { return rc; }
    rc = -EINVAL;
    if rt.sigdigits > 15 { return rc; }
    dev = x25_dev_get(rt.device.as_mut_ptr());
    if dev.is_null() { return rc; }
    if cmd == SIOCADDRT { rc = x25_add_route(&raw mut rt.address, rt.sigdigits, dev); }
    else { rc = x25_del_route(&raw mut rt.address, rt.sigdigits, dev); }
    dev_put(dev);
    rc
}

/* Release all memory associated with X.25 routing structures. */
pub unsafe fn x25_route_free() {
    let mut rt: *mut x25_route;
    let mut entry: *mut list_head;
    let mut tmp: *mut list_head;
    write_lock_bh(&raw mut x25_route_list_lock);
    list_for_each_safe!(entry, tmp, &raw mut x25_route_list) {
        rt = list_entry!(entry, x25_route, node);
        __x25_remove_route(rt);
    }
    write_unlock_bh(&raw mut x25_route_list_lock);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
