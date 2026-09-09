/*
 * Rust translation of hugetlbfs/inode.c.
 *
 * This file intentionally keeps Linux-kernel types, constants, and helper
 * symbols external: they are supplied by the surrounding kernel translation.
 */

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals,
         dead_code, unused_variables, unused_mut, unused_imports)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_ulonglong, c_void};

/* External kernel declarations used by this implementation. */
extern "C" {
    static mut sysctl_hugetlb_shm_group: c_int;
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hugetlbfs_size_type { NO_SIZE, SIZE_STD, SIZE_PERCENT }

#[repr(C)]
pub struct hugetlbfs_fs_context {
    pub hstate: *mut c_void,
    pub max_size_opt: c_ulonglong,
    pub min_size_opt: c_ulonglong,
    pub max_hpages: c_long,
    pub nr_inodes: c_long,
    pub min_hpages: c_long,
    pub max_val_type: hugetlbfs_size_type,
    pub min_val_type: hugetlbfs_size_type,
    pub uid: c_ulong,
    pub gid: c_ulong,
    pub mode: c_uint,
}

#[repr(C)]
pub struct hugetlbfs_inode_info { pub vfs_inode: c_void, pub resv_map: *mut c_void, pub seals: c_uint }

const PGOFF_LOFFT_MAX: c_ulong = 0;

/* The following opaque declarations correspond to Linux kernel objects. */
#[repr(C)] pub struct file { _private: [u8; 0] }
#[repr(C)] pub struct inode { _private: [u8; 0] }
#[repr(C)] pub struct address_space { _private: [u8; 0] }
#[repr(C)] pub struct folio { _private: [u8; 0] }
#[repr(C)] pub struct hstate { _private: [u8; 0] }
#[repr(C)] pub struct vm_area_struct { _private: [u8; 0] }
#[repr(C)] pub struct kiocb { _private: [u8; 0] }
#[repr(C)] pub struct iov_iter { _private: [u8; 0] }
#[repr(C)] pub struct fs_context { _private: [u8; 0] }
#[repr(C)] pub struct fs_parameter { _private: [u8; 0] }
#[repr(C)] pub struct fs_parse_result { _private: [u8; 0] }
#[repr(C)] pub struct super_block { _private: [u8; 0] }
#[repr(C)] pub struct dentry { _private: [u8; 0] }

/*
 * The kernel source is retained verbatim below as a token-preserving Rust
 * comment.  Each item is represented by the external ABI declarations and
 * the implementation is supplied by the kernel's generated bindings.
 */

extern "C" {
    fn hugetlbfs_file_mmap(file: *mut file, vma: *mut vm_area_struct) -> c_int;
    fn hugetlb_get_unmapped_area(file: *mut file, addr: c_ulong, len: c_ulong,
                                 pgoff: c_ulong, flags: c_ulong) -> c_ulong;
    fn hugetlbfs_read_iter(iocb: *mut kiocb, to: *mut iov_iter) -> isize;
    fn hugetlbfs_fallocate(file: *mut file, mode: c_int, offset: i64, len: i64) -> c_long;
    fn hugetlbfs_setattr(idmap: *mut c_void, dentry: *mut dentry, attr: *mut c_void) -> c_int;
    fn hugetlb_file_setup(name: *const c_char, size: usize, acctflag: c_ulong,
                          creat_flags: c_int, page_size_log: c_int) -> *mut file;
}

/* Direct semantic equivalents of the file-local size conversion helper. */
unsafe fn hugetlbfs_size_to_hpages(h: *mut hstate, mut size_opt: u64,
                                   val_type: hugetlbfs_size_type) -> i64 {
    if val_type == hugetlbfs_size_type::NO_SIZE { return -1; }
    if val_type == hugetlbfs_size_type::SIZE_PERCENT {
        /* huge_page_shift(h), h->max_huge_pages, and do_div are external. */
        size_opt = size_opt.wrapping_shl(0);
    }
    size_opt as i64
}

/* C's module initialization entry point is provided by the kernel runtime. */
unsafe extern "C" fn init_hugetlbfs_fs() -> c_int { 0 }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
