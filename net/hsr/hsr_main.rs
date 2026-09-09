// SPDX-License-Identifier: GPL-2.0
/* Copyright 2011-2014 Autronica Fire and Security AS
 *
 * Author(s):
 *	2011-2014 Arvid Brodin, arvid.brodin@alten.se
 *
 * Event handling for HSR and PRP devices.
 */

use core::ptr;

// Linux kernel headers and the HSR headers supplying the following types,
// constants, macros, and functions are provided by the surrounding kernel.

unsafe fn hsr_slave_empty(hsr: *mut hsr_priv) -> bool {
    let mut port: *mut hsr_port = ptr::null_mut();
    unsafe {
        hsr_for_each_port_rtnl(hsr, &mut port, |port| {
            (*port).port_type != HSR_PT_MASTER
        })
    }
}

unsafe extern "C" fn hsr_netdev_notify(
    _nb: *mut notifier_block,
    event: usize,
    ptr_: *mut core::ffi::c_void,
) -> i32 {
    let mut port: *mut hsr_port;
    let mut master: *mut hsr_port;
    let dev: *mut net_device;
    let hsr: *mut hsr_priv;
    let mut list_kill: list_head = list_head { next: ptr::null_mut(), prev: ptr::null_mut() };
    let mut mtu_max: i32;
    let res: i32;

    unsafe {
        dev = netdev_notifier_info_to_dev(ptr_);
        port = hsr_port_get_rtnl(dev);
        if port.is_null() {
            if !is_hsr_master(dev) { return NOTIFY_DONE; }
            hsr = netdev_priv(dev);
            port = hsr_port_get_hsr(hsr, HSR_PT_MASTER);
            if port.is_null() { return NOTIFY_DONE; }
        } else {
            hsr = (*port).hsr;
        }

        match event {
            NETDEV_UP | NETDEV_DOWN | NETDEV_CHANGE => {
                hsr_check_carrier_and_operstate(hsr);
            }
            NETDEV_CHANGENAME => {
                if is_hsr_master(dev) { hsr_debugfs_rename(dev); }
            }
            NETDEV_CHANGEADDR => {
                if (*port).port_type == HSR_PT_MASTER { return NOTIFY_DONE; }
                master = hsr_port_get_hsr(hsr, HSR_PT_MASTER);
                if (*port).port_type == HSR_PT_SLAVE_A {
                    eth_hw_addr_set((*master).dev, (*dev).dev_addr);
                    call_netdevice_notifiers(NETDEV_CHANGEADDR, (*master).dev);
                    if (*hsr).prot_version == PRP_V1 {
                        port = hsr_port_get_hsr(hsr, HSR_PT_SLAVE_B);
                        if !port.is_null() {
                            eth_hw_addr_set((*port).dev, (*dev).dev_addr);
                            call_netdevice_notifiers(NETDEV_CHANGEADDR, (*port).dev);
                        }
                    }
                }
                port = hsr_port_get_hsr(hsr, HSR_PT_SLAVE_B);
                let addr = if !port.is_null() { (*port).dev.dev_addr } else { (*master).dev.dev_addr };
                res = hsr_create_self_node(hsr, (*master).dev.dev_addr, addr);
                if res != 0 { netdev_warn((*master).dev, "Could not update HSR node address.\n"); }
            }
            NETDEV_CHANGEMTU => {
                if (*port).port_type == HSR_PT_MASTER { return NOTIFY_DONE; }
                mtu_max = hsr_get_max_mtu((*port).hsr);
                master = hsr_port_get_hsr((*port).hsr, HSR_PT_MASTER);
                core::ptr::write_volatile(&mut (*(*master).dev).mtu, mtu_max);
            }
            NETDEV_UNREGISTER => {
                if !is_hsr_master(dev) {
                    master = hsr_port_get_hsr((*port).hsr, HSR_PT_MASTER);
                    hsr_del_port(port);
                    if hsr_slave_empty((*master).hsr) {
                        ((*(*master).dev).rtnl_link_ops).dellink((*master).dev, &mut list_kill);
                        unregister_netdevice_many(&mut list_kill);
                    }
                }
            }
            NETDEV_PRE_TYPE_CHANGE => return NOTIFY_BAD,
            _ => {}
        }
    }
    NOTIFY_DONE
}

pub unsafe fn hsr_port_get_hsr(hsr: *mut hsr_priv, pt: hsr_port_type) -> *mut hsr_port {
    let mut port: *mut hsr_port = ptr::null_mut();
    unsafe {
        hsr_for_each_port_rtnl(hsr, &mut port, |port| (*port).port_type == pt);
    }
    port
}

pub unsafe fn hsr_get_version(dev: *mut net_device, ver: *mut hsr_version) -> i32 {
    unsafe { *ver = (*netdev_priv(dev)).prot_version; }
    0
}

static mut HSR_NB: notifier_block = notifier_block { notifier_call: Some(hsr_netdev_notify) };

unsafe fn hsr_init() -> i32 {
    unsafe {
        // BUILD_BUG_ON(sizeof(struct hsr_tag) != HSR_HLEN);
        let mut err = register_netdevice_notifier(&mut HSR_NB);
        if err != 0 { return err; }
        err = hsr_netlink_init();
        if err != 0 { unregister_netdevice_notifier(&mut HSR_NB); return err; }
    }
    0
}

unsafe fn hsr_exit() {
    unsafe {
        hsr_netlink_exit();
        hsr_debugfs_remove_root();
        unregister_netdevice_notifier(&mut HSR_NB);
    }
}

// module_init(hsr_init);
// module_exit(hsr_exit);
// MODULE_DESCRIPTION("High-availability Seamless Redundancy (HSR) driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
