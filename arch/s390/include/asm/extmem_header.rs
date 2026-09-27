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
pub const MAX_DCSS_ADDR: kernel::ffi::c_ulong = 512 as kernel::ffi::c_ulong * SZ_1G;

/* possible values for segment type as returned by segment_info */
pub const SEG_TYPE_SW: kernel::ffi::c_int = 0;
pub const SEG_TYPE_EW: kernel::ffi::c_int = 1;
pub const SEG_TYPE_SR: kernel::ffi::c_int = 2;
pub const SEG_TYPE_ER: kernel::ffi::c_int = 3;
pub const SEG_TYPE_SN: kernel::ffi::c_int = 4;
pub const SEG_TYPE_EN: kernel::ffi::c_int = 5;
pub const SEG_TYPE_SC: kernel::ffi::c_int = 6;
pub const SEG_TYPE_EWEN: kernel::ffi::c_int = 7;

pub const SEGMENT_SHARED: kernel::ffi::c_int = 0;
pub const SEGMENT_EXCLUSIVE: kernel::ffi::c_int = 1;

unsafe extern "C" {
    pub fn segment_load(
        name: *mut kernel::ffi::c_char,
        segtype: kernel::ffi::c_int,
        addr: *mut kernel::ffi::c_ulong,
        length: *mut kernel::ffi::c_ulong,
    ) -> kernel::ffi::c_int;
    pub fn segment_unload(name: *mut kernel::ffi::c_char);
    pub fn segment_save(name: *mut kernel::ffi::c_char);
    pub fn segment_type(name: *mut kernel::ffi::c_char) -> kernel::ffi::c_int;
    pub fn segment_modify_shared(
        name: *mut kernel::ffi::c_char,
        do_nonshared: kernel::ffi::c_int,
    ) -> kernel::ffi::c_int;
    pub fn segment_warning(rc: kernel::ffi::c_int, seg_name: *mut kernel::ffi::c_char);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
