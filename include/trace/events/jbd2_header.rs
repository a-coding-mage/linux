/* SPDX-License-Identifier: GPL-2.0 */
// Rust translation of trace/events/jbd2.h.
// The C TRACE_EVENT/DECLARE_EVENT_CLASS/DEFINE_EVENT invocations are represented
// by C-layout payload types and event metadata; their callbacks remain external
// tracepoint machinery supplied by the kernel integration.

// Dependencies supplied by the surrounding kernel translation.
use core::ffi::c_char;

pub type DevT = usize;
pub type TidT = u32;
pub type InoT = usize;
pub type BlkOpfT = u32;
pub type U32 = u32;

#[repr(C)]
pub struct TransactionChpStatsS { _private: [u8; 0] }
#[repr(C)]
pub struct TransactionRunStatsS { _private: [u8; 0] }

macro_rules! trace_event {
    ($name:ident, $($field:ident : $ty:ty),* $(,)?) => {
        #[repr(C)]
        pub struct $name { $(pub $field: $ty,)* }
    };
}

trace_event!(Jbd2CheckpointEntry, dev: DevT, result: i32);
trace_event!(Jbd2CommitEntry, dev: DevT, sync_commit: c_char, transaction: TidT);
trace_event!(Jbd2EndCommitEntry, dev: DevT, sync_commit: c_char, transaction: TidT, head: TidT);
trace_event!(Jbd2SubmitInodeDataEntry, dev: DevT, ino: InoT);
trace_event!(Jbd2HandleStartEntry, dev: DevT, tid: TidT, type_: u32, line_no: u32, requested_blocks: i32);
trace_event!(Jbd2HandleExtendEntry, dev: DevT, tid: TidT, type_: u32, line_no: u32, buffer_credits: i32, requested_blocks: i32);
trace_event!(Jbd2HandleStatsEntry, dev: DevT, tid: TidT, type_: u32, line_no: u32, interval: i32, sync: i32, requested_blocks: i32, dirtied_blocks: i32);
trace_event!(Jbd2RunStatsEntry, dev: DevT, tid: TidT, wait: usize, request_delay: usize, running: usize, locked: usize, flushing: usize, logging: usize, handle_count: U32, blocks: U32, blocks_logged: U32);
trace_event!(Jbd2CheckpointStatsEntry, dev: DevT, tid: TidT, chp_time: usize, forced_to_close: U32, written: U32, dropped: U32);
trace_event!(Jbd2UpdateLogTailEntry, dev: DevT, tail_sequence: TidT, first_tid: TidT, block_nr: usize, freed: usize);
trace_event!(Jbd2WriteSuperblockEntry, dev: DevT, write_flags: BlkOpfT);
trace_event!(Jbd2LockBufferStallEntry, dev: DevT, stall_ms: usize);
trace_event!(Jbd2JournalShrinkEntry, dev: DevT, nr_to_scan: usize, count: usize);
trace_event!(Jbd2ShrinkScanExitEntry, dev: DevT, nr_to_scan: usize, nr_shrunk: usize, count: usize);
trace_event!(Jbd2ShrinkCheckpointListEntry, dev: DevT, first_tid: TidT, tid: TidT, last_tid: TidT, nr_freed: usize, next_tid: TidT);

// Event names and their source-level prototypes.  These preserve the externally
// visible tracepoint interface and the DECLARE_EVENT_CLASS/DEFINE_EVENT aliases.
pub const JBD2_EVENTS: &[&str] = &[
    "jbd2_checkpoint", "jbd2_start_commit", "jbd2_commit_locking",
    "jbd2_commit_flushing", "jbd2_commit_logging", "jbd2_drop_transaction",
    "jbd2_end_commit", "jbd2_submit_inode_data", "jbd2_handle_start",
    "jbd2_handle_restart", "jbd2_handle_extend", "jbd2_handle_stats",
    "jbd2_run_stats", "jbd2_checkpoint_stats", "jbd2_update_log_tail",
    "jbd2_write_superblock", "jbd2_lock_buffer_stall", "jbd2_shrink_count",
    "jbd2_shrink_scan_enter", "jbd2_shrink_scan_exit",
    "jbd2_shrink_checkpoint_list",
];

// Original format strings (TP_printk), retained as constants for trace output.
pub const JBD2_CHECKPOINT_FORMAT: &str = "dev %d,%d result %d";
pub const JBD2_COMMIT_FORMAT: &str = "dev %d,%d transaction %u sync %d";
pub const JBD2_END_COMMIT_FORMAT: &str = "dev %d,%d transaction %u sync %d head %u";
pub const JBD2_SUBMIT_INODE_DATA_FORMAT: &str = "dev %d,%d ino %lu";
pub const JBD2_HANDLE_START_FORMAT: &str = "dev %d,%d tid %u type %u line_no %u requested_blocks %d";
pub const JBD2_HANDLE_EXTEND_FORMAT: &str = "dev %d,%d tid %u type %u line_no %u buffer_credits %d requested_blocks %d";
pub const JBD2_HANDLE_STATS_FORMAT: &str = "dev %d,%d tid %u type %u line_no %u interval %d sync %d requested_blocks %d dirtied_blocks %d";
pub const JBD2_RUN_STATS_FORMAT: &str = "dev %d,%d tid %u wait %u request_delay %u running %u locked %u flushing %u logging %u handle_count %u blocks %u blocks_logged %u";
pub const JBD2_CHECKPOINT_STATS_FORMAT: &str = "dev %d,%d tid %u chp_time %u forced_to_close %u written %u dropped %u";
pub const JBD2_UPDATE_LOG_TAIL_FORMAT: &str = "dev %d,%d from %u to %u offset %lu freed %lu";
pub const JBD2_WRITE_SUPERBLOCK_FORMAT: &str = "dev %d,%d write_flags %x";
pub const JBD2_LOCK_BUFFER_STALL_FORMAT: &str = "dev %d,%d stall_ms %lu";
pub const JBD2_JOURNAL_SHRINK_FORMAT: &str = "dev %d,%d nr_to_scan %lu count %lu";
pub const JBD2_SHRINK_SCAN_EXIT_FORMAT: &str = "dev %d,%d nr_to_scan %lu nr_shrunk %lu count %lu";
pub const JBD2_SHRINK_CHECKPOINT_LIST_FORMAT: &str = "dev %d,%d shrink transaction %u-%u(%u) freed %lu next transaction %u";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
