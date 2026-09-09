/* SPDX-License-Identifier: GPL-2.0 */

// The following types and symbols are supplied by other translation units.
use core::ffi::c_int;

pub enum net_device {}
pub enum net {}
pub enum mutex {}
pub enum static_key_false {}

// C: int __init netdev_kobject_init(void);
pub unsafe extern "C" fn netdev_kobject_init() -> c_int;

// C: int netdev_register_kobject(struct net_device *);
pub unsafe extern "C" fn netdev_register_kobject(dev: *mut net_device) -> c_int;

// C: void netdev_uevent_add(struct net_device *dev);
pub unsafe extern "C" fn netdev_uevent_add(dev: *mut net_device);

// C: void netdev_unregister_kobject(struct net_device *);
pub unsafe extern "C" fn netdev_unregister_kobject(dev: *mut net_device);

// C: int net_rx_queue_update_kobjects(struct net_device *, int old_num,
//                                     int new_num);
pub unsafe extern "C" fn net_rx_queue_update_kobjects(
    dev: *mut net_device,
    old_num: c_int,
    new_num: c_int,
) -> c_int;

// C: int netdev_queue_update_kobjects(struct net_device *net, int old_num,
//                                     int new_num);
pub unsafe extern "C" fn netdev_queue_update_kobjects(
    net: *mut net_device,
    old_num: c_int,
    new_num: c_int,
) -> c_int;

// C: int netdev_change_owner(struct net_device *, const struct net *net_old,
//                            const struct net *net_new);
pub unsafe extern "C" fn netdev_change_owner(
    dev: *mut net_device,
    net_old: *const net,
    net_new: *const net,
) -> c_int;

// extern struct mutex rps_default_mask_mutex;
pub static mut rps_default_mask_mutex: mutex = unsafe { core::mem::zeroed() };

// DECLARE_STATIC_KEY_FALSE(skb_defer_disable_key);
pub static mut skb_defer_disable_key: static_key_false = unsafe { core::mem::zeroed() };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
