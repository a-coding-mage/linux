/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/*
 * This file has definitions for some important file table structures
 * and constants and structures used by various generic file system
 * ioctl's.  Please do not make any changes in this file before
 * sending patches for review to linux-fsdevel@vger.kernel.org and
 * linux-api@vger.kernel.org.
 */

/* Dependencies from C includes: linux/limits.h, linux/ioctl.h, linux/types.h.
 * For non-kernel builds: linux/fscrypt.h and linux/mount.h.
 */

/*
 * It's silly to have NR_OPEN bigger than NR_FILE, but you can change
 * the file limit at runtime and only root can increase the per-process
 * nr_file rlimit, so it's safe to set up a ridiculously high absolute
 * upper limit on files-per-process.
 *
 * Some programs (notably those using select()) may have to be
 * recompiled to take full advantage of the new limits..
 */

/* Fixed constants first: */
pub const INR_OPEN_CUR: u32 = 1024; /* Initial setting for nfile rlimits */
pub const INR_OPEN_MAX: u32 = 4096; /* Hard limit for nfile rlimits */

pub const BLOCK_SIZE_BITS: u32 = 10;
pub const BLOCK_SIZE: u32 = 1 << BLOCK_SIZE_BITS;

/* flags for integrity meta */
pub const IO_INTEGRITY_CHK_GUARD: u32 = 1u32 << 0; /* enforce guard check */
pub const IO_INTEGRITY_CHK_REFTAG: u32 = 1u32 << 1; /* enforce ref check */
pub const IO_INTEGRITY_CHK_APPTAG: u32 = 1u32 << 2; /* enforce app check */

pub const IO_INTEGRITY_VALID_FLAGS: u32 =
    IO_INTEGRITY_CHK_GUARD | IO_INTEGRITY_CHK_REFTAG | IO_INTEGRITY_CHK_APPTAG;

pub const SEEK_SET: u32 = 0; /* seek relative to beginning of file */
pub const SEEK_CUR: u32 = 1; /* seek relative to current file position */
pub const SEEK_END: u32 = 2; /* seek relative to end of file */
pub const SEEK_DATA: u32 = 3; /* seek to the next data */
pub const SEEK_HOLE: u32 = 4; /* seek to the next hole */
pub const SEEK_MAX: u32 = SEEK_HOLE;

pub const RENAME_NOREPLACE: u32 = 1 << 0; /* Don't overwrite target */
pub const RENAME_EXCHANGE: u32 = 1 << 1; /* Exchange source and dest */
pub const RENAME_WHITEOUT: u32 = 1 << 2; /* Whiteout source */

#[repr(C)]
pub struct file_clone_range {
    pub src_fd: __s64,
    pub src_offset: __u64,
    pub src_length: __u64,
    pub dest_offset: __u64,
}

#[repr(C)]
pub struct fstrim_range {
    pub start: __u64,
    pub len: __u64,
    pub minlen: __u64,
}

/*
 * We include a length field because some filesystems (vfat) have an identifier
 * that we do want to expose as a UUID, but doesn't have the standard length.
 *
 * We use a fixed size buffer beacuse this interface will, by fiat, never
 * support "UUIDs" longer than 16 bytes; we don't want to force all downstream
 * users to have to deal with that.
 */
#[repr(C)]
pub struct fsuuid2 {
    pub len: __u8,
    pub uuid: [__u8; 16],
}

#[repr(C)]
pub struct fs_sysfs_path {
    pub len: __u8,
    pub name: [__u8; 128],
}

/* extent-same (dedupe) ioctls; these MUST match the btrfs ioctl definitions */
pub const FILE_DEDUPE_RANGE_SAME: u32 = 0;
pub const FILE_DEDUPE_RANGE_DIFFERS: u32 = 1;

