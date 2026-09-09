/* SPDX-License-Identifier: GPL-2.0 */

// Translated from the Linux netdev queue header. External kernel types and
// functions are intentionally referenced but not defined here.

#[repr(C)]
pub struct netdev_config {
    pub hds_thresh: u32,
    pub hds_config: u8,
}

#[repr(C)]
pub struct netdev_queue_config {
    pub rx_page_size: u32,
}

#[repr(C)]
pub struct netdev_queue_stats_rx {
    pub bytes: u64,
    pub packets: u64,
    pub alloc_fail: u64,
    pub hw_drops: u64,
    pub hw_drop_overruns: u64,
    pub csum_complete: u64,
    pub csum_unnecessary: u64,
    pub csum_none: u64,
    pub csum_bad: u64,
    pub hw_gro_packets: u64,
    pub hw_gro_bytes: u64,
    pub hw_gro_wire_packets: u64,
    pub hw_gro_wire_bytes: u64,
    pub hw_drop_ratelimits: u64,
}

#[repr(C)]
pub struct netdev_queue_stats_tx {
    pub bytes: u64,
    pub packets: u64,
    pub hw_drops: u64,
    pub hw_drop_errors: u64,
    pub csum_none: u64,
    pub needs_csum: u64,
    pub hw_gso_packets: u64,
    pub hw_gso_bytes: u64,
    pub hw_gso_wire_packets: u64,
    pub hw_gso_wire_bytes: u64,
    pub hw_drop_ratelimits: u64,
    pub stop: u64,
    pub wake: u64,
}

#[repr(C)]
pub struct netdev_stat_ops {
    pub get_queue_stats_rx: Option<unsafe extern "C" fn(*mut net_device, i32, *mut netdev_queue_stats_rx)>,
    pub get_queue_stats_tx: Option<unsafe extern "C" fn(*mut net_device, i32, *mut netdev_queue_stats_tx)>,
    pub get_base_stats: Option<unsafe extern "C" fn(*mut net_device, *mut netdev_queue_stats_rx, *mut netdev_queue_stats_tx)>,
}

extern "C" {
    pub fn netdev_stat_queue_sum(netdev: *mut net_device, rx_start: i32, rx_end: i32,
                                 rx_sum: *mut netdev_queue_stats_rx, tx_start: i32,
                                 tx_end: i32, tx_sum: *mut netdev_queue_stats_tx);
}

pub const QCFG_RX_PAGE_SIZE: u32 = 0x1;

#[repr(C)]
pub struct netdev_queue_mgmt_ops {
    pub ndo_queue_mem_size: usize,
    pub ndo_queue_mem_alloc: Option<unsafe extern "C" fn(*mut net_device, *mut netdev_queue_config, *mut core::ffi::c_void, i32) -> i32>,
    pub ndo_queue_mem_free: Option<unsafe extern "C" fn(*mut net_device, *mut core::ffi::c_void)>,
    pub ndo_queue_start: Option<unsafe extern "C" fn(*mut net_device, *mut netdev_queue_config, *mut core::ffi::c_void, i32) -> i32>,
    pub ndo_queue_stop: Option<unsafe extern "C" fn(*mut net_device, *mut core::ffi::c_void, i32) -> i32>,
    pub ndo_default_qcfg: Option<unsafe extern "C" fn(*mut net_device, *mut netdev_queue_config)>,
    pub ndo_validate_qcfg: Option<unsafe extern "C" fn(*mut net_device, *mut netdev_queue_config, *mut netlink_ext_ack) -> i32>,
    pub ndo_queue_get_dma_dev: Option<unsafe extern "C" fn(*mut net_device, i32) -> *mut device>,
    pub ndo_queue_create: Option<unsafe extern "C" fn(*mut net_device, *mut netlink_ext_ack) -> i32>,
    pub supported_params: u32,
}

