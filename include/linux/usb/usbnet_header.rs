// SPDX-License-Identifier: GPL-2.0+
/*
 * USB Networking Link Interface
 *
 * Copyright (C) 2000-2005 by David Brownell <dbrownell@users.sourceforge.net>
 * Copyright (C) 2003-2005 David Hollis <dhollis@davehollis.com>
 */

// Declarations supplied by the corresponding Linux kernel headers are external
// dependencies of this translation.
pub type c_ulong = kernel::ffi::c_ulong;
pub type c_long = kernel::ffi::c_long;

#[repr(C)]
pub struct usbnet {
    pub udev: *mut usb_device,
    pub intf: *mut usb_interface,
    pub driver_info: *const driver_info,
    pub driver_name: *const kernel::ffi::c_char,
    pub driver_priv: *mut kernel::ffi::c_void,
    pub wait: wait_queue_head_t,
    pub phy_mutex: mutex,
    pub suspend_count: u8,
    pub pkt_cnt: u8,
    pub pkt_err: u8,
    pub rx_qlen: u16,
    pub tx_qlen: u16,
    pub can_dma_sg: u32,
    pub in_: u32,
    pub out: u32,
    pub status: *mut usb_host_endpoint,
    pub maxpacket: u32,
    pub delay: timer_list,
    pub padding_pkt: *const kernel::ffi::c_char,
    pub net: *mut net_device,
    pub msg_enable: kernel::ffi::c_int,
    pub data: [c_ulong; 5],
    pub xid: u32,
    pub hard_mtu: u32,
    pub rx_urb_size: usize,
    pub mii: mii_if_info,
    pub rx_speed: c_long,
    pub tx_speed: c_long,
    pub rxq: sk_buff_head,
    pub txq: sk_buff_head,
    pub done: sk_buff_head,
    pub rxq_pause: sk_buff_head,
    pub interrupt: *mut urb,
    pub interrupt_count: u32,
    pub interrupt_mutex: mutex,
    pub deferred: usb_anchor,
    pub bh_work: work_struct,
    pub bql_spinlock: spinlock_t,
    pub kevent: work_struct,
    pub flags: c_ulong,
}

pub const SPEED_UNSET: c_long = -1;
pub const EVENT_TX_HALT: u32 = 0;
pub const EVENT_RX_HALT: u32 = 1;
pub const EVENT_RX_MEMORY: u32 = 2;
pub const EVENT_STS_SPLIT: u32 = 3;
pub const EVENT_LINK_RESET: u32 = 4;
pub const EVENT_RX_PAUSED: u32 = 5;
pub const EVENT_DEV_ASLEEP: u32 = 6;
pub const EVENT_DEV_OPEN: u32 = 7;
pub const EVENT_DEVICE_REPORT_IDLE: u32 = 8;
pub const EVENT_NO_RUNTIME_PM: u32 = 9;
pub const EVENT_RX_KILL: u32 = 10;
pub const EVENT_LINK_CHANGE: u32 = 11;
pub const EVENT_SET_RX_MODE: u32 = 12;
pub const EVENT_NO_IP_ALIGN: u32 = 13;
pub const EVENT_LINK_CARRIER_ON: u32 = 14;
pub const EVENT_UNPLUG: u32 = 31;

#[inline]
pub unsafe fn usbnet_going_away(ubn: *mut usbnet) -> bool {
    test_bit(EVENT_UNPLUG as usize, &(*ubn).flags as *const c_ulong)
}

#[inline]
pub unsafe fn usbnet_mark_going_away(ubn: *mut usbnet) {
    set_bit(EVENT_UNPLUG as usize, &mut (*ubn).flags as *mut c_ulong);
}

#[inline]
pub unsafe fn driver_of(intf: *mut usb_interface) -> *mut usb_driver {
    to_usb_driver((*intf).dev.driver)
}

