/*
 * JFFS2 -- Journalling Flash File System, Version 2.
 *
 * Copyright © 2004  Ferenc Havasi <havasi@inf.u-szeged.hu>,
 *                 Zoltan Sogor <weth@inf.u-szeged.hu>,
 *                 Patrik Kluba <pajko@halom.u-szeged.hu>,
 *                 University of Szeged, Hungary
 *
 * For licensing information, see the file 'LICENCE' in this directory.
 */

/* Limit summary size to 64KiB so that we can kmalloc it. */
pub const MAX_SUMMARY_SIZE: u32 = 65536;

pub const BLK_STATE_ALLFF: u32 = 0;
pub const BLK_STATE_CLEAN: u32 = 1;
pub const BLK_STATE_PARTDIRTY: u32 = 2;
pub const BLK_STATE_CLEANMARKER: u32 = 3;
pub const BLK_STATE_ALLDIRTY: u32 = 4;
pub const BLK_STATE_BADBLOCK: u32 = 5;

pub const JFFS2_SUMMARY_NOSUM_SIZE: u32 = 0xffff_ffff;
pub const JFFS2_SUMMARY_INODE_SIZE: usize = core::mem::size_of::<jffs2_sum_inode_flash>();
pub const fn jffs2_summary_dirent_size(x: usize) -> usize {
    core::mem::size_of::<jffs2_sum_dirent_flash>() + x
}
pub const JFFS2_SUMMARY_XATTR_SIZE: usize = core::mem::size_of::<jffs2_sum_xattr_flash>();
pub const JFFS2_SUMMARY_XREF_SIZE: usize = core::mem::size_of::<jffs2_sum_xref_flash>();

#[repr(C, packed)]
pub struct jffs2_sum_unknown_flash { pub nodetype: jint16_t }

#[repr(C, packed)]
pub struct jffs2_sum_inode_flash {
    pub nodetype: jint16_t,
    pub inode: jint32_t,
    pub version: jint32_t,
    pub offset: jint32_t,
    pub totlen: jint32_t,
}

#[repr(C, packed)]
pub struct jffs2_sum_dirent_flash {
    pub nodetype: jint16_t,
    pub totlen: jint32_t,
    pub offset: jint32_t,
    pub pino: jint32_t,
    pub version: jint32_t,
    pub ino: jint32_t,
    pub nsize: u8,
    pub r#type: u8,
    pub name: [u8; 0],
}

#[repr(C, packed)]
pub struct jffs2_sum_xattr_flash {
    pub nodetype: jint16_t, pub xid: jint32_t, pub version: jint32_t,
    pub offset: jint32_t, pub totlen: jint32_t,
}

#[repr(C, packed)]
pub struct jffs2_sum_xref_flash { pub nodetype: jint16_t, pub offset: jint32_t }

#[repr(C)]
pub union jffs2_sum_flash {
    pub u: jffs2_sum_unknown_flash,
    pub i: jffs2_sum_inode_flash,
    pub d: jffs2_sum_dirent_flash,
    pub x: jffs2_sum_xattr_flash,
    pub r: jffs2_sum_xref_flash,
}

#[repr(C)]
pub union jffs2_sum_mem {
    pub u: jffs2_sum_unknown_mem,
    pub i: jffs2_sum_inode_mem,
    pub d: jffs2_sum_dirent_mem,
    pub x: jffs2_sum_xattr_mem,
    pub r: jffs2_sum_xref_mem,
}

#[repr(C)]
pub struct jffs2_sum_unknown_mem { pub next: *mut jffs2_sum_mem, pub nodetype: jint16_t }

#[repr(C, packed)]
pub struct jffs2_sum_inode_mem {
    pub next: *mut jffs2_sum_mem, pub nodetype: jint16_t, pub inode: jint32_t,
    pub version: jint32_t, pub offset: jint32_t, pub totlen: jint32_t,
}

#[repr(C, packed)]
pub struct jffs2_sum_dirent_mem {
    pub next: *mut jffs2_sum_mem, pub nodetype: jint16_t, pub totlen: jint32_t,
    pub offset: jint32_t, pub pino: jint32_t, pub version: jint32_t,
    pub ino: jint32_t, pub nsize: u8, pub r#type: u8, pub name: [u8; 0],
}

#[repr(C, packed)]
pub struct jffs2_sum_xattr_mem {
    pub next: *mut jffs2_sum_mem, pub nodetype: jint16_t, pub xid: jint32_t,
    pub version: jint32_t, pub offset: jint32_t, pub totlen: jint32_t,
}

#[repr(C, packed)]
pub struct jffs2_sum_xref_mem {
    pub next: *mut jffs2_sum_mem, pub nodetype: jint16_t, pub offset: jint32_t,
}

