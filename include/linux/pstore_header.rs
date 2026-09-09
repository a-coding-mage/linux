/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Persistent Storage - pstore.h
 *
 * Copyright (C) 2010 Intel Corporation <tony.luck@intel.com>
 *
 * This code is the generic layer to export data records from platform
 * level persistent storage via a file system.
 */

// Kernel dependencies supplied by other translated units.

pub struct module;

/* pstore record types. These values may be written to storage and are ABI. */
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pstore_type_id {
    PSTORE_TYPE_DMESG = 0,
    PSTORE_TYPE_MCE = 1,
    PSTORE_TYPE_CONSOLE = 2,
    PSTORE_TYPE_FTRACE = 3,
    PSTORE_TYPE_PPC_RTAS = 4,
    PSTORE_TYPE_PPC_OF = 5,
    PSTORE_TYPE_PPC_COMMON = 6,
    PSTORE_TYPE_PMSG = 7,
    PSTORE_TYPE_PPC_OPAL = 8,
    PSTORE_TYPE_MAX,
}

unsafe extern "C" {
    pub fn pstore_type_to_name(type_: pstore_type_id) -> *const ::core::ffi::c_char;
    pub fn pstore_name_to_type(name: *const ::core::ffi::c_char) -> pstore_type_id;
}

pub struct pstore_info;

#[repr(C)]
pub struct pstore_record {
    pub psi: *mut pstore_info,
    pub type_: pstore_type_id,
    pub id: u64,
    pub time: timespec64,
    pub buf: *mut ::core::ffi::c_char,
    pub size: ssize_t,
    pub ecc_notice_size: ssize_t,
    pub priv_: *mut ::core::ffi::c_void,
    pub count: ::core::ffi::c_int,
    pub reason: kmsg_dump_reason,
    pub part: ::core::ffi::c_uint,
    pub compressed: bool,
}

#[repr(C)]
pub struct pstore_info {
    pub owner: *mut module,
    pub name: *const ::core::ffi::c_char,
    pub buf_lock: raw_spinlock_t,
    pub buf: *mut ::core::ffi::c_char,
    pub bufsize: usize,
    pub read_mutex: mutex,
    pub flags: ::core::ffi::c_int,
    pub max_reason: ::core::ffi::c_int,
    pub data: *mut ::core::ffi::c_void,
    pub open: Option<unsafe extern "C" fn(psi: *mut pstore_info) -> ::core::ffi::c_int>,
    pub close: Option<unsafe extern "C" fn(psi: *mut pstore_info) -> ::core::ffi::c_int>,
    pub read: Option<unsafe extern "C" fn(record: *mut pstore_record) -> ssize_t>,
    pub write: Option<unsafe extern "C" fn(record: *mut pstore_record) -> ::core::ffi::c_int>,
    pub write_user: Option<unsafe extern "C" fn(record: *mut pstore_record, buf: *const ::core::ffi::c_char) -> ::core::ffi::c_int>,
    pub erase: Option<unsafe extern "C" fn(record: *mut pstore_record) -> ::core::ffi::c_int>,
}

pub const PSTORE_FLAGS_DMESG: ::core::ffi::c_int = 1 << 0;
pub const PSTORE_FLAGS_CONSOLE: ::core::ffi::c_int = 1 << 1;
pub const PSTORE_FLAGS_FTRACE: ::core::ffi::c_int = 1 << 2;
pub const PSTORE_FLAGS_PMSG: ::core::ffi::c_int = 1 << 3;

unsafe extern "C" {
    pub fn pstore_register(psi: *mut pstore_info) -> ::core::ffi::c_int;
    pub fn pstore_unregister(psi: *mut pstore_info);
}

#[repr(C)]
pub struct pstore_ftrace_record {
    pub ip: ::core::ffi::c_ulong,
    pub parent_ip: ::core::ffi::c_ulong,
    pub ts: u64,
}

pub const TS_CPU_SHIFT: u32 = 8;
pub const TS_CPU_MASK: u64 = (1u64 << TS_CPU_SHIFT) - 1;

/* The original build-time NR_CPUS/CONFIG_ARM conditions are represented by cfg features. */
#[cfg(any(all(feature = "nr_cpus_le_2", feature = "config_arm_thumb"), all(feature = "nr_cpus_le_4", feature = "config_arm")))]
pub const PSTORE_CPU_IN_IP: ::core::ffi::c_uint = if cfg!(all(feature = "nr_cpus_le_2", feature = "config_arm_thumb")) { 0x1 } else { 0x3 };

#[cfg(any(all(feature = "nr_cpus_le_2", feature = "config_arm_thumb"), all(feature = "nr_cpus_le_4", feature = "config_arm")))]
pub unsafe fn pstore_ftrace_encode_cpu(rec: *mut pstore_ftrace_record, cpu: ::core::ffi::c_uint) { (*rec).ip |= cpu as ::core::ffi::c_ulong; }

#[cfg(any(all(feature = "nr_cpus_le_2", feature = "config_arm_thumb"), all(feature = "nr_cpus_le_4", feature = "config_arm")))]
pub unsafe fn pstore_ftrace_decode_cpu(rec: *mut pstore_ftrace_record) -> ::core::ffi::c_uint { ((*rec).ip & PSTORE_CPU_IN_IP as ::core::ffi::c_ulong) as ::core::ffi::c_uint }

#[cfg(any(all(feature = "nr_cpus_le_2", feature = "config_arm_thumb"), all(feature = "nr_cpus_le_4", feature = "config_arm")))]
pub unsafe fn pstore_ftrace_read_timestamp(rec: *mut pstore_ftrace_record) -> u64 { (*rec).ts }

#[cfg(any(all(feature = "nr_cpus_le_2", feature = "config_arm_thumb"), all(feature = "nr_cpus_le_4", feature = "config_arm")))]
pub unsafe fn pstore_ftrace_write_timestamp(rec: *mut pstore_ftrace_record, val: u64) { (*rec).ts = val; }

#[cfg(not(any(all(feature = "nr_cpus_le_2", feature = "config_arm_thumb"), all(feature = "nr_cpus_le_4", feature = "config_arm"))))]
pub unsafe fn pstore_ftrace_encode_cpu(rec: *mut pstore_ftrace_record, cpu: ::core::ffi::c_uint) { (*rec).ts &= !TS_CPU_MASK; (*rec).ts |= cpu as u64; }

#[cfg(not(any(all(feature = "nr_cpus_le_2", feature = "config_arm_thumb"), all(feature = "nr_cpus_le_4", feature = "config_arm"))))]
pub unsafe fn pstore_ftrace_decode_cpu(rec: *mut pstore_ftrace_record) -> ::core::ffi::c_uint { ((*rec).ts & TS_CPU_MASK) as ::core::ffi::c_uint }

#[cfg(not(any(all(feature = "nr_cpus_le_2", feature = "config_arm_thumb"), all(feature = "nr_cpus_le_4", feature = "config_arm"))))]
pub unsafe fn pstore_ftrace_read_timestamp(rec: *mut pstore_ftrace_record) -> u64 { (*rec).ts >> TS_CPU_SHIFT }

#[cfg(not(any(all(feature = "nr_cpus_le_2", feature = "config_arm_thumb"), all(feature = "nr_cpus_le_4", feature = "config_arm"))))]
pub unsafe fn pstore_ftrace_write_timestamp(rec: *mut pstore_ftrace_record, val: u64) { (*rec).ts = ((*rec).ts & TS_CPU_MASK) | (val << TS_CPU_SHIFT); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