extern "C" {
    pub fn netdev_queue_config(dev: *mut net_device, rxq: i32, qcfg: *mut netdev_queue_config);
    pub fn netif_rxq_has_unreadable_mp(dev: *mut net_device, rxq_idx: u32) -> bool;
    pub fn netdev_queue_get_dma_dev(dev: *mut net_device, idx: u32, kind: netdev_queue_type) -> *mut device;
    pub fn netdev_can_create_queue(dev: *const net_device, extack: *mut netlink_ext_ack) -> bool;
    pub fn netdev_can_lease_queue(dev: *const net_device, extack: *mut netlink_ext_ack) -> bool;
    pub fn netdev_queue_busy(dev: *mut net_device, idx: u32, kind: netdev_queue_type, extack: *mut netlink_ext_ack) -> bool;
}

pub unsafe fn netdev_txq_completed_mb(dev_queue: *mut netdev_queue, pkts: u32, bytes: u32) {
    if IS_ENABLED!(CONFIG_BQL) {
        netdev_tx_completed_queue(dev_queue, pkts, bytes);
    } else if bytes != 0 {
        smp_mb();
    }
}

#[macro_export]
macro_rules! netif_txq_try_stop {
    ($txq:expr, $get_desc:expr, $start_thrs:expr) => {{
        netif_tx_stop_queue($txq);
        smp_mb__after_atomic();
        let mut _res: i32 = 0;
        if unlikely!($get_desc >= $start_thrs) {
            netif_tx_start_queue($txq);
            _res = -1;
        }
        _res
    }};
}

#[macro_export]
macro_rules! netif_txq_maybe_stop {
    ($txq:expr, $get_desc:expr, $stop_thrs:expr, $start_thrs:expr) => {{
        let mut _res: i32 = 1;
        if unlikely!($get_desc < $stop_thrs) {
            _res = netif_txq_try_stop!($txq, $get_desc, $start_thrs);
        }
        _res
    }};
}

#[macro_export]
macro_rules! __netif_txq_completed_wake {
    ($txq:expr, $pkts:expr, $bytes:expr, $get_desc:expr, $start_thrs:expr, $down_cond:expr) => {{
        netdev_txq_completed_mb($txq, $pkts, $bytes);
        let mut _res: i32 = -1;
        if $pkts != 0 && likely!($get_desc >= $start_thrs) {
            _res = 1;
            if unlikely!(netif_tx_queue_stopped($txq)) && !($down_cond) {
                netif_tx_wake_queue($txq);
                _res = 0;
            }
        }
        _res
    }};
}

#[macro_export]
macro_rules! netif_txq_completed_wake {
    ($txq:expr, $pkts:expr, $bytes:expr, $get_desc:expr, $start_thrs:expr) => {
        __netif_txq_completed_wake!($txq, $pkts, $bytes, $get_desc, $start_thrs, false)
    };
}

#[macro_export]
macro_rules! netif_subqueue_try_stop {
    ($dev:expr, $idx:expr, $get_desc:expr, $start_thrs:expr) => {{
        let _txq = netdev_get_tx_queue($dev, $idx);
        netif_txq_try_stop!(_txq, $get_desc, $start_thrs)
    }};
}

pub unsafe fn netif_subqueue_sent(dev: *const net_device, idx: u32, bytes: u32) {
    let txq = netdev_get_tx_queue(dev, idx);
    netdev_tx_sent_queue(txq, bytes);
}

pub unsafe fn netif_xmit_timeout_ms(txq: *mut netdev_queue) -> u32 {
    let trans_start = core::ptr::read_volatile(core::ptr::addr_of!((*txq).trans_start));
    if netif_xmit_stopped(txq) && time_after(jiffies(), trans_start + (*(*txq).dev).watchdog_timeo) {
        jiffies_to_msecs(jiffies() - trans_start)
    } else { 0 }
}

#[macro_export]
macro_rules! netif_subqueue_maybe_stop {
    ($dev:expr, $idx:expr, $get_desc:expr, $stop_thrs:expr, $start_thrs:expr) => {{
        let _txq = netdev_get_tx_queue($dev, $idx);
        netif_txq_maybe_stop!(_txq, $get_desc, $stop_thrs, $start_thrs)
    }};
}

#[macro_export]
macro_rules! netif_subqueue_completed_wake {
    ($dev:expr, $idx:expr, $pkts:expr, $bytes:expr, $get_desc:expr, $start_thrs:expr) => {{
        let _txq = netdev_get_tx_queue($dev, $idx);
        netif_txq_completed_wake!(_txq, $pkts, $bytes, $get_desc, $start_thrs)
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