/* from struct btrfs_ioctl_file_extent_same_info */
#[repr(C)]
pub struct file_dedupe_range_info {
    pub dest_fd: __s64,       /* in - destination file */
    pub dest_offset: __u64,   /* in - start of extent in destination */
    pub bytes_deduped: __u64, /* out - total # of bytes we were able
                               * to dedupe from this file. */
    /* status of this dedupe operation:
     * < 0 for error
     * == FILE_DEDUPE_RANGE_SAME if dedupe succeeds
     * == FILE_DEDUPE_RANGE_DIFFERS if data differs
     */
    pub status: __s32,   /* out - see above description */
    pub reserved: __u32, /* must be zero */
}

/* from struct btrfs_ioctl_file_extent_same_args */
#[repr(C)]
pub struct file_dedupe_range {
    pub src_offset: __u64, /* in - start of extent in source */
    pub src_length: __u64, /* in - length of extent */
    pub dest_count: __u16, /* in - total elements in info array */
    pub reserved1: __u16,  /* must be zero */
    pub reserved2: __u32,  /* must be zero */
    pub info: [file_dedupe_range_info; 0],
}

/* And dynamically-tunable limits and defaults: */
#[repr(C)]
pub struct files_stat_struct {
    pub nr_files: ::core::ffi::c_ulong,      /* read only */
    pub nr_free_files: ::core::ffi::c_ulong, /* read only */
    pub max_files: ::core::ffi::c_ulong,     /* tunable */
}

#[repr(C)]
pub struct inodes_stat_t {
    pub nr_inodes: ::core::ffi::c_long,
    pub nr_unused: ::core::ffi::c_long,
    pub dummy: [::core::ffi::c_long; 5], /* padding for sysctl ABI compatibility */
}

pub const NR_FILE: u32 = 8192; /* this can well be larger on a larger system */

/*
 * Structure for FS_IOC_FSGETXATTR[A] and FS_IOC_FSSETXATTR.
 */
#[repr(C)]
pub struct fsxattr {
    pub fsx_xflags: __u32,    /* xflags field value (get/set) */
    pub fsx_extsize: __u32,   /* extsize field value (get/set)*/
    pub fsx_nextents: __u32,  /* nextents field value (get) */
    pub fsx_projid: __u32,    /* project identifier (get/set) */
    pub fsx_cowextsize: __u32, /* CoW extsize field value (get/set)*/
    pub fsx_pad: [::core::ffi::c_uchar; 8],
}

/*
 * Flags for the fsx_xflags field
 */
pub const FS_XFLAG_REALTIME: u32 = 0x00000001; /* data in realtime volume */
pub const FS_XFLAG_PREALLOC: u32 = 0x00000002; /* preallocated file extents */
pub const FS_XFLAG_IMMUTABLE: u32 = 0x00000008; /* file cannot be modified */
pub const FS_XFLAG_APPEND: u32 = 0x00000010; /* all writes append */
pub const FS_XFLAG_SYNC: u32 = 0x00000020; /* all writes synchronous */
pub const FS_XFLAG_NOATIME: u32 = 0x00000040; /* do not update access time */
pub const FS_XFLAG_NODUMP: u32 = 0x00000080; /* do not include in backups */
pub const FS_XFLAG_RTINHERIT: u32 = 0x00000100; /* create with rt bit set */
pub const FS_XFLAG_PROJINHERIT: u32 = 0x00000200; /* create with parents projid */
pub const FS_XFLAG_NOSYMLINKS: u32 = 0x00000400; /* disallow symlink creation */
pub const FS_XFLAG_EXTSIZE: u32 = 0x00000800; /* extent size allocator hint */
pub const FS_XFLAG_EXTSZINHERIT: u32 = 0x00001000; /* inherit inode extent size */
pub const FS_XFLAG_NODEFRAG: u32 = 0x00002000; /* do not defragment */
pub const FS_XFLAG_FILESTREAM: u32 = 0x00004000; /* use filestream allocator */
pub const FS_XFLAG_DAX: u32 = 0x00008000; /* use DAX for IO */
pub const FS_XFLAG_COWEXTSIZE: u32 = 0x00010000; /* CoW extent size allocator hint */
pub const FS_XFLAG_HASATTR: u32 = 0x80000000; /* no DIFLAG for this */

