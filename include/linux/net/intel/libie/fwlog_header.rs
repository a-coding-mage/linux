/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (C) 2022, Intel Corporation. */

// Dependency supplied by linux/net/intel/libie/adminq.h is referenced by name
// below and is not defined in this translation unit.

/* Only a single log level should be set and all log levels under the set value
 * are enabled, e.g. if log level is set to LIBIE_FW_LOG_LEVEL_VERBOSE, then all
 * other log levels are included (except LIBIE_FW_LOG_LEVEL_NONE)
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub enum libie_fwlog_level {
    LIBIE_FWLOG_LEVEL_NONE = 0,
    LIBIE_FWLOG_LEVEL_ERROR = 1,
    LIBIE_FWLOG_LEVEL_WARNING = 2,
    LIBIE_FWLOG_LEVEL_NORMAL = 3,
    LIBIE_FWLOG_LEVEL_VERBOSE = 4,
    /* all values >= this entry are invalid */
    LIBIE_FWLOG_LEVEL_INVALID,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct libie_fwlog_module_entry {
    /* module ID for the corresponding firmware logging event */
    pub module_id: u16,
    /* verbosity level for the module_id */
    pub log_level: u8,
}

#[repr(C)]
pub struct libie_fwlog_cfg {
    /* list of modules for configuring log level */
    pub module_entries: [libie_fwlog_module_entry; LIBIE_AQC_FW_LOG_ID_MAX as usize],
    /* options used to configure firmware logging */
    pub options: u16,
    pub log_resolution: u16,
}

pub const LIBIE_FWLOG_OPTION_ARQ_ENA: u16 = 1u16 << 0;
pub const LIBIE_FWLOG_OPTION_UART_ENA: u16 = 1u16 << 1;
/* set before calling libie_fwlog_init() so the PF registers for
 * firmware logging on initialization
 */
pub const LIBIE_FWLOG_OPTION_REGISTER_ON_INIT: u16 = 1u16 << 2;
/* set in the libie_aq_fwlog_get() response if the PF is registered for
 * FW logging events over ARQ
 */
pub const LIBIE_FWLOG_OPTION_IS_REGISTERED: u16 = 1u16 << 3;

#[repr(C)]
pub struct libie_fwlog_data {
    pub data_size: u16,
    pub data: *mut u8,
}

#[repr(C)]
pub struct libie_fwlog_ring {
    pub rings: *mut libie_fwlog_data,
    pub index: u16,
    pub size: u16,
    pub head: u16,
    pub tail: u16,
}

pub const LIBIE_FWLOG_RING_SIZE_INDEX_DFLT: u16 = 3;
pub const LIBIE_FWLOG_RING_SIZE_DFLT: u16 = 256;
pub const LIBIE_FWLOG_RING_SIZE_MAX: u16 = 512;

#[repr(C)]
pub struct libie_fwlog_api {
    pub pdev: *mut pci_dev,
    pub send_cmd: Option<unsafe extern "C" fn(*mut core::ffi::c_void, *mut libie_aq_desc, *mut core::ffi::c_void, u16) -> i32>,
    pub priv_: *mut core::ffi::c_void,
    pub debugfs_root: *mut dentry,
}

#[repr(C)]
pub struct libie_fwlog {
    pub cfg: libie_fwlog_cfg,
    pub supported: bool, /* does hardware support FW logging? */
    pub ring: libie_fwlog_ring,
    pub debugfs: *mut dentry,
    /* keep track of all the dentrys for FW log modules */
    pub debugfs_modules: *mut *mut dentry,
    pub api: libie_fwlog_api,
}

#[cfg(CONFIG_LIBIE_FWLOG)]
extern "C" {
    pub fn libie_fwlog_init(fwlog: *mut libie_fwlog, api: *mut libie_fwlog_api) -> i32;
    pub fn libie_fwlog_deinit(fwlog: *mut libie_fwlog);
    pub fn libie_fwlog_reregister(fwlog: *mut libie_fwlog);
    pub fn libie_get_fwlog_data(fwlog: *mut libie_fwlog, buf: *mut u8, len: u16);
}

#[cfg(not(CONFIG_LIBIE_FWLOG))]
#[inline]
pub unsafe fn libie_fwlog_init(_fwlog: *mut libie_fwlog, _api: *mut libie_fwlog_api) -> i32 {
    -EOPNOTSUPP
}

#[cfg(not(CONFIG_LIBIE_FWLOG))]
#[inline]
pub unsafe fn libie_fwlog_deinit(_fwlog: *mut libie_fwlog) {}

#[cfg(not(CONFIG_LIBIE_FWLOG))]
#[inline]
pub unsafe fn libie_fwlog_reregister(_fwlog: *mut libie_fwlog) {}

#[cfg(not(CONFIG_LIBIE_FWLOG))]
#[inline]
pub unsafe fn libie_get_fwlog_data(_fwlog: *mut libie_fwlog, _buf: *mut u8, _len: u16) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
