// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * net/l3mdev/l3mdev.c - L3 master device implementation
 * Copyright (c) 2015 Cumulus Networks
 * Copyright (c) 2015 David Ahern <dsa@cumulusnetworks.com>
 */

// External kernel declarations supplied by the surrounding repository.
use core::ffi::c_void;

type U32 = u32;
type LookupByTableIdT = unsafe extern "C" fn(*mut net, U32) -> i32;

#[repr(C)]
pub struct net {
    _private: [u8; 0],
}
#[repr(C)]
pub struct net_device {
    pub ifindex: i32,
    pub l3mdev_ops: *mut l3mdev_ops,
}
#[repr(C)]
pub struct l3mdev_ops {
    pub l3mdev_fib_table: Option<unsafe extern "C" fn(*const net_device) -> U32>,
    pub l3mdev_link_scope_lookup:
        Option<unsafe extern "C" fn(*mut net_device, *mut flowi6) -> *mut dst_entry>,
}
#[repr(C)]
pub struct flowi6 {
    pub flowi6_oif: i32,
    _private: [u8; 0],
}
#[repr(C)]
pub struct flowi {
    pub flowi_l3mdev: i32,
    pub flowi_oif: i32,
    pub flowi_iif: i32,
    pub flowi_flags: u32,
}
#[repr(C)]
pub struct fib_lookup_arg {
    pub table: U32,
}
#[repr(C)]
pub struct dst_entry {
    _private: [u8; 0],
}

#[repr(i32)]
pub enum l3mdev_type {
    L3MDEV_TYPE_UNSPEC = 0,
    L3MDEV_TYPE_MAX = 1,
}

const EINVAL: i32 = 22;
const EBUSY: i32 = 16;
const L3MDEV_TYPE_MAX: usize = 1;
const FLOWI_FLAG_L3MDEV_OIF: u32 = 1 << 4;
const LOOPBACK_IFINDEX: i32 = 1;

extern "C" {
    fn spin_lock(lock: *mut c_void);
    fn spin_unlock(lock: *mut c_void);
    fn netif_is_l3_master(dev: *const net_device) -> bool;
    fn netif_is_l3_slave(dev: *const net_device) -> bool;
    fn netdev_master_upper_dev_get_rcu(dev: *mut net_device) -> *mut net_device;
    fn dev_get_by_index_rcu(net: *mut net, ifindex: i32) -> *mut net_device;
    fn rcu_read_lock();
    fn rcu_read_unlock();
    fn rcu_read_lock_held() -> bool;
    fn warn_on_once(condition: bool);
}

static mut L3MDEV_LOCK: *mut c_void = core::ptr::null_mut();

#[repr(C)]
struct l3mdev_handler {
    dev_lookup: Option<LookupByTableIdT>,
}

static mut L3MDEV_HANDLERS: [l3mdev_handler; L3MDEV_TYPE_MAX + 1] =
    [l3mdev_handler { dev_lookup: None }; L3MDEV_TYPE_MAX + 1];

unsafe fn l3mdev_check_type(l3type: l3mdev_type) -> i32 {
    let value = l3type as i32;
    if value <= l3mdev_type::L3MDEV_TYPE_UNSPEC as i32
        || value > l3mdev_type::L3MDEV_TYPE_MAX as i32
    {
        return -EINVAL;
    }
    0
}

pub unsafe fn l3mdev_table_lookup_register(
    l3type: l3mdev_type,
    func: LookupByTableIdT,
) -> i32 {
    let res = l3mdev_check_type(l3type);
    if res != 0 {
        return res;
    }
    let hdlr = &mut L3MDEV_HANDLERS[l3type as usize];
    spin_lock(L3MDEV_LOCK);
    let result = if hdlr.dev_lookup.is_some() {
        -EBUSY
    } else {
        hdlr.dev_lookup = Some(func);
        0
    };
    spin_unlock(L3MDEV_LOCK);
    result
}

pub unsafe fn l3mdev_table_lookup_unregister(l3type: l3mdev_type, func: LookupByTableIdT) {
    if l3mdev_check_type(l3type) != 0 {
        return;
    }
    let hdlr = &mut L3MDEV_HANDLERS[l3type as usize];
    spin_lock(L3MDEV_LOCK);
    if hdlr.dev_lookup.map(|f| f as usize) == Some(func as usize) {
        hdlr.dev_lookup = None;
    }
    spin_unlock(L3MDEV_LOCK);
}

pub unsafe fn l3mdev_ifindex_lookup_by_table_id(
    l3type: l3mdev_type,
    net: *mut net,
    table_id: U32,
) -> i32 {
    let res = l3mdev_check_type(l3type);
    if res != 0 {
        return res;
    }
    let hdlr = &L3MDEV_HANDLERS[l3type as usize];
    let mut ifindex = -EINVAL;
    spin_lock(L3MDEV_LOCK);
    if let Some(lookup) = hdlr.dev_lookup {
        ifindex = lookup(net, table_id);
    }
    spin_unlock(L3MDEV_LOCK);
    ifindex
}