/* the read-only stuff doesn't really belong here, but any other place is
   probably as bad and I don't want to create yet another include file. */

pub const BLKROSET: ::core::ffi::c_ulong = _IO(0x12, 93); /* set device read-only (0 = read-write) */
pub const BLKROGET: ::core::ffi::c_ulong = _IO(0x12, 94); /* get read-only status (0 = read_write) */
pub const BLKRRPART: ::core::ffi::c_ulong = _IO(0x12, 95); /* re-read partition table */
pub const BLKGETSIZE: ::core::ffi::c_ulong = _IO(0x12, 96); /* return device size /512 (long *arg) */
pub const BLKFLSBUF: ::core::ffi::c_ulong = _IO(0x12, 97); /* flush buffer cache */
pub const BLKRASET: ::core::ffi::c_ulong = _IO(0x12, 98); /* set read ahead for block device */
pub const BLKRAGET: ::core::ffi::c_ulong = _IO(0x12, 99); /* get current read ahead setting */
pub const BLKFRASET: ::core::ffi::c_ulong = _IO(0x12, 100); /* set filesystem (mm/filemap.c) read-ahead */
pub const BLKFRAGET: ::core::ffi::c_ulong = _IO(0x12, 101); /* get filesystem (mm/filemap.c) read-ahead */
pub const BLKSECTSET: ::core::ffi::c_ulong = _IO(0x12, 102); /* set max sectors per request (ll_rw_blk.c) */
pub const BLKSECTGET: ::core::ffi::c_ulong = _IO(0x12, 103); /* get max sectors per request (ll_rw_blk.c) */
pub const BLKSSZGET: ::core::ffi::c_ulong = _IO(0x12, 104); /* get block device sector size */
/* Original C has a disabled #if 0 block for BLKPG, BLKELVGET, and BLKELVSET. */
/* A jump here: 108-111 have been used for various private purposes. */
pub const BLKBSZGET: ::core::ffi::c_ulong = _IOR::<usize>(0x12, 112);
pub const BLKBSZSET: ::core::ffi::c_ulong = _IOW::<usize>(0x12, 113);
pub const BLKGETSIZE64: ::core::ffi::c_ulong = _IOR::<usize>(0x12, 114); /* return device size in bytes (u64 *arg) */
pub const BLKTRACESETUP: ::core::ffi::c_ulong = _IOWR::<blk_user_trace_setup>(0x12, 115);
pub const BLKTRACESTART: ::core::ffi::c_ulong = _IO(0x12, 116);
pub const BLKTRACESTOP: ::core::ffi::c_ulong = _IO(0x12, 117);
pub const BLKTRACETEARDOWN: ::core::ffi::c_ulong = _IO(0x12, 118);
pub const BLKDISCARD: ::core::ffi::c_ulong = _IO(0x12, 119);
pub const BLKIOMIN: ::core::ffi::c_ulong = _IO(0x12, 120);
pub const BLKIOOPT: ::core::ffi::c_ulong = _IO(0x12, 121);
pub const BLKALIGNOFF: ::core::ffi::c_ulong = _IO(0x12, 122);
pub const BLKPBSZGET: ::core::ffi::c_ulong = _IO(0x12, 123);
pub const BLKDISCARDZEROES: ::core::ffi::c_ulong = _IO(0x12, 124);
pub const BLKSECDISCARD: ::core::ffi::c_ulong = _IO(0x12, 125);
pub const BLKROTATIONAL: ::core::ffi::c_ulong = _IO(0x12, 126);
pub const BLKZEROOUT: ::core::ffi::c_ulong = _IO(0x12, 127);
pub const BLKGETDISKSEQ: ::core::ffi::c_ulong = _IOR::<__u64>(0x12, 128);
/*
 * A jump here: 130-136 are reserved for zoned block devices
 * (see uapi/linux/blkzoned.h)
 */

