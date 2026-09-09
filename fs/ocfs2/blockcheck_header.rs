/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * blockcheck.h
 *
 * Checksum and ECC codes for the OCFS2 userspace library.
 *
 * Copyright (C) 2004, 2008 Oracle.  All rights reserved.
 */

use core::ffi::c_void;

/* Count errors and error correction from blockcheck.c */
#[repr(C)]
pub struct ocfs2_blockcheck_stats {
    pub b_lock: spinlock_t,
    pub b_check_count: u64,   /* Number of blocks we've checked */
    pub b_failure_count: u64, /* Number of failed checksums */
    pub b_recover_count: u64, /* Number of blocks fixed by ecc */

    /*
     * debugfs entries, used if this is passed to
     * ocfs2_blockcheck_stats_debugfs_install()
     */
    pub b_debug_dir: *mut dentry, /* Parent of the debugfs files */
}

/* High level block API */
extern "C" {
    pub fn ocfs2_compute_meta_ecc(
        sb: *mut super_block,
        data: *mut c_void,
        bc: *mut ocfs2_block_check,
    );
    pub fn ocfs2_validate_meta_ecc(
        sb: *mut super_block,
        data: *mut c_void,
        bc: *mut ocfs2_block_check,
    ) -> i32;
    pub fn ocfs2_compute_meta_ecc_bhs(
        sb: *mut super_block,
        bhs: *mut *mut buffer_head,
        nr: i32,
        bc: *mut ocfs2_block_check,
    );
    pub fn ocfs2_validate_meta_ecc_bhs(
        sb: *mut super_block,
        bhs: *mut *mut buffer_head,
        nr: i32,
        bc: *mut ocfs2_block_check,
    ) -> i32;

    /* Lower level API */
    pub fn ocfs2_block_check_compute(
        data: *mut c_void,
        blocksize: usize,
        bc: *mut ocfs2_block_check,
    );
    pub fn ocfs2_block_check_validate(
        data: *mut c_void,
        blocksize: usize,
        bc: *mut ocfs2_block_check,
        stats: *mut ocfs2_blockcheck_stats,
    ) -> i32;
    pub fn ocfs2_block_check_compute_bhs(
        bhs: *mut *mut buffer_head,
        nr: i32,
        bc: *mut ocfs2_block_check,
    );
    pub fn ocfs2_block_check_validate_bhs(
        bhs: *mut *mut buffer_head,
        nr: i32,
        bc: *mut ocfs2_block_check,
        stats: *mut ocfs2_blockcheck_stats,
    ) -> i32;

    /* Debug Initialization */
    pub fn ocfs2_blockcheck_stats_debugfs_install(
        stats: *mut ocfs2_blockcheck_stats,
        parent: *mut dentry,
    );
    pub fn ocfs2_blockcheck_stats_debugfs_remove(stats: *mut ocfs2_blockcheck_stats);

    /* Hamming code functions */

    /*
     * Encoding hamming code parity bits for a buffer.
     *
     * This is the low level encoder function.  It can be called across
     * multiple hunks just like the crc32 code.  'd' is the number of bits
     * _in_this_hunk_.  nr is the bit offset of this hunk.  So, if you had
     * two 512B buffers, you would do it like so:
     *
     * parity = ocfs2_hamming_encode(0, buf1, 512 * 8, 0);
     * parity = ocfs2_hamming_encode(parity, buf2, 512 * 8, 512 * 8);
     *
     * If you just have one buffer, use ocfs2_hamming_encode_block().
     */
    pub fn ocfs2_hamming_encode(
        parity: u32,
        data: *mut c_void,
        d: u32,
        nr: u32,
    ) -> u32;

    /*
     * Fix a buffer with a bit error.  The 'fix' is the original parity
     * xor'd with the parity calculated now.
     *
     * Like ocfs2_hamming_encode(), this can handle hunks.  nr is the bit
     * offset of the current hunk.  If bit to be fixed is not part of the
     * current hunk, this does nothing.
     *
     * If you only have one buffer, use ocfs2_hamming_fix_block().
     */
    pub fn ocfs2_hamming_fix(data: *mut c_void, d: u32, nr: u32, fix: u32);

    /* Convenience wrappers for a single buffer of data */
    pub fn ocfs2_hamming_encode_block(data: *mut c_void, blocksize: u32) -> u32;
    pub fn ocfs2_hamming_fix_block(data: *mut c_void, blocksize: u32, fix: u32);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
