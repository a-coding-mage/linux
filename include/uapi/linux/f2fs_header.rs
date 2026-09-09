/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Translated from <linux/f2fs.h>. Linux type and ioctl dependencies are
// represented by their Rust equivalents below.

#[allow(non_camel_case_types)]
pub type __u8 = u8;
#[allow(non_camel_case_types)]
pub type __u32 = u32;
#[allow(non_camel_case_types)]
pub type __u64 = u64;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct f2fs_gc_range {
    pub sync: __u32,
    pub start: __u64,
    pub len: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct f2fs_defragment {
    pub start: __u64,
    pub len: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct f2fs_move_range {
    pub dst_fd: __u32, // destination fd
    pub pos_in: __u64, // start position in src_fd
    pub pos_out: __u64, // start position in dst_fd
    pub len: __u64, // size to move
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct f2fs_flush_device {
    pub dev_num: __u32, // device number to flush
    pub segments: __u32, // # of segments to flush
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct f2fs_sectrim_range {
    pub start: __u64,
    pub len: __u64,
    pub flags: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct f2fs_comp_option {
    pub algorithm: __u8,
    pub log_cluster_size: __u8,
}

const IOC_NRBITS: u32 = 8;
const IOC_TYPEBITS: u32 = 8;
const IOC_SIZEBITS: u32 = 14;
const IOC_NRSHIFT: u32 = 0;
const IOC_TYPESHIFT: u32 = IOC_NRSHIFT + IOC_NRBITS;
const IOC_SIZESHIFT: u32 = IOC_TYPESHIFT + IOC_TYPEBITS;
const IOC_DIRSHIFT: u32 = IOC_SIZESHIFT + IOC_SIZEBITS;
const IOC_NONE: u32 = 0;
const IOC_WRITE: u32 = 1;
const IOC_READ: u32 = 2;

const fn ioc(dir: u32, ty: u32, nr: u32, size: usize) -> u32 {
    (dir << IOC_DIRSHIFT)
        | (ty << IOC_TYPESHIFT)
        | (nr << IOC_NRSHIFT)
        | ((size as u32) << IOC_SIZESHIFT)
}

const fn io(ty: u32, nr: u32) -> u32 { ioc(IOC_NONE, ty, nr, 0) }
const fn iow<T>(ty: u32, nr: u32) -> u32 { ioc(IOC_WRITE, ty, nr, core::mem::size_of::<T>()) }
const fn ior<T>(ty: u32, nr: u32) -> u32 { ioc(IOC_READ, ty, nr, core::mem::size_of::<T>()) }
const fn iowr<T>(ty: u32, nr: u32) -> u32 { ioc(IOC_READ | IOC_WRITE, ty, nr, core::mem::size_of::<T>()) }

pub const F2FS_IOCTL_MAGIC: u32 = 0xf5;
pub const F2FS_IOC_START_ATOMIC_WRITE: u32 = io(F2FS_IOCTL_MAGIC, 1);
pub const F2FS_IOC_COMMIT_ATOMIC_WRITE: u32 = io(F2FS_IOCTL_MAGIC, 2);
pub const F2FS_IOC_START_VOLATILE_WRITE: u32 = io(F2FS_IOCTL_MAGIC, 3);
pub const F2FS_IOC_RELEASE_VOLATILE_WRITE: u32 = io(F2FS_IOCTL_MAGIC, 4);
pub const F2FS_IOC_ABORT_ATOMIC_WRITE: u32 = io(F2FS_IOCTL_MAGIC, 5);
pub const F2FS_IOC_GARBAGE_COLLECT: u32 = iow::<__u32>(F2FS_IOCTL_MAGIC, 6);
pub const F2FS_IOC_WRITE_CHECKPOINT: u32 = io(F2FS_IOCTL_MAGIC, 7);
pub const F2FS_IOC_DEFRAGMENT: u32 = iowr::<f2fs_defragment>(F2FS_IOCTL_MAGIC, 8);
pub const F2FS_IOC_MOVE_RANGE: u32 = iowr::<f2fs_move_range>(F2FS_IOCTL_MAGIC, 9);
pub const F2FS_IOC_FLUSH_DEVICE: u32 = iow::<f2fs_flush_device>(F2FS_IOCTL_MAGIC, 10);
pub const F2FS_IOC_GARBAGE_COLLECT_RANGE: u32 = iow::<f2fs_gc_range>(F2FS_IOCTL_MAGIC, 11);
pub const F2FS_IOC_GET_FEATURES: u32 = ior::<__u32>(F2FS_IOCTL_MAGIC, 12);
pub const F2FS_IOC_SET_PIN_FILE: u32 = iow::<__u32>(F2FS_IOCTL_MAGIC, 13);
pub const F2FS_IOC_GET_PIN_FILE: u32 = ior::<__u32>(F2FS_IOCTL_MAGIC, 14);
pub const F2FS_IOC_PRECACHE_EXTENTS: u32 = io(F2FS_IOCTL_MAGIC, 15);
pub const F2FS_IOC_RESIZE_FS: u32 = iow::<__u64>(F2FS_IOCTL_MAGIC, 16);
pub const F2FS_IOC_GET_COMPRESS_BLOCKS: u32 = ior::<__u64>(F2FS_IOCTL_MAGIC, 17);
pub const F2FS_IOC_RELEASE_COMPRESS_BLOCKS: u32 = ior::<__u64>(F2FS_IOCTL_MAGIC, 18);
pub const F2FS_IOC_RESERVE_COMPRESS_BLOCKS: u32 = ior::<__u64>(F2FS_IOCTL_MAGIC, 19);
pub const F2FS_IOC_SEC_TRIM_FILE: u32 = iow::<f2fs_sectrim_range>(F2FS_IOCTL_MAGIC, 20);
pub const F2FS_IOC_GET_COMPRESS_OPTION: u32 = ior::<f2fs_comp_option>(F2FS_IOCTL_MAGIC, 21);
pub const F2FS_IOC_SET_COMPRESS_OPTION: u32 = iow::<f2fs_comp_option>(F2FS_IOCTL_MAGIC, 22);
pub const F2FS_IOC_DECOMPRESS_FILE: u32 = io(F2FS_IOCTL_MAGIC, 23);
pub const F2FS_IOC_COMPRESS_FILE: u32 = io(F2FS_IOCTL_MAGIC, 24);
pub const F2FS_IOC_START_ATOMIC_REPLACE: u32 = io(F2FS_IOCTL_MAGIC, 25);
pub const F2FS_IOC_GET_DEV_ALIAS_FILE: u32 = ior::<__u32>(F2FS_IOCTL_MAGIC, 26);
pub const F2FS_IOC_IO_PRIO: u32 = iow::<__u32>(F2FS_IOCTL_MAGIC, 27);
pub const F2FS_IOC_RESERVE_DEV_ALIAS: u32 = io(F2FS_IOCTL_MAGIC, 28);
pub const F2FS_IOC_RELEASE_DEV_ALIAS: u32 = io(F2FS_IOCTL_MAGIC, 29);
pub const F2FS_IOC_GET_DEV_ALIAS_STATUS: u32 = ior::<__u32>(F2FS_IOCTL_MAGIC, 30);

// Should be same as XFS_IOC_GOINGDOWN. Flags for FS_IOC_GOINGDOWN.
pub const F2FS_IOC_SHUTDOWN: u32 = ior::<__u32>(b'X' as u32, 125); // Shutdown
pub const F2FS_GOING_DOWN_FULLSYNC: u32 = 0x0; // going down with full sync
pub const F2FS_GOING_DOWN_METASYNC: u32 = 0x1; // going down with metadata
pub const F2FS_GOING_DOWN_NOSYNC: u32 = 0x2; // going down
pub const F2FS_GOING_DOWN_METAFLUSH: u32 = 0x3; // going down with meta flush
pub const F2FS_GOING_DOWN_NEED_FSCK: u32 = 0x4; // going down to trigger fsck

// Flags used by F2FS_IOC_SEC_TRIM_FILE.
pub const F2FS_TRIM_FILE_DISCARD: u32 = 0x1; // send discard command
pub const F2FS_TRIM_FILE_ZEROOUT: u32 = 0x2; // zero out
pub const F2FS_TRIM_FILE_MASK: u32 = 0x3;

// For F2FS_IOC_IO_PRIO.
pub const F2FS_IOPRIO_WRITE: u32 = 1; // high write priority
pub const F2FS_IOPRIO_MAX: u32 = 2;

// For F2FS_IOC_GET_DEV_ALIAS_STATUS.
pub const F2FS_DEV_ALIAS_STATUS_RELEASED: u32 = 0;
pub const F2FS_DEV_ALIAS_STATUS_RESERVED: u32 = 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