pub const BMAP_IOCTL: u32 = 1; /* obsolete - kept for compatibility */
pub const FIBMAP: ::core::ffi::c_ulong = _IO(0x00, 1); /* bmap access */
pub const FIGETBSZ: ::core::ffi::c_ulong = _IO(0x00, 2); /* get the block size used for bmap */
pub const FIFREEZE: ::core::ffi::c_ulong = _IOWR::<::core::ffi::c_int>(b'X', 119); /* Freeze */
pub const FITHAW: ::core::ffi::c_ulong = _IOWR::<::core::ffi::c_int>(b'X', 120); /* Thaw */
pub const FITRIM: ::core::ffi::c_ulong = _IOWR::<fstrim_range>(b'X', 121); /* Trim */
pub const FICLONE: ::core::ffi::c_ulong = _IOW::<::core::ffi::c_int>(0x94, 9);
pub const FICLONERANGE: ::core::ffi::c_ulong = _IOW::<file_clone_range>(0x94, 13);
pub const FIDEDUPERANGE: ::core::ffi::c_ulong = _IOWR::<file_dedupe_range>(0x94, 54);

pub const FSLABEL_MAX: usize = 256; /* Max chars for the interface; each fs may differ */

pub const FS_IOC_GETFLAGS: ::core::ffi::c_ulong = _IOR::<::core::ffi::c_long>(b'f', 1);
pub const FS_IOC_SETFLAGS: ::core::ffi::c_ulong = _IOW::<::core::ffi::c_long>(b'f', 2);
pub const FS_IOC_GETVERSION: ::core::ffi::c_ulong = _IOR::<::core::ffi::c_long>(b'v', 1);
pub const FS_IOC_SETVERSION: ::core::ffi::c_ulong = _IOW::<::core::ffi::c_long>(b'v', 2);
pub const FS_IOC_FIEMAP: ::core::ffi::c_ulong = _IOWR::<fiemap>(b'f', 11);
pub const FS_IOC32_GETFLAGS: ::core::ffi::c_ulong = _IOR::<::core::ffi::c_int>(b'f', 1);
pub const FS_IOC32_SETFLAGS: ::core::ffi::c_ulong = _IOW::<::core::ffi::c_int>(b'f', 2);
pub const FS_IOC32_GETVERSION: ::core::ffi::c_ulong = _IOR::<::core::ffi::c_int>(b'v', 1);
pub const FS_IOC32_SETVERSION: ::core::ffi::c_ulong = _IOW::<::core::ffi::c_int>(b'v', 2);
pub const FS_IOC_FSGETXATTR: ::core::ffi::c_ulong = _IOR::<fsxattr>(b'X', 31);
pub const FS_IOC_FSSETXATTR: ::core::ffi::c_ulong = _IOW::<fsxattr>(b'X', 32);
pub const FS_IOC_GETFSLABEL: ::core::ffi::c_ulong = _IOR::<[::core::ffi::c_char; FSLABEL_MAX]>(0x94, 49);
pub const FS_IOC_SETFSLABEL: ::core::ffi::c_ulong = _IOW::<[::core::ffi::c_char; FSLABEL_MAX]>(0x94, 50);
/* Returns the external filesystem UUID, the same one blkid returns */
pub const FS_IOC_GETFSUUID: ::core::ffi::c_ulong = _IOR::<fsuuid2>(0x15, 0);
/*
 * Returns the path component under /sys/fs/ that refers to this filesystem;
 * also /sys/kernel/debug/ for filesystems with debugfs exports
 */
pub const FS_IOC_GETFSSYSFSPATH: ::core::ffi::c_ulong = _IOR::<fs_sysfs_path>(0x15, 1);

