// SPDX-License-Identifier: GPL-2.0
/*
 * Data gathering module for Linux-VM Monitor Stream, Stage 1.
 * Collects accumulated network statistics (Packets received/transmitted,
 * dropped, errors, ...).
 *
 * Copyright IBM Corp. 2003, 2006
 *
 * Author: Gerald Schaefer <gerald.schaefer@de.ibm.com>
 */

// Linux kernel dependencies supplied by other files.

use core::ffi::c_void;

/*
 * Network data
 *
 * This is accessed as binary data by z/VM. If changes to it can't be avoided,
 * the structure version (product ID, see appldata_base.c) needs to be changed
 * as well and all documentation and z/VM applications using it must be updated.
 */
#[repr(C, packed)]
pub struct appldata_net_sum_data {
    pub timestamp: u64,
    pub sync_count_1: u32, // after VM collected the record data,
    pub sync_count_2: u32, // sync_count_1 and sync_count_2 should be the same.
                           // If not, the record has been updated on the Linux
                           // side while VM was collecting the (possibly corrupt) data

    pub nr_interfaces: u32, // nr. of network interfaces being monitored

    pub padding: u32, // next value is 64-bit aligned, so these
                      // 4 byte would be padded out by compiler

    pub rx_packets: u64,  // total packets received
    pub tx_packets: u64,  // total packets transmitted
    pub rx_bytes: u64,    // total bytes received
    pub tx_bytes: u64,    // total bytes transmitted
    pub rx_errors: u64,   // bad packets received
    pub tx_errors: u64,   // packet transmit problems
    pub rx_dropped: u64,  // no space in linux buffers
    pub tx_dropped: u64,  // no space available in linux
    pub collisions: u64,  // collisions while transmitting
}

extern "C" {
    fn rcu_read_lock();
    fn rcu_read_unlock();
    fn get_tod_clock() -> u64;
    fn dev_get_stats(dev: *mut net_device, temp: *mut rtnl_link_stats64) -> *const rtnl_link_stats64;
    fn appldata_register_ops(ops: *mut appldata_ops) -> i32;
    fn appldata_unregister_ops(ops: *mut appldata_ops);
    fn kzalloc_obj<T>() -> *mut T;
    fn kfree(ptr: *mut c_void);
}

#[repr(C)]
pub struct net_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rtnl_link_stats64 {
    pub rx_packets: u64,
    pub tx_packets: u64,
    pub rx_bytes: u64,
    pub tx_bytes: u64,
    pub rx_errors: u64,
    pub tx_errors: u64,
    pub rx_dropped: u64,
    pub tx_dropped: u64,
    pub collisions: u64,
}

#[repr(C)]
pub struct appldata_ops {
    pub name: *const u8,
    pub record_nr: u32,
    pub size: usize,
    pub callback: Option<unsafe extern "C" fn(*mut c_void)>,
    pub owner: *mut c_void,
    pub mod_lvl: [u8; 2],
    pub data: *mut c_void,
}

extern "C" {
    static init_net: c_void;
}

// appldata_get_net_sum_data()
//
// gather accumulated network statistics
unsafe extern "C" fn appldata_get_net_sum_data(data: *mut c_void) {
    let net_data = data as *mut appldata_net_sum_data;
    (*net_data).sync_count_1 = (*net_data).sync_count_1.wrapping_add(1);

    let mut i: i32 = 0;
    let mut rx_packets: u64 = 0;
    let mut tx_packets: u64 = 0;
    let mut rx_bytes: u64 = 0;
    let mut tx_bytes: u64 = 0;
    let mut rx_errors: u64 = 0;
    let mut tx_errors: u64 = 0;
    let mut rx_dropped: u64 = 0;
    let mut tx_dropped: u64 = 0;
    let mut collisions: u64 = 0;

    rcu_read_lock();
    // for_each_netdev_rcu(&init_net, dev)
    let mut dev: *mut net_device = core::ptr::null_mut();
    while !dev.is_null() {
        let mut temp = core::mem::zeroed::<rtnl_link_stats64>();
        let stats = dev_get_stats(dev, &mut temp);
        rx_packets = rx_packets.wrapping_add((*stats).rx_packets);
        tx_packets = tx_packets.wrapping_add((*stats).tx_packets);
        rx_bytes = rx_bytes.wrapping_add((*stats).rx_bytes);
        tx_bytes = tx_bytes.wrapping_add((*stats).tx_bytes);
        rx_errors = rx_errors.wrapping_add((*stats).rx_errors);
        tx_errors = tx_errors.wrapping_add((*stats).tx_errors);
        rx_dropped = rx_dropped.wrapping_add((*stats).rx_dropped);
        tx_dropped = tx_dropped.wrapping_add((*stats).tx_dropped);
        collisions = collisions.wrapping_add((*stats).collisions);
        i += 1;
        // The kernel macro advances dev through the namespace device list.
        break;
    }
    rcu_read_unlock();

    (*net_data).nr_interfaces = i as u32;
    (*net_data).rx_packets = rx_packets;
    (*net_data).tx_packets = tx_packets;
    (*net_data).rx_bytes = rx_bytes;
    (*net_data).tx_bytes = tx_bytes;
    (*net_data).rx_errors = rx_errors;
    (*net_data).tx_errors = tx_errors;
    (*net_data).rx_dropped = rx_dropped;
    (*net_data).tx_dropped = tx_dropped;
    (*net_data).collisions = collisions;

    (*net_data).timestamp = get_tod_clock();
    (*net_data).sync_count_2 = (*net_data).sync_count_2.wrapping_add(1);
}

static mut ops: appldata_ops = appldata_ops {
    name: b"net_sum\0".as_ptr(),
    record_nr: APPLDATA_RECORD_NET_SUM_ID,
    size: core::mem::size_of::<appldata_net_sum_data>(),
    callback: Some(appldata_get_net_sum_data),
    owner: core::ptr::null_mut(), // THIS_MODULE
    mod_lvl: [0xF0, 0xF0], // EBCDIC "00"
    data: core::ptr::null_mut(),
};

// APPLDATA_RECORD_NET_SUM_ID is supplied by appldata.h.
extern "C" {
    static APPLDATA_RECORD_NET_SUM_ID: u32;
}

// appldata_net_init()
//
// init data, register ops
unsafe extern "C" fn appldata_net_init() -> i32 {
    ops.data = kzalloc_obj::<appldata_net_sum_data>() as *mut c_void;
    if ops.data.is_null() {
        return -12; // -ENOMEM
    }

    let ret = appldata_register_ops(&mut ops);
    if ret != 0 {
        kfree(ops.data);
    }

    ret
}

// appldata_net_exit()
//
// unregister ops
unsafe extern "C" fn appldata_net_exit() {
    appldata_unregister_ops(&mut ops);
    kfree(ops.data);
}

// module_init(appldata_net_init);
// module_exit(appldata_net_exit);
// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Gerald Schaefer");
// MODULE_DESCRIPTION("Linux-VM Monitor Stream, accumulated network statistics");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
