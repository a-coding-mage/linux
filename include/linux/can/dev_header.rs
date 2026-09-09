/* SPDX-License-Identifier: GPL-2.0 */
/*
 * linux/can/dev.h
 *
 * Definitions for the CAN network device driver interface
 *
 * Copyright (C) 2006 Andrey Volkov <avolkov@varma-el.com>
 *               Varma Electronics Oy
 *
 * Copyright (C) 2008 Wolfgang Grandegger <wg@grandegger.com>
 *
 */

// C dependencies: <linux/can.h>, <linux/can/bittiming.h>,
// <linux/can/error.h>, <linux/can/length.h>, <linux/can/netlink.h>,
// <linux/can/skb.h>, <linux/ethtool.h>, and <linux/netdevice.h>.

/* CAN mode */
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum can_mode {
    CAN_MODE_STOP = 0,
    CAN_MODE_START,
    CAN_MODE_SLEEP,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum can_termination_gpio {
    CAN_TERMINATION_GPIO_DISABLED = 0,
    CAN_TERMINATION_GPIO_ENABLED,
    CAN_TERMINATION_GPIO_MAX,
}

/* CAN common private data */
#[repr(C)]
pub struct can_priv {
    pub dev: *mut net_device,
    pub can_stats: can_device_stats,

    pub bittiming_const: *const can_bittiming_const,
    pub bittiming: can_bittiming,
    pub fd: data_bittiming_params,
    pub xl: data_bittiming_params,
    pub bitrate_const_cnt: libc::c_uint,
    pub bitrate_const: *const u32,
    pub bitrate_max: u32,
    pub clock: can_clock,

    pub termination_const_cnt: libc::c_uint,
    pub termination_const: *const u16,
    pub termination: u16,
    pub termination_gpio: *mut gpio_desc,
    pub termination_gpio_ohms: [u16; CAN_TERMINATION_GPIO_MAX as usize],

    pub echo_skb_max: libc::c_uint,
    pub echo_skb: *mut *mut sk_buff,

    pub state: can_state,

    /* CAN controller features - see include/uapi/linux/can/netlink.h */
    pub ctrlmode: u32, /* current options setting */
    pub ctrlmode_supported: u32, /* options that can be modified by netlink */

    pub restart_ms: libc::c_int,
    pub restart_work: delayed_work,

    pub do_set_bittiming: Option<unsafe extern "C" fn(dev: *mut net_device) -> libc::c_int>,
    pub do_set_mode: Option<unsafe extern "C" fn(dev: *mut net_device, mode: can_mode) -> libc::c_int>,
    pub do_set_termination: Option<unsafe extern "C" fn(dev: *mut net_device, term: u16) -> libc::c_int>,
    pub do_get_state: Option<unsafe extern "C" fn(dev: *const net_device, state: *mut can_state) -> libc::c_int>,
    pub do_get_berr_counter: Option<unsafe extern "C" fn(dev: *const net_device, bec: *mut can_berr_counter) -> libc::c_int>,
}

#[inline]
pub unsafe fn can_fd_tdc_is_enabled(priv_: *const can_priv) -> bool {
    ((*priv_).ctrlmode & CAN_CTRLMODE_FD_TDC_MASK) != 0
}

#[inline]
pub unsafe fn can_xl_tdc_is_enabled(priv_: *const can_priv) -> bool {
    ((*priv_).ctrlmode & CAN_CTRLMODE_XL_TDC_MASK) != 0
}

#[inline]
pub unsafe fn can_get_static_ctrlmode(priv_: *mut can_priv) -> u32 {
    (*priv_).ctrlmode & !(*priv_).ctrlmode_supported
}

#[inline]
pub fn can_is_canxl_dev_mtu(mtu: libc::c_uint) -> bool {
    mtu >= CANXL_MIN_MTU && mtu <= CANXL_MAX_MTU
}

extern "C" {
    pub fn can_setup(dev: *mut net_device);
    pub fn alloc_candev_mqs(sizeof_priv: libc::c_int, echo_skb_max: libc::c_uint,
                            txqs: libc::c_uint, rxqs: libc::c_uint) -> *mut net_device;
    pub fn free_candev(dev: *mut net_device);
    pub fn safe_candev_priv(dev: *mut net_device) -> *mut can_priv;
    pub fn open_candev(dev: *mut net_device) -> libc::c_int;
    pub fn close_candev(dev: *mut net_device);
    pub fn can_set_default_mtu(dev: *mut net_device);
    pub fn can_set_cap_info(dev: *mut net_device);
    pub fn can_set_static_ctrlmode(dev: *mut net_device, static_mode: u32) -> libc::c_int;
    pub fn can_hwtstamp_get(netdev: *mut net_device, cfg: *mut kernel_hwtstamp_config) -> libc::c_int;
    pub fn can_hwtstamp_set(netdev: *mut net_device, cfg: *mut kernel_hwtstamp_config,
                            extack: *mut netlink_ext_ack) -> libc::c_int;
    pub fn can_ethtool_op_get_ts_info_hwts(dev: *mut net_device, info: *mut kernel_ethtool_ts_info) -> libc::c_int;
    pub fn register_candev(dev: *mut net_device) -> libc::c_int;
    pub fn unregister_candev(dev: *mut net_device);
    pub fn can_restart_now(dev: *mut net_device) -> libc::c_int;
    pub fn can_bus_off(dev: *mut net_device);
    pub fn can_get_state_str(state: can_state) -> *const libc::c_char;
    pub fn can_get_ctrlmode_str(ctrlmode: u32) -> *const libc::c_char;
}

#[inline]
pub unsafe fn alloc_candev(sizeof_priv: libc::c_int, echo_skb_max: libc::c_uint) -> *mut net_device {
    alloc_candev_mqs(sizeof_priv, echo_skb_max, 1, 1)
}

#[inline]
pub unsafe fn alloc_candev_mq(sizeof_priv: libc::c_int, echo_skb_max: libc::c_uint,
                              count: libc::c_uint) -> *mut net_device {
    alloc_candev_mqs(sizeof_priv, echo_skb_max, count, count)
}

#[inline]
pub unsafe fn can_dev_in_xl_only_mode(priv_: *mut can_priv) -> bool {
    let mixed_mode: u32 = CAN_CTRLMODE_FD | CAN_CTRLMODE_XL;
    ((*priv_).ctrlmode & mixed_mode) == CAN_CTRLMODE_XL
}

/* drop skb if it does not contain a valid CAN frame for sending */
#[inline]
pub unsafe fn can_dev_dropped_skb(dev: *mut net_device, skb: *mut sk_buff) -> bool {
    let priv_ = netdev_priv(dev);
    let silent_mode = (*priv_).ctrlmode & (CAN_CTRLMODE_LISTENONLY | CAN_CTRLMODE_RESTRICTED);

    if silent_mode != 0 {
        netdev_info_once(dev, "interface in %s mode, dropping skb\n", can_get_ctrlmode_str(silent_mode));
        kfree_skb(skb);
        (*dev).stats.tx_dropped += 1;
        return true;
    }
    if ((*priv_).ctrlmode & CAN_CTRLMODE_FD) == 0 && can_is_canfd_skb(skb) {
        netdev_info_once(dev, "CAN FD is disabled, dropping skb\n");
        kfree_skb(skb);
        (*dev).stats.tx_dropped += 1;
        return true;
    }
    if can_dev_in_xl_only_mode(priv_) && !can_is_canxl_skb(skb) {
        netdev_info_once(dev, "Error signaling is disabled, dropping skb\n");
        kfree_skb(skb);
        (*dev).stats.tx_dropped += 1;
        return true;
    }
    can_dropped_invalid_skb(dev, skb)
}

extern "C" {
    pub fn can_state_get_by_berr_counter(dev: *const net_device, bec: *const can_berr_counter,
                                         tx_state: *mut can_state, rx_state: *mut can_state);
    pub fn can_change_state(dev: *mut net_device, cf: *mut can_frame,
                            tx_state: can_state, rx_state: can_state);
    pub static mut can_link_ops: rtnl_link_ops;
    pub fn can_netlink_register() -> libc::c_int;
    pub fn can_netlink_unregister();
}

#[cfg(CONFIG_OF)]
extern "C" {
    pub fn of_can_transceiver(dev: *mut net_device);
}

#[cfg(not(CONFIG_OF))]
#[inline]
pub unsafe fn of_can_transceiver(_dev: *mut net_device) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
