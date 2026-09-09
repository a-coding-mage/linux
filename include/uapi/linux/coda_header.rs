/* Rust translation of coda.h. C preprocessor conditions are retained only
 * where they affect declarations; external platform types remain external. */

use core::ffi::{c_char, c_int, c_long, c_void};
use core::mem::ManuallyDrop;

pub type u_long = u64;
pub type u_short = u16;
pub type u_quad_t = u64;
pub type ino_t = u_long;
pub type dev_t = u_long;
pub type caddr_t = *mut c_void;
pub type __kernel_pid_t = i32;
pub type int8_t = i8;
pub type u_int8_t = u8;
pub type int16_t = i16;
pub type u_int16_t = u16;
pub type int32_t = i32;
pub type u_int32_t = u32;
pub type cdev_t = u_quad_t;

pub const CODA_MAXSYMLINKS: u32 = 10;
pub const CODA_MAXNAMLEN: usize = 255;
pub const CODA_MAXPATHLEN: usize = 1024;
pub const CODA_MAXSYMLINK: u32 = 10;
pub const C_O_READ: u32 = 0x001;
pub const C_O_WRITE: u32 = 0x002;
pub const C_O_TRUNC: u32 = 0x010;
pub const C_O_EXCL: u32 = 0x100;
pub const C_O_CREAT: u32 = 0x200;
pub const C_M_READ: u32 = 0o400;
pub const C_M_WRITE: u32 = 0o200;
pub const C_A_C_OK: u32 = 8;
pub const C_A_R_OK: u32 = 4;
pub const C_A_W_OK: u32 = 2;
pub const C_A_X_OK: u32 = 1;
pub const C_A_F_OK: u32 = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct venus_dirent { pub d_fileno: u32, pub d_reclen: u16, pub d_type: u8, pub d_namlen: u8, pub d_name: [c_char; CODA_MAXNAMLEN + 1] }
pub const CDT_UNKNOWN: u32 = 0; pub const CDT_FIFO: u32 = 1; pub const CDT_CHR: u32 = 2; pub const CDT_DIR: u32 = 4;
pub const CDT_BLK: u32 = 6; pub const CDT_REG: u32 = 8; pub const CDT_LNK: u32 = 10; pub const CDT_SOCK: u32 = 12; pub const CDT_WHT: u32 = 14;
pub const fn dirsiz(namlen: u8) -> usize { (core::mem::size_of::<venus_dirent>() - (CODA_MAXNAMLEN + 1)) + (((namlen as usize + 1 + 3) & !3)) }
pub const fn iftocdt(mode: u32) -> u32 { (mode & 0o170000) >> 12 }
pub const fn cdttoif(dirtype: u32) -> u32 { dirtype << 12 }

pub type vuid_t = u32; pub type vgid_t = u32;
#[repr(C)] #[derive(Copy, Clone)] pub struct CodaFid { pub opaque: [u32; 4] }
pub unsafe fn coda_f2i(fid: *const CodaFid) -> u32 { if fid.is_null() { 0 } else { (*fid).opaque[3] ^ ((*fid).opaque[2] << 10) ^ ((*fid).opaque[1] << 20) ^ (*fid).opaque[0] } }

#[repr(C)] #[derive(Copy, Clone)] pub enum coda_vtype { C_VNON, C_VREG, C_VDIR, C_VBLK, C_VCHR, C_VLNK, C_VSOCK, C_VFIFO, C_VBAD }
#[repr(C)] #[derive(Copy, Clone)] pub struct coda_timespec { pub tv_sec: i64, pub tv_nsec: c_long }
#[repr(C)] #[derive(Copy, Clone)] pub struct coda_vattr { pub va_type: c_long, pub va_mode: u_short, pub va_nlink: i16, pub va_uid: vuid_t, pub va_gid: vgid_t, pub va_fileid: c_long, pub va_size: u_quad_t, pub va_blocksize: c_long, pub va_atime: coda_timespec, pub va_mtime: coda_timespec, pub va_ctime: coda_timespec, pub va_gen: u_long, pub va_flags: u_long, pub va_rdev: cdev_t, pub va_bytes: u_quad_t, pub va_filerev: u_quad_t }
#[repr(C)] #[derive(Copy, Clone)] pub struct coda_statfs { pub f_blocks: i32, pub f_bfree: i32, pub f_bavail: i32, pub f_files: i32, pub f_ffree: i32 }

