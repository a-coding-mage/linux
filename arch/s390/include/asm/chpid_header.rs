/* SPDX-License-Identifier: GPL-2.0 */
/*
 *    Copyright IBM Corp. 2007, 2012
 *    Author(s): Peter Oberparleiter <peter.oberparleiter@de.ibm.com>
 */

// Dependency intent from <uapi/asm/chpid.h> and <asm/cio.h> is preserved
// through the externally supplied `chp_id`, `__MAX_CHPID`, and `__MAX_CSSID`.

#[repr(C, packed)]
pub struct channel_path_desc_fmt0 {
    pub flags: u8,
    pub lsn: u8,
    pub desc: u8,
    pub chpid: u8,
    pub swla: u8,
    pub zeroes: u8,
    pub chla: u8,
    pub chpp: u8,
}

#[inline]
pub unsafe fn chp_id_init(chpid: *mut chp_id) {
    core::ptr::write_bytes(chpid.cast::<u8>(), 0, core::mem::size_of::<chp_id>());
}

#[inline]
pub unsafe fn chp_id_is_equal(a: *mut chp_id, b: *mut chp_id) -> i32 {
    ((*a).id == (*b).id && (*a).cssid == (*b).cssid) as i32
}

#[inline]
pub unsafe fn chp_id_next(chpid: *mut chp_id) {
    if (*chpid).id < __MAX_CHPID {
        (*chpid).id += 1;
    } else {
        (*chpid).id = 0;
        (*chpid).cssid += 1;
    }
}

#[inline]
pub unsafe fn chp_id_is_valid(chpid: *mut chp_id) -> i32 {
    ((*chpid).cssid <= __MAX_CSSID) as i32
}

#[macro_export]
macro_rules! chp_id_for_each {
    ($c:expr) => {
        for (;;) {
            unsafe { $crate::chp_id_init($c); }
            if unsafe { $crate::chp_id_is_valid($c) } == 0 {
                break;
            }
            // Loop body is supplied by the caller's surrounding loop construct.
            unsafe { $crate::chp_id_next($c); }
        }
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
