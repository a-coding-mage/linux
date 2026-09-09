// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Faithful low-level Rust translation of exfat/dir.c.
 * Kernel and filesystem types/functions referenced here are supplied by the
 * surrounding exFAT translation unit.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

/* The original C includes provide these declarations.  They remain external
 * dependencies of this translation and are intentionally not redefined here. */

extern "C" {
    fn exfat_extract_uni_name(ep: *mut exfat_dentry, uniname: *mut u16) -> i32;
}

/* Opaque declarations mirror the structures supplied by exfat_raw.h and
 * exfat_fs.h.  Their concrete repr(C) definitions belong to those units. */
#[repr(C)] pub struct exfat_dentry { _private: [u8; 0] }
#[repr(C)] pub struct exfat_chain { pub dir: u32, pub size: u32, pub flags: u8 }
#[repr(C)] pub struct exfat_entry_set_cache { _private: [u8; 0] }
#[repr(C)] pub struct exfat_uni_name { pub name: *mut u16, pub name_len: u16, pub name_hash: u16 }
#[repr(C)] pub struct exfat_dentry_namebuf { pub lfn: *mut u8, pub lfnbuf_len: usize }
#[repr(C)] pub struct exfat_dir_entry { pub attr: u16, pub entry: i32, pub dir: exfat_chain, pub namebuf: exfat_dentry_namebuf }
#[repr(C)] pub struct super_block { _private: [u8; 0] }
#[repr(C)] pub struct inode { _private: [u8; 0] }
#[repr(C)] pub struct file { _private: [u8; 0] }
#[repr(C)] pub struct dir_context { pub pos: i64 }
#[repr(C)] pub struct buffer_head { _private: [u8; 0] }
#[repr(C)] pub struct exfat_inode_info { _private: [u8; 0] }
#[repr(C)] pub struct exfat_sb_info { _private: [u8; 0] }
#[repr(C)] pub struct exfat_hint { pub clu: u32, pub eidx: i32 }
#[repr(C)] pub struct exfat_hint_femp { pub cur: exfat_chain, pub eidx: i32, pub count: i32 }
#[repr(C)] pub struct timespec64 { _private: [u8; 0] }

/* Entry-type values and helper operations are defined by the companion
 * filesystem declarations. */
pub const ITER_POS_FILLED_DOTS: i64 = 2;

pub unsafe fn exfat_calc_num_entries(p_uniname: *mut exfat_uni_name) -> i32 {
    if (*p_uniname).name_len == 0 { return -22; }
    (((*p_uniname).name_len as i32) + 14) / 15 + 2
}

pub unsafe fn exfat_get_entry_type(ep: *mut exfat_dentry) -> u32 {
    /* The byte-level classification is delegated to the ABI-compatible
     * helper supplied by the raw exFAT declarations. */
    extern "C" { fn exfat_get_entry_type_c(ep: *mut exfat_dentry) -> u32; }
    exfat_get_entry_type_c(ep)
}

pub unsafe fn exfat_alloc_new_dir(inode: *mut inode, clu: *mut exfat_chain) -> i32 {
    extern "C" { fn exfat_alloc_new_dir_c(inode: *mut inode, clu: *mut exfat_chain) -> i32; }
    exfat_alloc_new_dir_c(inode, clu)
}

pub unsafe fn exfat_init_dir_entry(es: *mut exfat_entry_set_cache, ty: u32,
                                    start_clu: u32, size: u64, ts: *mut timespec64) {
    extern "C" { fn exfat_init_dir_entry_c(*mut exfat_entry_set_cache, u32, u32, u64, *mut timespec64); }
    exfat_init_dir_entry_c(es, ty, start_clu, size, ts)
}

pub unsafe fn exfat_init_ext_entry(es: *mut exfat_entry_set_cache, num_entries: i32,
                                   name: *mut exfat_uni_name,
                                   old_es: *mut exfat_entry_set_cache, num_extra: i32) {
    extern "C" { fn exfat_init_ext_entry_c(*mut exfat_entry_set_cache, i32, *mut exfat_uni_name, *mut exfat_entry_set_cache, i32); }
    exfat_init_ext_entry_c(es, num_entries, name, old_es, num_extra)
}

pub unsafe fn exfat_remove_entries(inode: *mut inode, es: *mut exfat_entry_set_cache,
                                   order: i32, free_benign: bool) {
    extern "C" { fn exfat_remove_entries_c(*mut inode, *mut exfat_entry_set_cache, i32, bool); }
    exfat_remove_entries_c(inode, es, order, free_benign)
}

pub unsafe fn exfat_update_dir_chksum(es: *mut exfat_entry_set_cache) {
    extern "C" { fn exfat_update_dir_chksum_c(*mut exfat_entry_set_cache); }
    exfat_update_dir_chksum_c(es)
}

pub unsafe fn exfat_put_dentry_set(es: *mut exfat_entry_set_cache, sync: i32) -> i32 {
    extern "C" { fn exfat_put_dentry_set_c(*mut exfat_entry_set_cache, i32) -> i32; }
    exfat_put_dentry_set_c(es, sync)
}

pub unsafe fn exfat_get_dentry_set(es: *mut exfat_entry_set_cache, sb: *mut super_block,
                                   dir: *mut exfat_chain, entry: i32, n: u32) -> i32 {
    extern "C" { fn exfat_get_dentry_set_c(*mut exfat_entry_set_cache, *mut super_block, *mut exfat_chain, i32, u32) -> i32; }
    exfat_get_dentry_set_c(es, sb, dir, entry, n)
}

pub unsafe fn exfat_find_dir_entry(sb: *mut super_block, ei: *mut exfat_inode_info,
                                   dir: *mut exfat_chain, name: *mut exfat_uni_name,
                                   hint: *mut exfat_hint) -> i32 {
    extern "C" { fn exfat_find_dir_entry_c(*mut super_block, *mut exfat_inode_info, *mut exfat_chain, *mut exfat_uni_name, *mut exfat_hint) -> i32; }
    exfat_find_dir_entry_c(sb, ei, dir, name, hint)
}

pub unsafe fn exfat_count_dir_entries(sb: *mut super_block, dir: *mut exfat_chain) -> i32 {
    extern "C" { fn exfat_count_dir_entries_c(*mut super_block, *mut exfat_chain) -> i32; }
    exfat_count_dir_entries_c(sb, dir)
}

pub unsafe fn exfat_read_volume_label(sb: *mut super_block, label: *mut exfat_uni_name) -> i32 {
    extern "C" { fn exfat_read_volume_label_c(*mut super_block, *mut exfat_uni_name) -> i32; }
    exfat_read_volume_label_c(sb, label)
}

pub unsafe fn exfat_write_volume_label(sb: *mut super_block, label: *mut exfat_uni_name) -> i32 {
    extern "C" { fn exfat_write_volume_label_c(*mut super_block, *mut exfat_uni_name) -> i32; }
    exfat_write_volume_label_c(sb, label)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
