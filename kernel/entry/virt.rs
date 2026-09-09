// SPDX-License-Identifier: GPL-2.0

unsafe fn xfer_to_guest_mode_work(ti_work: ::core::ffi::c_ulong) -> ::core::ffi::c_int {
    let mut ti_work = ti_work;
    loop {
        let ret: ::core::ffi::c_int;

        if ti_work & (_TIF_SIGPENDING | _TIF_NOTIFY_SIGNAL) != 0 {
            return -EINTR;
        }

        if ti_work & (_TIF_NEED_RESCHED | _TIF_NEED_RESCHED_LAZY) != 0 {
            schedule();
        }

        if ti_work & _TIF_NOTIFY_RESUME != 0 {
            resume_user_mode_work(::core::ptr::null_mut());
        }

        ret = arch_xfer_to_guest_mode_handle_work(ti_work);
        if ret != 0 {
            return ret;
        }

        ti_work = read_thread_flags();
        if ti_work & XFER_TO_GUEST_MODE_WORK == 0 {
            break;
        }
    }
    0
}

pub unsafe fn xfer_to_guest_mode_handle_work() -> ::core::ffi::c_int {
    let ti_work: ::core::ffi::c_ulong;

    /*
     * This is invoked from the outer guest loop with interrupts and
     * preemption enabled.
     *
     * KVM invokes xfer_to_guest_mode_work_pending() with interrupts
     * disabled in the inner loop before going into guest mode. No need
     * to disable interrupts here.
     */
    ti_work = read_thread_flags();
    if ti_work & XFER_TO_GUEST_MODE_WORK == 0 {
        return 0;
    }

    xfer_to_guest_mode_work(ti_work)
}

// EXPORT_SYMBOL_GPL(xfer_to_guest_mode_handle_work);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
