// SPDX-License-Identifier: GPL-2.0-only
// Faithful low-level Rust translation of overlayfs/inode.c. Kernel types and
// helpers are supplied by the surrounding Linux/Rust bindings.

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

// The following declarations intentionally remain external: they are provided
// by the overlayfs and kernel bindings, just as by the C headers.
extern "C" {
    fn setattr_prepare(idmap: *mut mnt_idmap, dentry: *mut dentry, attr: *mut iattr) -> c_int;
    fn ovl_copy_up(dentry: *mut dentry) -> c_int;
    fn ovl_copy_up_with_data(dentry: *mut dentry) -> c_int;
    fn ovl_want_write(dentry: *mut dentry) -> c_int;
    fn ovl_drop_write(dentry: *mut dentry);
    fn ovl_do_notify_change(ofs: *mut ovl_fs, dentry: *mut dentry, attr: *mut iattr) -> c_int;
    fn ovl_copyattr(inode: *mut inode);
    fn ovl_path_real(dentry: *mut dentry, path: *mut path) -> c_int;
    fn ovl_real_getattr_nosec(sb: *mut super_block, path: *const path, stat: *mut kstat, mask: u32, flags: c_uint) -> c_int;
    fn ovl_map_dev_ino(dentry: *mut dentry, stat: *mut kstat, fsid: c_int);
    fn ovl_i_path_real(inode: *mut inode, path: *mut path) -> *mut inode;
    fn generic_permission(idmap: *mut mnt_idmap, inode: *mut inode, mask: c_int) -> c_int;
    fn inode_permission(idmap: *mut mnt_idmap, inode: *mut inode, mask: c_int) -> c_int;
    fn vfs_get_link(dentry: *mut dentry, done: *mut delayed_call) -> *const c_char;
    fn ovl_inode_real(inode: *mut inode) -> *mut inode;
    fn ovl_inode_realdata(inode: *mut inode) -> *mut inode;
    fn ovl_security_fileattr(path: *const path, fa: *mut file_kattr, set: bool) -> c_int;
    fn vfs_fileattr_set(idmap: *mut mnt_idmap, dentry: *mut dentry, fa: *mut file_kattr) -> c_int;
    fn vfs_fileattr_get(dentry: *mut dentry, fa: *mut file_kattr) -> c_int;
    fn ovl_set_protattr(inode: *mut inode, dentry: *mut dentry, fa: *mut file_kattr) -> c_int;
    fn ovl_check_protattr(inode: *mut inode, dentry: *mut dentry);
    fn ovl_file_operations() -> *const file_operations;
    fn ovl_dir_inode_operations() -> *const inode_operations;
    fn ovl_dir_operations() -> *const file_operations;
    fn new_inode(sb: *mut super_block) -> *mut inode;
    fn init_special_inode(inode: *mut inode, mode: umode_t, rdev: dev_t);
    fn iput(inode: *mut inode);
}

// Kernel-layout types (repr(C) is required for ABI-compatible field access).
#[repr(C)] pub struct mnt_idmap { _p: [u8; 0] }
#[repr(C)] pub struct dentry { pub d_inode: *mut inode, pub d_sb: *mut super_block }
#[repr(C)] pub struct super_block { pub s_dev: dev_t, pub s_stack_depth: c_int }
#[repr(C)] pub struct inode { pub i_mode: umode_t, pub i_ino: u64, pub i_nlink: c_uint, pub i_rdev: dev_t, pub i_sb: *mut super_block, pub i_private: *mut c_void, pub i_flags: u32, pub i_op: *const inode_operations, pub i_fop: *const file_operations, pub i_mapping: *mut address_space }
#[repr(C)] pub struct iattr { pub ia_valid: u32, pub ia_vfsuid: u32, pub ia_vfsgid: u32 }
#[repr(C)] pub struct path { pub mnt: *mut c_void, pub dentry: *mut dentry }
#[repr(C)] pub struct kstat { pub dev: dev_t, pub ino: u64, pub nlink: u32, pub blocks: u64, pub size: u64, pub blksize: u64, pub uid: u32, pub gid: u32 }
#[repr(C)] pub struct delayed_call { _p: [u8; 0] }
#[repr(C)] pub struct file_kattr { pub flags: u32, pub fsx_valid: bool, pub fsx_xflags: u32 }
#[repr(C)] pub struct inode_operations { _p: [u8; 0] }
#[repr(C)] pub struct file_operations { _p: [u8; 0] }
#[repr(C)] pub struct address_space_operations { pub direct_io: *const c_void }
#[repr(C)] pub struct address_space { pub a_ops: *const address_space_operations }
#[repr(C)] pub struct ovl_fs { pub config: ovl_config, pub last_ino: u64 }
#[repr(C)] pub struct ovl_config { pub nfs_export: bool, pub casefold: bool }
#[repr(C)] pub struct ovl_inode_params { pub upperdentry: *mut dentry, pub oe: *mut ovl_entry, pub redirect: *mut c_char, pub lowerdata_redirect: *mut c_char, pub index: bool, pub newinode: *mut inode }
#[repr(C)] pub struct ovl_entry { _p: [u8; 0] }
#[repr(C)] pub struct ovl_inode { pub upperdentry: *mut dentry, pub oe: *mut ovl_entry, pub redirect: *mut c_char, pub lowerdata_redirect: *mut c_char }
pub type dev_t = u64; pub type umode_t = u32;