/*
 * Inode flags (FS_IOC_GETFLAGS / FS_IOC_SETFLAGS)
 *
 * Note: for historical reasons, these flags were originally used and
 * defined for use by ext2/ext3, and then other file systems started
 * using these flags so they wouldn't need to write their own version
 * of chattr/lsattr (which was shipped as part of e2fsprogs).  You
 * should think twice before trying to use these flags in new
 * contexts, or trying to assign these flags, since they are used both
 * as the UAPI and the on-disk encoding for ext2/3/4.  Also, we are
 * almost out of 32-bit flags.  :-)
 *
 * We have recently hoisted FS_IOC_FSGETXATTR / FS_IOC_FSSETXATTR from
 * XFS to the generic FS level interface.  This uses a structure that
 * has padding and hence has more room to grow, so it may be more
 * appropriate for many new use cases.
 *
 * Please do not change these flags or interfaces before checking with
 * linux-fsdevel@vger.kernel.org and linux-api@vger.kernel.org.
 */
pub const FS_SECRM_FL: u32 = 0x00000001; /* Secure deletion */
pub const FS_UNRM_FL: u32 = 0x00000002; /* Undelete */
pub const FS_COMPR_FL: u32 = 0x00000004; /* Compress file */
pub const FS_SYNC_FL: u32 = 0x00000008; /* Synchronous updates */
pub const FS_IMMUTABLE_FL: u32 = 0x00000010; /* Immutable file */
pub const FS_APPEND_FL: u32 = 0x00000020; /* writes to file may only append */
pub const FS_NODUMP_FL: u32 = 0x00000040; /* do not dump file */
pub const FS_NOATIME_FL: u32 = 0x00000080; /* do not update atime */
/* Reserved for compression usage... */
pub const FS_DIRTY_FL: u32 = 0x00000100;
pub const FS_COMPRBLK_FL: u32 = 0x00000200; /* One or more compressed clusters */
pub const FS_NOCOMP_FL: u32 = 0x00000400; /* Don't compress */
/* End compression flags --- maybe not all used */
pub const FS_ENCRYPT_FL: u32 = 0x00000800; /* Encrypted file */
pub const FS_BTREE_FL: u32 = 0x00001000; /* btree format dir */
pub const FS_INDEX_FL: u32 = 0x00001000; /* hash-indexed directory */
pub const FS_IMAGIC_FL: u32 = 0x00002000; /* AFS directory */
pub const FS_JOURNAL_DATA_FL: u32 = 0x00004000; /* Reserved for ext3 */
pub const FS_NOTAIL_FL: u32 = 0x00008000; /* file tail should not be merged */
pub const FS_DIRSYNC_FL: u32 = 0x00010000; /* dirsync behaviour (directories only) */
pub const FS_TOPDIR_FL: u32 = 0x00020000; /* Top of directory hierarchies*/
pub const FS_HUGE_FILE_FL: u32 = 0x00040000; /* Reserved for ext4 */
pub const FS_EXTENT_FL: u32 = 0x00080000; /* Extents */
pub const FS_VERITY_FL: u32 = 0x00100000; /* Verity protected inode */
pub const FS_EA_INODE_FL: u32 = 0x00200000; /* Inode used for large EA */
pub const FS_EOFBLOCKS_FL: u32 = 0x00400000; /* Reserved for ext4 */
pub const FS_NOCOW_FL: u32 = 0x00800000; /* Do not cow file */
pub const FS_DAX_FL: u32 = 0x02000000; /* Inode is DAX */
pub const FS_INLINE_DATA_FL: u32 = 0x10000000; /* Reserved for ext4 */
pub const FS_PROJINHERIT_FL: u32 = 0x20000000; /* Create with parents projid */
pub const FS_CASEFOLD_FL: u32 = 0x40000000; /* Folder is case insensitive */
pub const FS_RESERVED_FL: u32 = 0x80000000; /* reserved for ext2 lib */

pub const FS_FL_USER_VISIBLE: u32 = 0x0003DFFF; /* User visible flags */
pub const FS_FL_USER_MODIFIABLE: u32 = 0x000380FF; /* User modifiable flags */

