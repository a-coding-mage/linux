/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* Translated from uapi/linux/fs.h. */

pub const INR_OPEN_CUR: u32 = 1024;
pub const INR_OPEN_MAX: u32 = 4096;
pub const BLOCK_SIZE_BITS: u32 = 10;
pub const BLOCK_SIZE: u32 = 1 << BLOCK_SIZE_BITS;

pub const IO_INTEGRITY_CHK_GUARD: u32 = 1 << 0;
pub const IO_INTEGRITY_CHK_REFTAG: u32 = 1 << 1;
pub const IO_INTEGRITY_CHK_APPTAG: u32 = 1 << 2;
pub const IO_INTEGRITY_VALID_FLAGS: u32 = IO_INTEGRITY_CHK_GUARD | IO_INTEGRITY_CHK_REFTAG | IO_INTEGRITY_CHK_APPTAG;
pub const SEEK_SET: i32 = 0;
pub const SEEK_CUR: i32 = 1;
pub const SEEK_END: i32 = 2;
pub const SEEK_DATA: i32 = 3;
pub const SEEK_HOLE: i32 = 4;
pub const SEEK_MAX: i32 = SEEK_HOLE;
pub const RENAME_NOREPLACE: i32 = 1 << 0;
pub const RENAME_EXCHANGE: i32 = 1 << 1;
pub const RENAME_WHITEOUT: i32 = 1 << 2;

#[repr(i32)]
pub enum procfs_ino { PROCFS_ROOT_INO = 1 }

#[repr(C)]
pub struct file_clone_range { pub src_fd: __s64, pub src_offset: __u64, pub src_length: __u64, pub dest_offset: __u64 }
#[repr(C)]
pub struct fstrim_range { pub start: __u64, pub len: __u64, pub minlen: __u64 }
#[repr(C)]
pub struct fsuuid2 { pub len: __u8, pub uuid: [__u8; 16] }
#[repr(C)]
pub struct fs_sysfs_path { pub len: __u8, pub name: [__u8; 128] }

pub const LBMD_PI_CAP_INTEGRITY: u32 = 1 << 0;
pub const LBMD_PI_CAP_REFTAG: u32 = 1 << 1;
pub const LBMD_PI_CSUM_NONE: u32 = 0;
pub const LBMD_PI_CSUM_IP: u32 = 1;
pub const LBMD_PI_CSUM_CRC16_T10DIF: u32 = 2;
pub const LBMD_PI_CSUM_CRC64_NVME: u32 = 4;
pub const LBMD_SIZE_VER0: u32 = 16;
#[repr(C)]
pub struct logical_block_metadata_cap {
    pub lbmd_flags: __u32, pub lbmd_interval: __u16, pub lbmd_size: __u8,
    pub lbmd_opaque_size: __u8, pub lbmd_opaque_offset: __u8, pub lbmd_pi_size: __u8,
    pub lbmd_pi_offset: __u8, pub lbmd_guard_tag_type: __u8, pub lbmd_app_tag_size: __u8,
    pub lbmd_ref_tag_size: __u8, pub lbmd_storage_tag_size: __u8, pub pad: __u8,
}
pub const FILE_DEDUPE_RANGE_SAME: i32 = 0;
pub const FILE_DEDUPE_RANGE_DIFFERS: i32 = 1;
#[repr(C)]
pub struct file_dedupe_range_info { pub dest_fd: __s64, pub dest_offset: __u64, pub bytes_deduped: __u64, pub status: __s32, pub reserved: __u32 }
#[repr(C)]
pub struct file_dedupe_range { pub src_offset: __u64, pub src_length: __u64, pub dest_count: __u16, pub reserved1: __u16, pub reserved2: __u32, pub info: [file_dedupe_range_info; 0] }
#[repr(C)]
pub struct files_stat_struct { pub nr_files: ::core::ffi::c_ulong, pub nr_free_files: ::core::ffi::c_ulong, pub max_files: ::core::ffi::c_ulong }
#[repr(C)]
pub struct inodes_stat_t { pub nr_inodes: ::core::ffi::c_long, pub nr_unused: ::core::ffi::c_long, pub dummy: [::core::ffi::c_long; 5] }
pub const NR_FILE: u32 = 8192;
#[repr(C)]
pub struct fsxattr { pub fsx_xflags: __u32, pub fsx_extsize: __u32, pub fsx_nextents: __u32, pub fsx_projid: __u32, pub fsx_cowextsize: __u32, pub fsx_pad: [u8; 8] }
#[repr(C)]
pub struct file_attr { pub fa_xflags: __u64, pub fa_extsize: __u32, pub fa_nextents: __u32, pub fa_projid: __u32, pub fa_cowextsize: __u32 }
pub const FILE_ATTR_SIZE_VER0: u32 = 24;
pub const FILE_ATTR_SIZE_LATEST: u32 = FILE_ATTR_SIZE_VER0;

