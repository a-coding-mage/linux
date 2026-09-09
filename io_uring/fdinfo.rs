// SPDX-License-Identifier: GPL-2.0
// Translated from fdinfo.c. Kernel and project dependencies are supplied externally.

#[cfg(feature = "CONFIG_NET_RX_BUSY_POLL")]
unsafe fn common_tracking_show_fdinfo(
    ctx: *mut io_ring_ctx,
    m: *mut seq_file,
    tracking_strategy: *const core::ffi::c_char,
) {
    seq_puts(m, "NAPI:\tenabled\n");
    seq_printf(m, "napi tracking:\t%s\n", tracking_strategy);
    seq_printf(m, "napi_busy_poll_dt:\t%llu\n", (*ctx).napi_busy_poll_dt);
    if (*ctx).napi_prefer_busy_poll {
        seq_puts(m, "napi_prefer_busy_poll:\ttrue\n");
    } else {
        seq_puts(m, "napi_prefer_busy_poll:\tfalse\n");
    }
}

#[cfg(feature = "CONFIG_NET_RX_BUSY_POLL")]
unsafe fn napi_show_fdinfo(ctx: *mut io_ring_ctx, m: *mut seq_file) {
    let mode: u32 = read_once(&(*ctx).napi_track_mode);
    match mode {
        IO_URING_NAPI_TRACKING_INACTIVE => seq_puts(m, "NAPI:\tdisabled\n"),
        IO_URING_NAPI_TRACKING_DYNAMIC => common_tracking_show_fdinfo(ctx, m, c"dynamic".as_ptr()),
        IO_URING_NAPI_TRACKING_STATIC => common_tracking_show_fdinfo(ctx, m, c"static".as_ptr()),
        _ => seq_printf(m, "NAPI:\tunknown mode (%u)\n", mode),
    }
}

#[cfg(not(feature = "CONFIG_NET_RX_BUSY_POLL"))]
unsafe fn napi_show_fdinfo(_ctx: *mut io_ring_ctx, _m: *mut seq_file) {}

