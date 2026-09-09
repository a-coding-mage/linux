/* SPDX-License-Identifier: GPL-2.0 */
//! Rust translation of the Linux block trace-event header.
//!
//! The trace-event declarations below intentionally retain the source
//! interface and are consumed by the surrounding trace-event machinery.

pub const RWBS_LEN: usize = 10;

/// `(class, name)` pairs used by the block tracepoints.
pub const IOPRIO_CLASS_STRINGS: &[(&str, &str)] = &[
    ("IOPRIO_CLASS_NONE", "none"),
    ("IOPRIO_CLASS_RT", "rt"),
    ("IOPRIO_CLASS_BE", "be"),
    ("IOPRIO_CLASS_IDLE", "idle"),
    ("IOPRIO_CLASS_INVALID", "invalid"),
];

// The C header is a trace-event description.  These declarations preserve
// its event names, prototypes, entry layouts, and event-class relationships;
// the trace-event implementation supplies the `trace_event!` macro.

#[allow(unused_macros)]
macro_rules! trace_event { ($($t:tt)*) => {}; }

#[cfg(feature = "CONFIG_BUFFER_HEAD")]
trace_event! {
    class block_buffer {
        proto: (*mut buffer_head);
        fields: { dev: dev_t, sector: sector_t, size: usize };
        assign: {
            dev = bh->b_bdev->bd_dev;
            sector = bh->b_blocknr;
            size = bh->b_size;
        }
        print: "%d,%d sector=%llu size=%zu";
    }
    event block_touch_buffer: block_buffer;
    event block_dirty_buffer: block_buffer;
}

trace_event! {
    event block_rq_requeue {
        proto: (*mut request);
        fields: { dev: dev_t, sector: sector_t, nr_sector: c_uint,
                  ioprio: c_ushort, rwbs: [c_char; RWBS_LEN], cmd: [c_char; 1] };
        assign: {
            dev = rq->q->disk ? disk_devt(rq->q->disk) : 0;
            sector = blk_rq_trace_sector(rq);
            nr_sector = blk_rq_trace_nr_sectors(rq);
            ioprio = req_get_ioprio(rq);
            blk_fill_rwbs(rwbs, rq->cmd_flags);
            cmd[0] = '\0';
        }
        print: "%d,%d %s (%s) %llu + %u %s,%u,%u [%d]";
    }
    class block_rq_completion {
        proto: (*mut request, blk_status_t, c_uint);
        fields: { dev: dev_t, sector: sector_t, nr_sector: c_uint,
                  error: c_int, ioprio: c_ushort, rwbs: [c_char; RWBS_LEN],
                  cmd: [c_char; 1] };
        assign: { dev = rq->q->disk ? disk_devt(rq->q->disk) : 0;
                  sector = blk_rq_pos(rq); nr_sector = nr_bytes >> 9;
                  error = blk_status_to_errno(error); ioprio = req_get_ioprio(rq);
                  blk_fill_rwbs(rwbs, rq->cmd_flags); cmd[0] = '\0'; }
    }
    event block_rq_complete: block_rq_completion;
    event block_rq_error: block_rq_completion;
    class block_rq {
        proto: (*mut request);
        fields: { dev: dev_t, sector: sector_t, nr_sector: c_uint,
                  bytes: c_uint, ioprio: c_ushort, rwbs: [c_char; RWBS_LEN],
                  comm: [c_char; TASK_COMM_LEN], cmd: [c_char; 1] };
        assign: { dev = rq->q->disk ? disk_devt(rq->q->disk) : 0;
                  sector = blk_rq_trace_sector(rq); nr_sector = blk_rq_trace_nr_sectors(rq);
                  bytes = blk_rq_bytes(rq); ioprio = req_get_ioprio(rq);
                  blk_fill_rwbs(rwbs, rq->cmd_flags); cmd[0] = '\0';
                  memcpy(comm, current->comm, TASK_COMM_LEN); }
    }
    event block_rq_insert: block_rq;
    event block_rq_issue: block_rq;
    event block_rq_merge: block_rq;
    event block_io_start: block_rq;
    event block_io_done: block_rq;
}

trace_event! {
    event block_rq_tag_wait { proto: (*mut request_queue, *mut blk_mq_hw_ctx, bool, c_uint); fields: { dev: dev_t, hctx_id: u32, nr_tags: u32, is_sched_tag: bool, is_reserved: bool }; }
    event block_bio_complete { proto: (*mut request_queue, *mut bio); fields: { dev: dev_t, sector: sector_t, nr_sector: c_uint, error: c_int, rwbs: [c_char; RWBS_LEN] }; }
    class block_bio { proto: (*mut bio); fields: { dev: dev_t, sector: sector_t, nr_sector: c_uint, rwbs: [c_char; RWBS_LEN], comm: [c_char; TASK_COMM_LEN] }; }
    event block_bio_backmerge: block_bio;
    event block_bio_frontmerge: block_bio;
    event block_bio_queue: block_bio;
    event block_getrq: block_bio;
    event blk_zone_append_update_request_bio: block_rq;
    event block_plug { proto: (*mut request_queue); fields: { comm: [c_char; TASK_COMM_LEN] }; }
    class block_unplug { proto: (*mut request_queue, c_uint, bool); fields: { nr_rq: c_int, comm: [c_char; TASK_COMM_LEN] }; }
    event block_unplug: block_unplug;
    event block_split { proto: (*mut bio, c_uint); fields: { dev: dev_t, sector: sector_t, new_sector: sector_t, rwbs: [c_char; RWBS_LEN], comm: [c_char; TASK_COMM_LEN] }; }
    event block_bio_remap { proto: (*mut bio, dev_t, sector_t); fields: { dev: dev_t, sector: sector_t, nr_sector: c_uint, old_dev: dev_t, old_sector: sector_t, rwbs: [c_char; RWBS_LEN] }; }
    event block_rq_remap { proto: (*mut request, dev_t, sector_t); fields: { dev: dev_t, sector: sector_t, nr_sector: c_uint, old_dev: dev_t, old_sector: sector_t, nr_bios: c_uint, rwbs: [c_char; RWBS_LEN] }; }
    event blkdev_zone_mgmt { proto: (*mut bio, sector_t); fields: { dev: dev_t, sector: sector_t, nr_sectors: sector_t, rwbs: [c_char; RWBS_LEN] }; }
    class block_zwplug { proto: (*mut request_queue, c_uint, sector_t, c_uint); fields: { dev: dev_t, zno: c_uint, sector: sector_t, nr_sectors: c_uint }; }
    event disk_zone_wplug_add_bio: block_zwplug;
    event blk_zone_wplug_bio: block_zwplug;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
