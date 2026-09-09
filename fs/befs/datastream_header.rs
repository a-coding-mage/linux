/* SPDX-License-Identifier: GPL-2.0 */
/*
 * datastream.h
 *
 */

// External types and symbols are supplied by the surrounding translation unit.

unsafe extern "C" {
    pub fn befs_read_datastream(
        sb: *mut super_block,
        ds: *const befs_data_stream,
        pos: befs_off_t,
        off: *mut core::ffi::c_uint,
    ) -> *mut buffer_head;

    pub fn befs_fblock2brun(
        sb: *mut super_block,
        data: *const befs_data_stream,
        fblock: befs_blocknr_t,
        run: *mut befs_block_run,
    ) -> core::ffi::c_int;

    pub fn befs_read_lsymlink(
        sb: *mut super_block,
        data: *const befs_data_stream,
        buff: *mut core::ffi::c_void,
        len: befs_off_t,
    ) -> usize;

    pub fn befs_count_blocks(
        sb: *mut super_block,
        ds: *const befs_data_stream,
    ) -> befs_blocknr_t;

    pub static BAD_IADDR: befs_inode_addr;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