unsafe fn __io_uring_show_fdinfo(ctx: *mut io_ring_ctx, m: *mut seq_file) {
    let r = (*ctx).rings;
    let sq_mask = (*ctx).sq_entries - 1;
    let cq_mask = (*ctx).cq_entries - 1;
    let mut sq_head = read_once(&(*r).sq.head);
    let sq_tail = read_once(&(*r).sq.tail);
    let mut cq_head = read_once(&(*r).cq.head);
    let cq_tail = read_once(&(*r).cq.tail);
    let sq_shift = if (*ctx).flags & IORING_SETUP_SQE128 != 0 { 1 } else { 0 };
    let mut sq_entries;
    let mut cq_entries;
    let mut sq_pid: i32 = -1;
    let mut sq_cpu: i32 = -1;
    let mut sq_total_time: u64 = 0;
    let mut sq_work_time: u64 = 0;
    let mut i: u32;

    seq_printf(m, "SqMask:\t0x%x\n", sq_mask);
    seq_printf(m, "SqHead:\t%u\n", sq_head);
    seq_printf(m, "SqTail:\t%u\n", sq_tail);
    seq_printf(m, "CachedSqHead:\t%u\n", data_race((*ctx).cached_sq_head));
    seq_printf(m, "CqMask:\t0x%x\n", cq_mask);
    seq_printf(m, "CqHead:\t%u\n", cq_head);
    seq_printf(m, "CqTail:\t%u\n", cq_tail);
    seq_printf(m, "CachedCqTail:\t%u\n", data_race((*ctx).cached_cq_tail));
    seq_printf(m, "SQEs:\t%u\n", sq_tail - sq_head);
    sq_entries = core::cmp::min(sq_tail - sq_head, (*ctx).sq_entries);
    i = 0;
    while i < sq_entries {
        let entry = i + sq_head;
        let sq_idx = if (*ctx).flags & IORING_SETUP_NO_SQARRAY != 0 {
            entry & sq_mask
        } else {
            read_once(&(*ctx).sq_array[(entry & sq_mask) as usize])
        };
        if sq_idx <= sq_mask {
            let sqe = (*ctx).sq_sqes.add((sq_idx << sq_shift) as usize);
            let mut opcode = read_once(&(*sqe).opcode);
            if opcode < IORING_OP_LAST {
                opcode = array_index_nospec(opcode, IORING_OP_LAST);
                let mut sqe128 = false;
                if sq_shift != 0 {
                    sqe128 = true;
                } else if io_issue_defs[opcode as usize].is_128 {
                    if (*ctx).flags & IORING_SETUP_SQE_MIXED == 0 {
                        seq_printf(m, "%5u: invalid sqe, 128B entry on non-mixed sq\n", sq_idx);
                        break;
                    }
                    if sq_idx == sq_mask {
                        seq_printf(m, "%5u: corrupted sqe, wrapping 128B entry\n", sq_idx);
                        break;
                    }
                    sq_head += 1;
                    i += 1;
                    sqe128 = true;
                }
                seq_printf(m, "%5u: opcode:%s, fd:%d, flags:%x, off:%llu, addr:0x%llx, rw_flags:0x%x buf_index:%d user_data:%llu", sq_idx, io_uring_get_opcode(opcode), (*sqe).fd, (*sqe).flags, (*sqe).off, (*sqe).addr, (*sqe).rw_flags, (*sqe).buf_index, (*sqe).user_data);
                if sqe128 {
                    let mut sqeb = (sqe.add(1)) as *mut u64;
                    let size = core::mem::size_of::<io_uring_sqe>() / core::mem::size_of::<u64>();
                    for j in 0..size { seq_printf(m, ", e%d:0x%llx", j, *sqeb); sqeb = sqeb.add(1); }
                }
                seq_printf(m, "\n");
                cond_resched();
            }
        }
        i += 1;
    }
    seq_printf(m, "CQEs:\t%u\n", cq_tail - cq_head);
    cq_entries = core::cmp::min(cq_tail - cq_head, (*ctx).cq_entries);
    i = 0;
    while i < cq_entries {
        let cqe = &mut (*r).cqes[(cq_head & cq_mask) as usize];
        let cqe32 = cqe.flags & IORING_CQE_F_32 != 0 || (*ctx).flags & IORING_SETUP_CQE32 != 0;
        seq_printf(m, "%5u: user_data:%llu, res:%d, flags:%x", cq_head & cq_mask, cqe.user_data, cqe.res, cqe.flags);
        if cqe32 { seq_printf(m, ", extra1:%llu, extra2:%llu", cqe.big_cqe[0], cqe.big_cqe[1]); }
        seq_printf(m, "\n"); cq_head += 1;
        if cqe32 { cq_head += 1; i += 1; }
        cond_resched(); i += 1;
    }
    seq_printf(m, "SqThread:\t%d\n", sq_pid);
    seq_printf(m, "SqThreadCpu:\t%d\n", sq_cpu);
    seq_printf(m, "SqTotalTime:\t%llu\n", sq_total_time);
    seq_printf(m, "SqWorkTime:\t%llu\n", sq_work_time);
    seq_printf(m, "UserFiles:\t%u\n", (*ctx).file_table.data.nr);
    for i in 0..(*ctx).file_table.data.nr { if !(*ctx).file_table.data.nodes[i as usize].is_null() { if let Some(f) = io_slot_file((*ctx).file_table.data.nodes[i as usize]) { seq_printf(m, "%5u: ", i); seq_file_path(m, f, " \t\n\\"); seq_puts(m, "\n"); } } }
    seq_printf(m, "UserBufs:\t%u\n", (*ctx).buf_table.nr);
    for i in 0..(*ctx).buf_table.nr { let buf = if !(*ctx).buf_table.nodes[i as usize].is_null() { (*(*ctx).buf_table.nodes[i as usize]).buf } else { core::ptr::null_mut() }; if !buf.is_null() { seq_printf(m, "%5u: 0x%llx/%zu\n", i, (*buf).ubuf, (*buf).len); } else { seq_printf(m, "%5u: <none>\n", i); } }
    seq_puts(m, "PollList:\n");
    for i in 0..(1u32 << (*ctx).cancel_table.hash_bits) { let hb = &mut (*ctx).cancel_table.hbs[i as usize]; hlist_for_each_entry(hb, |req: *mut io_kiocb| { seq_printf(m, "  op=%d, task_works=%d\n", (*req).opcode, task_work_pending((*req).tctx.task)); }); }
    seq_puts(m, "CqOverflowList:\n"); spin_lock(&mut (*ctx).completion_lock); list_for_each_entry(&mut (*ctx).cq_overflow_list, |ocqe: *mut io_overflow_cqe| { let cqe = &(*ocqe).cqe; seq_printf(m, "  user_data=%llu, res=%d, flags=%x\n", cqe.user_data, cqe.res, cqe.flags); }); spin_unlock(&mut (*ctx).completion_lock);
    napi_show_fdinfo(ctx, m);
}

// Caller holds a reference to the file already, so no extra reference is needed.
pub unsafe fn io_uring_show_fdinfo(m: *mut seq_file, file: *mut file) {
    let ctx = (*file).private_data as *mut io_ring_ctx;
    if mutex_trylock(&mut (*ctx).uring_lock) { __io_uring_show_fdinfo(ctx, m); mutex_unlock(&mut (*ctx).uring_lock); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
