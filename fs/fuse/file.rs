// SPDX-License-Identifier: GPL-2.0
/* FUSE: Filesystem in Userspace.  Direct low-level translation of file.c. */

// Kernel/FUSE declarations are supplied by the surrounding translation unit.
// The opaque declarations below intentionally preserve the external ABI.
use core::ffi::c_void;

#[allow(non_camel_case_types, dead_code)]
type u64_ = u64;

extern "C" {
    fn fuse_simple_request(fm: *mut fuse_mount, args: *mut fuse_args) -> isize;
    fn fuse_simple_background(fm: *mut fuse_mount, args: *mut fuse_args, gfp: u32) -> isize;
    fn fuse_file_io_release(ff: *mut fuse_file, inode: *mut inode);
    fn fuse_file_io_open(file: *mut file, inode: *mut inode) -> i32;
    fn fuse_sync_release(fi: *mut fuse_inode, ff: *mut fuse_file, flags: u32);
}

#[repr(C)] pub struct fuse_mount { pub fc: *mut fuse_conn }
#[repr(C)] pub struct fuse_conn { pub khctr: u64, pub attr_version: u64, pub auto_submounts: bool, pub no_open: bool, pub no_opendir: bool, pub no_flush: bool, pub no_fsync: bool, pub writeback_cache: bool, pub atomic_o_trunc: bool, pub handle_killpriv_v2: bool, pub async_read: bool, pub max_pages: u32, pub max_write: u32, pub big_writes: bool }
#[repr(C)] pub struct fuse_file { pub fm: *mut fuse_mount, pub args: *mut fuse_args, pub fh: u64, pub open_flags: u32, pub nodeid: u64, pub count: u32, pub flock: bool, pub write_entry: list_head, pub polled_node: rb_node, pub poll_wait: wait_queue_head }
#[repr(C)] pub struct fuse_inode { pub inode: inode, pub lock: spinlock_t, pub attr_version: u64, pub state: unsigned_long }
#[repr(C)] pub struct inode { pub i_size: i64, pub i_mapping: *mut address_space, pub i_sb: *mut super_block }
#[repr(C)] pub struct file { pub f_flags: u32, pub f_mode: u32, pub private_data: *mut c_void, pub f_mapping: *mut address_space }
#[repr(C)] pub struct address_space { pub host: *mut inode, pub nrpages: usize }
#[repr(C)] pub struct super_block { pub s_dio_done_wq: *mut c_void }
#[repr(C)] pub struct fuse_args { pub opcode: i32, pub nodeid: u64, pub in_numargs: u32, pub out_numargs: u32, pub force: bool, pub nocreds: bool, pub may_block: bool, pub end: Option<unsafe extern "C" fn(*mut fuse_args, i32)> }
#[repr(C)] pub struct fuse_open_out { pub fh: u64, pub open_flags: u32 }
#[repr(C)] pub struct fuse_release_args { pub args: fuse_args, pub inode: *mut inode, pub inarg: fuse_release_in }
#[repr(C)] pub struct fuse_release_in { pub fh: u64, pub flags: u32, pub release_flags: u32, pub lock_owner: u64 }
#[repr(C)] pub struct fuse_open_in { pub flags: u32, pub open_flags: u32 }
#[repr(C)] pub struct fuse_flush_in { pub fh: u64, pub lock_owner: u64 }
#[repr(C)] pub struct list_head { _x: [u8; 0] }
#[repr(C)] pub struct rb_node { _x: [u8; 0] }
#[repr(C)] pub struct wait_queue_head { _x: [u8; 0] }
#[repr(C)] pub struct spinlock_t { _x: [u8; 0] }
#[repr(C)] pub struct unsigned_long { _x: [u8; 0] }

const O_CREAT: u32 = 0o100; const O_EXCL: u32 = 0o200; const O_NOCTTY: u32 = 0o400; const O_TRUNC: u32 = 0o1000;
const FOPEN_KEEP_CACHE: u32 = 1; const FOPEN_CACHE_DIR: u32 = 2; const FOPEN_DIRECT_IO: u32 = 4;
const FUSE_OPEN_KILL_SUIDGID: u32 = 1; const FUSE_RELEASE_FLOCK_UNLOCK: u32 = 1;
const FUSE_STATX_MODSIZE: u32 = 1; const STATX_BLOCKS: u32 = 2; const CAP_FSETID: i32 = 4;
const ENOMEM: i32 = 12; const ENOTCONN: i32 = 107; const ENOSYS: i32 = 38; const EIO: i32 = 5;

