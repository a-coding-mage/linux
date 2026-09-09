/* SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR BSD-2-Clause) */
/* Rust translation of the Linux FUSE kernel interface header. */

#![allow(non_camel_case_types, non_upper_case_globals, dead_code)]

pub const FUSE_KERNEL_VERSION: u32 = 7;
pub const FUSE_KERNEL_MINOR_VERSION: u32 = 46;
pub const FUSE_ROOT_ID: u64 = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fuse_attr { pub ino:u64,pub size:u64,pub blocks:u64,pub atime:u64,pub mtime:u64,pub ctime:u64,pub atimensec:u32,pub mtimensec:u32,pub ctimensec:u32,pub mode:u32,pub nlink:u32,pub uid:u32,pub gid:u32,pub rdev:u32,pub blksize:u32,pub flags:u32 }
#[repr(C)] #[derive(Copy,Clone)] pub struct fuse_sx_time { pub tv_sec:i64,pub tv_nsec:u32,pub __reserved:i32 }
#[repr(C)] #[derive(Copy,Clone)] pub struct fuse_statx { pub mask:u32,pub blksize:u32,pub attributes:u64,pub nlink:u32,pub uid:u32,pub gid:u32,pub mode:u16,pub __spare0:[u16;1],pub ino:u64,pub size:u64,pub blocks:u64,pub attributes_mask:u64,pub atime:fuse_sx_time,pub btime:fuse_sx_time,pub ctime:fuse_sx_time,pub mtime:fuse_sx_time,pub rdev_major:u32,pub rdev_minor:u32,pub dev_major:u32,pub dev_minor:u32,pub __spare2:[u64;14] }
#[repr(C)] #[derive(Copy,Clone)] pub struct fuse_kstatfs { pub blocks:u64,pub bfree:u64,pub bavail:u64,pub files:u64,pub ffree:u64,pub bsize:u32,pub namelen:u32,pub frsize:u32,pub padding:u32,pub spare:[u32;6] }
#[repr(C)] #[derive(Copy,Clone)] pub struct fuse_file_lock { pub start:u64,pub end:u64,pub r#type:u32,pub pid:u32 }

macro_rules! cconsts { ($($n:ident = $v:expr),* $(,)?) => { $(pub const $n: u64 = $v;)* }; }
cconsts!(FATTR_MODE=1<<0,FATTR_UID=1<<1,FATTR_GID=1<<2,FATTR_SIZE=1<<3,FATTR_ATIME=1<<4,FATTR_MTIME=1<<5,FATTR_FH=1<<6,FATTR_ATIME_NOW=1<<7,FATTR_MTIME_NOW=1<<8,FATTR_LOCKOWNER=1<<9,FATTR_CTIME=1<<10,FATTR_KILL_SUIDGID=1<<11,
FOPEN_DIRECT_IO=1<<0,FOPEN_KEEP_CACHE=1<<1,FOPEN_NONSEEKABLE=1<<2,FOPEN_CACHE_DIR=1<<3,FOPEN_STREAM=1<<4,FOPEN_NOFLUSH=1<<5,FOPEN_PARALLEL_DIRECT_WRITES=1<<6,FOPEN_PASSTHROUGH=1<<7,FOPEN_IO_URING_ZERO_COPY=1<<8,
FUSE_ASYNC_READ=1<<0,FUSE_POSIX_LOCKS=1<<1,FUSE_FILE_OPS=1<<2,FUSE_ATOMIC_O_TRUNC=1<<3,FUSE_EXPORT_SUPPORT=1<<4,FUSE_BIG_WRITES=1<<5,FUSE_DONT_MASK=1<<6,FUSE_SPLICE_WRITE=1<<7,FUSE_SPLICE_MOVE=1<<8,FUSE_SPLICE_READ=1<<9,FUSE_FLOCK_LOCKS=1<<10,FUSE_HAS_IOCTL_DIR=1<<11,FUSE_AUTO_INVAL_DATA=1<<12,FUSE_DO_READDIRPLUS=1<<13,FUSE_READDIRPLUS_AUTO=1<<14,FUSE_ASYNC_DIO=1<<15,FUSE_WRITEBACK_CACHE=1<<16,FUSE_NO_OPEN_SUPPORT=1<<17,FUSE_PARALLEL_DIROPS=1<<18,FUSE_HANDLE_KILLPRIV=1<<19,FUSE_POSIX_ACL=1<<20,FUSE_ABORT_ERROR=1<<21,FUSE_MAX_PAGES=1<<22,FUSE_CACHE_SYMLINKS=1<<23,FUSE_NO_OPENDIR_SUPPORT=1<<24,FUSE_EXPLICIT_INVAL_DATA=1<<25,FUSE_MAP_ALIGNMENT=1<<26,FUSE_SUBMOUNTS=1<<27,FUSE_HANDLE_KILLPRIV_V2=1<<28,FUSE_SETXATTR_EXT=1<<29,FUSE_INIT_EXT=1<<30,FUSE_INIT_RESERVED=1<<31);
pub const FUSE_SECURITY_CTX:u64=1<<32; pub const FUSE_HAS_INODE_DAX:u64=1<<33; pub const FUSE_CREATE_SUPP_GROUP:u64=1<<34; pub const FUSE_HAS_EXPIRE_ONLY:u64=1<<35; pub const FUSE_DIRECT_IO_ALLOW_MMAP:u64=1<<36; pub const FUSE_PASSTHROUGH:u64=1<<37; pub const FUSE_NO_EXPORT_SUPPORT:u64=1<<38; pub const FUSE_HAS_RESEND:u64=1<<39; pub const FUSE_DIRECT_IO_RELAX:u64=FUSE_DIRECT_IO_ALLOW_MMAP; pub const FUSE_ALLOW_IDMAP:u64=1<<40; pub const FUSE_OVER_IO_URING:u64=1<<41; pub const FUSE_REQUEST_TIMEOUT:u64=1<<42; pub const FUSE_HAS_IO_URING_BUFPOOL:u64=1<<43;