pub const FS_XFLAG_REALTIME: u32 = 0x00000001; pub const FS_XFLAG_PREALLOC: u32 = 0x00000002; pub const FS_XFLAG_IMMUTABLE: u32 = 0x00000008; pub const FS_XFLAG_APPEND: u32 = 0x10; pub const FS_XFLAG_SYNC: u32 = 0x20; pub const FS_XFLAG_NOATIME: u32 = 0x40; pub const FS_XFLAG_NODUMP: u32 = 0x80; pub const FS_XFLAG_RTINHERIT: u32 = 0x100; pub const FS_XFLAG_PROJINHERIT: u32 = 0x200; pub const FS_XFLAG_NOSYMLINKS: u32 = 0x400; pub const FS_XFLAG_EXTSIZE: u32 = 0x800; pub const FS_XFLAG_EXTSZINHERIT: u32 = 0x1000; pub const FS_XFLAG_NODEFRAG: u32 = 0x2000; pub const FS_XFLAG_FILESTREAM: u32 = 0x4000; pub const FS_XFLAG_DAX: u32 = 0x8000; pub const FS_XFLAG_COWEXTSIZE: u32 = 0x10000; pub const FS_XFLAG_VERITY: u32 = 0x20000; pub const FS_XFLAG_CASEFOLD: u32 = 0x40000; pub const FS_XFLAG_CASENONPRESERVING: u32 = 0x80000; pub const FS_XFLAG_HASATTR: u32 = 0x80000000;

