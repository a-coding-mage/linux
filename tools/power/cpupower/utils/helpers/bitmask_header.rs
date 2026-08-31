/* SPDX-License-Identifier: GPL-2.0 */

/* Taken over from libbitmask, a project initiated from sgi:
 * Url:            http://oss.sgi.com/projects/cpusets/
 * Unfortunately it's not very widespread, therefore relevant parts are
 * pasted here.
 */

#[repr(C)]
pub struct bitmask {
    pub size: ::std::os::raw::c_uint,
    pub maskp: *mut ::std::os::raw::c_ulong,
}

unsafe extern "C" {
    pub fn bitmask_alloc(n: ::std::os::raw::c_uint) -> *mut bitmask;
    pub fn bitmask_free(bmp: *mut bitmask);

    pub fn bitmask_setbit(bmp: *mut bitmask, i: ::std::os::raw::c_uint) -> *mut bitmask;
    pub fn bitmask_setall(bmp: *mut bitmask) -> *mut bitmask;
    pub fn bitmask_clearall(bmp: *mut bitmask) -> *mut bitmask;

    pub fn bitmask_first(bmp: *const bitmask) -> ::std::os::raw::c_uint;
    pub fn bitmask_next(
        bmp: *const bitmask,
        i: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_uint;
    pub fn bitmask_last(bmp: *const bitmask) -> ::std::os::raw::c_uint;
    pub fn bitmask_isallclear(bmp: *const bitmask) -> ::std::os::raw::c_int;
    pub fn bitmask_isbitset(
        bmp: *const bitmask,
        i: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;

    pub fn bitmask_parselist(buf: *const ::std::os::raw::c_char, bmp: *mut bitmask) -> ::std::os::raw::c_int;
    pub fn bitmask_displaylist(
        buf: *mut ::std::os::raw::c_char,
        len: ::std::os::raw::c_int,
        bmp: *const bitmask,
    ) -> ::std::os::raw::c_int;
}
