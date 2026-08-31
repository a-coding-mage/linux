// SPDX-License-Identifier: GPL-2.0

/*
 * Fault injection ublk target. Hack this up however you like for
 * testing specific behaviors of ublk_drv. Currently is a null target
 * with a configurable delay before completing each I/O. This delay can
 * be used to test ublk_drv's handling of I/O outstanding to the ublk
 * server when it dies.
 */

/* C dependency intent: #include "kublk.h" */
use crate::*;

#[repr(C)]
pub struct fi_opts {
    pub delay_ns: longlong,
    pub die_during_fetch: bool,
}

unsafe extern "C" fn ublk_fault_inject_tgt_init(
    ctx: *const dev_ctx,
    dev: *mut ublk_dev,
) -> ::std::os::raw::c_int {
    let info: *const ublksrv_ctrl_dev_info = unsafe { &(*dev).dev_info };
    let dev_size: ::std::os::raw::c_ulong = 250 as ::std::os::raw::c_ulong << 30;
    let mut opts: *mut fi_opts = ::std::ptr::null_mut();

    if unsafe { (*ctx).auto_zc_fallback } {
        unsafe {
            ublk_err(
                c"%s: not support auto_zc_fallback\n".as_ptr(),
                c"ublk_fault_inject_tgt_init".as_ptr(),
            );
        }
        return -EINVAL;
    }

    unsafe {
        (*dev).tgt.dev_size = dev_size;
        (*dev).tgt.params = ublk_params {
            types: UBLK_PARAM_TYPE_BASIC,
            basic: ublk_params_basic {
                logical_bs_shift: 9,
                physical_bs_shift: 12,
                io_opt_shift: 12,
                io_min_shift: 9,
                max_sectors: (*info).max_io_buf_bytes >> 9,
                dev_sectors: dev_size >> 9,
            },
        };
        ublk_set_integrity_params(ctx, &mut (*dev).tgt.params);
    }

    opts = unsafe {
        calloc(
            1,
            ::std::mem::size_of::<fi_opts>() as ::std::os::raw::c_ulong,
        ) as *mut fi_opts
    };
    if opts.is_null() {
        unsafe {
            ublk_err(
                c"%s: couldn't allocate memory for opts\n".as_ptr(),
                c"ublk_fault_inject_tgt_init".as_ptr(),
            );
        }
        return -ENOMEM;
    }

    unsafe {
        (*opts).delay_ns = (*ctx).fault_inject.delay_us * 1000;
        (*opts).die_during_fetch = (*ctx).fault_inject.die_during_fetch;
        (*dev).private_data = opts as *mut ::std::ffi::c_void;
    }

    0
}

unsafe extern "C" fn ublk_fault_inject_pre_fetch_io(
    t: *mut ublk_thread,
    q: *mut ublk_queue,
    tag: ::std::os::raw::c_int,
    batch: bool,
) {
    let opts: *mut fi_opts = unsafe { (*(*q).dev).private_data as *mut fi_opts };

    if unsafe { !(*opts).die_during_fetch } {
        return;
    }

    /*
     * Each queue fetches its IOs in increasing order of tags, so
     * dying just before we're about to fetch tag 1 (regardless of
     * what queue we're on) guarantees that we've fetched a nonempty
     * proper subset of the tags on that queue.
     */
    if tag == 1 {
        /*
         * Ensure our commands are actually live in the kernel
         * before we die.
         */
        unsafe {
            io_uring_submit(&mut (*t).ring);
            raise(SIGKILL);
        }
    }
}

unsafe extern "C" fn ublk_fault_inject_queue_io(
    t: *mut ublk_thread,
    q: *mut ublk_queue,
    tag: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    let iod: *const ublksrv_io_desc = unsafe { ublk_get_iod(q, tag) };
    let mut sqe: *mut io_uring_sqe = ::std::ptr::null_mut();
    let opts: *mut fi_opts = unsafe { (*(*q).dev).private_data as *mut fi_opts };
    let mut ts = __kernel_timespec {
        tv_sec: 0,
        tv_nsec: unsafe { (*opts).delay_ns },
    };

    unsafe {
        ublk_io_alloc_sqes(t, &mut sqe, 1);
        io_uring_prep_timeout(sqe, &mut ts, 1, 0);
        (*sqe).user_data = build_user_data(tag, ublksrv_get_op(iod), 0, (*q).q_id, 1);

        ublk_queued_tgt_io(t, q, tag, 1);
    }

    0
}

