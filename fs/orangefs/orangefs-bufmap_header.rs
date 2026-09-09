/* SPDX-License-Identifier: GPL-2.0 */
/*
 * (C) 2001 Clemson University and The University of Chicago
 *
 * See COPYING in top-level directory.
 */

// C header guard: __ORANGEFS_BUFMAP_H

// External types supplied by other translated dependencies.
#[allow(non_camel_case_types)]
pub enum ORANGEFS_dev_map_desc {}

#[allow(non_camel_case_types)]
pub enum iov_iter {}

unsafe extern "C" {
    pub fn orangefs_bufmap_size_query() -> ::core::ffi::c_int;

    pub fn orangefs_bufmap_initialize(
        user_desc: *mut ORANGEFS_dev_map_desc,
    ) -> ::core::ffi::c_int;

    pub fn orangefs_bufmap_finalize();

    pub fn orangefs_bufmap_run_down();

    pub fn orangefs_bufmap_get() -> ::core::ffi::c_int;

    pub fn orangefs_bufmap_put(buffer_index: ::core::ffi::c_int);

    pub fn orangefs_readdir_index_get() -> ::core::ffi::c_int;

    pub fn orangefs_readdir_index_put(buffer_index: ::core::ffi::c_int);

    pub fn orangefs_bufmap_copy_from_iovec(
        iter: *mut iov_iter,
        buffer_index: ::core::ffi::c_int,
        size: usize,
    ) -> ::core::ffi::c_int;

    pub fn orangefs_bufmap_copy_to_iovec(
        iter: *mut iov_iter,
        buffer_index: ::core::ffi::c_int,
        size: usize,
    ) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