pub const CUSE_UNRESTRICTED_IOCTL:u64=1; pub const FUSE_RELEASE_FLUSH:u64=1; pub const FUSE_RELEASE_FLOCK_UNLOCK:u64=2; pub const FUSE_GETATTR_FH:u64=1; pub const FUSE_LK_FLOCK:u64=1; pub const FUSE_WRITE_CACHE:u64=1; pub const FUSE_WRITE_LOCKOWNER:u64=2; pub const FUSE_WRITE_KILL_SUIDGID:u64=4; pub const FUSE_WRITE_KILL_PRIV:u64=FUSE_WRITE_KILL_SUIDGID; pub const FUSE_READ_LOCKOWNER:u64=2; pub const FUSE_IOCTL_COMPAT:u64=1; pub const FUSE_IOCTL_UNRESTRICTED:u64=2; pub const FUSE_IOCTL_RETRY:u64=4; pub const FUSE_IOCTL_32BIT:u64=8; pub const FUSE_IOCTL_DIR:u64=16; pub const FUSE_IOCTL_COMPAT_X32:u64=32; pub const FUSE_IOCTL_MAX_IOV:u64=256; pub const FUSE_POLL_SCHEDULE_NOTIFY:u64=1; pub const FUSE_FSYNC_FDATASYNC:u64=1; pub const FUSE_ATTR_SUBMOUNT:u64=1; pub const FUSE_ATTR_DAX:u64=2; pub const FUSE_OPEN_KILL_SUIDGID:u64=1; pub const FUSE_SETXATTR_ACL_KILL_SGID:u64=1; pub const FUSE_EXPIRE_ONLY:u64=1;