#[repr(C)]
pub struct jffs2_summary {
    pub sum_size: u32,
    pub sum_num: u32,
    pub sum_padded: u32,
    pub sum_list_head: *mut jffs2_sum_mem,
    pub sum_list_tail: *mut jffs2_sum_mem,
    pub sum_buf: *mut jint32_t,
}

#[repr(C)]
pub struct jffs2_sum_marker { pub offset: jint32_t, pub magic: jint32_t }

/* sizeof(struct jffs2_raw_summary) + sizeof(struct jffs2_sum_marker). */
pub const JFFS2_SUMMARY_FRAME_SIZE: usize = core::mem::size_of::<jffs2_raw_summary>()
    + core::mem::size_of::<jffs2_sum_marker>();

/* CONFIG_JFFS2_SUMMARY conditionally supplies the declarations below. */
#[cfg(feature = "CONFIG_JFFS2_SUMMARY")]
pub const fn jffs2_sum_active() -> i32 { 1 }

#[cfg(feature = "CONFIG_JFFS2_SUMMARY")]
unsafe extern "C" {
    pub fn jffs2_sum_init(c: *mut jffs2_sb_info) -> i32;
    pub fn jffs2_sum_exit(c: *mut jffs2_sb_info);
    pub fn jffs2_sum_disable_collecting(s: *mut jffs2_summary);
    pub fn jffs2_sum_is_disabled(s: *mut jffs2_summary) -> i32;
    pub fn jffs2_sum_reset_collected(s: *mut jffs2_summary);
    pub fn jffs2_sum_move_collected(c: *mut jffs2_sb_info, s: *mut jffs2_summary);
    pub fn jffs2_sum_add_kvec(c: *mut jffs2_sb_info, invecs: *const kvec, count: c_ulong, to: u32) -> i32;
    pub fn jffs2_sum_write_sumnode(c: *mut jffs2_sb_info) -> i32;
    pub fn jffs2_sum_add_padding_mem(s: *mut jffs2_summary, size: u32);
    pub fn jffs2_sum_add_inode_mem(s: *mut jffs2_summary, ri: *mut jffs2_raw_inode, ofs: u32);
    pub fn jffs2_sum_add_dirent_mem(s: *mut jffs2_summary, rd: *mut jffs2_raw_dirent, ofs: u32);
    pub fn jffs2_sum_add_xattr_mem(s: *mut jffs2_summary, rx: *mut jffs2_raw_xattr, ofs: u32);
    pub fn jffs2_sum_add_xref_mem(s: *mut jffs2_summary, rr: *mut jffs2_raw_xref, ofs: u32);
    pub fn jffs2_sum_scan_sumnode(c: *mut jffs2_sb_info, jeb: *mut jffs2_eraseblock,
        summary: *mut jffs2_raw_summary, sumlen: u32, pseudo_random: *mut u32) -> i32;
}

#[cfg(not(feature = "CONFIG_JFFS2_SUMMARY"))]
pub const fn jffs2_sum_active() -> i32 { 0 }

#[cfg(not(feature = "CONFIG_JFFS2_SUMMARY"))]
pub const fn jffs2_sum_init(_a: *mut jffs2_sb_info) -> i32 { 0 }
#[cfg(not(feature = "CONFIG_JFFS2_SUMMARY"))]
pub const unsafe fn jffs2_sum_exit(_a: *mut jffs2_sb_info) {}
#[cfg(not(feature = "CONFIG_JFFS2_SUMMARY"))]
pub const unsafe fn jffs2_sum_disable_collecting(_a: *mut jffs2_summary) {}
#[cfg(not(feature = "CONFIG_JFFS2_SUMMARY"))]
pub const unsafe fn jffs2_sum_is_disabled(_a: *mut jffs2_summary) -> i32 { 0 }
#[cfg(not(feature = "CONFIG_JFFS2_SUMMARY"))]
pub const unsafe fn jffs2_sum_reset_collected(_a: *mut jffs2_summary) {}
#[cfg(not(feature = "CONFIG_JFFS2_SUMMARY"))]
pub const unsafe fn jffs2_sum_add_kvec(_a: *mut jffs2_sb_info, _b: *const kvec, _c: c_ulong, _d: u32) -> i32 { 0 }
#[cfg(not(feature = "CONFIG_JFFS2_SUMMARY"))]
pub const unsafe fn jffs2_sum_write_sumnode(_a: *mut jffs2_sb_info) -> i32 { 0 }
#[cfg(not(feature = "CONFIG_JFFS2_SUMMARY"))]
pub const unsafe fn jffs2_sum_scan_sumnode(_a: *mut jffs2_sb_info, _b: *mut jffs2_eraseblock,
    _c: *mut jffs2_raw_summary, _d: u32, _e: *mut u32) -> i32 { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