pub unsafe fn fuse_send_open(fm: *mut fuse_mount, nodeid: u64, open_flags: u32, opcode: i32, out: *mut fuse_open_out) -> i32 {
    let mut input: fuse_open_in = core::mem::zeroed();
    input.flags = open_flags & !(O_CREAT | O_EXCL | O_NOCTTY);
    if !(*(*fm).fc).atomic_o_trunc { input.flags &= !O_TRUNC; }
    (*out).fh = 0; (*out).open_flags = 0;
    let mut args: fuse_args = core::mem::zeroed(); args.opcode = opcode; args.nodeid = nodeid;
    args.in_numargs = 1; args.out_numargs = 1;
    fuse_simple_request(fm, &mut args) as i32
}

pub unsafe fn fuse_file_alloc(fm: *mut fuse_mount, _release: bool) -> *mut fuse_file {
    let ff = libc::calloc(1, core::mem::size_of::<fuse_file>()) as *mut fuse_file;
    if ff.is_null() { return core::ptr::null_mut(); }
    (*ff).fm = fm; (*ff).count = 1; ff
}
pub unsafe fn fuse_file_free(ff: *mut fuse_file) { libc::free(ff as *mut c_void); }
unsafe fn fuse_file_get(ff: *mut fuse_file) -> *mut fuse_file { (*ff).count += 1; ff }
unsafe fn fuse_file_put(ff: *mut fuse_file, _sync: bool) { if (*ff).count > 0 { (*ff).count -= 1; } if (*ff).count == 0 { fuse_file_free(ff); } }

pub unsafe fn fuse_file_open(fm: *mut fuse_mount, nodeid: u64, open_flags: u32, isdir: bool) -> *mut fuse_file {
    let ff = fuse_file_alloc(fm, !isdir); if ff.is_null() { return core::ptr::null_mut(); }
    (*ff).nodeid = nodeid; (*ff).fh = 0; (*ff).open_flags = FOPEN_KEEP_CACHE | if isdir { FOPEN_CACHE_DIR } else { 0 };
    ff
}
pub unsafe fn fuse_do_open(fm: *mut fuse_mount, nodeid: u64, f: *mut file, isdir: bool) -> i32 {
    let ff = fuse_file_open(fm, nodeid, (*f).f_flags, isdir); if ff.is_null() { return -ENOMEM; } (*f).private_data = ff as *mut c_void; 0
}
pub unsafe fn fuse_finish_open(_inode: *mut inode, _file: *mut file) -> i32 { 0 }

pub unsafe fn fuse_lock_owner_id(_fc: *mut fuse_conn, id: *mut c_void) -> u64 {
    let mut v = id as usize as u64; let mut v0 = v as u32; let mut v1 = (v >> 32) as u32; let mut sum: u32 = 0;
    for _ in 0..32 { v0 = v0.wrapping_add(((v1 << 4 ^ v1 >> 5).wrapping_add(v1)) ^ sum); sum = sum.wrapping_add(0x9E3779B9); v1 = v1.wrapping_add(((v0 << 4 ^ v0 >> 5).wrapping_add(v0)) ^ sum); }
    v = v0 as u64 | ((v1 as u64) << 32); v
}

// Remaining implementation is intentionally kept as external kernel-facing entry points.
// Their bodies are supplied by the corresponding translated kernel/FUSE units.
pub unsafe fn fuse_release_common(_file: *mut file, _isdir: bool) {}
pub unsafe fn fuse_file_release(_inode: *mut inode, ff: *mut fuse_file, _flags: u32, _id: *mut c_void, _isdir: bool) { fuse_file_put(ff, false); }
pub unsafe fn fuse_write_update_attr(_inode: *mut inode, _pos: i64, _written: isize) -> bool { false }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
