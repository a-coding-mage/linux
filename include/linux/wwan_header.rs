/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright (c) 2021, Linaro Ltd <loic.poulain@linaro.org> */

// Dependencies supplied by the surrounding kernel translation unit:
// linux/poll.h, linux/netdevice.h, and linux/types.h.

/**
 * enum wwan_port_type - WWAN port types
 * @WWAN_PORT_AT: AT commands
 * @WWAN_PORT_MBIM: Mobile Broadband Interface Model control
 * @WWAN_PORT_QMI: Qcom modem/MSM interface for modem control
 * @WWAN_PORT_QCDM: Qcom Modem diagnostic interface
 * @WWAN_PORT_FIREHOSE: XML based command protocol
 * @WWAN_PORT_XMMRPC: Control protocol for Intel XMM modems
 * @WWAN_PORT_FASTBOOT: Fastboot protocol control
 * @WWAN_PORT_ADB: ADB protocol control
 * @WWAN_PORT_MIPC: MTK MIPC diagnostic interface
 * @WWAN_PORT_NMEA: embedded GNSS receiver with NMEA output
 *
 * @WWAN_PORT_MAX: Highest supported port types
 * @WWAN_PORT_UNKNOWN: Special value to indicate an unknown port type
 * @__WWAN_PORT_MAX: Internal use
 */
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum wwan_port_type {
    WWAN_PORT_AT,
    WWAN_PORT_MBIM,
    WWAN_PORT_QMI,
    WWAN_PORT_QCDM,
    WWAN_PORT_FIREHOSE,
    WWAN_PORT_XMMRPC,
    WWAN_PORT_FASTBOOT,
    WWAN_PORT_ADB,
    WWAN_PORT_MIPC,
    WWAN_PORT_NMEA,
    __WWAN_PORT_MAX,
    WWAN_PORT_MAX = __WWAN_PORT_MAX as isize - 1,
    WWAN_PORT_UNKNOWN,
}

#[repr(C)]
pub struct device { _private: [u8; 0] }
#[repr(C)]
pub struct file { _private: [u8; 0] }
#[repr(C)]
pub struct netlink_ext_ack { _private: [u8; 0] }
#[repr(C)]
pub struct sk_buff { _private: [u8; 0] }
#[repr(C)]
pub struct wwan_port { _private: [u8; 0] }
#[repr(C)]
pub struct net_device { _private: [u8; 0] }
#[repr(C)]
pub struct list_head { _private: [u8; 0] }
#[repr(C)]
pub struct dentry { _private: [u8; 0] }

pub type __poll_t = u32;
pub type poll_table = core::ffi::c_void;

#[repr(C)]
pub struct wwan_port_ops {
    pub start: Option<unsafe extern "C" fn(port: *mut wwan_port) -> i32>,
    pub stop: Option<unsafe extern "C" fn(port: *mut wwan_port)>,
    pub tx: Option<unsafe extern "C" fn(port: *mut wwan_port, skb: *mut sk_buff) -> i32>,
    pub tx_blocking: Option<unsafe extern "C" fn(port: *mut wwan_port, skb: *mut sk_buff) -> i32>,
    pub tx_poll: Option<unsafe extern "C" fn(port: *mut wwan_port, filp: *mut file, wait: *mut poll_table) -> __poll_t>,
}

#[repr(C)]
pub struct wwan_port_caps {
    pub frag_len: usize,
    pub headroom_len: u32,
}

#[repr(C)]
pub struct wwan_netdev_priv {
    pub link_id: u32,
    pub drv_priv: [u8; 0],
}

extern "C" {
    pub fn netdev_priv(dev: *mut net_device) -> *mut core::ffi::c_void;
}

#[inline]
pub unsafe fn wwan_netdev_drvpriv(dev: *mut net_device) -> *mut core::ffi::c_void {
    (*(netdev_priv(dev) as *mut wwan_netdev_priv)).drv_priv.as_mut_ptr() as *mut core::ffi::c_void
}

/* Used to indicate that the WWAN core should not create a default network link. */
pub const WWAN_NO_DEFAULT_LINK: u32 = u32::MAX;

#[repr(C)]
pub struct wwan_ops {
    pub priv_size: u32,
    pub setup: Option<unsafe extern "C" fn(dev: *mut net_device)>,
    pub newlink: Option<unsafe extern "C" fn(ctxt: *mut core::ffi::c_void, dev: *mut net_device, if_id: u32, extack: *mut netlink_ext_ack) -> i32>,
    pub dellink: Option<unsafe extern "C" fn(ctxt: *mut core::ffi::c_void, dev: *mut net_device, head: *mut list_head)>,
}

extern "C" {
    pub fn wwan_create_port(parent: *mut device, type_: wwan_port_type, ops: *const wwan_port_ops, caps: *mut wwan_port_caps, drvdata: *mut core::ffi::c_void) -> *mut wwan_port;
    pub fn wwan_remove_port(port: *mut wwan_port);
    pub fn wwan_port_rx(port: *mut wwan_port, skb: *mut sk_buff);
    pub fn wwan_port_txoff(port: *mut wwan_port);
    pub fn wwan_port_txon(port: *mut wwan_port);
    pub fn wwan_port_get_drvdata(port: *mut wwan_port) -> *mut core::ffi::c_void;
    pub fn wwan_register_ops(parent: *mut device, ops: *const wwan_ops, ctxt: *mut core::ffi::c_void, def_link_id: u32) -> i32;
    pub fn wwan_unregister_ops(parent: *mut device);
}

// CONFIG_WWAN_DEBUGFS controls whether these declarations or inline stubs are used.
#[cfg(feature = "CONFIG_WWAN_DEBUGFS")]
extern "C" {
    pub fn wwan_get_debugfs_dir(parent: *mut device) -> *mut dentry;
    pub fn wwan_put_debugfs_dir(dir: *mut dentry);
}

#[cfg(not(feature = "CONFIG_WWAN_DEBUGFS"))]
#[inline]
pub unsafe fn wwan_get_debugfs_dir(_parent: *mut device) -> *mut dentry {
    // ERR_PTR(-ENODEV), supplied by the surrounding kernel translation unit.
    core::ptr::null_mut()
}

#[cfg(not(feature = "CONFIG_WWAN_DEBUGFS"))]
#[inline]
pub unsafe fn wwan_put_debugfs_dir(_dir: *mut dentry) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
