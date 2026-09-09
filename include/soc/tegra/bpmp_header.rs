/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright (c) 2016, NVIDIA CORPORATION. All rights reserved. */

// C dependencies supplied by the surrounding kernel translation unit:
// linux/iosys-map.h, linux/mailbox_client.h, linux/pm_domain.h,
// linux/reset-controller.h, linux/semaphore.h, linux/types.h,
// soc/tegra/bpmp-abi.h

use core::ffi::c_void;

extern "C" {
    pub type tegra_bpmp_clk;
    pub type tegra_bpmp_ops;
    pub type tegra_bpmp;
    pub type tegra_ivc;
    pub type device;
    pub type mbox_client;
    pub type mbox_chan;
    pub type dentry;
    pub type iosys_map;
    pub type completion;
    pub type list_head;
    pub type semaphore;
    pub type reset_controller_dev;
    pub type genpd_onecell_data;
}

pub const MSG_DATA_MIN_SZ: usize = 0; // Supplied by bpmp-abi.h.

#[repr(C)]
pub struct tegra_bpmp_soc {
    pub channels: tegra_bpmp_soc_channels,
    pub ops: *const tegra_bpmp_ops,
    pub num_resets: u32,
}

#[repr(C)]
pub struct tegra_bpmp_soc_channels {
    pub cpu_tx: tegra_bpmp_soc_channel,
    pub thread: tegra_bpmp_soc_channel,
    pub cpu_rx: tegra_bpmp_soc_channel,
}

#[repr(C)]
pub struct tegra_bpmp_soc_channel {
    pub offset: u32,
    pub count: u32,
    pub timeout: u32,
}

#[repr(C, packed)]
pub struct tegra_bpmp_mb_data {
    pub code: u32,
    pub flags: u32,
    pub data: [u8; MSG_DATA_MIN_SZ],
}

#[macro_export]
macro_rules! tegra_bpmp_mb_read { ($dst:expr, $mb:expr, $size:expr) => {
    iosys_map_memcpy_from($dst, core::mem::offset_of!(tegra_bpmp_mb_data, data), $mb, $size)
}; }
#[macro_export]
macro_rules! tegra_bpmp_mb_write { ($mb:expr, $src:expr, $size:expr) => {
    iosys_map_memcpy_to($mb, core::mem::offset_of!(tegra_bpmp_mb_data, data), $src, $size)
}; }
#[macro_export]
macro_rules! tegra_bpmp_mb_read_field { ($mb:expr, $field:ident) => {
    iosys_map_rd_field($mb, 0, tegra_bpmp_mb_data, $field)
}; }
#[macro_export]
macro_rules! tegra_bpmp_mb_write_field { ($mb:expr, $field:ident, $value:expr) => {
    iosys_map_wr_field($mb, 0, tegra_bpmp_mb_data, $field, $value)
}; }

#[repr(C)]
pub struct tegra_bpmp_channel {
    pub bpmp: *mut tegra_bpmp,
    pub ib: iosys_map,
    pub ob: iosys_map,
    pub completion: completion,
    pub ivc: *mut tegra_ivc,
    pub index: u32,
}

pub type tegra_bpmp_mrq_handler_t = Option<unsafe extern "C" fn(u32, *mut tegra_bpmp_channel, *mut c_void)>;

#[repr(C)]
pub struct tegra_bpmp_mrq {
    pub list: list_head,
    pub mrq: u32,
    pub handler: tegra_bpmp_mrq_handler_t,
    pub data: *mut c_void,
}

#[repr(C)]
pub struct tegra_bpmp {
    pub soc: *const tegra_bpmp_soc,
    pub dev: *mut device,
    pub priv_: *mut c_void,
    pub mbox: tegra_bpmp_mbox,
    pub atomic_tx_lock: u8, // spinlock_t; supplied by Linux headers.
    pub tx_channel: *mut tegra_bpmp_channel,
    pub rx_channel: *mut tegra_bpmp_channel,
    pub threaded_channels: *mut tegra_bpmp_channel,
    pub threaded: tegra_bpmp_threaded,
    pub mrqs: list_head,
    pub lock: u8, // spinlock_t; supplied by Linux headers.
    pub clocks: *mut *mut tegra_bpmp_clk,
    pub num_clocks: u32,
    pub rstc: reset_controller_dev,
    pub genpd: genpd_onecell_data,
    #[cfg(feature = "CONFIG_DEBUG_FS")]
    pub debugfs_mirror: *mut dentry,
    pub suspended: bool,
}

#[repr(C)]
pub struct tegra_bpmp_mbox { pub client: mbox_client, pub channel: *mut mbox_chan }
#[repr(C)]
pub struct tegra_bpmp_threaded {
    pub allocated: *mut usize,
    pub busy: *mut usize,
    pub count: u32,
    pub lock: semaphore,
}

pub const TEGRA_BPMP_MESSAGE_RESET: u64 = 1 << 0;

#[repr(C)]
pub struct tegra_bpmp_message {
    pub mrq: u32,
    pub tx: tegra_bpmp_message_tx,
    pub rx: tegra_bpmp_message_rx,
    pub flags: usize,
}
#[repr(C)]
pub struct tegra_bpmp_message_tx { pub data: *const c_void, pub size: usize }
#[repr(C)]
pub struct tegra_bpmp_message_rx { pub data: *mut c_void, pub size: usize, pub ret: i32 }

// The following declarations are selected by the corresponding kernel Kconfig options.
extern "C" {
    pub fn tegra_bpmp_get(dev: *mut device) -> *mut tegra_bpmp;
    pub fn tegra_bpmp_get_with_id(dev: *mut device, id: *mut u32) -> *mut tegra_bpmp;
    pub fn tegra_bpmp_put(bpmp: *mut tegra_bpmp);
    pub fn tegra_bpmp_transfer_atomic(bpmp: *mut tegra_bpmp, msg: *mut tegra_bpmp_message) -> i32;
    pub fn tegra_bpmp_transfer(bpmp: *mut tegra_bpmp, msg: *mut tegra_bpmp_message) -> i32;
    pub fn tegra_bpmp_mrq_return(channel: *mut tegra_bpmp_channel, code: i32, data: *const c_void, size: usize);
    pub fn tegra_bpmp_request_mrq(bpmp: *mut tegra_bpmp, mrq: u32, handler: tegra_bpmp_mrq_handler_t, data: *mut c_void) -> i32;
    pub fn tegra_bpmp_free_mrq(bpmp: *mut tegra_bpmp, mrq: u32, data: *mut c_void);
    pub fn tegra_bpmp_mrq_is_supported(bpmp: *mut tegra_bpmp, mrq: u32) -> bool;
    pub fn tegra_bpmp_handle_rx(bpmp: *mut tegra_bpmp);
    pub fn tegra_bpmp_init_clocks(bpmp: *mut tegra_bpmp) -> i32;
    pub fn tegra_bpmp_init_resets(bpmp: *mut tegra_bpmp) -> i32;
    pub fn tegra_bpmp_init_powergates(bpmp: *mut tegra_bpmp) -> i32;
    pub fn tegra_bpmp_init_debugfs(bpmp: *mut tegra_bpmp) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