pub const FSLABEL_MAX: usize = 256;
/* ioctl values retain their dependency on the external _IO/_IOR/_IOW/_IOWR definitions. */
pub const BLKROSET: usize = _IO(0x12, 93); pub const BLKROGET: usize = _IO(0x12, 94); pub const BLKRRPART: usize = _IO(0x12, 95); pub const BLKGETSIZE: usize = _IO(0x12, 96); pub const BLKFLSBUF: usize = _IO(0x12, 97); pub const BLKRASET: usize = _IO(0x12, 98); pub const BLKRAGET: usize = _IO(0x12, 99); pub const BLKFRASET: usize = _IO(0x12, 100); pub const BLKFRAGET: usize = _IO(0x12, 101); pub const BLKSECTSET: usize = _IO(0x12, 102); pub const BLKSECTGET: usize = _IO(0x12, 103); pub const BLKSSZGET: usize = _IO(0x12, 104);
pub const BLKBSZGET: usize = _IOR(0x12, 112, usize); pub const BLKBSZSET: usize = _IOW(0x12, 113, usize); pub const BLKGETSIZE64: usize = _IOR(0x12, 114, usize); pub const BLKTRACESETUP: usize = _IOWR(0x12, 115, blk_user_trace_setup); pub const BLKTRACESTART: usize = _IO(0x12, 116); pub const BLKTRACESTOP: usize = _IO(0x12, 117); pub const BLKTRACETEARDOWN: usize = _IO(0x12, 118); pub const BLKDISCARD: usize = _IO(0x12, 119); pub const BLKIOMIN: usize = _IO(0x12, 120); pub const BLKIOOPT: usize = _IO(0x12, 121); pub const BLKALIGNOFF: usize = _IO(0x12, 122); pub const BLKPBSZGET: usize = _IO(0x12, 123); pub const BLKDISCARDZEROES: usize = _IO(0x12, 124); pub const BLKSECDISCARD: usize = _IO(0x12, 125); pub const BLKROTATIONAL: usize = _IO(0x12, 126); pub const BLKZEROOUT: usize = _IO(0x12, 127); pub const BLKGETDISKSEQ: usize = _IOR(0x12, 128, __u64); pub const BLKTRACESETUP2: usize = _IOWR(0x12, 142, blk_user_trace_setup2);
pub const BMAP_IOCTL: i32 = 1; pub const FIBMAP: usize = _IO(0, 1); pub const FIGETBSZ: usize = _IO(0, 2); pub const FIFREEZE: usize = _IOWR(b'X', 119, i32); pub const FITHAW: usize = _IOWR(b'X', 120, i32); pub const FITRIM: usize = _IOWR(b'X', 121, fstrim_range); pub const FICLONE: usize = _IOW(0x94, 9, i32); pub const FICLONERANGE: usize = _IOW(0x94, 13, file_clone_range); pub const FIDEDUPERANGE: usize = _IOWR(0x94, 54, file_dedupe_range);
pub const FS_IOC_GETFLAGS: usize = _IOR(b'f', 1, ::core::ffi::c_long); pub const FS_IOC_SETFLAGS: usize = _IOW(b'f', 2, ::core::ffi::c_long); pub const FS_IOC_GETVERSION: usize = _IOR(b'v', 1, ::core::ffi::c_long); pub const FS_IOC_SETVERSION: usize = _IOW(b'v', 2, ::core::ffi::c_long); pub const FS_IOC_FIEMAP: usize = _IOWR(b'f', 11, fiemap); pub const FS_IOC_FSGETXATTR: usize = _IOR(b'X', 31, fsxattr); pub const FS_IOC_FSSETXATTR: usize = _IOW(b'X', 32, fsxattr); pub const FS_IOC_GETFSUUID: usize = _IOR(0x15, 0, fsuuid2); pub const FS_IOC_GETFSSYSFSPATH: usize = _IOR(0x15, 1, fs_sysfs_path); pub const FS_IOC_GETLBMD_CAP: usize = _IOWR(0x15, 2, logical_block_metadata_cap);
pub const FS_SECRM_FL: u32=0x1; pub const FS_UNRM_FL:u32=0x2; pub const FS_COMPR_FL:u32=0x4; pub const FS_SYNC_FL:u32=0x8; pub const FS_IMMUTABLE_FL:u32=0x10; pub const FS_APPEND_FL:u32=0x20; pub const FS_NODUMP_FL:u32=0x40; pub const FS_NOATIME_FL:u32=0x80; pub const FS_DIRTY_FL:u32=0x100; pub const FS_COMPRBLK_FL:u32=0x200; pub const FS_NOCOMP_FL:u32=0x400; pub const FS_ENCRYPT_FL:u32=0x800; pub const FS_BTREE_FL:u32=0x1000; pub const FS_INDEX_FL:u32=0x1000; pub const FS_IMAGIC_FL:u32=0x2000; pub const FS_JOURNAL_DATA_FL:u32=0x4000; pub const FS_NOTAIL_FL:u32=0x8000; pub const FS_DIRSYNC_FL:u32=0x10000; pub const FS_TOPDIR_FL:u32=0x20000; pub const FS_HUGE_FILE_FL:u32=0x40000; pub const FS_EXTENT_FL:u32=0x80000; pub const FS_VERITY_FL:u32=0x100000; pub const FS_EA_INODE_FL:u32=0x200000; pub const FS_EOFBLOCKS_FL:u32=0x400000; pub const FS_NOCOW_FL:u32=0x800000; pub const FS_DAX_FL:u32=0x2000000; pub const FS_INLINE_DATA_FL:u32=0x10000000; pub const FS_PROJINHERIT_FL:u32=0x20000000; pub const FS_CASEFOLD_FL:u32=0x40000000; pub const FS_RESERVED_FL:u32=0x80000000;
pub const FS_FL_USER_VISIBLE:u32=0x0003DFFF; pub const FS_FL_USER_MODIFIABLE:u32=0x000380FF;

