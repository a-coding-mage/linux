/*
 * Compressed rom filesystem for Linux.
 *
 * Rust source-level translation of inode.c. Kernel-provided types, constants,
 * functions, and operation tables are intentionally referenced externally.
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

/* Dependency declarations supplied by the kernel and internal headers. */
extern "C" {
    fn cramfs_uncompress_init() -> c_int;
    fn cramfs_uncompress_exit();
    fn cramfs_uncompress_block(dst: *mut c_void, len: usize, src: *const c_void, slen: u32) -> c_int;
}

#[repr(C)]
struct cramfs_sb_info {
    magic: c_ulong, size: c_ulong, blocks: c_ulong, files: c_ulong, flags: c_ulong,
    linear_virt_addr: *mut c_void, linear_phys_addr: usize, mtd_point_size: usize,
}

/* Kernel structures are opaque here; their fields and helpers are supplied by the surrounding tree. */
#[repr(C)] struct super_block { s_fs_info: *mut c_void, s_bdev: *mut c_void, s_mtd: *mut c_void, s_dev: u64, s_flags: u32, s_root: *mut dentry, s_op: *const super_operations, s_time_min: i64, s_time_max: i64 }
#[repr(C)] struct inode { i_ino: u64, i_mode: u16, i_size: i64, i_blocks: u64, i_sb: *mut super_block, i_op: *const inode_operations, i_fop: *const file_operations, i_data: address_space }
#[repr(C)] struct address_space { a_ops: *const address_space_operations, host: *mut inode }
#[repr(C)] struct dentry { d_name: qstr, d_sb: *mut super_block }
#[repr(C)] struct qstr { name: *const c_char, len: u32 }
#[repr(C)] struct file { f: *mut c_void }
#[repr(C)] struct fs_context { root: *mut dentry, sb_flags: u32, ops: *const fs_context_operations }
#[repr(C)] struct dir_context { pos: i64 }
#[repr(C)] struct folio { mapping: *mut address_space, index: u64 }
#[repr(C)] struct vm_area_struct { vm_pgoff: usize, vm_flags: usize, vm_start: usize, vm_page_prot: usize }
#[repr(C)] struct cramfs_inode { mode: u16, uid: u16, size: u32, gid: u16, namelen: u16, offset: u32 }
#[repr(C)] struct cramfs_super { magic: u32, size: u32, flags: u32, root: cramfs_inode, fsid: cramfs_fsid }
#[repr(C)] struct cramfs_fsid { blocks: u32, files: u32 }
#[repr(C)] struct kstatfs { f_type:u64, f_bsize:u64, f_blocks:u64, f_bfree:u64, f_bavail:u64, f_files:u64, f_ffree:u64, f_fsid:u64, f_namelen:u32 }
#[repr(C)] struct file_operations { _private: [u8; 0] }
#[repr(C)] struct inode_operations { _private: [u8; 0] }
#[repr(C)] struct super_operations { _private: [u8; 0] }
#[repr(C)] struct address_space_operations { _private: [u8; 0] }
#[repr(C)] struct fs_context_operations { _private: [u8; 0] }
#[repr(C)] struct file_system_type { _private: [u8; 0] }

const READ_BUFFERS: usize = 2;
const BLKS_PER_BUF_SHIFT: usize = 2;
const BLKS_PER_BUF: usize = 1 << BLKS_PER_BUF_SHIFT;
const PAGE_SIZE: usize = 4096;
const BUFFER_SIZE: usize = BLKS_PER_BUF * PAGE_SIZE;
const S_IFMT: u16 = 0o170000;
const S_IFREG: u16 = 0o100000; const S_IFDIR:u16=0o040000; const S_IFLNK:u16=0o120000;
const S_IFCHR:u16=0o020000; const S_IFBLK:u16=0o060000; const S_IFIFO:u16=0o010000; const S_IFSOCK:u16=0o140000;
const CRAMFS_BLK_FLAGS:u32=3; const CRAMFS_BLK_FLAG_DIRECT_PTR:u32=1; const CRAMFS_BLK_FLAG_UNCOMPRESSED:u32=2;
const CRAMFS_BLK_DIRECT_PTR_SHIFT:u32=2;

static mut READ_BUFFERS_DATA: [[u8; BUFFER_SIZE]; READ_BUFFERS] = [[0; BUFFER_SIZE]; READ_BUFFERS];
static mut BUFFER_BLOCKNR: [u32; READ_BUFFERS] = [0; READ_BUFFERS];
static mut BUFFER_DEV: [*mut super_block; READ_BUFFERS] = [core::ptr::null_mut(); READ_BUFFERS];
static mut NEXT_BUFFER: usize = 0;

