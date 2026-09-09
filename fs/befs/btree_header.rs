/* SPDX-License-Identifier: GPL-2.0 */
/*
 * btree.h
 *
 */

extern "C" {
    pub fn befs_btree_find(
        sb: *mut super_block,
        ds: *const befs_data_stream,
        key: *const core::ffi::c_char,
        value: *mut befs_off_t,
    ) -> core::ffi::c_int;

    pub fn befs_btree_read(
        sb: *mut super_block,
        ds: *const befs_data_stream,
        key_no: loff_t,
        bufsize: usize,
        keybuf: *mut core::ffi::c_char,
        keysize: *mut usize,
        value: *mut befs_off_t,
    ) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