pub const SYNC_FILE_RANGE_WAIT_BEFORE:u32=1; pub const SYNC_FILE_RANGE_WRITE:u32=2; pub const SYNC_FILE_RANGE_WAIT_AFTER:u32=4; pub const SYNC_FILE_RANGE_WRITE_AND_WAIT:u32=7;
pub type __kernel_rwf_t = i32;
pub const RWF_HIPRI: __kernel_rwf_t=1; pub const RWF_DSYNC:__kernel_rwf_t=2; pub const RWF_SYNC:__kernel_rwf_t=4; pub const RWF_NOWAIT:__kernel_rwf_t=8; pub const RWF_APPEND:__kernel_rwf_t=0x10; pub const RWF_NOAPPEND:__kernel_rwf_t=0x20; pub const RWF_ATOMIC:__kernel_rwf_t=0x40; pub const RWF_DONTCACHE:__kernel_rwf_t=0x80; pub const RWF_NOSIGNAL:__kernel_rwf_t=0x100; pub const RWF_SUPPORTED:__kernel_rwf_t=0x1ff;
pub const PROCFS_IOCTL_MAGIC: u8 = b'f';
#[repr(C)] pub struct page_region { pub start: __u64, pub end: __u64, pub categories: __u64 }
pub const PAGE_IS_WPALLOWED:u32=1<<0; pub const PAGE_IS_WRITTEN:u32=1<<1; pub const PAGE_IS_FILE:u32=1<<2; pub const PAGE_IS_PRESENT:u32=1<<3; pub const PAGE_IS_SWAPPED:u32=1<<4; pub const PAGE_IS_PFNZERO:u32=1<<5; pub const PAGE_IS_HUGE:u32=1<<6; pub const PAGE_IS_SOFT_DIRTY:u32=1<<7; pub const PAGE_IS_GUARD:u32=1<<8; pub const PAGE_IS_ACCESSED:u32=1<<9;
pub const PM_SCAN_WP_MATCHING:u32=1; pub const PM_SCAN_CHECK_WPASYNC:u32=2;
pub const PAGEMAP_SCAN: usize = _IOWR(PROCFS_IOCTL_MAGIC, 16, pm_scan_arg);
#[repr(C)] pub struct pm_scan_arg { pub size:__u64,pub flags:__u64,pub start:__u64,pub end:__u64,pub walk_end:__u64,pub vec:__u64,pub vec_len:__u64,pub max_pages:__u64,pub category_inverted:__u64,pub category_mask:__u64,pub category_anyof_mask:__u64,pub return_mask:__u64 }
#[repr(i32)] pub enum procmap_query_flags { PROCMAP_QUERY_VMA_READABLE=1, PROCMAP_QUERY_VMA_WRITABLE=2, PROCMAP_QUERY_VMA_EXECUTABLE=4, PROCMAP_QUERY_VMA_SHARED=8, PROCMAP_QUERY_COVERING_OR_NEXT_VMA=0x10, PROCMAP_QUERY_FILE_BACKED_VMA=0x20 }
#[repr(C)] pub struct procmap_query { pub size:__u64,pub query_flags:__u64,pub query_addr:__u64,pub vma_start:__u64,pub vma_end:__u64,pub vma_flags:__u64,pub vma_page_size:__u64,pub vma_offset:__u64,pub inode:__u64,pub dev_major:__u32,pub dev_minor:__u32,pub vma_name_size:__u32,pub build_id_size:__u32,pub vma_name_addr:__u64,pub build_id_addr:__u64 }
pub const PROCMAP_QUERY: usize = _IOWR(PROCFS_IOCTL_MAGIC, 17, procmap_query);
pub const FS_IOC_SHUTDOWN: usize = _IOR(b'X', 125, __u32);
pub const FS_SHUTDOWN_FLAGS_DEFAULT:u32=0; pub const FS_SHUTDOWN_FLAGS_LOGFLUSH:u32=1; pub const FS_SHUTDOWN_FLAGS_NOLOGFLUSH:u32=2;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