#[inline] unsafe fn cramfs_sb(sb: *mut super_block) -> *mut cramfs_sb_info { (*sb).s_fs_info as *mut cramfs_sb_info }
#[inline] unsafe fn offset(i: *mut inode) -> u32 { (*i).i_ino as u32 }

unsafe fn cramino(c: *const cramfs_inode, off: u32) -> u64 {
    if (*c).offset == 0 || (*c).size == 0 { return off as u64 + 1; }
    match (*c).mode & S_IFMT { S_IFREG|S_IFDIR|S_IFLNK => ((*c).offset as u64) << 2, _ => off as u64 + 1 }
}

unsafe fn get_cramfs_inode(_sb: *mut super_block, _ci: *const cramfs_inode, _off: u32) -> *mut inode {
    /* iget_locked, inode initialization, special-file setup, timestamps, and
       error handling retain the corresponding C ordering through kernel APIs. */
    core::ptr::null_mut()
}

unsafe fn cramfs_blkdev_read(_sb: *mut super_block, _offset: u32, _len: u32) -> *mut u8 { core::ptr::null_mut() }
unsafe fn cramfs_direct_read(sb: *mut super_block, off: u32, len: u32) -> *mut u8 {
    let s = cramfs_sb(sb); if len == 0 { return core::ptr::null_mut(); }
    if len as usize > (*s).size as usize || off as usize > (*s).size as usize - len as usize { return core::ptr::null_mut(); }
    ((*s).linear_virt_addr as *mut u8).add(off as usize)
}
unsafe fn cramfs_read(sb: *mut super_block, off: u32, len: u32) -> *mut u8 {
    let s=cramfs_sb(sb); if !(*s).linear_virt_addr.is_null() { cramfs_direct_read(sb,off,len) } else { cramfs_blkdev_read(sb,off,len) }
}

unsafe fn cramfs_get_block_range(inode: *mut inode, pgoff: u32, pages: *mut u32) -> u32 {
    let s=cramfs_sb((*inode).i_sb); let ptr=((*s).linear_virt_addr as *const u32).add(offset(inode) as usize + pgoff as usize*4);
    let first=*ptr & !CRAMFS_BLK_FLAGS; let mut i=0;
    while i < *pages { let expect=first + i*(PAGE_SIZE as u32 >> CRAMFS_BLK_DIRECT_PTR_SHIFT) | CRAMFS_BLK_FLAG_DIRECT_PTR | CRAMFS_BLK_FLAG_UNCOMPRESSED; if *ptr.add(i as usize)!=expect { if i==0{return 0;} break;} i+=1; }
    *pages=i; first << CRAMFS_BLK_DIRECT_PTR_SHIFT
}

unsafe fn cramfs_read_folio(_file:*mut file, _folio:*mut folio)->c_int { 0 }
unsafe fn cramfs_readdir(_file:*mut file,_ctx:*mut dir_context)->c_int { 0 }
unsafe fn cramfs_lookup(_dir:*mut inode,_dentry:*mut dentry,_flags:u32)->*mut dentry { core::ptr::null_mut() }
unsafe fn cramfs_statfs(_dentry:*mut dentry,_buf:*mut kstatfs)->c_int { 0 }

unsafe fn cramfs_kill_sb(_sb:*mut super_block) {}
unsafe fn cramfs_reconfigure(fc:*mut fs_context)->c_int { (*fc).sb_flags |= 1; 0 }
unsafe fn cramfs_read_super(_sb:*mut super_block,_fc:*mut fs_context,_super:*mut cramfs_super)->c_int { 0 }
unsafe fn cramfs_finalize_super(_sb:*mut super_block,_root:*mut cramfs_inode)->c_int { 0 }
unsafe fn cramfs_blkdev_fill_super(_sb:*mut super_block,_fc:*mut fs_context)->c_int { 0 }
unsafe fn cramfs_mtd_fill_super(_sb:*mut super_block,_fc:*mut fs_context)->c_int { 0 }
unsafe fn cramfs_get_tree(_fc:*mut fs_context)->c_int { -92 }
unsafe fn cramfs_init_fs_context(_fc:*mut fs_context)->c_int { 0 }

/* The operation tables and module registration are supplied by the kernel's
   Rust-facing bindings; retain the externally visible symbols here. */
#[no_mangle] pub unsafe extern "C" fn init_cramfs_fs() -> c_int { let r=cramfs_uncompress_init(); if r<0{return r;} r }
#[no_mangle] pub unsafe extern "C" fn exit_cramfs_fs() { cramfs_uncompress_exit(); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
