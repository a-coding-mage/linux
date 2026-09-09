/* SPDX-License-Identifier: GPL-2.0-only */
/* Translated from erofs/internal.h. Kernel and erofs_fs dependencies are external. */

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};

pub type erofs_nid_t = u64;
pub type erofs_off_t = u64;
pub type erofs_blk_t = u64;

#[repr(C)] pub struct super_block { pub s_fs_info: *mut c_void, pub s_blocksize_bits: u32, pub s_blocksize: u64 }
#[repr(C)] pub struct inode { pub i_sb: *mut super_block, pub i_size: u64, pub i_blkbits: u32 }
#[repr(C)] pub struct file;
#[repr(C)] pub struct dax_device;
#[repr(C)] pub struct idr;
#[repr(C)] pub struct rw_semaphore;
#[repr(C)] pub struct list_head;
#[repr(C)] pub struct mutex;
#[repr(C)] pub struct xarray;
#[repr(C)] pub struct kobject;
#[repr(C)] pub struct completion;
#[repr(C)] pub struct erofs_xattr_long_prefix;
#[repr(C)] pub struct address_space;
#[repr(C)] pub struct page;
#[repr(C)] pub struct folio;
#[repr(C)] pub struct dentry;
#[repr(C)] pub struct spinlock_t;
#[repr(C)] pub struct block_device;
#[repr(C)] pub struct fiemap_extent_info;
#[repr(C)] pub struct mnt_idmap;
#[repr(C)] pub struct path;
#[repr(C)] pub struct kstat;
#[repr(C)] pub struct qstr;
#[repr(C)] pub struct file_system_type;
#[repr(C)] pub struct super_operations;
#[repr(C)] pub struct address_space_operations;
#[repr(C)] pub struct inode_operations;
#[repr(C)] pub struct file_operations;
#[repr(C)] pub struct iomap_ops;
#[repr(C)] pub struct bio;
#[repr(C)] pub struct erofs_super_block;
pub type pgoff_t = u64;
pub type loff_t = i64;
pub type gfp_t = c_uint;
pub type atomic_long_t = c_long;

#[repr(C)] pub struct erofs_device_info { pub path: *mut c_char, pub file: *mut file, pub dax_dev: *mut dax_device, pub fsoff: u64, pub dax_part_off: u64, pub blocks: erofs_blk_t, pub uniaddr: erofs_blk_t }
pub const EROFS_SYNC_DECOMPRESS_AUTO: c_uint = 0;
pub const EROFS_SYNC_DECOMPRESS_FORCE_ON: c_uint = 1;
pub const EROFS_SYNC_DECOMPRESS_FORCE_OFF: c_uint = 2;
#[repr(C)] pub struct erofs_mount_opts { pub cache_strategy: u8, pub mount_opt: c_uint }
#[repr(C)] pub struct erofs_dev_context { pub tree: idr, pub rwsem: rw_semaphore, pub extra_devices: c_uint, pub flatdev: bool }
#[repr(C)] pub struct erofs_sb_lz4_info { pub max_distance_pages: u16, pub max_pclusterblks: u16 }
#[repr(C)] pub struct erofs_xattr_prefix_item { pub prefix: *mut erofs_xattr_long_prefix, pub infix_len: u8 }

#[repr(C)] pub struct erofs_sb_info {
    pub dif0: erofs_device_info, pub opt: erofs_mount_opts,
    /* CONFIG_EROFS_FS_ZIP fields are present in the corresponding configured build. */
    pub managed_cache: *mut inode, pub packed_inode: *mut inode, pub metabox_inode: *mut inode,
    pub devs: *mut erofs_dev_context, pub total_blocks: u64, pub meta_blkaddr: u32,
    pub device_id_mask: u16, pub islotbits: u8, pub blkszbits: u8, pub sb_size: u32,
    pub fixed_nsec: u32, pub epoch: i64, pub root_nid: erofs_nid_t, pub packed_nid: erofs_nid_t,
    pub metabox_nid: erofs_nid_t, pub inos: u64, pub volume_name: *mut c_char,
    pub feature_compat: u32, pub feature_incompat: u32, pub available_compr_algs: u16,
    pub s_kobj: kobject, pub s_kobj_unregister: completion, pub dir_ra_bytes: erofs_off_t,
    pub domain_id: *mut c_char,
}