pub const SYNC_FILE_RANGE_WAIT_BEFORE: u32 = 1;
pub const SYNC_FILE_RANGE_WRITE: u32 = 2;
pub const SYNC_FILE_RANGE_WAIT_AFTER: u32 = 4;
pub const SYNC_FILE_RANGE_WRITE_AND_WAIT: u32 =
    SYNC_FILE_RANGE_WRITE | SYNC_FILE_RANGE_WAIT_BEFORE | SYNC_FILE_RANGE_WAIT_AFTER;

/*
 * Flags for preadv2/pwritev2:
 */

pub type __kernel_rwf_t = ::core::ffi::c_int;

/* high priority request, poll if possible */
pub const RWF_HIPRI: __kernel_rwf_t = 0x00000001 as __kernel_rwf_t;

/* per-IO O_DSYNC */
pub const RWF_DSYNC: __kernel_rwf_t = 0x00000002 as __kernel_rwf_t;

/* per-IO O_SYNC */
pub const RWF_SYNC: __kernel_rwf_t = 0x00000004 as __kernel_rwf_t;

/* per-IO, return -EAGAIN if operation would block */
pub const RWF_NOWAIT: __kernel_rwf_t = 0x00000008 as __kernel_rwf_t;

/* per-IO O_APPEND */
pub const RWF_APPEND: __kernel_rwf_t = 0x00000010 as __kernel_rwf_t;

/* per-IO negation of O_APPEND */
pub const RWF_NOAPPEND: __kernel_rwf_t = 0x00000020 as __kernel_rwf_t;

/* Atomic Write */
pub const RWF_ATOMIC: __kernel_rwf_t = 0x00000040 as __kernel_rwf_t;

/* buffered IO that drops the cache after reading or writing data */
pub const RWF_DONTCACHE: __kernel_rwf_t = 0x00000080 as __kernel_rwf_t;

/* mask of flags supported by the kernel */
pub const RWF_SUPPORTED: __kernel_rwf_t = RWF_HIPRI
    | RWF_DSYNC
    | RWF_SYNC
    | RWF_NOWAIT
    | RWF_APPEND
    | RWF_NOAPPEND
    | RWF_ATOMIC
    | RWF_DONTCACHE;

pub const PROCFS_IOCTL_MAGIC: u8 = b'f';

/* Pagemap ioctl */
pub const PAGEMAP_SCAN: ::core::ffi::c_ulong = _IOWR::<pm_scan_arg>(PROCFS_IOCTL_MAGIC, 16);

/* Bitmasks provided in pm_scan_args masks and reported in page_region.categories. */
pub const PAGE_IS_WPALLOWED: u32 = 1 << 0;
pub const PAGE_IS_WRITTEN: u32 = 1 << 1;
pub const PAGE_IS_FILE: u32 = 1 << 2;
pub const PAGE_IS_PRESENT: u32 = 1 << 3;
pub const PAGE_IS_SWAPPED: u32 = 1 << 4;
pub const PAGE_IS_PFNZERO: u32 = 1 << 5;
pub const PAGE_IS_HUGE: u32 = 1 << 6;
pub const PAGE_IS_SOFT_DIRTY: u32 = 1 << 7;
pub const PAGE_IS_GUARD: u32 = 1 << 8;
pub const PAGE_IS_ACCESSED: u32 = 1 << 9;

/*
 * struct page_region - Page region with flags
 * @start: Start of the region
 * @end: End of the region (exclusive)
 * @categories: PAGE_IS_* category bitmask for the region
 */
#[repr(C)]
pub struct page_region {
    pub start: __u64,
    pub end: __u64,
    pub categories: __u64,
}

/* Flags for PAGEMAP_SCAN ioctl */
pub const PM_SCAN_WP_MATCHING: u32 = 1 << 0; /* Write protect the pages matched. */
pub const PM_SCAN_CHECK_WPASYNC: u32 = 1 << 1; /* Abort the scan when a non-WP-enabled page is found. */

