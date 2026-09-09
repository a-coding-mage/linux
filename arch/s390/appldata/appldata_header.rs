/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Definitions and interface for Linux - z/VM Monitor Stream.
 *
 * Copyright IBM Corp. 2003, 2008
 *
 * Author: Gerald Schaefer <gerald.schaefer@de.ibm.com>
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

pub const APPLDATA_MAX_REC_SIZE: c_uint = 4024; /* Maximum size of the */
                                                /* data buffer */
pub const APPLDATA_MAX_PROCS: c_uint = 100;

pub const APPLDATA_PROC_NAME_LENGTH: usize = 16; /* Max. length of /proc name */

pub const APPLDATA_RECORD_MEM_ID: u8 = 0x01; /* IDs to identify the */
pub const APPLDATA_RECORD_OS_ID: u8 = 0x02; /* individual records, */
pub const APPLDATA_RECORD_NET_SUM_ID: u8 = 0x03; /* must be < 256 !     */
pub const APPLDATA_RECORD_PROC_ID: u8 = 0x04;

pub const CTL_APPLDATA_TIMER: c_int = 2121; /* sysctl IDs, must be unique */
pub const CTL_APPLDATA_INTERVAL: c_int = 2122;
pub const CTL_APPLDATA_MEM: c_int = 2123;
pub const CTL_APPLDATA_OS: c_int = 2124;
pub const CTL_APPLDATA_NET_SUM: c_int = 2125;
pub const CTL_APPLDATA_PROC: c_int = 2126;

#[repr(C)]
pub struct appldata_ops {
    pub list: crate::list_head,
    pub sysctl_header: *mut crate::ctl_table_header,
    pub ctl_table: *mut crate::ctl_table,
    pub active: c_int, /* monitoring status */

    /* fill in from here */
    pub name: [c_char; APPLDATA_PROC_NAME_LENGTH], /* name of /proc fs node */
    pub record_nr: u8, /* Record Nr. for Product ID */
    pub callback: Option<unsafe extern "C" fn(data: *mut c_void)>, /* callback function */
    pub data: *mut c_void, /* record data */
    pub size: c_uint, /* size of record */
    pub owner: *mut crate::module, /* THIS_MODULE */
    pub mod_lvl: [c_char; 2], /* modification level, EBCDIC */
}

extern "C" {
    pub fn appldata_register_ops(ops: *mut appldata_ops) -> c_int;
    pub fn appldata_unregister_ops(ops: *mut appldata_ops);
    pub fn appldata_diag(
        record_nr: c_char,
        function: u16,
        buffer: c_ulong,
        length: u16,
        mod_lvl: *mut c_char,
    ) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