unsafe extern "C" fn ublk_fault_inject_tgt_io_done(
    t: *mut ublk_thread,
    q: *mut ublk_queue,
    cqe: *const io_uring_cqe,
) {
    let tag: ::std::os::raw::c_uint = unsafe { user_data_to_tag((*cqe).user_data) };
    let iod: *const ublksrv_io_desc = unsafe { ublk_get_iod(q, tag as ::std::os::raw::c_int) };

    if unsafe { (*cqe).res != -ETIME } {
        unsafe {
            ublk_err(
                c"%s: unexpected cqe res %d\n".as_ptr(),
                c"ublk_fault_inject_tgt_io_done".as_ptr(),
                (*cqe).res,
            );
        }
    }

    if unsafe { ublk_completed_tgt_io(t, q, tag as ::std::os::raw::c_int) != 0 } {
        unsafe {
            ublk_complete_io(
                t,
                q,
                tag as ::std::os::raw::c_int,
                (*iod).nr_sectors << 9,
            );
        }
    } else {
        unsafe {
            ublk_err(
                c"%s: io not complete after 1 cqe\n".as_ptr(),
                c"ublk_fault_inject_tgt_io_done".as_ptr(),
            );
        }
    }
}

unsafe extern "C" fn ublk_fault_inject_cmd_line(
    ctx: *mut dev_ctx,
    argc: ::std::os::raw::c_int,
    argv: *mut *mut ::std::os::raw::c_char,
) {
    static LONGOPTS: [option; 3] = [
        option {
            name: c"delay_us".as_ptr(),
            has_arg: 1,
            flag: ::std::ptr::null_mut(),
            val: 0,
        },
        option {
            name: c"die_during_fetch".as_ptr(),
            has_arg: 1,
            flag: ::std::ptr::null_mut(),
            val: 0,
        },
        option {
            name: ::std::ptr::null(),
            has_arg: 0,
            flag: ::std::ptr::null_mut(),
            val: 0,
        },
    ];
    let mut option_idx: ::std::os::raw::c_int = 0;
    let mut opt: ::std::os::raw::c_int;

    unsafe {
        (*ctx).fault_inject.delay_us = 0;
        (*ctx).fault_inject.die_during_fetch = false;
    }
    loop {
        opt = unsafe {
            getopt_long(
                argc,
                argv,
                c"".as_ptr(),
                LONGOPTS.as_ptr(),
                &mut option_idx,
            )
        };
        if opt == -1 {
            break;
        }
        match opt {
            0 => unsafe {
                if strcmp(LONGOPTS[option_idx as usize].name, c"delay_us".as_ptr()) == 0 {
                    (*ctx).fault_inject.delay_us = strtoll(optarg, ::std::ptr::null_mut(), 10);
                }
                if strcmp(
                    LONGOPTS[option_idx as usize].name,
                    c"die_during_fetch".as_ptr(),
                ) == 0
                {
                    (*ctx).fault_inject.die_during_fetch =
                        strtoll(optarg, ::std::ptr::null_mut(), 10) != 0;
                }
            },
            _ => {}
        }
    }
}

unsafe extern "C" fn ublk_fault_inject_usage(ops: *const ublk_tgt_ops) {
    unsafe {
        printf(c"\tfault_inject: [--delay_us us (default 0)] [--die_during_fetch 1]\n".as_ptr());
    }
}

#[no_mangle]
pub static fault_inject_tgt_ops: ublk_tgt_ops = ublk_tgt_ops {
    name: c"fault_inject".as_ptr(),
    init_tgt: Some(ublk_fault_inject_tgt_init),
    pre_fetch_io: Some(ublk_fault_inject_pre_fetch_io),
    queue_io: Some(ublk_fault_inject_queue_io),
    tgt_io_done: Some(ublk_fault_inject_tgt_io_done),
    parse_cmd_line: Some(ublk_fault_inject_cmd_line),
    usage: Some(ublk_fault_inject_usage),
};
