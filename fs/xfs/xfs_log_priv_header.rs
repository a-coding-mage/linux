// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2000-2003,2005 Silicon Graphics, Inc. All Rights Reserved. */

// Dependency declarations from the C header are intentionally omitted; the
// corresponding names are supplied by other translated units.

#[repr(C)]
pub struct xfs_log_iovec {
    pub i_addr: *mut core::ffi::c_void,
    pub i_len: core::ffi::c_int,
    pub i_type: uint,
}

#[repr(C)]
pub struct xfs_log_vec {
    pub lv_list: list_head,
    pub lv_order_id: u32,
    pub lv_niovecs: core::ffi::c_int,
    pub lv_iovecp: *mut xfs_log_iovec,
    pub lv_item: *mut xfs_log_item,
    pub lv_buf: *mut core::ffi::c_char,
    pub lv_bytes: core::ffi::c_int,
    pub lv_buf_used: core::ffi::c_int,
    pub lv_alloc_size: core::ffi::c_int,
}

#[inline]
pub unsafe fn xlog_get_client_id(i: __be32) -> uint {
    be32_to_cpu(i) >> 24
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum xlog_iclog_state {
    XLOG_STATE_ACTIVE,
    XLOG_STATE_WANT_SYNC,
    XLOG_STATE_SYNCING,
    XLOG_STATE_DONE_SYNC,
    XLOG_STATE_CALLBACK,
    XLOG_STATE_DIRTY,
}

pub const XLOG_ICL_NEED_FLUSH: uint = 1u << 0;
pub const XLOG_ICL_NEED_FUA: uint = 1u << 1;
pub const XLOG_TIC_PERM_RESERV: uint = 1u << 0;
pub const XLOG_STATE_COVER_IDLE: core::ffi::c_int = 0;
pub const XLOG_STATE_COVER_NEED: core::ffi::c_int = 1;
pub const XLOG_STATE_COVER_DONE: core::ffi::c_int = 2;
pub const XLOG_STATE_COVER_NEED2: core::ffi::c_int = 3;
pub const XLOG_STATE_COVER_DONE2: core::ffi::c_int = 4;
pub const XLOG_COVER_OPS: core::ffi::c_int = 5;

#[repr(C)]
pub struct xlog_ticket {
    pub t_queue: list_head,
    pub t_task: *mut task_struct,
    pub t_tid: xlog_tid_t,
    pub t_ref: atomic_t,
    pub t_curr_res: core::ffi::c_int,
    pub t_unit_res: core::ffi::c_int,
    pub t_ocnt: core::ffi::c_char,
    pub t_cnt: core::ffi::c_char,
    pub t_flags: u8,
    pub t_iclog_hdrs: core::ffi::c_int,
}

#[repr(C)]
pub struct xlog_in_core {
    pub ic_force_wait: wait_queue_head_t,
    pub ic_write_wait: wait_queue_head_t,
    pub ic_next: *mut xlog_in_core,
    pub ic_prev: *mut xlog_in_core,
    pub ic_log: *mut xlog,
    pub ic_size: u32,
    pub ic_offset: u32,
    pub ic_state: xlog_iclog_state,
    pub ic_flags: core::ffi::c_uint,
    pub ic_datap: *mut core::ffi::c_void,
    pub ic_callbacks: list_head,
    pub ic_refcnt: atomic_t,
    pub ic_header: *mut xlog_rec_header,
    // Present only under the C DEBUG configuration; this represents the
    // single-bit C bitfield as a Rust bool.
    #[cfg(feature = "DEBUG")]
    pub ic_fail_crc: bool,
    pub ic_sema: semaphore,
    pub ic_end_io_work: work_struct,
    pub ic_bio: bio,
    pub ic_bvec: [bio_vec; 0],
}

#[repr(C)]
pub struct xfs_cil_ctx {
    pub cil: *mut xfs_cil,
    pub sequence: xfs_csn_t,
    pub start_lsn: xfs_lsn_t,
    pub commit_lsn: xfs_lsn_t,
    pub commit_iclog: *mut xlog_in_core,
    pub ticket: *mut xlog_ticket,
    pub space_used: atomic_t,
    pub busy_extents: xfs_busy_extents,
    pub log_items: list_head,
    pub lv_chain: list_head,
    pub iclog_entry: list_head,
    pub committing: list_head,
    pub push_work: work_struct,
    pub order_id: atomic_t,
    pub cil_pcpmask: cpumask,
}

#[repr(C)]
pub struct xlog_cil_pcp {
    pub space_used: i32,
    pub space_reserved: u32,
    pub busy_extents: list_head,
    pub log_items: list_head,
}

#[repr(C)]
pub struct xfs_cil {
    pub xc_log: *mut xlog,
    pub xc_flags: core::ffi::c_ulong,
    pub xc_iclog_hdrs: atomic_t,
    pub xc_push_wq: *mut workqueue_struct,
    pub xc_ctx_lock: rw_semaphore,
    pub xc_ctx: *mut xfs_cil_ctx,
    pub xc_push_lock: spinlock_t,
    pub xc_push_seq: xfs_csn_t,
    pub xc_push_commit_stable: bool,
    pub xc_committing: list_head,
    pub xc_commit_wait: wait_queue_head_t,
    pub xc_start_wait: wait_queue_head_t,
    pub xc_current_sequence: xfs_csn_t,
    pub xc_push_wait: wait_queue_head_t,
    pub xc_pcp: *mut core::ffi::c_void,
}

pub const XLOG_CIL_EMPTY: core::ffi::c_ulong = 1;
pub const XLOG_CIL_PCP_SPACE: core::ffi::c_ulong = 2;

#[inline]
pub unsafe fn XLOG_CIL_SPACE_LIMIT(log: *mut xlog) -> core::ffi::c_int {
    min_t(core::ffi::c_int, (*log).l_logsize >> 3,
        BBTOB(XLOG_TOTAL_REC_SHIFT(log)) << 4)
}

#[inline]
pub unsafe fn XLOG_CIL_BLOCKING_SPACE_LIMIT(log: *mut xlog) -> core::ffi::c_int {
    XLOG_CIL_SPACE_LIMIT(log) * 2
}

#[repr(C)]
pub struct xlog_grant_head {
    pub lock: spinlock_t,
    pub waiters: list_head,
    pub grant: atomic64_t,
}

#[repr(C)]
pub struct xlog {
    pub l_mp: *mut xfs_mount,
    pub l_ailp: *mut xfs_ail,
    pub l_cilp: *mut xfs_cil,
    pub l_targ: *mut xfs_buftarg,
    pub l_ioend_workqueue: *mut workqueue_struct,
    pub l_work: delayed_work,
    pub l_opstate: core::ffi::c_long,
    pub l_quotaoffs_flag: uint,
    pub l_buf_cancel_table: *mut list_head,
    pub r_dfops: list_head,
    pub l_iclog_hsize: core::ffi::c_int,
    pub l_sectBBsize: uint,
    pub l_iclog_size: core::ffi::c_int,
    pub l_iclog_bufs: core::ffi::c_int,
    pub l_logBBstart: xfs_daddr_t,
    pub l_logsize: core::ffi::c_int,
    pub l_logBBsize: core::ffi::c_int,
    pub l_flush_wait: wait_queue_head_t,
    pub l_covered_state: core::ffi::c_int,
    pub l_iclog: *mut xlog_in_core,
    pub l_icloglock: spinlock_t,
    pub l_curr_cycle: core::ffi::c_int,
    pub l_prev_cycle: core::ffi::c_int,
    pub l_curr_block: core::ffi::c_int,
    pub l_prev_block: core::ffi::c_int,
    pub l_tail_lsn: atomic64_t,
    pub l_reserve_head: xlog_grant_head,
    pub l_write_head: xlog_grant_head,
    pub l_tail_space: u64,
    pub l_kobj: xfs_kobj,
    pub l_recovery_lsn: xfs_lsn_t,
    pub l_iclog_roundoff: u32,
}

pub const XLOG_ACTIVE_RECOVERY: core::ffi::c_int = 0;
pub const XLOG_RECOVERY_NEEDED: core::ffi::c_int = 1;
pub const XLOG_IO_ERROR: core::ffi::c_int = 2;
pub const XLOG_TAIL_WARN: core::ffi::c_int = 3;
pub const XLOG_SHUTDOWN_STARTED: core::ffi::c_int = 4;

#[inline]
pub unsafe fn xlog_recovery_needed(log: *mut xlog) -> bool { test_bit(XLOG_RECOVERY_NEEDED, &mut (*log).l_opstate) }
#[inline]
pub unsafe fn xlog_in_recovery(log: *mut xlog) -> bool { test_bit(XLOG_ACTIVE_RECOVERY, &mut (*log).l_opstate) }
#[inline]
pub unsafe fn xlog_is_shutdown(log: *mut xlog) -> bool { test_bit(XLOG_IO_ERROR, &mut (*log).l_opstate) }

#[inline]
pub unsafe fn xlog_shutdown_wait(log: *mut xlog) {
    wait_var_event(&mut (*log).l_opstate, xlog_is_shutdown(log));
}

extern "C" {
    pub fn xlog_recover(log: *mut xlog) -> core::ffi::c_int;
    pub fn xlog_recover_finish(log: *mut xlog) -> core::ffi::c_int;
    pub fn xlog_recover_cancel(log: *mut xlog);
    pub fn xlog_cksum(log: *mut xlog, rhead: *mut xlog_rec_header, dp: *mut core::ffi::c_char, hdrsize: core::ffi::c_uint, size: core::ffi::c_uint) -> __le32;
    pub static mut xfs_log_ticket_cache: *mut kmem_cache;
    pub fn xlog_ticket_alloc(log: *mut xlog, unit_bytes: core::ffi::c_int, count: core::ffi::c_int, permanent: bool) -> *mut xlog_ticket;
    pub fn xlog_print_tic_res(mp: *mut xfs_mount, ticket: *mut xlog_ticket);
    pub fn xlog_print_trans(tp: *mut xfs_trans);
    pub fn xlog_write(log: *mut xlog, ctx: *mut xfs_cil_ctx, lv_chain: *mut list_head, tic: *mut xlog_ticket, len: u32) -> core::ffi::c_int;
    pub fn xlog_write_one_vec(log: *mut xlog, ctx: *mut xfs_cil_ctx, reg: *mut xfs_log_iovec, ticket: *mut xlog_ticket) -> core::ffi::c_int;
    pub fn xfs_log_ticket_ungrant(log: *mut xlog, ticket: *mut xlog_ticket);
    pub fn xfs_log_ticket_regrant(log: *mut xlog, ticket: *mut xlog_ticket);
    pub fn xlog_state_switch_iclogs(log: *mut xlog, iclog: *mut xlog_in_core, eventual_size: core::ffi::c_int);
    pub fn xlog_state_release_iclog(log: *mut xlog, iclog: *mut xlog_in_core, ticket: *mut xlog_ticket) -> core::ffi::c_int;
    pub fn xlog_cil_init(log: *mut xlog) -> core::ffi::c_int;
    pub fn xlog_cil_init_post_recovery(log: *mut xlog);
    pub fn xlog_cil_destroy(log: *mut xlog);
    pub fn xlog_cil_empty(log: *mut xlog) -> bool;
    pub fn xlog_cil_commit(log: *mut xlog, tp: *mut xfs_trans, commit_seq: *mut xfs_csn_t, regrant: bool);
    pub fn xlog_cil_set_ctx_write_state(ctx: *mut xfs_cil_ctx, iclog: *mut xlog_in_core);
    pub fn xlog_cil_flush(log: *mut xlog);
    pub fn xlog_cil_force_seq(log: *mut xlog, sequence: xfs_csn_t) -> xfs_lsn_t;
    pub fn xlog_wait_on_iclog(iclog: *mut xlog_in_core) -> core::ffi::c_int;
    pub fn xlog_grant_return_space(log: *mut xlog, old_head: xfs_lsn_t, new_head: xfs_lsn_t);
}

#[inline]
pub unsafe fn xlog_cil_force(log: *mut xlog) {
    xlog_cil_force_seq(log, (*(*log).l_cilp).xc_current_sequence);
}

#[inline]
pub unsafe fn xlog_crack_atomic_lsn(lsn: *mut atomic64_t, cycle: *mut uint, block: *mut uint) {
    let val = atomic64_read(lsn);
    *cycle = CYCLE_LSN(val);
    *block = BLOCK_LSN(val);
}

#[inline]
pub unsafe fn xlog_assign_atomic_lsn(lsn: *mut atomic64_t, cycle: uint, block: uint) {
    atomic64_set(lsn, xlog_assign_lsn(cycle, block));
}

#[inline]
pub unsafe fn xlog_lsn_sub(log: *mut xlog, high: xfs_lsn_t, low: xfs_lsn_t) -> u64 {
    let hi_cycle = CYCLE_LSN(high);
    let hi_block = BLOCK_LSN(high);
    let lo_cycle = CYCLE_LSN(low);
    let lo_block = BLOCK_LSN(low);
    if hi_cycle == lo_cycle { return BBTOB(hi_block - lo_block); }
    ASSERT((hi_cycle == lo_cycle + 1) || xlog_is_shutdown(log));
    (*log).l_logsize as u64 - BBTOB(lo_block - hi_block)
}

#[inline]
pub unsafe fn xlog_valid_lsn(log: *mut xlog, lsn: xfs_lsn_t) -> bool {
    let mut cur_cycle = READ_ONCE((*log).l_curr_cycle);
    smp_rmb();
    let mut cur_block = READ_ONCE((*log).l_curr_block);
    let mut valid = true;
    if CYCLE_LSN(lsn) > cur_cycle || (CYCLE_LSN(lsn) == cur_cycle && BLOCK_LSN(lsn) > cur_block) {
        spin_lock(&mut (*log).l_icloglock);
        cur_cycle = (*log).l_curr_cycle;
        cur_block = (*log).l_curr_block;
        spin_unlock(&mut (*log).l_icloglock);
        if CYCLE_LSN(lsn) > cur_cycle || (CYCLE_LSN(lsn) == cur_cycle && BLOCK_LSN(lsn) > cur_block) { valid = false; }
    }
    valid
}

#[inline]
pub unsafe fn xlog_kvmalloc(buf_size: usize) -> *mut core::ffi::c_void {
    let mut flags = GFP_KERNEL;
    flags &= !__GFP_DIRECT_RECLAIM;
    flags |= __GFP_NOWARN | __GFP_NORETRY;
    loop {
        let mut p = kmalloc(buf_size, flags);
        if p.is_null() { p = vmalloc(buf_size); }
        if !p.is_null() { return p; }
    }
}

#[inline]
pub fn xlog_item_space(mut niovecs: core::ffi::c_uint, mut nbytes: core::ffi::c_uint) -> core::ffi::c_uint {
    nbytes += niovecs * (core::mem::size_of::<u64>() as u32 + core::mem::size_of::<xlog_op_header>() as u32);
    round_up(nbytes, core::mem::size_of::<u64>() as u32)
}

#[inline]
pub unsafe fn xlog_cycle_data(rhead: *mut xlog_rec_header, i: core::ffi::c_uint) -> *mut __be32 {
    if i >= XLOG_CYCLE_DATA_SIZE {
        let j = i / XLOG_CYCLE_DATA_SIZE;
        let k = i % XLOG_CYCLE_DATA_SIZE;
        return &mut (*rhead).h_ext[(j - 1) as usize].xh_cycle_data[k as usize];
    }
    &mut (*rhead).h_cycle_data[i as usize]
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