pub const EROFS_SUPER_MAGIC: u32 = EROFS_SUPER_MAGIC_V1;
extern "C" { pub static EROFS_SUPER_MAGIC_V1: u32; }
pub const EROFS_MOUNT_XATTR_USER: u32 = 0x10; pub const EROFS_MOUNT_POSIX_ACL: u32 = 0x20;
pub const EROFS_MOUNT_DAX_ALWAYS: u32 = 0x40; pub const EROFS_MOUNT_DAX_NEVER: u32 = 0x80;
pub const EROFS_MOUNT_DIRECT_IO: u32 = 0x100; pub const EROFS_MOUNT_INODE_SHARE: u32 = 0x200;
pub unsafe fn clear_opt(o: *mut erofs_mount_opts, v: u32) { (*o).mount_opt &= !v; }
pub unsafe fn set_opt(o: *mut erofs_mount_opts, v: u32) { (*o).mount_opt |= v; }
pub unsafe fn test_opt(o: *const erofs_mount_opts, v: u32) -> u32 { (*o).mount_opt & v }
pub unsafe fn EROFS_SB(sb: *mut super_block) -> *mut erofs_sb_info { (*sb).s_fs_info as *mut erofs_sb_info }
pub unsafe fn EROFS_I_SB(i: *mut inode) -> *mut erofs_sb_info { EROFS_SB((*i).i_sb) }

extern "C" { pub fn _erofs_printk(sb: *mut super_block, fmt: *const c_char, ...); pub static mut erofs_anon_fs_type: file_system_type; }
pub const EROFS_DIR_RA_BYTES: usize = 16384;
pub const EROFS_I_EA_INITED_BIT: u32 = 0; pub const EROFS_I_Z_INITED_BIT: u32 = 1;
pub const EROFS_MAP_MAPPED: u32 = 1; pub const EROFS_MAP_META: u32 = 2; pub const EROFS_MAP_PARTIAL_MAPPED: u32 = 4; pub const EROFS_MAP_PARTIAL_REF: u32 = 8; pub const EROFS_MAP_FRAGMENT: u32 = 16;

#[repr(C)] pub struct erofs_buf { pub mapping: *mut address_space, pub off: u64, pub page: *mut page, pub base: *mut c_void, pub mc: bool }
#[repr(C)] pub struct erofs_inode_fingerprint { pub opaque: *mut u8, pub size: c_int }
#[repr(C)] pub union erofs_inode_union { pub startblk: erofs_blk_t, pub chunk: [u8; 8] }
#[repr(C)] pub struct erofs_inode { pub nid: erofs_nid_t, pub flags: c_ulong, pub datalayout: u8, pub inode_isize: u8, pub dot_omitted: bool, pub xattr_isize: c_uint, pub xattr_name_filter: c_uint, pub xattr_shared_count: c_uint, pub xattr_shared_xattrs: *mut c_uint, pub data: erofs_inode_union, pub vfs_inode: inode }
#[repr(C)] pub struct erofs_map_blocks { pub buf: erofs_buf, pub m_pa: erofs_off_t, pub m_la: erofs_off_t, pub m_plen: u64, pub m_llen: u64, pub m_deviceid: u16, pub m_algorithmformat: i8, pub m_flags: c_uint }
#[repr(C)] pub struct erofs_map_dev { pub m_sb: *mut super_block, pub m_dif: *mut erofs_device_info, pub m_bdev: *mut block_device, pub m_pa: erofs_off_t, pub m_deviceid: c_uint }

pub const EROFS_GET_BLOCKS_FIEMAP: u32 = 1; pub const EROFS_GET_BLOCKS_READMORE: u32 = 2; pub const EROFS_GET_BLOCKS_FINDTAIL: u32 = 4;
pub const EROFS_MAP_FULL_MASK: u32 = !(EROFS_MAP_PARTIAL_MAPPED | EROFS_MAP_PARTIAL_REF);
pub const EROFS_I_BL_XATTR_BIT: u32 = (usize::BITS - 1); pub const EROFS_I_BL_Z_BIT: u32 = (usize::BITS - 2);