#[repr(C)] #[derive(Copy,Clone)] pub struct fuse_entry_out{pub nodeid:u64,pub generation:u64,pub entry_valid:u64,pub attr_valid:u64,pub entry_valid_nsec:u32,pub attr_valid_nsec:u32,pub attr:fuse_attr}
#[repr(C)] #[derive(Copy,Clone)] pub struct fuse_forget_in{pub nlookup:u64}
#[repr(C)] #[derive(Copy,Clone)] pub struct fuse_forget_one{pub nodeid:u64,pub nlookup:u64}
#[repr(C)] #[derive(Copy,Clone)] pub struct fuse_batch_forget_in{pub count:u32,pub dummy:u32}
#[repr(C)] #[derive(Copy,Clone)] pub struct fuse_getattr_in{pub getattr_flags:u32,pub dummy:u32,pub fh:u64}
#[repr(C)] #[derive(Copy,Clone)] pub struct fuse_attr_out{pub attr_valid:u64,pub attr_valid_nsec:u32,pub dummy:u32,pub attr:fuse_attr}
#[repr(C)] #[derive(Copy,Clone)] pub struct fuse_statx_in{pub getattr_flags:u32,pub reserved:u32,pub fh:u64,pub sx_flags:u32,pub sx_mask:u32}
#[repr(C)] #[derive(Copy,Clone)] pub struct fuse_statx_out{pub attr_valid:u64,pub attr_valid_nsec:u32,pub flags:u32,pub spare:[u64;2],pub stat:fuse_statx}
#[repr(C)] #[derive(Copy,Clone)] pub struct fuse_mknod_in{pub mode:u32,pub rdev:u32,pub umask:u32,pub padding:u32}
#[repr(C)] #[derive(Copy,Clone)] pub struct fuse_mkdir_in{pub mode:u32,pub umask:u32}
#[repr(C)] #[derive(Copy,Clone)] pub struct fuse_rename_in{pub newdir:u64}
#[repr(C)] #[derive(Copy,Clone)] pub struct fuse_rename2_in{pub newdir:u64,pub flags:u32,pub padding:u32}
#[repr(C)] #[derive(Copy,Clone)] pub struct fuse_link_in{pub oldnodeid:u64}
#[repr(C)] #[derive(Copy,Clone)] pub struct fuse_setattr_in{pub valid:u32,pub padding:u32,pub fh:u64,pub size:u64,pub lock_owner:u64,pub atime:u64,pub mtime:u64,pub ctime:u64,pub atimensec:u32,pub mtimensec:u32,pub ctimensec:u32,pub mode:u32,pub unused4:u32,pub uid:u32,pub gid:u32,pub unused5:u32}
#[repr(C)] #[derive(Copy,Clone)] pub struct fuse_open_in{pub flags:u32,pub open_flags:u32}
#[repr(C)] #[derive(Copy,Clone)] pub struct fuse_create_in{pub flags:u32,pub mode:u32,pub umask:u32,pub open_flags:u32}
#[repr(C)] #[derive(Copy,Clone)] pub struct fuse_open_out{pub fh:u64,pub open_flags:u32,pub backing_id:i32}
#[repr(C)] #[derive(Copy,Clone)] pub struct fuse_release_in{pub fh:u64,pub flags:u32,pub release_flags:u32,pub lock_owner:u64}
#[repr(C)] #[derive(Copy,Clone)] pub struct fuse_flush_in{pub fh:u64,pub unused:u32,pub padding:u32,pub lock_owner:u64}
#[repr(C)] #[derive(Copy,Clone)] pub struct fuse_read_in{pub fh:u64,pub offset:u64,pub size:u32,pub read_flags:u32,pub lock_owner:u64,pub flags:u32,pub padding:u32}
#[repr(C)] #[derive(Copy,Clone)] pub struct fuse_write_in{pub fh:u64,pub offset:u64,pub size:u32,pub write_flags:u32,pub lock_owner:u64,pub flags:u32,pub padding:u32}
#[repr(C)] #[derive(Copy,Clone)] pub struct fuse_write_out{pub size:u32,pub padding:u32}
#[repr(C)] #[derive(Copy,Clone)] pub struct fuse_statfs_out{pub st:fuse_kstatfs}
#[repr(C)] #[derive(Copy,Clone)] pub struct fuse_fsync_in{pub fh:u64,pub fsync_flags:u32,pub padding:u32}
#[repr(C)] #[derive(Copy,Clone)] pub struct fuse_setxattr_in{pub size:u32,pub flags:u32,pub setxattr_flags:u32,pub padding:u32}
#[repr(C)] #[derive(Copy,Clone)] pub struct fuse_getxattr_in{pub size:u32,pub padding:u32}
#[repr(C)] #[derive(Copy,Clone)] pub struct fuse_getxattr_out{pub size:u32,pub padding:u32}
#[repr(C)] #[derive(Copy,Clone)] pub struct fuse_lk_in{pub fh:u64,pub owner:u64,pub lk:fuse_file_lock,pub lk_flags:u32,pub padding:u32}
#[repr(C)] #[derive(Copy,Clone)] pub struct fuse_lk_out{pub lk:fuse_file_lock}
#[repr(C)] #[derive(Copy,Clone)] pub struct fuse_access_in{pub mask:u32,pub padding:u32}
#[repr(C)] #[derive(Copy,Clone)] pub struct fuse_init_in{pub major:u32,pub minor:u32,pub max_readahead:u32,pub flags:u32,pub flags2:u32,pub unused:[u32;11]}
#[repr(C)] #[derive(Copy,Clone)] pub struct fuse_init_out{pub major:u32,pub minor:u32,pub max_readahead:u32,pub flags:u32,pub max_background:u16,pub congestion_threshold:u16,pub max_write:u32,pub time_gran:u32,pub max_pages:u16,pub map_alignment:u16,pub flags2:u32,pub max_stack_depth:u32,pub request_timeout:u16,pub unused:[u16;11]}
#[repr(C)] #[derive(Copy,Clone)] pub struct cuse_init_in{pub major:u32,pub minor:u32,pub unused:u32,pub flags:u32}
#[repr(C)] #[derive(Copy,Clone)] pub struct cuse_init_out{pub major:u32,pub minor:u32,pub unused:u32,pub flags:u32,pub max_read:u32,pub max_write:u32,pub dev_major:u32,pub dev_minor:u32,pub spare:[u32;10]}
#[repr(C)] #[derive(Copy,Clone)] pub struct fuse_interrupt_in{pub unique:u64}
#[repr(C)] #[derive(Copy,Clone)] pub struct fuse_bmap_in{pub block:u64,pub blocksize:u32,pub padding:u32}
#[repr(C)] #[derive(Copy,Clone)] pub struct fuse_bmap_out{pub block:u64}
#[repr(C)] #[derive(Copy,Clone)] pub struct fuse_ioctl_in{pub fh:u64,pub flags:u32,pub cmd:u32,pub arg:u64,pub in_size:u32,pub out_size:u32}
#[repr(C)] #[derive(Copy,Clone)] pub struct fuse_ioctl_iovec{pub base:u64,pub len:u64}
#[repr(C)] #[derive(Copy,Clone)] pub struct fuse_ioctl_out{pub result:i32,pub flags:u32,pub in_iovs:u32,pub out_iovs:u32}
#[repr(C)] #[derive(Copy,Clone)] pub struct fuse_poll_in{pub fh:u64,pub kh:u64,pub flags:u32,pub events:u32}
#[repr(C)] #[derive(Copy,Clone)] pub struct fuse_poll_out{pub revents:u32,pub padding:u32}
#[repr(C)] #[derive(Copy,Clone)] pub struct fuse_notify_poll_wakeup_out{pub kh:u64}
#[repr(C)] #[derive(Copy,Clone)] pub struct fuse_fallocate_in{pub fh:u64,pub offset:u64,pub length:u64,pub mode:u32,pub padding:u32}
pub const FUSE_UNIQUE_RESEND:u64=1<<63; pub const FUSE_INVALID_UIDGID:u32=u32::MAX;
#[repr(C)] #[derive(Copy,Clone)] pub struct fuse_in_header{pub len:u32,pub opcode:u32,pub unique:u64,pub nodeid:u64,pub uid:u32,pub gid:u32,pub pid:u32,pub total_extlen:u16,pub padding:u16}
#[repr(C)] #[derive(Copy,Clone)] pub struct fuse_out_header{pub len:u32,pub error:i32,pub unique:u64}
#[repr(C)] pub struct fuse_dirent{pub ino:u64,pub off:u64,pub namelen:u32,pub r#type:u32,pub name:[u8;0]}
#[repr(C)] #[derive(Copy,Clone)] pub struct fuse_notify_inval_inode_out{pub ino:u64,pub off:i64,pub len:i64}
#[repr(C)] #[derive(Copy,Clone)] pub struct fuse_notify_inval_entry_out{pub parent:u64,pub namelen:u32,pub flags:u32}
#[repr(C)] #[derive(Copy,Clone)] pub struct fuse_notify_delete_out{pub parent:u64,pub child:u64,pub namelen:u32,pub padding:u32}
#[repr(C)] #[derive(Copy,Clone)] pub struct fuse_notify_store_out{pub nodeid:u64,pub offset:u64,pub size:u32,pub padding:u32}
#[repr(C)] #[derive(Copy,Clone)] pub struct fuse_notify_retrieve_out{pub notify_unique:u64,pub nodeid:u64,pub offset:u64,pub size:u32,pub padding:u32}
#[repr(C)] #[derive(Copy,Clone)] pub struct fuse_notify_retrieve_in{pub dummy1:u64,pub offset:u64,pub size:u32,pub dummy2:u32,pub dummy3:u64,pub dummy4:u64}
#[repr(C)] #[derive(Copy,Clone)] pub struct fuse_notify_prune_out{pub count:u32,pub padding:u32,pub spare:u64}
#[repr(C)] #[derive(Copy,Clone)] pub struct fuse_backing_map{pub fd:i32,pub flags:u32,pub padding:u64}
#[repr(C)] #[derive(Copy,Clone)] pub struct fuse_lseek_in{pub fh:u64,pub offset:u64,pub whence:u32,pub padding:u32}
#[repr(C)] #[derive(Copy,Clone)] pub struct fuse_lseek_out{pub offset:u64}
#[repr(C)] #[derive(Copy,Clone)] pub struct fuse_copy_file_range_in{pub fh_in:u64,pub off_in:u64,pub nodeid_out:u64,pub fh_out:u64,pub off_out:u64,pub len:u64,pub flags:u64}
#[repr(C)] #[derive(Copy,Clone)] pub struct fuse_copy_file_range_out{pub bytes_copied:u64}
pub const FUSE_SETUPMAPPING_FLAG_WRITE:u64=1; pub const FUSE_SETUPMAPPING_FLAG_READ:u64=2;
#[repr(C)] #[derive(Copy,Clone)] pub struct fuse_setupmapping_in{pub fh:u64,pub foffset:u64,pub len:u64,pub flags:u64,pub moffset:u64}
#[repr(C)] #[derive(Copy,Clone)] pub struct fuse_removemapping_in{pub count:u32}
#[repr(C)] #[derive(Copy,Clone)] pub struct fuse_removemapping_one{pub moffset:u64,pub len:u64}
#[repr(C)] #[derive(Copy,Clone)] pub struct fuse_syncfs_in{pub padding:u64}
#[repr(C)] #[derive(Copy,Clone)] pub struct fuse_secctx{pub size:u32,pub padding:u32}
#[repr(C)] #[derive(Copy,Clone)] pub struct fuse_secctx_header{pub size:u32,pub nr_secctx:u32}
#[repr(C)] #[derive(Copy,Clone)] pub struct fuse_ext_header{pub size:u32,pub r#type:u32}
#[repr(C)] pub struct fuse_supp_groups{pub nr_groups:u32,pub groups:[u32;0]}
pub const FUSE_URING_IN_OUT_HEADER_SZ:usize=128; pub const FUSE_URING_OP_IN_OUT_SZ:usize=128; pub const FUSE_URING_ENT_ZERO_COPY:u64=1;
#[repr(C)] #[derive(Copy,Clone)] pub struct fuse_uring_ent_in_out{pub flags:u64,pub commit_id:u64,pub payload_sz:u32,pub offset:u32,pub reserved:u64}
#[repr(C)] #[derive(Copy,Clone)] pub struct fuse_uring_req_header{pub in_out:[u8;128],pub op_in:[u8;128],pub ring_ent_in_out:fuse_uring_ent_in_out}
#[repr(C)] #[derive(Copy,Clone)] pub struct fuse_uring_bufpool{pub uaddr:u64,pub len:u32,pub reserved:u32}
#[repr(C)] #[derive(Copy,Clone)] pub union fuse_uring_cmd_req_union{pub bufpool:fuse_uring_bufpool,pub ent_zero_copy_buf_index:u16}
#[repr(C)] #[derive(Copy,Clone)] pub struct fuse_uring_cmd_req{pub flags:u64,pub commit_id:u64,pub qid:u16,pub padding:[u8;6],pub data:fuse_uring_cmd_req_union}
pub const FUSE_URING_ZERO_COPY:u64=1;
#[repr(i32)] pub enum fuse_ext_type{FUSE_MAX_NR_SECCTX=31,FUSE_EXT_GROUPS=32}
#[repr(i32)] pub enum fuse_notify_code{FUSE_NOTIFY_POLL=1,FUSE_NOTIFY_INVAL_INODE=2,FUSE_NOTIFY_INVAL_ENTRY=3,FUSE_NOTIFY_STORE=4,FUSE_NOTIFY_RETRIEVE=5,FUSE_NOTIFY_DELETE=6,FUSE_NOTIFY_RESEND=7,FUSE_NOTIFY_INC_EPOCH=8,FUSE_NOTIFY_PRUNE=9}
#[repr(i32)] pub enum fuse_uring_cmd{FUSE_IO_URING_CMD_INVALID=0,FUSE_IO_URING_CMD_REGISTER=1,FUSE_IO_URING_CMD_COMMIT_AND_FETCH=2,FUSE_IO_URING_CMD_ADD_QUEUE=3,FUSE_IO_URING_CMD_ADD_BUFPOOL=4}
pub const FUSE_MIN_READ_BUFFER:u32=8192; pub const FUSE_COMPAT_ENTRY_OUT_SIZE:u32=120; pub const FUSE_COMPAT_ATTR_OUT_SIZE:u32=96; pub const FUSE_COMPAT_MKNOD_IN_SIZE:u32=8; pub const FUSE_COMPAT_WRITE_IN_SIZE:u32=24; pub const FUSE_COMPAT_STATFS_SIZE:u32=48; pub const FUSE_COMPAT_SETXATTR_IN_SIZE:u32=8; pub const FUSE_COMPAT_INIT_OUT_SIZE:u32=8; pub const FUSE_COMPAT_22_INIT_OUT_SIZE:u32=24; pub const CUSE_INIT_INFO_MAX:u32=4096; pub const FUSE_MAX_NR_SECCTX:u32=31;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