/*
 * struct pm_scan_arg - Pagemap ioctl argument
 * @size: Size of the structure
 * @flags: Flags for the IOCTL
 * @start: Starting address of the region
 * @end: Ending address of the region
 * @walk_end Address where the scan stopped (written by kernel).
 *           walk_end == end (address tags cleared) informs that the scan completed on entire range.
 * @vec: Address of page_region struct array for output
 * @vec_len: Length of the page_region struct array
 * @max_pages: Optional limit for number of returned pages (0 = disabled)
 * @category_inverted: PAGE_IS_* categories which values match if 0 instead of 1
 * @category_mask: Skip pages for which any category doesn't match
 * @category_anyof_mask: Skip pages for which no category matches
 * @return_mask: PAGE_IS_* categories that are to be reported in `page_region`s returned
 */
#[repr(C)]
pub struct pm_scan_arg {
    pub size: __u64,
    pub flags: __u64,
    pub start: __u64,
    pub end: __u64,
    pub walk_end: __u64,
    pub vec: __u64,
    pub vec_len: __u64,
    pub max_pages: __u64,
    pub category_inverted: __u64,
    pub category_mask: __u64,
    pub category_anyof_mask: __u64,
    pub return_mask: __u64,
}

/* /proc/<pid>/maps ioctl */
pub const PROCMAP_QUERY: ::core::ffi::c_ulong = _IOWR::<procmap_query>(PROCFS_IOCTL_MAGIC, 17);

#[repr(C)]
pub enum procmap_query_flags {
    /*
     * VMA permission flags.
     *
     * Can be used as part of procmap_query.query_flags field to look up
     * only VMAs satisfying specified subset of permissions. E.g., specifying
     * PROCMAP_QUERY_VMA_READABLE only will return both readable and read/write VMAs,
     * while having PROCMAP_QUERY_VMA_READABLE | PROCMAP_QUERY_VMA_WRITABLE will only
     * return read/write VMAs, though both executable/non-executable and
     * private/shared will be ignored.
     *
     * PROCMAP_QUERY_VMA_* flags are also returned in procmap_query.vma_flags
     * field to specify actual VMA permissions.
     */
    PROCMAP_QUERY_VMA_READABLE = 0x01,
    PROCMAP_QUERY_VMA_WRITABLE = 0x02,
    PROCMAP_QUERY_VMA_EXECUTABLE = 0x04,
    PROCMAP_QUERY_VMA_SHARED = 0x08,
    /*
     * Query modifier flags.
     *
     * By default VMA that covers provided address is returned, or -ENOENT
     * is returned. With PROCMAP_QUERY_COVERING_OR_NEXT_VMA flag set, closest
     * VMA with vma_start > addr will be returned if no covering VMA is
     * found.
     *
     * PROCMAP_QUERY_FILE_BACKED_VMA instructs query to consider only VMAs that
     * have file backing. Can be combined with PROCMAP_QUERY_COVERING_OR_NEXT_VMA
     * to iterate all VMAs with file backing.
     */
    PROCMAP_QUERY_COVERING_OR_NEXT_VMA = 0x10,
    PROCMAP_QUERY_FILE_BACKED_VMA = 0x20,
}

/*
 * Input/output argument structured passed into ioctl() call. It can be used
 * to query a set of VMAs (Virtual Memory Areas) of a process.
 *
 * Each field can be one of three kinds, marked in a short comment to the
 * right of the field:
 *   - "in", input argument, user has to provide this value, kernel doesn't modify it;
 *   - "out", output argument, kernel sets this field with VMA data;
 *   - "in/out", input and output argument; user provides initial value (used
 *     to specify maximum allowable buffer size), and kernel sets it to actual
 *     amount of data written (or zero, if there is no data).
 *
 * If matching VMA is found (according to criterias specified by
 * query_addr/query_flags, all the out fields are filled out, and ioctl()
 * returns 0. If there is no matching VMA, -ENOENT will be returned.
 * In case of any other error, negative error code other than -ENOENT is
 * returned.
 *
 * Most of the data is similar to the one returned as text in /proc/<pid>/maps
 * file, but procmap_query provides more querying flexibility. There are no
 * consistency guarantees between subsequent ioctl() calls, but data returned
 * for matched VMA is self-consistent.
 */
