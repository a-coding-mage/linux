/*
 * include/asm-ppc/rheap.h
 *
 * Header file for the implementation of a remote heap.
 *
 * Author: Pantelis Antoniou <panto@intracom.gr>
 *
 * 2004 (c) INTRACOM S.A. Greece. This file is licensed under
 * the terms of the GNU General Public License version 2. This program
 * is licensed "as is" without any warranty of any kind, whether express
 * or implied.
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

// Supplied by the Linux list dependency corresponding to <linux/list.h>.
use crate::linux::list::list_head;

#[repr(C)]
pub struct rh_block_t {
    pub list: list_head,
    pub start: c_ulong,
    pub size: c_int,
    pub owner: *const c_char,
}

#[repr(C)]
pub struct rh_info_t {
    pub alignment: c_uint,
    pub max_blocks: c_int,
    pub empty_slots: c_int,
    pub block: *mut rh_block_t,
    pub empty_list: list_head,
    pub free_list: list_head,
    pub taken_list: list_head,
    pub flags: c_uint,
}

pub const RHIF_STATIC_INFO: c_uint = 0x1;
pub const RHIF_STATIC_BLOCK: c_uint = 0x2;

#[repr(C)]
pub struct rh_stats_t {
    pub start: c_ulong,
    pub size: c_int,
    pub owner: *const c_char,
}

pub const RHGS_FREE: c_int = 0;
pub const RHGS_TAKEN: c_int = 1;

/* Create a remote heap dynamically */
unsafe extern "C" {
    pub fn rh_create(alignment: c_uint) -> *mut rh_info_t;

    /* Destroy a remote heap, created by rh_create() */
    pub fn rh_destroy(info: *mut rh_info_t);

    /* Initialize in place a remote info block */
    pub fn rh_init(
        info: *mut rh_info_t,
        alignment: c_uint,
        max_blocks: c_int,
        block: *mut rh_block_t,
    );

    /* Attach a free region to manage */
    pub fn rh_attach_region(info: *mut rh_info_t, start: c_ulong, size: c_int) -> c_int;

    /* Detach a free region */
    pub fn rh_detach_region(info: *mut rh_info_t, start: c_ulong, size: c_int) -> c_ulong;

    /* Allocate the given size from the remote heap (with alignment) */
    pub fn rh_alloc_align(
        info: *mut rh_info_t,
        size: c_int,
        alignment: c_int,
        owner: *const c_char,
    ) -> c_ulong;

    /* Allocate the given size from the remote heap */
    pub fn rh_alloc(info: *mut rh_info_t, size: c_int, owner: *const c_char) -> c_ulong;

    /* Allocate the given size from the given address */
    pub fn rh_alloc_fixed(
        info: *mut rh_info_t,
        start: c_ulong,
        size: c_int,
        owner: *const c_char,
    ) -> c_ulong;

    /* Free the allocated area */
    pub fn rh_free(info: *mut rh_info_t, start: c_ulong) -> c_int;

    /* Get stats for debugging purposes */
    pub fn rh_get_stats(
        info: *mut rh_info_t,
        what: c_int,
        max_stats: c_int,
        stats: *mut rh_stats_t,
    ) -> c_int;

    /* Simple dump of remote heap info */
    pub fn rh_dump(info: *mut rh_info_t);

    /* Simple dump of remote info block */
    pub fn rh_dump_blk(info: *mut rh_info_t, blk: *mut rh_block_t);

    /* Set owner of taken block */
    pub fn rh_set_owner(info: *mut rh_info_t, start: c_ulong, owner: *const c_char) -> c_int;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