#[repr(C)]
pub struct driver_info {
    pub description: *mut kernel::ffi::c_char,
    pub flags: kernel::ffi::c_int,
    pub bind: Option<unsafe extern "C" fn(*mut usbnet, *mut usb_interface) -> kernel::ffi::c_int>,
    pub unbind: Option<unsafe extern "C" fn(*mut usbnet, *mut usb_interface)>,
    pub reset: Option<unsafe extern "C" fn(*mut usbnet) -> kernel::ffi::c_int>,
    pub stop: Option<unsafe extern "C" fn(*mut usbnet) -> kernel::ffi::c_int>,
    pub check_connect: Option<unsafe extern "C" fn(*mut usbnet) -> kernel::ffi::c_int>,
    pub manage_power: Option<unsafe extern "C" fn(*mut usbnet, kernel::ffi::c_int) -> kernel::ffi::c_int>,
    pub status: Option<unsafe extern "C" fn(*mut usbnet, *mut urb)>,
    pub link_reset: Option<unsafe extern "C" fn(*mut usbnet) -> kernel::ffi::c_int>,
    pub rx_fixup: Option<unsafe extern "C" fn(*mut usbnet, *mut sk_buff) -> kernel::ffi::c_int>,
    pub tx_fixup: Option<unsafe extern "C" fn(*mut usbnet, *mut sk_buff, gfp_t) -> *mut sk_buff>,
    pub recover: Option<unsafe extern "C" fn(*mut usbnet)>,
    pub early_init: Option<unsafe extern "C" fn(*mut usbnet) -> kernel::ffi::c_int>,
    pub indication: Option<unsafe extern "C" fn(*mut usbnet, *mut kernel::ffi::c_void, kernel::ffi::c_int)>,
    pub set_rx_mode: Option<unsafe extern "C" fn(*mut usbnet)>,
    pub in_: kernel::ffi::c_int,
    pub out: kernel::ffi::c_int,
    pub data: c_ulong,
}

pub const FLAG_FRAMING_NC: kernel::ffi::c_int = 0x0001;
pub const FLAG_FRAMING_GL: kernel::ffi::c_int = 0x0002;
pub const FLAG_FRAMING_Z: kernel::ffi::c_int = 0x0004;
pub const FLAG_FRAMING_RN: kernel::ffi::c_int = 0x0008;
pub const FLAG_NO_SETINT: kernel::ffi::c_int = 0x0010;
pub const FLAG_ETHER: kernel::ffi::c_int = 0x0020;
pub const FLAG_FRAMING_AX: kernel::ffi::c_int = 0x0040;
pub const FLAG_WLAN: kernel::ffi::c_int = 0x0080;
pub const FLAG_AVOID_UNLINK_URBS: kernel::ffi::c_int = 0x0100;
pub const FLAG_SEND_ZLP: kernel::ffi::c_int = 0x0200;
pub const FLAG_WWAN: kernel::ffi::c_int = 0x0400;
pub const FLAG_LINK_INTR: kernel::ffi::c_int = 0x0800;
pub const FLAG_POINTTOPOINT: kernel::ffi::c_int = 0x1000;
pub const FLAG_MULTI_PACKET: kernel::ffi::c_int = 0x2000;
pub const FLAG_RX_ASSEMBLE: kernel::ffi::c_int = 0x4000;
pub const FLAG_NOARP: kernel::ffi::c_int = 0x8000;
pub const FLAG_NOMAXMTU: kernel::ffi::c_int = 0x10000;

#[repr(C)]
pub struct cdc_state {
    pub header: *mut usb_cdc_header_desc,
    pub u: *mut usb_cdc_union_desc,
    pub ether: *mut usb_cdc_ether_desc,
    pub control: *mut usb_interface,
    pub data: *mut usb_interface,
}

pub const DEFAULT_FILTER: u16 = USB_CDC_PACKET_TYPE_BROADCAST | USB_CDC_PACKET_TYPE_ALL_MULTICAST | USB_CDC_PACKET_TYPE_PROMISCUOUS | USB_CDC_PACKET_TYPE_DIRECTED;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum skb_state { illegal = 0, tx_start, tx_done, rx_start, rx_done, rx_cleanup, unlink_start }

#[repr(C)]
pub struct skb_data {
    pub urb: *mut urb,
    pub dev: *mut usbnet,
    pub state: skb_state,
    pub length: c_long,
    pub packets: c_ulong,
}

#[inline]
pub unsafe fn usbnet_set_skb_tx_stats(skb: *mut sk_buff, packets: c_ulong, bytes_delta: c_long) {
    let entry = (*skb).cb.as_mut_ptr() as *mut skb_data;
    (*entry).packets = packets;
    (*entry).length = bytes_delta;
}

#[repr(C)]
pub struct __usbnet_externs;

