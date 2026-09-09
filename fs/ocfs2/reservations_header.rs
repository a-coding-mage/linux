/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * reservations.h
 *
 * Allocation reservations function prototypes and structures.
 *
 * Copyright (C) 2010 Novell.  All rights reserved.
 */

/* Dependency types supplied by the surrounding kernel translation. */

pub const OCFS2_DEFAULT_RESV_LEVEL: u32 = 2;
pub const OCFS2_MAX_RESV_LEVEL: u32 = 9;
pub const OCFS2_MIN_RESV_LEVEL: u32 = 0;

#[repr(C)]
pub struct ocfs2_alloc_reservation {
    pub r_node: rb_node,

    pub r_start: core::ffi::c_uint, /* Beginning of current window */
    pub r_len: core::ffi::c_uint, /* Length of the window */

    pub r_last_len: core::ffi::c_uint, /* Length of most recent alloc */
    pub r_last_start: core::ffi::c_uint, /* Start of most recent alloc */
    pub r_lru: list_head, /* LRU list head */

    pub r_flags: core::ffi::c_uint,
}

pub const OCFS2_RESV_FLAG_INUSE: u32 = 0x01; /* Set when r_node is part of a btree */
pub const OCFS2_RESV_FLAG_TMP: u32 = 0x02; /* Temporary reservation, will be
                                            * destroyed immediately after use */
pub const OCFS2_RESV_FLAG_DIR: u32 = 0x04; /* Reservation is for an unindexed
                                            * directory btree */

#[repr(C)]
pub struct ocfs2_reservation_map {
    pub m_reservations: rb_root,
    pub m_disk_bitmap: *mut core::ffi::c_char,

    pub m_osb: *mut ocfs2_super,

    /* The following are not initialized to meaningful values until a disk
     * bitmap is provided. */
    pub m_bitmap_len: u32, /* Number of valid
                            * bits available */

    pub m_lru: list_head, /* LRU of reservations
                           * structures. */
}

pub const OCFS2_RESV_TYPES: u32 = OCFS2_RESV_FLAG_TMP | OCFS2_RESV_FLAG_DIR;

unsafe extern "C" {
    pub fn ocfs2_resv_init_once(resv: *mut ocfs2_alloc_reservation);

    pub fn ocfs2_resv_set_type(
        resv: *mut ocfs2_alloc_reservation,
        flags: core::ffi::c_uint,
    );

    pub fn ocfs2_dir_resv_allowed(osb: *mut ocfs2_super) -> core::ffi::c_int;

    /**
     * ocfs2_resv_discard() - truncate a reservation
     * @resmap:
     * @resv: the reservation to truncate.
     *
     * After this function is called, the reservation will be empty, and
     * unlinked from the rbtree.
     */
    pub fn ocfs2_resv_discard(
        resmap: *mut ocfs2_reservation_map,
        resv: *mut ocfs2_alloc_reservation,
    );

    /**
     * ocfs2_resmap_init() - Initialize fields of a reservations bitmap
     * @osb: struct ocfs2_super to be saved in resmap
     * @resmap: struct ocfs2_reservation_map to initialize
     */
    pub fn ocfs2_resmap_init(
        osb: *mut ocfs2_super,
        resmap: *mut ocfs2_reservation_map,
    );

    /**
     * ocfs2_resmap_restart() - "restart" a reservation bitmap
     * @resmap: reservations bitmap
     * @clen: Number of valid bits in the bitmap
     * @disk_bitmap: the disk bitmap this resmap should refer to.
     *
     * Re-initialize the parameters of a reservation bitmap. This is
     * useful for local alloc window slides.
     *
     * This function will call ocfs2_trunc_resv against all existing
     * reservations. A future version will recalculate existing
     * reservations based on the new bitmap.
     */
    pub fn ocfs2_resmap_restart(
        resmap: *mut ocfs2_reservation_map,
        clen: core::ffi::c_uint,
        disk_bitmap: *mut core::ffi::c_char,
    );

    /**
     * ocfs2_resmap_uninit() - uninitialize a reservation bitmap structure
     * @resmap: the struct ocfs2_reservation_map to uninitialize
     */
    pub fn ocfs2_resmap_uninit(resmap: *mut ocfs2_reservation_map);

    /**
     * ocfs2_resmap_resv_bits() - Return still-valid reservation bits
     * @resmap: reservations bitmap
     * @resv: reservation to base search from
     * @cstart: start of proposed allocation
     * @clen: length (in clusters) of proposed allocation
     *
     * Using the reservation data from resv, this function will compare
     * resmap and resmap->m_disk_bitmap to determine what part (if any) of
     * the reservation window is still clear to use. If resv is empty,
     * this function will try to allocate a window for it.
     *
     * On success, zero is returned and the valid allocation area is set in cstart
     * and clen.
     *
     * Returns -ENOSPC if reservations are disabled.
     */
    pub fn ocfs2_resmap_resv_bits(
        resmap: *mut ocfs2_reservation_map,
        resv: *mut ocfs2_alloc_reservation,
        cstart: *mut core::ffi::c_int,
        clen: *mut core::ffi::c_int,
    ) -> core::ffi::c_int;

    /**
     * ocfs2_resmap_claimed_bits() - Tell the reservation code that bits were used.
     * @resmap: reservations bitmap
     * @resv: optional reservation to recalculate based on new bitmap
     * @cstart: start of allocation in clusters
     * @clen: end of allocation in clusters.
     *
     * Tell the reservation code that bits were used to fulfill allocation in
     * resmap. The bits don't have to have been part of any existing
     * reservation. But we must always call this function when bits are claimed.
     * Internally, the reservations code will use this information to mark the
     * reservations bitmap. If resv is passed, it's next allocation window will be
     * calculated. It also expects that 'cstart' is the same as we passed back
     * from ocfs2_resmap_resv_bits().
     */
    pub fn ocfs2_resmap_claimed_bits(
        resmap: *mut ocfs2_reservation_map,
        resv: *mut ocfs2_alloc_reservation,
        cstart: u32,
        clen: u32,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