pub unsafe fn l3mdev_master_ifindex_rcu(dev: *const net_device) -> i32 {
    if dev.is_null() {
        return 0;
    }
    if netif_is_l3_master(dev) {
        (*dev).ifindex
    } else if netif_is_l3_slave(dev) {
        let master = netdev_master_upper_dev_get_rcu(dev as *mut net_device);
        if master.is_null() { 0 } else { (*master).ifindex }
    } else {
        0
    }
}

pub unsafe fn l3mdev_master_upper_ifindex_by_index_rcu(net: *mut net, mut ifindex: i32) -> i32 {
    let mut dev = dev_get_by_index_rcu(net, ifindex);
    while !dev.is_null() && !netif_is_l3_master(dev) {
        dev = netdev_master_upper_dev_get_rcu(dev);
    }
    if dev.is_null() { 0 } else { (*dev).ifindex }
}

pub unsafe fn l3mdev_fib_table_rcu(dev: *const net_device) -> U32 {
    if dev.is_null() {
        return 0;
    }
    if netif_is_l3_master(dev) {
        let ops = (*dev).l3mdev_ops;
        if !ops.is_null() {
            if let Some(f) = (*ops).l3mdev_fib_table { return f(dev); }
        }
    } else if netif_is_l3_slave(dev) {
        let master = netdev_master_upper_dev_get_rcu(dev as *mut net_device);
        if !master.is_null() {
            let ops = (*master).l3mdev_ops;
            if !ops.is_null() {
                if let Some(f) = (*ops).l3mdev_fib_table { return f(master); }
            }
        }
    }
    0
}

pub unsafe fn l3mdev_fib_table_by_index(net: *mut net, ifindex: i32) -> U32 {
    if ifindex == 0 { return 0; }
    rcu_read_lock();
    let dev = dev_get_by_index_rcu(net, ifindex);
    let tb_id = if dev.is_null() { 0 } else { l3mdev_fib_table_rcu(dev) };
    rcu_read_unlock();
    tb_id
}

pub unsafe fn l3mdev_link_scope_lookup(net: *mut net, fl6: *mut flowi6) -> *mut dst_entry {
    warn_on_once(!rcu_read_lock_held());
    let mut dst = core::ptr::null_mut();
    if (*fl6).flowi6_oif != 0 {
        let mut dev = dev_get_by_index_rcu(net, (*fl6).flowi6_oif);
        if !dev.is_null() && netif_is_l3_slave(dev) {
            dev = netdev_master_upper_dev_get_rcu(dev);
        }
        if !dev.is_null() && netif_is_l3_master(dev) {
            let ops = (*dev).l3mdev_ops;
            if !ops.is_null() {
                if let Some(f) = (*ops).l3mdev_link_scope_lookup { dst = f(dev, fl6); }
            }
        }
    }
    dst
}

pub unsafe fn l3mdev_fib_rule_match(net: *mut net, fl: *mut flowi, arg: *mut fib_lookup_arg) -> i32 {
    if (*fl).flowi_l3mdev == 0 { return 0; }
    rcu_read_lock();
    let dev = dev_get_by_index_rcu(net, (*fl).flowi_l3mdev);
    let mut rc = 0;
    if !dev.is_null() && netif_is_l3_master(dev) {
        let ops = (*dev).l3mdev_ops;
        if !ops.is_null() {
            if let Some(f) = (*ops).l3mdev_fib_table {
                (*arg).table = f(dev);
                rc = 1;
            }
        }
    }
    rcu_read_unlock();
    rc
}

pub unsafe fn l3mdev_update_flow(net: *mut net, fl: *mut flowi) {
    rcu_read_lock();
    if (*fl).flowi_oif != 0 {
        let dev = dev_get_by_index_rcu(net, (*fl).flowi_oif);
        if !dev.is_null() {
            if (*fl).flowi_l3mdev == 0 {
                (*fl).flowi_l3mdev = l3mdev_master_ifindex_rcu(dev);
                (*fl).flowi_flags |= FLOWI_FLAG_L3MDEV_OIF;
            }
            if netif_is_l3_master(dev) {
                (*fl).flowi_oif = 0;
            }
            rcu_read_unlock();
            return;
        }
    }
    if (*fl).flowi_iif > LOOPBACK_IFINDEX && (*fl).flowi_l3mdev == 0 {
        let dev = dev_get_by_index_rcu(net, (*fl).flowi_iif);
        if !dev.is_null() {
            (*fl).flowi_l3mdev = l3mdev_master_ifindex_rcu(dev);
        }
    }
    rcu_read_unlock();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
