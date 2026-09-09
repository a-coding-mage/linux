/* SPDX-License-Identifier: GPL-2.0 */

/* Dependencies are supplied by the surrounding kernel translation. */

#[cfg(CONFIG_TMPFS_QUOTA)]
pub const SHMEM_MAXQUOTAS: usize = 2;

/* Suppress pre-accounting of the entire object size. */
pub const SHMEM_F_NORESERVE: ::core::ffi::c_ulong = 1 << 0;
/* Disallow swapping. */
pub const SHMEM_F_LOCKED: ::core::ffi::c_ulong = 1 << 1;
/* Disallow growing, shrinking, or hole punching in the inode. */
pub const SHMEM_F_MAPPING_FROZEN: ::core::ffi::c_ulong = 1 << 2;

#[repr(C)]
pub union shmem_inode_info__bindgen_ty_1 {
    pub dir_offsets: offset_ctx,
    pub __bindgen_anon_1: shmem_inode_info__bindgen_ty_1__bindgen_ty_1,
}

#[repr(C)]
pub struct shmem_inode_info__bindgen_ty_1__bindgen_ty_1 {
    pub shrinklist: list_head,
    pub swaplist: list_head,
}

#[repr(C)]
pub struct shmem_inode_info {
    pub lock: spinlock_t,
    pub seals: ::core::ffi::c_uint,
    pub flags: ::core::ffi::c_ulong,
    pub alloced: ::core::ffi::c_ulong,
    pub swapped: ::core::ffi::c_ulong,
    pub __bindgen_anon_1: shmem_inode_info__bindgen_ty_1,
    pub i_crtime: timespec64,
    pub policy: shared_policy,
    pub xattrs: list_head,
    pub fallocend: pgoff_t,
    pub fsflags: ::core::ffi::c_uint,
    pub stop_eviction: atomic_t,
    #[cfg(CONFIG_TMPFS_QUOTA)]
    pub i_dquot: [*mut dquot; MAXQUOTAS],
    pub vfs_inode: inode,
}

pub const SHMEM_FL_USER_VISIBLE: ::core::ffi::c_ulong = FS_FL_USER_VISIBLE | FS_CASEFOLD_FL;
pub const SHMEM_FL_USER_MODIFIABLE: ::core::ffi::c_ulong =
    FS_IMMUTABLE_FL | FS_APPEND_FL | FS_NODUMP_FL | FS_NOATIME_FL | FS_CASEFOLD_FL;
pub const SHMEM_FL_INHERITED: ::core::ffi::c_ulong = FS_NODUMP_FL | FS_NOATIME_FL | FS_CASEFOLD_FL;

#[repr(C)]
pub struct shmem_quota_limits {
    pub usrquota_bhardlimit: qsize_t,
    pub usrquota_ihardlimit: qsize_t,
    pub grpquota_bhardlimit: qsize_t,
    pub grpquota_ihardlimit: qsize_t,
}

#[repr(C)]
pub struct shmem_sb_info {
    pub max_blocks: ::core::ffi::c_ulong,
    pub used_blocks: percpu_counter,
    pub max_inodes: ::core::ffi::c_ulong,
    pub free_ispace: ::core::ffi::c_ulong,
    pub stat_lock: raw_spinlock_t,
    pub mode: umode_t,
    pub huge: ::core::ffi::c_uchar,
    pub uid: kuid_t,
    pub gid: kgid_t,
    pub full_inums: bool,
    pub noswap: bool,
    pub next_ino: ino_t,
    pub ino_batch: *mut ino_t,
    pub mpol: *mut mempolicy,
    pub shrinklist_lock: spinlock_t,
    pub shrinklist: list_head,
    pub shrinklist_len: ::core::ffi::c_ulong,
    pub qlimits: shmem_quota_limits,
    pub xa_cache: simple_xattr_cache,
}

#[inline]
pub unsafe fn SHMEM_I(inode: *mut inode) -> *mut shmem_inode_info {
    container_of!(inode, shmem_inode_info, vfs_inode)
}

