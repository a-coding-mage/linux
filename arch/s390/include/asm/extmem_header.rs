/* SPDX-License-Identifier: GPL-2.0 */
/*
 *  definitions for external memory segment support
 *  Copyright IBM Corp. 2003
 */

/*
 * DCSS segment is defined as a contiguous range of pages using DEFSEG command.
 * The range start and end is a page number with a value less than or equal to
 * 0x7ffffff (see CP Commands and Utilities Reference).
 */
pub const MAX_DCSS_ADDR: core::ffi::c_ulong = 512 as core::ffi::c_ulong * SZ_1G;

/* possible values for segment type as returned by segment_info */
pub const SEG_TYPE_SW: core::ffi::c_int = 0;
pub const SEG_TYPE_EW: core::ffi::c_int = 1;
pub const SEG_TYPE_SR: core::ffi::c_int = 2;
pub const SEG_TYPE_ER: core::ffi::c_int = 3;
pub const SEG_TYPE_SN: core::ffi::c_int = 4;
pub const SEG_TYPE_EN: core::ffi::c_int = 5;
pub const SEG_TYPE_SC: core::ffi::c_int = 6;
pub const SEG_TYPE_EWEN: core::ffi::c_int = 7;

pub const SEGMENT_SHARED: core::ffi::c_int = 0;
pub const SEGMENT_EXCLUSIVE: core::ffi::c_int = 1;

unsafe extern "C" {
    pub fn segment_load(
        name: *mut core::ffi::c_char,
        segtype: core::ffi::c_int,
        addr: *mut core::ffi::c_ulong,
        length: *mut core::ffi::c_ulong,
    ) -> core::ffi::c_int;
    pub fn segment_unload(name: *mut core::ffi::c_char);
    pub fn segment_save(name: *mut core::ffi::c_char);
    pub fn segment_type(name: *mut core::ffi::c_char) -> core::ffi::c_int;
    pub fn segment_modify_shared(
        name: *mut core::ffi::c_char,
        do_nonshared: core::ffi::c_int,
    ) -> core::ffi::c_int;
    pub fn segment_warning(rc: core::ffi::c_int, seg_name: *mut core::ffi::c_char);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