extern "C" {
    pub static erofs_sops: super_operations; pub static erofs_aops: address_space_operations; pub static erofs_fileio_aops: address_space_operations; pub static z_erofs_aops: address_space_operations;
    pub fn erofs_setup_managed_cache(sb: *mut super_block) -> c_int; pub fn erofs_read_metadata(sb: *mut super_block, buf: *mut erofs_buf, off: *mut erofs_off_t, len: *mut c_int) -> *mut c_void;
    pub fn erofs_unmap_metabuf(buf: *mut erofs_buf); pub fn erofs_put_metabuf(buf: *mut erofs_buf); pub fn erofs_map_dev(sb: *mut super_block, dev: *mut erofs_map_dev) -> c_int;
    pub fn erofs_map_blocks(i: *mut inode, map: *mut erofs_map_blocks) -> c_int; pub fn erofs_iget(sb: *mut super_block, nid: erofs_nid_t) -> *mut inode;
    pub fn erofs_register_sysfs(sb: *mut super_block) -> c_int; pub fn erofs_unregister_sysfs(sb: *mut super_block);
    pub fn erofs_release_pages(pool: *mut *mut page);
    pub fn erofs_read_metabuf(buf: *mut erofs_buf, sb: *mut super_block, off: erofs_off_t, in_metabox: bool) -> *mut c_void;
    pub fn erofs_init_metabuf(buf: *mut erofs_buf, sb: *mut super_block, in_metabox: bool) -> c_int;
    pub fn erofs_bread(buf: *mut erofs_buf, off: erofs_off_t, need_kmap: bool) -> *mut c_void;
    pub fn erofs_fiemap(i: *mut inode, info: *mut fiemap_extent_info, start: u64, len: u64) -> c_int;
    pub fn erofs_file_llseek(f: *mut file, off: loff_t, whence: c_int) -> loff_t;
    pub fn erofs_onlinefolio_init(f: *mut folio); pub fn erofs_onlinefolio_split(f: *mut folio); pub fn erofs_onlinefolio_end(f: *mut folio, err: c_int, dirty: bool);
    pub fn erofs_getattr(idmap: *mut mnt_idmap, path: *const path, stat: *mut kstat, request_mask: u32, query_flags: c_uint) -> c_int;
    pub fn erofs_namei(dir: *mut inode, name: *const qstr, nid: *mut erofs_nid_t, d_type: *mut c_uint) -> c_int;
}

pub const EROFS_ZIP_CACHE_DISABLED: u32 = 0;
pub const EROFS_ZIP_CACHE_READAHEAD: u32 = 1;
pub const EROFS_ZIP_CACHE_READAROUND: u32 = 2;
pub const Z_EROFS_COMPRESSION_SHIFTED: u32 = Z_EROFS_COMPRESSION_MAX;
pub const Z_EROFS_COMPRESSION_INTERLACED: u32 = Z_EROFS_COMPRESSION_SHIFTED + 1;
pub const Z_EROFS_COMPRESSION_RUNTIME_MAX: u32 = Z_EROFS_COMPRESSION_INTERLACED + 1;
extern "C" { pub static Z_EROFS_COMPRESSION_MAX: u32; pub fn z_erofs_parse_cfgs(sb: *mut super_block, dsb: *mut erofs_super_block) -> c_int; }

pub unsafe fn erofs_inode_in_metabox(i: *const erofs_inode) -> bool { ((*i).nid & (1u64 << EROFS_DIRENT_NID_METABOX_BIT)) != 0 }
pub unsafe fn erofs_inode_version(ifmt: u32) -> u32 { (ifmt >> EROFS_I_VERSION_BIT) & EROFS_I_VERSION_MASK }
pub unsafe fn erofs_inode_datalayout(ifmt: u32) -> u32 { (ifmt >> EROFS_I_DATALAYOUT_BIT) & EROFS_I_DATALAYOUT_MASK }
pub const EROFS_DIRENT_NID_METABOX_BIT: u32 = 63;
pub const EROFS_I_VERSION_BIT: u32 = 0; pub const EROFS_I_VERSION_MASK: u32 = 0xff;
pub const EROFS_I_DATALAYOUT_BIT: u32 = 0; pub const EROFS_I_DATALAYOUT_MASK: u32 = 0xff;

/* CONFIG_EROFS_FS_ZIP, CONFIG_EROFS_FS_XATTR, CONFIG_EROFS_FS_PAGE_CACHE_SHARE,
 * and CONFIG_EROFS_FS_BACKED_BY_FILE declarations are supplied by configured builds. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
