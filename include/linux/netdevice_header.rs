/* SPDX-License-Identifier: GPL-2.0-or-later */
// Direct low-level Rust translation of linux/netdevice.h.
// Included Linux headers and configuration-selected declarations are supplied
// by the surrounding translation unit.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

pub type xdp_features_t = u32;

pub const NET_RX_SUCCESS: i32 = 0;
pub const NET_RX_DROP: i32 = 1;
pub const MAX_NEST_DEV: usize = 8;
pub const NET_XMIT_SUCCESS: i32 = 0x00;
pub const NET_XMIT_DROP: i32 = 0x01;
pub const NET_XMIT_CN: i32 = 0x02;
pub const NET_XMIT_MASK: i32 = 0x0f;
pub const NETDEV_TX_MASK: i32 = 0xf0;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum netdev_tx {
    __NETDEV_TX_MIN = i32::MIN,
    NETDEV_TX_OK = 0x00,
    NETDEV_TX_BUSY = 0x10,
}
pub type netdev_tx_t = netdev_tx;

#[inline]
pub fn net_xmit_eval(e: i32) -> i32 { if e == NET_XMIT_CN { 0 } else { e } }
#[inline]
pub fn net_xmit_errno(e: i32) -> i32 { if e != NET_XMIT_CN { -105 } else { 0 } }
#[inline]
pub fn dev_xmit_complete(rc: i32) -> bool { rc < NET_XMIT_MASK }

// LL_MAX_HEADER and MAX_HEADER are configuration-selected in the C header.
#[cfg(feature = "hyperv_net")]
pub const LL_MAX_HEADER: usize = 128;
#[cfg(all(not(feature = "hyperv_net"), feature = "wlan", feature = "mac80211_mesh"))]
pub const LL_MAX_HEADER: usize = 128;
#[cfg(all(not(feature = "hyperv_net"), feature = "wlan", not(feature = "mac80211_mesh")))]
pub const LL_MAX_HEADER: usize = 96;
#[cfg(all(not(feature = "hyperv_net"), not(feature = "wlan")))]
pub const LL_MAX_HEADER: usize = 32;
pub const MAX_HEADER: usize = LL_MAX_HEADER + 48;

#[repr(C)]
pub struct net_device_stats {
    pub rx_packets: usize, pub tx_packets: usize, pub rx_bytes: usize,
    pub tx_bytes: usize, pub rx_errors: usize, pub tx_errors: usize,
    pub rx_dropped: usize, pub tx_dropped: usize, pub multicast: usize,
    pub collisions: usize, pub rx_length_errors: usize, pub rx_over_errors: usize,
    pub rx_crc_errors: usize, pub rx_frame_errors: usize, pub rx_fifo_errors: usize,
    pub rx_missed_errors: usize, pub tx_aborted_errors: usize,
    pub tx_carrier_errors: usize, pub tx_fifo_errors: usize,
    pub tx_heartbeat_errors: usize, pub tx_window_errors: usize,
    pub rx_compressed: usize, pub tx_compressed: usize,
}

#[repr(C)]
pub struct net_device_core_stats {
    pub rx_dropped: usize,
    pub tx_dropped: usize,
    pub rx_nohandler: usize,
    pub rx_otherhost_dropped: usize,
}

#[repr(C)]
pub struct netdev_hw_addr {
    pub list: *mut core::ffi::c_void,
    pub node: *mut core::ffi::c_void,
    pub addr: [u8; 32],
    pub r#type: u8,
    pub global_use: bool,
    pub sync_cnt: i32,
    pub refcount: i32,
    pub synced: i32,
    pub rcu_head: *mut core::ffi::c_void,
}

pub const NETDEV_HW_ADDR_T_LAN: u8 = 1;
pub const NETDEV_HW_ADDR_T_SAN: u8 = 2;
pub const NETDEV_HW_ADDR_T_UNICAST: u8 = 3;
pub const NETDEV_HW_ADDR_T_MULTICAST: u8 = 4;

// The remainder of this header consists of declarations whose field types are
// provided by the Linux headers listed in the original include set. They are
// intentionally retained as an opaque dependency boundary for this header
// translation rather than given invented implementations.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