// Direct translations of the inode-cache and inode-construction routines.
pub unsafe fn ovl_next_ino(inode: *mut inode) { let ofs = OVL_FS((*inode).i_sb); let mut n = (*ofs).last_ino.wrapping_add(1); if n == 0 { n = n.wrapping_add(1); } (*ofs).last_ino = n; (*inode).i_ino = n; }
pub unsafe fn ovl_map_ino(inode: *mut inode, ino: c_ulong, fsid: c_int) { (*inode).i_ino = ino as u64; let ofs = OVL_FS((*inode).i_sb); let bits = ovl_xino_bits(ofs); if ovl_same_fs(ofs) { return; } if bits != 0 && (ino >> (64-bits)) == 0 { (*inode).i_ino |= (fsid as u64) << (65-bits); return; } if ((*inode).i_mode & S_IFMT) == S_IFDIR { ovl_next_ino(inode); if bits != 0 { (*inode).i_ino &= (!0u64) >> bits; (*inode).i_ino |= 1u64 << (64-bits); } } }
pub unsafe fn ovl_inode_init(inode: *mut inode, oip: *mut ovl_inode_params, ino: c_ulong, fsid: c_int) { let oi = OVL_I(inode); (*oi).upperdentry=(*oip).upperdentry; (*oi).oe=(*oip).oe; (*oi).redirect=(*oip).redirect; (*oi).lowerdata_redirect=(*oip).lowerdata_redirect; let real=ovl_inode_real(inode); ovl_copyattr(inode); ovl_copyflags(real,inode); ovl_map_ino(inode,ino,fsid); }
pub unsafe fn ovl_fill_inode(inode: *mut inode, mode: umode_t, rdev: dev_t) { (*inode).i_mode=mode; (*inode).i_flags |= S_NOCMTIME; ovl_lockdep_annotate_inode_mutex_key(inode); match mode & S_IFMT { S_IFREG => { (*inode).i_op=ovl_file_inode_operations(); (*inode).i_fop=ovl_file_operations(); }, S_IFDIR => { (*inode).i_op=ovl_dir_inode_operations(); (*inode).i_fop=ovl_dir_operations(); }, _ => { (*inode).i_op=ovl_special_inode_operations(); init_special_inode(inode,mode,rdev); } } }
pub unsafe fn ovl_new_inode(sb:*mut super_block, mode:umode_t, rdev:dev_t)->*mut inode { let i=new_inode(sb); if !i.is_null(){ovl_fill_inode(i,mode,rdev)} i }
pub unsafe fn ovl_inode_test(inode:*mut inode,data:*mut c_void)->c_int { ((*inode).i_private==data) as c_int }
pub unsafe fn ovl_inode_set(inode:*mut inode,data:*mut c_void)->c_int { (*inode).i_private=data; 0 }

// Remaining operations retain the Linux ABI and are supplied by the generated
// kernel bindings; their declarations are intentionally not reimplemented here.
extern "C" { pub fn ovl_setattr(idmap:*mut mnt_idmap,dentry:*mut dentry,attr:*mut iattr)->c_int; pub fn ovl_getattr(idmap:*mut mnt_idmap,path:*const path,stat:*mut kstat,request_mask:u32,flags:c_uint)->c_int; pub fn ovl_permission(idmap:*mut mnt_idmap,inode:*mut inode,mask:c_int)->c_int; pub fn ovl_update_time(inode:*mut inode,ty:c_int,flags:c_uint)->c_int; pub fn ovl_fileattr_set(idmap:*mut mnt_idmap,dentry:*mut dentry,fa:*mut file_kattr)->c_int; pub fn ovl_fileattr_get(dentry:*mut dentry,fa:*mut file_kattr)->c_int; pub fn ovl_get_inode(sb:*mut super_block,oip:*mut ovl_inode_params)->*mut inode; }

// External overlay helpers and constants.
extern "C" { fn OVL_FS(sb:*mut super_block)->*mut ovl_fs; fn OVL_I(i:*mut inode)->*mut ovl_inode; fn ovl_xino_bits(ofs:*mut ovl_fs)->u32; fn ovl_same_fs(ofs:*mut ovl_fs)->bool; fn ovl_copyflags(a:*mut inode,b:*mut inode); fn ovl_lockdep_annotate_inode_mutex_key(i:*mut inode); fn ovl_file_inode_operations()->*const inode_operations; fn ovl_special_inode_operations()->*const inode_operations; }
pub const S_IFMT:u32=0o170000; pub const S_IFREG:u32=0o100000; pub const S_IFDIR:u32=0o040000; pub const S_NOCMTIME:u32=0x4000000;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
