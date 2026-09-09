/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/* Dependency: linux/types.h supplies u8/u16/u32/u64 and __be32/__be64. */

/* Trace categories */
pub const BLK_TC_READ: u64 = 1 << 0; /* reads */
pub const BLK_TC_WRITE: u64 = 1 << 1; /* writes */
pub const BLK_TC_FLUSH: u64 = 1 << 2; /* flush */
pub const BLK_TC_SYNC: u64 = 1 << 3; /* sync IO */
pub const BLK_TC_SYNCIO: u64 = BLK_TC_SYNC;
pub const BLK_TC_QUEUE: u64 = 1 << 4; /* queueing/merging */
pub const BLK_TC_REQUEUE: u64 = 1 << 5; /* requeueing */
pub const BLK_TC_ISSUE: u64 = 1 << 6; /* issue */
pub const BLK_TC_COMPLETE: u64 = 1 << 7; /* completions */
pub const BLK_TC_FS: u64 = 1 << 8; /* fs requests */
pub const BLK_TC_PC: u64 = 1 << 9; /* pc requests */
pub const BLK_TC_NOTIFY: u64 = 1 << 10; /* special message */
pub const BLK_TC_AHEAD: u64 = 1 << 11; /* readahead */
pub const BLK_TC_META: u64 = 1 << 12; /* metadata */
pub const BLK_TC_DISCARD: u64 = 1 << 13; /* discard requests */
pub const BLK_TC_DRV_DATA: u64 = 1 << 14; /* binary per-driver data */
pub const BLK_TC_FUA: u64 = 1 << 15; /* fua requests */
pub const BLK_TC_END_V1: u64 = 1 << 15; /* we've run out of bits! */
pub const BLK_TC_ZONE_APPEND: u64 = 1u64 << 16; /* zone append */
pub const BLK_TC_ZONE_RESET: u64 = 1u64 << 17; /* zone reset */
pub const BLK_TC_ZONE_RESET_ALL: u64 = 1u64 << 18; /* zone reset all */
pub const BLK_TC_ZONE_FINISH: u64 = 1u64 << 19; /* zone finish */
pub const BLK_TC_ZONE_OPEN: u64 = 1u64 << 20; /* zone open */
pub const BLK_TC_ZONE_CLOSE: u64 = 1u64 << 21; /* zone close */
pub const BLK_TC_WRITE_ZEROES: u64 = 1u64 << 22; /* write-zeroes */
pub const BLK_TC_END_V2: u64 = 1u64 << 22;

pub const BLK_TC_SHIFT: u32 = 16;
#[inline]
pub const fn BLK_TC_ACT(act: u64) -> u64 { act << BLK_TC_SHIFT }

/* Basic trace actions */
pub const __BLK_TA_QUEUE: u32 = 1;
pub const __BLK_TA_BACKMERGE: u32 = 2;
pub const __BLK_TA_FRONTMERGE: u32 = 3;
pub const __BLK_TA_GETRQ: u32 = 4;
pub const __BLK_TA_SLEEPRQ: u32 = 5;
pub const __BLK_TA_REQUEUE: u32 = 6;
pub const __BLK_TA_ISSUE: u32 = 7;
pub const __BLK_TA_COMPLETE: u32 = 8;
pub const __BLK_TA_PLUG: u32 = 9;
pub const __BLK_TA_UNPLUG_IO: u32 = 10;
pub const __BLK_TA_UNPLUG_TIMER: u32 = 11;
pub const __BLK_TA_INSERT: u32 = 12;
pub const __BLK_TA_SPLIT: u32 = 13;
pub const __BLK_TA_BOUNCE: u32 = 14;
pub const __BLK_TA_REMAP: u32 = 15;
pub const __BLK_TA_ABORT: u32 = 16;
pub const __BLK_TA_DRV_DATA: u32 = 17;
pub const __BLK_TA_ZONE_PLUG: u32 = 18;
pub const __BLK_TA_ZONE_UNPLUG: u32 = 19;
pub const __BLK_TA_CGROUP: u32 = 1 << 8;

/* Notify events. */
pub const __BLK_TN_PROCESS: u32 = 0;
pub const __BLK_TN_TIMESTAMP: u32 = 1;
pub const __BLK_TN_MESSAGE: u32 = 2;
pub const __BLK_TN_CGROUP: u32 = __BLK_TA_CGROUP;