extern "C" {
    pub fn usbnet_probe(_: *mut usb_interface, _: *const usb_device_id) -> kernel::ffi::c_int;
    pub fn usbnet_suspend(_: *mut usb_interface, _: pm_message_t) -> kernel::ffi::c_int;
    pub fn usbnet_resume(_: *mut usb_interface) -> kernel::ffi::c_int;
    pub fn usbnet_disconnect(_: *mut usb_interface);
    pub fn usbnet_device_suggests_idle(_: *mut usbnet);
    pub fn usbnet_read_cmd(_: *mut usbnet, _: u8, _: u8, _: u16, _: u16, _: *mut kernel::ffi::c_void, _: u16) -> kernel::ffi::c_int;
    pub fn usbnet_write_cmd(_: *mut usbnet, _: u8, _: u8, _: u16, _: u16, _: *const kernel::ffi::c_void, _: u16) -> kernel::ffi::c_int;
    pub fn usbnet_read_cmd_nopm(_: *mut usbnet, _: u8, _: u8, _: u16, _: u16, _: *mut kernel::ffi::c_void, _: u16) -> kernel::ffi::c_int;
    pub fn usbnet_write_cmd_nopm(_: *mut usbnet, _: u8, _: u8, _: u16, _: u16, _: *const kernel::ffi::c_void, _: u16) -> kernel::ffi::c_int;
    pub fn usbnet_write_cmd_async(_: *mut usbnet, _: u8, _: u8, _: u16, _: u16, _: *const kernel::ffi::c_void, _: u16) -> kernel::ffi::c_int;
    pub fn usbnet_cdc_update_filter(_: *mut usbnet);
    pub fn usbnet_generic_cdc_bind(_: *mut usbnet, _: *mut usb_interface) -> kernel::ffi::c_int;
    pub fn usbnet_ether_cdc_bind(_: *mut usbnet, _: *mut usb_interface) -> kernel::ffi::c_int;
    pub fn usbnet_cdc_bind(_: *mut usbnet, _: *mut usb_interface) -> kernel::ffi::c_int;
    pub fn usbnet_cdc_unbind(_: *mut usbnet, _: *mut usb_interface);
    pub fn usbnet_cdc_status(_: *mut usbnet, _: *mut urb);
    pub fn usbnet_cdc_zte_rx_fixup(_: *mut usbnet, _: *mut sk_buff) -> kernel::ffi::c_int;
    pub fn usbnet_open(_: *mut net_device) -> kernel::ffi::c_int;
    pub fn usbnet_stop(_: *mut net_device) -> kernel::ffi::c_int;
    pub fn usbnet_start_xmit(_: *mut sk_buff, _: *mut net_device) -> netdev_tx_t;
    pub fn usbnet_tx_timeout(_: *mut net_device, _: u32);
    pub fn usbnet_change_mtu(_: *mut net_device, _: kernel::ffi::c_int) -> kernel::ffi::c_int;
    pub fn usbnet_get_endpoints(_: *mut usbnet, _: *mut usb_interface) -> kernel::ffi::c_int;
    pub fn usbnet_get_ethernet_addr(_: *mut usbnet, _: kernel::ffi::c_int) -> kernel::ffi::c_int;
    pub fn usbnet_defer_kevent(_: *mut usbnet, _: kernel::ffi::c_int);
    pub fn usbnet_skb_return(_: *mut usbnet, _: *mut sk_buff);
    pub fn usbnet_unlink_rx_urbs(_: *mut usbnet);
    pub fn usbnet_pause_rx(_: *mut usbnet);
    pub fn usbnet_resume_rx(_: *mut usbnet);
    pub fn usbnet_purge_paused_rxq(_: *mut usbnet);
    pub fn usbnet_get_link_ksettings_mii(_: *mut net_device, _: *mut ethtool_link_ksettings) -> kernel::ffi::c_int;
    pub fn usbnet_set_link_ksettings_mii(_: *mut net_device, _: *const ethtool_link_ksettings) -> kernel::ffi::c_int;
    pub fn usbnet_get_link_ksettings_internal(_: *mut net_device, _: *mut ethtool_link_ksettings) -> kernel::ffi::c_int;
    pub fn usbnet_get_link(_: *mut net_device) -> u32;
    pub fn usbnet_get_msglevel(_: *mut net_device) -> u32;
    pub fn usbnet_set_msglevel(_: *mut net_device, _: u32);
    pub fn usbnet_set_rx_mode(_: *mut net_device);
    pub fn usbnet_get_drvinfo(_: *mut net_device, _: *mut ethtool_drvinfo);
    pub fn usbnet_mii_ioctl(_: *mut net_device, _: *mut ifreq, _: kernel::ffi::c_int) -> kernel::ffi::c_int;
    pub fn usbnet_nway_reset(_: *mut net_device) -> kernel::ffi::c_int;
    pub fn usbnet_manage_power(_: *mut usbnet, _: kernel::ffi::c_int) -> kernel::ffi::c_int;
    pub fn usbnet_link_change(_: *mut usbnet, _: bool, _: bool);
    pub fn usbnet_status_start(_: *mut usbnet, _: gfp_t) -> kernel::ffi::c_int;
    pub fn usbnet_status_stop(_: *mut usbnet);
    pub fn usbnet_update_max_qlen(_: *mut usbnet);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
