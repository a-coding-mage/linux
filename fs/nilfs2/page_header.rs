/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * Buffer/page management specific to NILFS
 *
 * Copyright (C) 2005-2008 Nippon Telegraph and Telephone Corporation.
 *
 * Written by Ryusuke Konishi and Seiji Kihara.
 */

/* Dependencies: linux/buffer_head.h and nilfs.h. */

/*
 * Extended buffer state bits
 */
pub const BH_NILFS_Allocated: usize = BH_PrivateStart;
pub const BH_NILFS_Node: usize = BH_NILFS_Allocated + 1;
pub const BH_NILFS_Volatile: usize = BH_NILFS_Node + 1;
pub const BH_NILFS_Checked: usize = BH_NILFS_Volatile + 1;
pub const BH_NILFS_Redirected: usize = BH_NILFS_Checked + 1;

/* BUFFER_FNS(NILFS_Node, nilfs_node)        -- nilfs node buffers */
/* BUFFER_FNS(NILFS_Volatile, nilfs_volatile) */
/* BUFFER_FNS(NILFS_Checked, nilfs_checked)  -- buffer is verified */
/* BUFFER_FNS(NILFS_Redirected, nilfs_redirected) -- redirected to a copy */

extern "C" {
    pub fn __nilfs_clear_folio_dirty(folio: *mut folio);

    pub fn nilfs_grab_buffer(
        inode: *mut inode,
        mapping: *mut address_space,
        index: c_ulong,
        blocksize: c_ulong,
    ) -> *mut buffer_head;
    pub fn nilfs_forget_buffer(bh: *mut buffer_head);
    pub fn nilfs_copy_buffer(dst: *mut buffer_head, src: *mut buffer_head);
    pub fn nilfs_folio_buffers_clean(folio: *mut folio) -> bool;
    pub fn nilfs_folio_bug(folio: *mut folio);

    pub fn nilfs_copy_dirty_pages(
        from: *mut address_space,
        to: *mut address_space,
    ) -> c_int;
    pub fn nilfs_copy_back_pages(from: *mut address_space, to: *mut address_space);
    pub fn nilfs_clear_folio_dirty(folio: *mut folio);
    pub fn nilfs_clear_dirty_pages(mapping: *mut address_space);
    pub fn nilfs_page_count_clean_buffers(
        folio: *mut folio,
        from: c_uint,
        to: c_uint,
    ) -> c_uint;
    pub fn nilfs_find_uncommitted_extent(
        inode: *mut inode,
        start_blk: sector_t,
        blkoff: *mut sector_t,
    ) -> c_ulong;
}

/* The C variadic macro ignores its extra arguments and invokes BUG(). */
#[macro_export]
macro_rules! NILFS_FOLIO_BUG {
    ($folio:expr, $m:expr $(, $a:tt)*) => {{
        unsafe { $crate::nilfs_folio_bug($folio); }
        BUG!();
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