pub const CODA_ROOT: u32 = 2; pub const CODA_OPEN_BY_FD: u32 = 3; pub const CODA_OPEN: u32 = 4; pub const CODA_CLOSE: u32 = 5; pub const CODA_IOCTL: u32 = 6; pub const CODA_GETATTR: u32 = 7; pub const CODA_SETATTR: u32 = 8; pub const CODA_ACCESS: u32 = 9; pub const CODA_LOOKUP: u32 = 10; pub const CODA_CREATE: u32 = 11; pub const CODA_REMOVE: u32 = 12; pub const CODA_LINK: u32 = 13; pub const CODA_RENAME: u32 = 14; pub const CODA_MKDIR: u32 = 15; pub const CODA_RMDIR: u32 = 16; pub const CODA_SYMLINK: u32 = 18; pub const CODA_READLINK: u32 = 19; pub const CODA_FSYNC: u32 = 20; pub const CODA_VGET: u32 = 22; pub const CODA_SIGNAL: u32 = 23; pub const CODA_REPLACE: u32 = 24; pub const CODA_FLUSH: u32 = 25; pub const CODA_PURGEUSER: u32 = 26; pub const CODA_ZAPFILE: u32 = 27; pub const CODA_ZAPDIR: u32 = 28; pub const CODA_PURGEFID: u32 = 30; pub const CODA_OPEN_BY_PATH: u32 = 31; pub const CODA_RESOLVE: u32 = 32; pub const CODA_REINTEGRATE: u32 = 33; pub const CODA_STATFS: u32 = 34; pub const CODA_STORE: u32 = 35; pub const CODA_RELEASE: u32 = 36; pub const CODA_ACCESS_INTENT: u32 = 37; pub const CODA_NCALLS: u32 = 38;
pub const fn downcall(opcode: u32) -> bool { opcode >= CODA_REPLACE && opcode <= CODA_PURGEFID }
pub const VC_MAXDATASIZE: usize = 8192; pub const CODA_KERNEL_VERSION: u32 = 5;

#[repr(C)] #[derive(Copy, Clone)] pub struct coda_in_hdr { pub opcode: u32, pub unique: u32, pub pid: __kernel_pid_t, pub pgid: __kernel_pid_t, pub uid: vuid_t }
#[repr(C)] #[derive(Copy, Clone)] pub struct coda_out_hdr { pub opcode: u32, pub unique: u32, pub result: u32 }

