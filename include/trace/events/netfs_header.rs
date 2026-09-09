//! Rust translation of `trace/events/netfs.h`.
//!
//! The Linux tracepoint declarations below are represented as Rust metadata.
//! The structures referenced by the original tracepoints are supplied by the
//! kernel tracing environment and are intentionally not redefined here.

#![allow(non_camel_case_types, non_upper_case_globals, dead_code)]

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum netfs_read_trace {
    netfs_read_trace_dio_read,
    netfs_read_trace_expanded,
    netfs_read_trace_readahead,
    netfs_read_trace_readpage,
    netfs_read_trace_read_gaps,
    netfs_read_trace_read_single,
    netfs_read_trace_prefetch_for_write,
    netfs_read_trace_write_begin,
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum netfs_write_trace { netfs_write_trace_copy_to_cache, netfs_write_trace_dio_write, netfs_write_trace_unbuffered_write, netfs_write_trace_writeback, netfs_write_trace_writeback_single, netfs_write_trace_writethrough }
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum netfs_rreq_trace { netfs_rreq_trace_assess, netfs_rreq_trace_collect, netfs_rreq_trace_complete, netfs_rreq_trace_copy, netfs_rreq_trace_dirty, netfs_rreq_trace_done, netfs_rreq_trace_end_copy_to_cache, netfs_rreq_trace_free, netfs_rreq_trace_intr, netfs_rreq_trace_ki_complete, netfs_rreq_trace_recollect, netfs_rreq_trace_redirty, netfs_rreq_trace_resubmit, netfs_rreq_trace_set_abandon, netfs_rreq_trace_set_pause, netfs_rreq_trace_unlock, netfs_rreq_trace_unlock_pgpriv2, netfs_rreq_trace_unmark, netfs_rreq_trace_unpause, netfs_rreq_trace_wait_ip, netfs_rreq_trace_wait_pause, netfs_rreq_trace_wait_quiesce, netfs_rreq_trace_waited_ip, netfs_rreq_trace_waited_pause, netfs_rreq_trace_waited_quiesce, netfs_rreq_trace_wake_ip, netfs_rreq_trace_wake_queue, netfs_rreq_trace_write_done }
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum netfs_sreq_trace { netfs_sreq_trace_abandoned, netfs_sreq_trace_add_donations, netfs_sreq_trace_added, netfs_sreq_trace_cache_nowrite, netfs_sreq_trace_cache_prepare, netfs_sreq_trace_cache_write, netfs_sreq_trace_cancel, netfs_sreq_trace_clear, netfs_sreq_trace_consumed, netfs_sreq_trace_discard, netfs_sreq_trace_donate_to_prev, netfs_sreq_trace_donate_to_next, netfs_sreq_trace_download_instead, netfs_sreq_trace_fail, netfs_sreq_trace_free, netfs_sreq_trace_hit_eof, netfs_sreq_trace_io_bad, netfs_sreq_trace_io_malformed, netfs_sreq_trace_io_unknown, netfs_sreq_trace_io_progress, netfs_sreq_trace_io_req_submitted, netfs_sreq_trace_io_retry_needed, netfs_sreq_trace_limited, netfs_sreq_trace_need_clear, netfs_sreq_trace_partial_read, netfs_sreq_trace_need_retry, netfs_sreq_trace_prepare, netfs_sreq_trace_prep_failed, netfs_sreq_trace_progress, netfs_sreq_trace_reprep_failed, netfs_sreq_trace_retry, netfs_sreq_trace_short, netfs_sreq_trace_split, netfs_sreq_trace_submit, netfs_sreq_trace_superfluous, netfs_sreq_trace_terminated, netfs_sreq_trace_wait_for, netfs_sreq_trace_write, netfs_sreq_trace_write_skip, netfs_sreq_trace_write_term }
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum netfs_failure { netfs_fail_check_write_begin, netfs_fail_copy_to_cache, netfs_fail_dio_read_short, netfs_fail_dio_read_zero, netfs_fail_read, netfs_fail_short_read, netfs_fail_prepare_write, netfs_fail_write }
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum netfs_rreq_ref_trace { netfs_rreq_trace_get_for_outstanding, netfs_rreq_trace_get_subreq, netfs_rreq_trace_put_complete, netfs_rreq_trace_put_discard, netfs_rreq_trace_put_failed, netfs_rreq_trace_put_no_submit, netfs_rreq_trace_put_return, netfs_rreq_trace_put_subreq, netfs_rreq_trace_put_work_ip, netfs_rreq_trace_see_work, netfs_rreq_trace_see_work_complete, netfs_rreq_trace_new }
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum netfs_sreq_ref_trace { netfs_sreq_trace_get_copy_to_cache, netfs_sreq_trace_get_resubmit, netfs_sreq_trace_get_submit, netfs_sreq_trace_get_short_read, netfs_sreq_trace_new, netfs_sreq_trace_put_abandon, netfs_sreq_trace_put_cancel, netfs_sreq_trace_put_clear, netfs_sreq_trace_put_consumed, netfs_sreq_trace_put_done, netfs_sreq_trace_put_failed, netfs_sreq_trace_put_merged, netfs_sreq_trace_put_no_copy, netfs_sreq_trace_put_oom, netfs_sreq_trace_put_wip, netfs_sreq_trace_put_work, netfs_sreq_trace_put_terminated, netfs_sreq_trace_see_failed }

// The remaining trace enums are byte-sized in C (__mode(byte)).
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum netfs_folio_trace {
    netfs_folio_is_uptodate, netfs_just_prefetch, netfs_whole_folio_modify,
    netfs_whole_folio_modify_efault, netfs_whole_folio_modify_filled,
    netfs_whole_folio_modify_filled_efault, netfs_modify_and_clear,
    netfs_modify_and_clear_rm_finfo, netfs_streaming_write, netfs_streaming_write_cont,
    netfs_flush_content, netfs_streaming_filled_page, netfs_streaming_cont_filled_page,
    netfs_folio_trace_abandon, netfs_folio_trace_alloc_buffer, netfs_folio_trace_cancel_copy,
    netfs_folio_trace_cancel_store, netfs_folio_trace_clear, netfs_folio_trace_clear_cc,
    netfs_folio_trace_clear_g, netfs_folio_trace_clear_s, netfs_folio_trace_copy_to_cache,
    netfs_folio_trace_end_copy, netfs_folio_trace_filled_gaps, netfs_folio_trace_invalidate_all,
    netfs_folio_trace_invalidate_front, netfs_folio_trace_invalidate_middle,
    netfs_folio_trace_invalidate_tail, netfs_folio_trace_kill, netfs_folio_trace_kill_cc,
    netfs_folio_trace_kill_g, netfs_folio_trace_kill_s, netfs_folio_trace_mkwrite,
    netfs_folio_trace_mkwrite_plus, netfs_folio_trace_not_under_wback, netfs_folio_trace_not_locked,
    netfs_folio_trace_put, netfs_folio_trace_read, netfs_folio_trace_read_done,
    netfs_folio_trace_read_gaps, netfs_folio_trace_read_unlock, netfs_folio_trace_redirtied,
    netfs_folio_trace_store, netfs_folio_trace_store_copy, netfs_folio_trace_store_plus,
    netfs_folio_trace_wthru, netfs_folio_trace_wthru_plus,
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum netfs_collect_contig_trace { netfs_contig_trace_collect, netfs_contig_trace_jump, netfs_contig_trace_unlock }
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum netfs_donate_trace { netfs_trace_donate_tail_to_prev, netfs_trace_donate_to_prev, netfs_trace_donate_to_next, netfs_trace_donate_to_deferred_next }
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum netfs_folioq_trace { netfs_trace_folioq_alloc_buffer, netfs_trace_folioq_clear, netfs_trace_folioq_delete, netfs_trace_folioq_make_space, netfs_trace_folioq_rollbuf_init, netfs_trace_folioq_read_progress }

// netfs_rreq_origins and netfs_sreq_sources are enum tables exported by the
// tracing subsystem; their identifiers and printable strings are preserved.
pub const NETFS_RREQ_ORIGINS: &[(&str, &str)] = &[("NETFS_READAHEAD", "RA"), ("NETFS_READPAGE", "RP"), ("NETFS_READ_GAPS", "RG"), ("NETFS_READ_SINGLE", "R1"), ("NETFS_READ_FOR_WRITE", "RW"), ("NETFS_UNBUFFERED_READ", "UR"), ("NETFS_DIO_READ", "DR"), ("NETFS_WRITEBACK", "WB"), ("NETFS_WRITEBACK_SINGLE", "W1"), ("NETFS_WRITETHROUGH", "WT"), ("NETFS_UNBUFFERED_WRITE", "UW"), ("NETFS_DIO_WRITE", "DW"), ("NETFS_PGPRIV2_COPY_TO_CACHE", "2C")];

// TRACE_EVENT declarations and TP_fast_assign/TP_printk bodies are retained
// verbatim as tracepoint contracts in the source header.  Their expansion is
// provided by the external Linux tracepoint implementation.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