extern "C" {
    pub static shmem_fs_parameters: [fs_parameter_spec; 0];
    pub fn shmem_init();
    pub fn shmem_init_fs_context(fc: *mut fs_context) -> ::core::ffi::c_int;
    pub fn shmem_file_setup(name: *const ::core::ffi::c_char, size: loff_t, flags: vma_flags_t) -> *mut file;
    pub fn shmem_kernel_file_setup(name: *const ::core::ffi::c_char, size: loff_t, vma_flags: vma_flags_t) -> *mut file;
    pub fn shmem_file_setup_with_mnt(mnt: *mut vfsmount, name: *const ::core::ffi::c_char, size: loff_t, flags: vma_flags_t) -> *mut file;
    pub fn shmem_zero_setup(vma: *mut vm_area_struct) -> ::core::ffi::c_int;
    pub fn shmem_zero_setup_desc(desc: *mut vm_area_desc) -> ::core::ffi::c_int;
    pub fn shmem_get_unmapped_area(file: *mut file, addr: ::core::ffi::c_ulong, len: ::core::ffi::c_ulong, pgoff: ::core::ffi::c_ulong, flags: ::core::ffi::c_ulong) -> ::core::ffi::c_ulong;
    pub fn shmem_lock(file: *mut file, lock: ::core::ffi::c_int, ucounts: *mut ucounts) -> ::core::ffi::c_int;
}

#[cfg(CONFIG_SHMEM)]
extern "C" { pub fn shmem_mapping(mapping: *const address_space) -> bool; }
#[cfg(not(CONFIG_SHMEM))]
#[inline] pub fn shmem_mapping(_: *const address_space) -> bool { false }

extern "C" {
    pub fn shmem_unlock_mapping(mapping: *mut address_space);
    pub fn shmem_read_mapping_page_gfp(mapping: *mut address_space, index: pgoff_t, gfp_mask: gfp_t) -> *mut page;
    pub fn shmem_write_folio(folio: *mut folio) -> ::core::ffi::c_int;
    pub fn shmem_truncate_range(inode: *mut inode, start: loff_t, end: uoff_t);
    pub fn shmem_unuse(type_: ::core::ffi::c_uint) -> ::core::ffi::c_int;
}

#[cfg(all(CONFIG_TRANSPARENT_HUGEPAGE, CONFIG_SHMEM))]
extern "C" {
    pub fn shmem_allowable_huge_orders(inode: *mut inode, vma: *mut vm_area_struct, index: pgoff_t, write_end: loff_t, shmem_huge_force: bool) -> ::core::ffi::c_ulong;
    pub fn shmem_hpage_pmd_enabled() -> bool;
}
#[cfg(not(all(CONFIG_TRANSPARENT_HUGEPAGE, CONFIG_SHMEM)))]
#[inline] pub fn shmem_allowable_huge_orders(_: *mut inode, _: *mut vm_area_struct, _: pgoff_t, _: loff_t, _: bool) -> ::core::ffi::c_ulong { 0 }
#[cfg(not(all(CONFIG_TRANSPARENT_HUGEPAGE, CONFIG_SHMEM)))]
#[inline] pub fn shmem_hpage_pmd_enabled() -> bool { false }

#[cfg(CONFIG_SHMEM)]
extern "C" { pub fn shmem_swap_usage(vma: *mut vm_area_struct) -> ::core::ffi::c_ulong; pub fn shmem_uncharge(inode: *mut inode, pages: ::core::ffi::c_long); }
#[cfg(not(CONFIG_SHMEM))]
#[inline] pub fn shmem_swap_usage(_: *mut vm_area_struct) -> ::core::ffi::c_ulong { 0 }
#[cfg(not(CONFIG_SHMEM))]
#[inline] pub fn shmem_uncharge(_: *mut inode, _: ::core::ffi::c_long) {}

extern "C" {
    pub fn shmem_partial_swap_usage(mapping: *mut address_space, start: pgoff_t, end: pgoff_t) -> ::core::ffi::c_ulong;
    pub fn shmem_get_folio(inode: *mut inode, index: pgoff_t, write_end: loff_t, foliop: *mut *mut folio, sgp: sgp_type) -> ::core::ffi::c_int;
    pub fn shmem_read_folio_gfp(mapping: *mut address_space, index: pgoff_t, gfp: gfp_t) -> *mut folio;
    pub fn shmem_charge(inode: *mut inode, pages: ::core::ffi::c_long) -> bool;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum sgp_type { SGP_READ, SGP_NOALLOC, SGP_CACHE, SGP_WRITE, SGP_FALLOC }

pub const SHMEM_QUOTA_MAX_SPC_LIMIT: i64 = 0x7fffffffffffffff;
pub const SHMEM_QUOTA_MAX_INO_LIMIT: i64 = 0x7fffffffffffffff;

#[cfg(CONFIG_TMPFS_QUOTA)]
extern "C" {
    pub static shmem_quota_operations: dquot_operations;
    pub static shmem_quota_format: quota_format_type;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