macro_rules! simple_in { ($n:ident { $($f:ident : $t:ty),* $(,)? }) => { #[repr(C)] #[derive(Copy,Clone)] pub struct $n { pub ih: coda_in_hdr, $(pub $f: $t),* } }; }
macro_rules! simple_out { ($n:ident) => { #[repr(C)] #[derive(Copy,Clone)] pub struct $n { pub out: coda_out_hdr } }; }
#[repr(C)] #[derive(Copy,Clone)] pub struct coda_root_out { pub oh:coda_out_hdr, pub VFid:CodaFid } #[repr(C)] #[derive(Copy,Clone)] pub struct coda_root_in { pub r#in:coda_in_hdr }
simple_in!(coda_open_in { VFid:CodaFid, flags:c_int }); #[repr(C)] #[derive(Copy,Clone)] pub struct coda_open_out { pub oh:coda_out_hdr, pub dev:cdev_t, pub inode:ino_t }
simple_in!(coda_store_in { VFid:CodaFid, flags:c_int }); simple_out!(coda_store_out); simple_in!(coda_release_in { VFid:CodaFid, flags:c_int }); simple_out!(coda_release_out); simple_in!(coda_close_in { VFid:CodaFid, flags:c_int }); simple_out!(coda_close_out);
simple_in!(coda_getattr_in { VFid:CodaFid }); #[repr(C)] #[derive(Copy,Clone)] pub struct coda_getattr_out { pub oh:coda_out_hdr, pub attr:coda_vattr }
simple_in!(coda_setattr_in { VFid:CodaFid, attr:coda_vattr }); simple_out!(coda_setattr_out); simple_in!(coda_access_in { VFid:CodaFid, flags:c_int }); simple_out!(coda_access_out);
#[repr(C)] #[derive(Copy,Clone)] pub struct coda_ioctl_in { pub ih:coda_in_hdr,pub VFid:CodaFid,pub cmd:c_int,pub len:c_int,pub rwflag:c_int,pub data:*mut c_char } #[repr(C)] #[derive(Copy,Clone)] pub struct coda_ioctl_out { pub oh:coda_out_hdr,pub len:c_int,pub data:caddr_t }
#[repr(C)] #[derive(Copy,Clone)] pub struct coda_lookup_in { pub ih:coda_in_hdr,pub VFid:CodaFid,pub name:c_int,pub flags:c_int } #[repr(C)] #[derive(Copy,Clone)] pub struct coda_lookup_out { pub oh:coda_out_hdr,pub VFid:CodaFid,pub vtype:c_int }
#[repr(C)] #[derive(Copy,Clone)] pub struct coda_create_in { pub ih:coda_in_hdr,pub VFid:CodaFid,pub attr:coda_vattr,pub excl:c_int,pub mode:c_int,pub name:c_int } #[repr(C)] #[derive(Copy,Clone)] pub struct coda_create_out { pub oh:coda_out_hdr,pub VFid:CodaFid,pub attr:coda_vattr }
simple_in!(coda_remove_in { VFid:CodaFid, name:c_int }); simple_out!(coda_remove_out);
#[repr(C)] #[derive(Copy,Clone)] pub struct coda_link_in { pub ih:coda_in_hdr,pub sourceFid:CodaFid,pub destFid:CodaFid,pub tname:c_int } simple_out!(coda_link_out);
#[repr(C)] #[derive(Copy,Clone)] pub struct coda_rename_in { pub ih:coda_in_hdr,pub sourceFid:CodaFid,pub srcname:c_int,pub destFid:CodaFid,pub destname:c_int } simple_out!(coda_rename_out);
#[repr(C)] #[derive(Copy,Clone)] pub struct coda_mkdir_in { pub ih:coda_in_hdr,pub VFid:CodaFid,pub attr:coda_vattr,pub name:c_int } #[repr(C)] #[derive(Copy,Clone)] pub struct coda_mkdir_out { pub oh:coda_out_hdr,pub VFid:CodaFid,pub attr:coda_vattr }
simple_in!(coda_rmdir_in { VFid:CodaFid, name:c_int }); simple_out!(coda_rmdir_out);
#[repr(C)] #[derive(Copy,Clone)] pub struct coda_symlink_in { pub ih:coda_in_hdr,pub VFid:CodaFid,pub srcname:c_int,pub attr:coda_vattr,pub tname:c_int } simple_out!(coda_symlink_out);
simple_in!(coda_readlink_in { VFid:CodaFid }); #[repr(C)] #[derive(Copy,Clone)] pub struct coda_readlink_out { pub oh:coda_out_hdr,pub count:c_int,pub data:caddr_t }
simple_in!(coda_fsync_in { VFid:CodaFid }); simple_out!(coda_fsync_out); simple_in!(coda_vget_in { VFid:CodaFid }); #[repr(C)] #[derive(Copy,Clone)] pub struct coda_vget_out { pub oh:coda_out_hdr,pub VFid:CodaFid,pub vtype:c_int }
#[repr(C)] #[derive(Copy,Clone)] pub struct coda_purgeuser_out { pub oh:coda_out_hdr,pub uid:vuid_t } #[repr(C)] #[derive(Copy,Clone)] pub struct coda_zapfile_out { pub oh:coda_out_hdr,pub CodaFid:CodaFid } #[repr(C)] #[derive(Copy,Clone)] pub struct coda_zapdir_out { pub oh:coda_out_hdr,pub CodaFid:CodaFid } #[repr(C)] #[derive(Copy,Clone)] pub struct coda_purgefid_out { pub oh:coda_out_hdr,pub CodaFid:CodaFid } #[repr(C)] #[derive(Copy,Clone)] pub struct coda_replace_out { pub oh:coda_out_hdr,pub NewFid:CodaFid,pub OldFid:CodaFid }
simple_in!(coda_open_by_fd_in { VFid:CodaFid, flags:c_int }); #[repr(C)] #[derive(Copy,Clone)] pub struct coda_open_by_fd_out { pub oh:coda_out_hdr,pub fd:c_int /* __KERNEL__: struct file *fh */ }
simple_in!(coda_open_by_path_in { VFid:CodaFid, flags:c_int }); #[repr(C)] #[derive(Copy,Clone)] pub struct coda_open_by_path_out { pub oh:coda_out_hdr,pub path:c_int }
#[repr(C)] #[derive(Copy,Clone)] pub struct coda_statfs_in { pub r#in:coda_in_hdr } #[repr(C)] #[derive(Copy,Clone)] pub struct coda_statfs_out { pub oh:coda_out_hdr,pub stat:coda_statfs }
simple_in!(coda_access_intent_in { VFid:CodaFid, count:c_int, pos:c_int, type_:c_int }); simple_out!(coda_access_intent_out);
pub const CLU_CASE_SENSITIVE:u32=1; pub const CLU_CASE_INSENSITIVE:u32=2; pub const CODA_ACCESS_TYPE_READ:u32=1; pub const CODA_ACCESS_TYPE_WRITE:u32=2; pub const CODA_ACCESS_TYPE_MMAP:u32=3; pub const CODA_ACCESS_TYPE_READ_FINISH:u32=4; pub const CODA_ACCESS_TYPE_WRITE_FINISH:u32=5; pub const CODA_NOCACHE:u32=0x80000000;

#[repr(C)] pub union inputArgs { pub ih:ManuallyDrop<coda_in_hdr>, pub coda_open:ManuallyDrop<coda_open_in>, pub coda_store:ManuallyDrop<coda_store_in>, pub coda_release:ManuallyDrop<coda_release_in>, pub coda_close:ManuallyDrop<coda_close_in>, pub coda_ioctl:ManuallyDrop<coda_ioctl_in>, pub coda_getattr:ManuallyDrop<coda_getattr_in>, pub coda_setattr:ManuallyDrop<coda_setattr_in>, pub coda_access:ManuallyDrop<coda_access_in>, pub coda_lookup:ManuallyDrop<coda_lookup_in>, pub coda_create:ManuallyDrop<coda_create_in>, pub coda_remove:ManuallyDrop<coda_remove_in>, pub coda_link:ManuallyDrop<coda_link_in>, pub coda_rename:ManuallyDrop<coda_rename_in>, pub coda_mkdir:ManuallyDrop<coda_mkdir_in>, pub coda_rmdir:ManuallyDrop<coda_rmdir_in>, pub coda_symlink:ManuallyDrop<coda_symlink_in>, pub coda_readlink:ManuallyDrop<coda_readlink_in>, pub coda_fsync:ManuallyDrop<coda_fsync_in>, pub coda_vget:ManuallyDrop<coda_vget_in>, pub coda_open_by_fd:ManuallyDrop<coda_open_by_fd_in>, pub coda_open_by_path:ManuallyDrop<coda_open_by_path_in>, pub coda_statfs:ManuallyDrop<coda_statfs_in>, pub coda_access_intent:ManuallyDrop<coda_access_intent_in> }
#[repr(C)] pub union outputArgs { pub oh:ManuallyDrop<coda_out_hdr>, pub coda_root:ManuallyDrop<coda_root_out>, pub coda_open:ManuallyDrop<coda_open_out>, pub coda_ioctl:ManuallyDrop<coda_ioctl_out>, pub coda_getattr:ManuallyDrop<coda_getattr_out>, pub coda_lookup:ManuallyDrop<coda_lookup_out>, pub coda_create:ManuallyDrop<coda_create_out>, pub coda_mkdir:ManuallyDrop<coda_mkdir_out>, pub coda_readlink:ManuallyDrop<coda_readlink_out>, pub coda_vget:ManuallyDrop<coda_vget_out>, pub coda_purgeuser:ManuallyDrop<coda_purgeuser_out>, pub coda_zapfile:ManuallyDrop<coda_zapfile_out>, pub coda_zapdir:ManuallyDrop<coda_zapdir_out>, pub coda_purgefid:ManuallyDrop<coda_purgefid_out>, pub coda_replace:ManuallyDrop<coda_replace_out>, pub coda_open_by_fd:ManuallyDrop<coda_open_by_fd_out>, pub coda_open_by_path:ManuallyDrop<coda_open_by_path_out>, pub coda_statfs:ManuallyDrop<coda_statfs_out> }
#[repr(C)] pub union coda_downcalls { pub purgeuser:ManuallyDrop<coda_purgeuser_out>, pub zapfile:ManuallyDrop<coda_zapfile_out>, pub zapdir:ManuallyDrop<coda_zapdir_out>, pub purgefid:ManuallyDrop<coda_purgefid_out>, pub replace:ManuallyDrop<coda_replace_out> }

pub const PIOCPARM_MASK:u32=0x0000ffff; #[repr(C)] #[derive(Copy,Clone)] pub struct ViceIoctl { pub r#in:*mut c_void,pub out:*mut c_void,pub in_size:u16,pub out_size:u16 } #[repr(C)] #[derive(Copy,Clone)] pub struct PioctlData { pub path:*const c_char,pub follow:c_int,pub vi:ViceIoctl }
pub const CODA_CONTROL:&[u8]=b".CONTROL\0"; pub const CODA_CONTROLLEN:u32=8; pub const CTL_INO:c_int=-1; pub const CODA_MOUNT_VERSION:c_int=1;
#[repr(C)] #[derive(Copy,Clone)] pub struct coda_mount_data { pub version:c_int,pub fd:c_int }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