#[repr(C)]
pub struct procmap_query {
    /* Query struct size, for backwards/forward compatibility */
    pub size: __u64,
    /*
     * Query flags, a combination of enum procmap_query_flags values.
     * Defines query filtering and behavior, see enum procmap_query_flags.
     *
     * Input argument, provided by user. Kernel doesn't modify it.
     */
    pub query_flags: __u64, /* in */
    /*
     * Query address. By default, VMA that covers this address will
     * be looked up. PROCMAP_QUERY_* flags above modify this default
     * behavior further.
     *
     * Input argument, provided by user. Kernel doesn't modify it.
     */
    pub query_addr: __u64, /* in */
    /* VMA starting (inclusive) and ending (exclusive) address, if VMA is found. */
    pub vma_start: __u64, /* out */
    pub vma_end: __u64,   /* out */
    /* VMA permissions flags. A combination of PROCMAP_QUERY_VMA_* flags. */
    pub vma_flags: __u64, /* out */
    /* VMA backing page size granularity. */
    pub vma_page_size: __u64, /* out */
    /*
     * VMA file offset. If VMA has file backing, this specifies offset
     * within the file that VMA's start address corresponds to.
     * Is set to zero if VMA has no backing file.
     */
    pub vma_offset: __u64, /* out */
    /* Backing file's inode number, or zero, if VMA has no backing file. */
    pub inode: __u64, /* out */
    /* Backing file's device major/minor number, or zero, if VMA has no backing file. */
    pub dev_major: __u32, /* out */
    pub dev_minor: __u32, /* out */
    /*
     * If set to non-zero value, signals the request to return VMA name
     * (i.e., VMA's backing file's absolute path, with " (deleted)" suffix
     * appended, if file was unlinked from FS) for matched VMA. VMA name
     * can also be some special name (e.g., "[heap]", "[stack]") or could
     * be even user-supplied with prctl(PR_SET_VMA, PR_SET_VMA_ANON_NAME).
     *
     * Kernel will set this field to zero, if VMA has no associated name.
     * Otherwise kernel will return actual amount of bytes filled in
     * user-supplied buffer (see vma_name_addr field below), including the
     * terminating zero.
     *
     * If VMA name is longer that user-supplied maximum buffer size,
     * -E2BIG error is returned.
     *
     * If this field is set to non-zero value, vma_name_addr should point
     * to valid user space memory buffer of at least vma_name_size bytes.
     * If set to zero, vma_name_addr should be set to zero as well
     */
    pub vma_name_size: __u32, /* in/out */
    /*
     * If set to non-zero value, signals the request to extract and return
     * VMA's backing file's build ID, if the backing file is an ELF file
     * and it contains embedded build ID.
     *
     * Kernel will set this field to zero, if VMA has no backing file,
     * backing file is not an ELF file, or ELF file has no build ID
     * embedded.
     *
     * Build ID is a binary value (not a string). Kernel will set
     * build_id_size field to exact number of bytes used for build ID.
     * If build ID is requested and present, but needs more bytes than
     * user-supplied maximum buffer size (see build_id_addr field below),
     * -E2BIG error will be returned.
     *
     * If this field is set to non-zero value, build_id_addr should point
     * to valid user space memory buffer of at least build_id_size bytes.
     * If set to zero, build_id_addr should be set to zero as well
     */
    pub build_id_size: __u32, /* in/out */
    /*
     * User-supplied address of a buffer of at least vma_name_size bytes
     * for kernel to fill with matched VMA's name (see vma_name_size field
     * description above for details).
     *
     * Should be set to zero if VMA name should not be returned.
     */
    pub vma_name_addr: __u64, /* in */
    /*
     * User-supplied address of a buffer of at least build_id_size bytes
     * for kernel to fill with matched VMA's ELF build ID, if available
     * (see build_id_size field description above for details).
     *
     * Should be set to zero if build ID should not be returned.
     */
    pub build_id_addr: __u64, /* in */
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