/* Trace actions in full. Additionally, read or write is masked */
pub const BLK_TA_QUEUE: u64 = (__BLK_TA_QUEUE as u64) | BLK_TC_ACT(BLK_TC_QUEUE);
pub const BLK_TA_BACKMERGE: u64 = (__BLK_TA_BACKMERGE as u64) | BLK_TC_ACT(BLK_TC_QUEUE);
pub const BLK_TA_FRONTMERGE: u64 = (__BLK_TA_FRONTMERGE as u64) | BLK_TC_ACT(BLK_TC_QUEUE);
pub const BLK_TA_GETRQ: u64 = (__BLK_TA_GETRQ as u64) | BLK_TC_ACT(BLK_TC_QUEUE);
pub const BLK_TA_SLEEPRQ: u64 = (__BLK_TA_SLEEPRQ as u64) | BLK_TC_ACT(BLK_TC_QUEUE);
pub const BLK_TA_REQUEUE: u64 = (__BLK_TA_REQUEUE as u64) | BLK_TC_ACT(BLK_TC_REQUEUE);
pub const BLK_TA_ISSUE: u64 = (__BLK_TA_ISSUE as u64) | BLK_TC_ACT(BLK_TC_ISSUE);
pub const BLK_TA_COMPLETE: u64 = (__BLK_TA_COMPLETE as u64) | BLK_TC_ACT(BLK_TC_COMPLETE);
pub const BLK_TA_PLUG: u64 = (__BLK_TA_PLUG as u64) | BLK_TC_ACT(BLK_TC_QUEUE);
pub const BLK_TA_UNPLUG_IO: u64 = (__BLK_TA_UNPLUG_IO as u64) | BLK_TC_ACT(BLK_TC_QUEUE);
pub const BLK_TA_UNPLUG_TIMER: u64 = (__BLK_TA_UNPLUG_TIMER as u64) | BLK_TC_ACT(BLK_TC_QUEUE);
pub const BLK_TA_INSERT: u64 = (__BLK_TA_INSERT as u64) | BLK_TC_ACT(BLK_TC_QUEUE);
pub const BLK_TA_SPLIT: u64 = __BLK_TA_SPLIT as u64;
pub const BLK_TA_BOUNCE: u64 = __BLK_TA_BOUNCE as u64;
pub const BLK_TA_REMAP: u64 = (__BLK_TA_REMAP as u64) | BLK_TC_ACT(BLK_TC_QUEUE);
pub const BLK_TA_ABORT: u64 = (__BLK_TA_ABORT as u64) | BLK_TC_ACT(BLK_TC_QUEUE);
pub const BLK_TA_DRV_DATA: u64 = (__BLK_TA_DRV_DATA as u64) | BLK_TC_ACT(BLK_TC_DRV_DATA);
pub const BLK_TA_ZONE_APPEND: u64 = (__BLK_TA_COMPLETE as u64) | BLK_TC_ACT(BLK_TC_ZONE_APPEND);
pub const BLK_TA_ZONE_PLUG: u64 = (__BLK_TA_ZONE_PLUG as u64) | BLK_TC_ACT(BLK_TC_QUEUE);
pub const BLK_TA_ZONE_UNPLUG: u64 = (__BLK_TA_ZONE_UNPLUG as u64) | BLK_TC_ACT(BLK_TC_QUEUE);
pub const BLK_TN_PROCESS: u64 = (__BLK_TN_PROCESS as u64) | BLK_TC_ACT(BLK_TC_NOTIFY);
pub const BLK_TN_TIMESTAMP: u64 = (__BLK_TN_TIMESTAMP as u64) | BLK_TC_ACT(BLK_TC_NOTIFY);
pub const BLK_TN_MESSAGE: u64 = (__BLK_TN_MESSAGE as u64) | BLK_TC_ACT(BLK_TC_NOTIFY);

pub const BLK_IO_TRACE_MAGIC: u32 = 0x65617400;
pub const BLK_IO_TRACE_VERSION: u32 = 0x07;
pub const BLK_IO_TRACE2_VERSION: u32 = 0x08;

#[repr(C)]
pub struct blk_io_trace {
    pub magic: u32, pub sequence: u32, pub time: u64, pub sector: u64,
    pub bytes: u32, pub action: u32, pub pid: u32, pub device: u32,
    pub cpu: u32, pub error: u16, pub pdu_len: u16,
}

#[repr(C)]
pub struct blk_io_trace2 {
    pub magic: u32, pub sequence: u32, pub time: u64, pub sector: u64,
    pub bytes: u32, pub pid: u32, pub action: u64, pub device: u32,
    pub cpu: u32, pub error: u16, pub pdu_len: u16, pub pad: [u8; 12],
}

#[repr(C)]
pub struct blk_io_trace_remap { pub device_from: __be32, pub device_to: __be32, pub sector_from: __be64 }

pub const Blktrace_setup: u32 = 1;
pub const Blktrace_running: u32 = 2;
pub const Blktrace_stopped: u32 = 3;
pub const BLKTRACE_BDEV_SIZE: usize = 32;
pub const BLKTRACE_BDEV_SIZE2: usize = 64;

#[repr(C)]
pub struct blk_user_trace_setup {
    pub name: [i8; BLKTRACE_BDEV_SIZE], pub act_mask: u16, pub buf_size: u32,
    pub buf_nr: u32, pub start_lba: u64, pub end_lba: u64, pub pid: u32,
}

#[repr(C)]
pub struct blk_user_trace_setup2 {
    pub name: [i8; BLKTRACE_BDEV_SIZE2], pub act_mask: u64, pub buf_size: u32,
    pub buf_nr: u32, pub start_lba: u64, pub end_lba: u64, pub pid: u32,
    pub flags: u32, pub reserved: [u64; 11],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
