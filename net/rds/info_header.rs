/* SPDX-License-Identifier: GPL-2.0 */

#[repr(C)]
pub struct rds_info_lengths {
    pub nr: u32,
    pub each: u32,
}

#[repr(C)]
pub struct rds_info_iterator {
    _private: [u8; 0],
}

/*
 * These functions must fill in the fields of @lens to reflect the size
 * of the available info source.  If the snapshot fits in @len then it
 * should be copied using @iter.  The caller will deduce if it was copied
 * or not by comparing the lengths.
 */
pub type rds_info_func = unsafe extern "C" fn(
    sock: *mut socket,
    len: u32,
    iter: *mut rds_info_iterator,
    lens: *mut rds_info_lengths,
);

unsafe extern "C" {
    pub fn rds_info_register_func(optname: i32, func: rds_info_func);
    pub fn rds_info_deregister_func(optname: i32, func: rds_info_func);
    pub fn rds_info_getsockopt(sock: *mut socket, optname: i32, opt: *mut sockopt_t) -> i32;
    pub fn rds_info_copy(iter: *mut rds_info_iterator, data: *mut core::ffi::c_void, bytes: core::ffi::c_ulong);
    pub fn rds_info_iter_unmap(iter: *mut rds_info_iterator);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
