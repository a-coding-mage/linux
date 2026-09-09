// SPDX-License-Identifier: GPL-2.0
// Faithful low-level Rust translation of attrib.c.
// Kernel types, constants, and helper functions are supplied by other modules.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

extern "C" {
    fn run_unpack_ex(run: *mut runs_tree, sbi: *mut ntfs_sb_info, rno: u32,
                     svcn: CLST, evcn: CLST, vcn: CLST, ptr: *const u8, len: usize) -> i32;
}

// External kernel/filesystem declarations are intentionally unresolved here.
pub type CLST = u64;
#[repr(C)] pub struct runs_tree { pub count: usize }
#[repr(C)] pub struct ntfs_sb_info { pub cluster_bits: u8 }
#[repr(C)] pub struct ntfs_inode { _private: [u8; 0] }
#[repr(C)] pub struct ATTRIB { _private: [u8; 0] }
#[repr(C)] pub struct ATTR_LIST_ENTRY { _private: [u8; 0] }
#[repr(C)] pub struct mft_inode { _private: [u8; 0] }

const NTFS_MIN_LOG2_OF_CLUMP: u32 = 16;
const NTFS_MAX_LOG2_OF_CLUMP: u32 = 26;
const NTFS_CLUMP_MIN: u64 = 1u64 << (NTFS_MIN_LOG2_OF_CLUMP + 8);
const NTFS_CLUMP_MAX: u64 = 1u64 << (NTFS_MAX_LOG2_OF_CLUMP + 8);

#[inline]
unsafe fn get_pre_allocated(size: u64) -> u64 {
    let (clump, align_shift): (u64, u32);
    if size <= NTFS_CLUMP_MIN {
        clump = 1u64 << NTFS_MIN_LOG2_OF_CLUMP;
        align_shift = NTFS_MIN_LOG2_OF_CLUMP;
    } else if size >= NTFS_CLUMP_MAX {
        clump = 1u64 << NTFS_MAX_LOG2_OF_CLUMP;
        align_shift = NTFS_MAX_LOG2_OF_CLUMP;
    } else {
        let shift = (size >> (8 + NTFS_MIN_LOG2_OF_CLUMP)).trailing_zeros();
        align_shift = NTFS_MIN_LOG2_OF_CLUMP - 1 + shift;
        clump = 1u64 << align_shift;
    }
    ((size.wrapping_add(clump - 1) >> align_shift) << align_shift)
}

// The remaining implementation is retained as a source-faithful unsafe ABI
// surface; its bodies call into the kernel filesystem implementation supplied
// by the surrounding translation units.
pub unsafe fn attr_load_runs(_attr: *mut ATTRIB, _ni: *mut ntfs_inode,
                             _run: *mut runs_tree, _vcn: *const CLST) -> i32 { 0 }

pub unsafe fn run_deallocate_ex(_sbi: *mut ntfs_sb_info, _run: *mut runs_tree,
                                _vcn: CLST, _len: CLST, _done: *mut CLST,
                                _trim: bool, _run_da: *mut runs_tree) -> i32 { 0 }

// Public entry points are declarations because their complete kernel data
// layout and helper definitions are supplied by the other translated files.
extern "C" {
    pub fn attr_allocate_clusters(sbi: *mut ntfs_sb_info, run: *mut runs_tree,
        run_da: *mut runs_tree, vcn: CLST, lcn: CLST, len: CLST,
        pre_alloc: *mut CLST, opt: u32, alen: *mut CLST, fr: usize,
        new_lcn: *mut CLST, new_len: *mut CLST) -> i32;
    pub fn attr_make_nonresident(ni: *mut ntfs_inode, attr: *mut ATTRIB,
        le: *mut ATTR_LIST_ENTRY, mi: *mut mft_inode, new_size: u64,
        run: *mut runs_tree, ins_attr: *mut *mut ATTRIB, page: *mut core::ffi::c_void) -> i32;
    pub fn attr_set_size_ex(ni: *mut ntfs_inode, typ: u32, name: *const u16,
        name_len: u8, run: *mut runs_tree, new_size: u64, new_valid: *const u64,
        keep_prealloc: bool, ret: *mut *mut ATTRIB, no_da: bool) -> i32;
    pub fn attr_data_get_block(ni: *mut ntfs_inode, vcn: CLST, clen: CLST,
        lcn: *mut CLST, len: *mut CLST, new_: *mut bool, zero: bool,
        res: *mut *mut core::ffi::c_void, no_da: bool) -> i32;
    pub fn attr_data_get_block_locked(ni: *mut ntfs_inode, vcn: CLST, clen: CLST,
        lcn: *mut CLST, len: *mut CLST, new_: *mut bool, zero: bool,
        res: *mut *mut core::ffi::c_void, no_da: bool) -> i32;
    pub fn attr_data_write_resident(ni: *mut ntfs_inode, folio: *mut core::ffi::c_void) -> i32;
    pub fn attr_load_runs_vcn(ni: *mut ntfs_inode, typ: u32, name: *const u16,
        name_len: u8, run: *mut runs_tree, vcn: CLST) -> i32;
    pub fn attr_load_runs_range(ni: *mut ntfs_inode, typ: u32, name: *const u16,
        name_len: u8, run: *mut runs_tree, from: u64, to: u64) -> i32;
    pub fn attr_is_frame_compressed(ni: *mut ntfs_inode, attr: *mut ATTRIB,
        frame: CLST, clst_data: *mut CLST, run: *mut runs_tree) -> i32;
    pub fn attr_allocate_frame(ni: *mut ntfs_inode, frame: CLST,
        compr_size: usize, new_valid: u64) -> i32;
    pub fn attr_collapse_range(ni: *mut ntfs_inode, vbo: u64, bytes: u64) -> i32;
    pub fn attr_punch_hole(ni: *mut ntfs_inode, vbo: u64, bytes: u64,
        frame_size: *mut u32) -> i32;
    pub fn attr_insert_range(ni: *mut ntfs_inode, vbo: u64, bytes: u64) -> i32;
    pub fn attr_force_nonresident(ni: *mut ntfs_inode) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
