// SPDX-License-Identifier: GPL-2.0

// Translated from C source that included "kublk.h"; required external symbols
// and C-compatible types are expected to be supplied by the surrounding crate.

pub const IORING_NOP_INJECT_RESULT: u32 = 1u32 << 0;
pub const IORING_NOP_FIXED_BUFFER: u32 = 1u32 << 3;

unsafe fn ublk_null_tgt_init(ctx: *const dev_ctx, dev: *mut ublk_dev) -> i32 {
    let info: *const ublksrv_ctrl_dev_info = &(*dev).dev_info;
    let dev_size: libc::c_ulong = 250u64.wrapping_shl(30) as libc::c_ulong;

    (*dev).tgt.dev_size = dev_size;
    (*dev).tgt.params = ublk_params {
        types: UBLK_PARAM_TYPE_BASIC | UBLK_PARAM_TYPE_DMA_ALIGN | UBLK_PARAM_TYPE_SEGMENT,
        basic: ublk_param_basic {
            logical_bs_shift: 9,
            physical_bs_shift: 12,
            io_opt_shift: 12,
            io_min_shift: 9,
            max_sectors: (*info).max_io_buf_bytes >> 9,
            dev_sectors: dev_size >> 9,
        },
        dma: ublk_param_dma {
            alignment: 4095,
        },
        seg: ublk_param_segment {
            seg_boundary_mask: 4095,
            max_segment_size: 32 << 10,
            max_segments: 32,
        },
    };
    ublk_set_integrity_params(ctx, &mut (*dev).tgt.params);

    if ((*info).flags & UBLK_F_SUPPORT_ZERO_COPY) != 0 {
        (*dev).tgt.cq_depth = 2 * (*info).queue_depth;
        (*dev).tgt.sq_depth = (*dev).tgt.cq_depth;
    }
    0
}

unsafe fn __setup_nop_io(
    tag: i32,
    iod: *const ublksrv_io_desc,
    sqe: *mut io_uring_sqe,
    q_id: i32,
    buf_idx: libc::c_uint,
) {
    let ublk_op: libc::c_uint = ublksrv_get_op(iod);

    io_uring_prep_nop(sqe);
    (*sqe).buf_index = buf_idx as _;
    (*sqe).flags |= IOSQE_FIXED_FILE;
    (*sqe).rw_flags = IORING_NOP_FIXED_BUFFER | IORING_NOP_INJECT_RESULT;
    (*sqe).len = (*iod).nr_sectors << 9; /* injected result */
    (*sqe).user_data = build_user_data(tag, ublk_op, 0, q_id, 1);
}

unsafe fn null_queue_zc_io(t: *mut ublk_thread, q: *mut ublk_queue, tag: i32) -> i32 {
    let iod: *const ublksrv_io_desc = ublk_get_iod(q, tag);
    let mut sqe: [*mut io_uring_sqe; 3] = [core::ptr::null_mut(); 3];
    let buf_idx: libc::c_ushort = ublk_io_buf_idx(t, q, tag);

    ublk_io_alloc_sqes(t, sqe.as_mut_ptr(), 3);

    io_uring_prep_buf_register(sqe[0], q, tag, (*q).q_id, buf_idx);
    (*sqe[0]).user_data = build_user_data(
        tag,
        ublk_cmd_op_nr((*sqe[0]).cmd_op),
        0,
        (*q).q_id,
        1,
    );
    (*sqe[0]).flags |= IOSQE_CQE_SKIP_SUCCESS | IOSQE_IO_HARDLINK;

    __setup_nop_io(tag, iod, sqe[1], (*q).q_id, buf_idx as libc::c_uint);
    (*sqe[1]).flags |= IOSQE_IO_HARDLINK;

    io_uring_prep_buf_unregister(sqe[2], q, tag, (*q).q_id, buf_idx);
    (*sqe[2]).user_data = build_user_data(
        tag,
        ublk_cmd_op_nr((*sqe[2]).cmd_op),
        0,
        (*q).q_id,
        1,
    );

    // buf register is marked as IOSQE_CQE_SKIP_SUCCESS
    2
}

unsafe fn null_queue_auto_zc_io(t: *mut ublk_thread, q: *mut ublk_queue, tag: i32) -> i32 {
    let iod: *const ublksrv_io_desc = ublk_get_iod(q, tag);
    let mut sqe: [*mut io_uring_sqe; 1] = [core::ptr::null_mut(); 1];

    ublk_io_alloc_sqes(t, sqe.as_mut_ptr(), 1);
    __setup_nop_io(
        tag,
        iod,
        sqe[0],
        (*q).q_id,
        ublk_io_buf_idx(t, q, tag) as libc::c_uint,
    );
    1
}

unsafe fn ublk_null_io_done(
    t: *mut ublk_thread,
    q: *mut ublk_queue,
    cqe: *const io_uring_cqe,
) {
    let tag: libc::c_uint = user_data_to_tag((*cqe).user_data);
    let op: libc::c_uint = user_data_to_op((*cqe).user_data);
    let io: *mut ublk_io = ublk_get_io(q, tag);

    if (*cqe).res < 0 || op != ublk_cmd_op_nr(UBLK_U_IO_UNREGISTER_IO_BUF) {
        if (*io).result == 0 {
            (*io).result = (*cqe).res;
        }
        if (*cqe).res < 0 {
            ublk_err(
                c"%s: io failed op %x user_data %lx\n".as_ptr(),
                c"ublk_null_io_done".as_ptr(),
                op,
                (*cqe).user_data,
            );
        }
    }

    /* buffer register op is IOSQE_CQE_SKIP_SUCCESS */
    if op == ublk_cmd_op_nr(UBLK_U_IO_REGISTER_IO_BUF) {
        (*io).tgt_ios += 1;
    }

    if ublk_completed_tgt_io(t, q, tag) != 0 {
        ublk_complete_io(t, q, tag as i32, (*io).result);
    }
}

unsafe fn ublk_null_queue_io(t: *mut ublk_thread, q: *mut ublk_queue, tag: i32) -> i32 {
    let iod: *const ublksrv_io_desc = ublk_get_iod(q, tag);
    let auto_zc: libc::c_uint = ublk_queue_use_auto_zc(q);
    let zc: libc::c_uint = ublk_queue_use_zc(q);
    let queued: i32;

    if auto_zc != 0 && ublk_io_auto_zc_fallback(iod) == 0 {
        queued = null_queue_auto_zc_io(t, q, tag);
    } else if zc != 0 {
        queued = null_queue_zc_io(t, q, tag);
    } else {
        ublk_complete_io(t, q, tag, (*iod).nr_sectors << 9);
        return 0;
    }
    ublk_queued_tgt_io(t, q, tag, queued);
    0
}

/*
 * return invalid buffer index for triggering auto buffer register failure,
 * then UBLK_IO_RES_NEED_REG_BUF handling is covered
 */
unsafe fn ublk_null_buf_index(
    t: *const ublk_thread,
    q: *const ublk_queue,
    tag: i32,
) -> libc::c_ushort {
    if ublk_queue_auto_zc_fallback(q) != 0 {
        return (-1i32) as libc::c_ushort;
    }
    ublk_io_buf_idx(t, q, tag)
}

pub static null_tgt_ops: ublk_tgt_ops = ublk_tgt_ops {
    name: c"null".as_ptr(),
    init_tgt: Some(ublk_null_tgt_init),
    queue_io: Some(ublk_null_queue_io),
    tgt_io_done: Some(ublk_null_io_done),
    buf_index: Some(ublk_null_buf_index),
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
