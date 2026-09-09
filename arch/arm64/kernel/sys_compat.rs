// SPDX-License-Identifier: GPL-2.0-only
/*
 * Based on arch/arm/kernel/sys_arm.c
 *
 * Copyright (C) People who wrote linux/arch/i386/kernel/sys_i386.c
 * Copyright (C) 1995, 1996 Russell King.
 * Copyright (C) 2012 ARM Ltd.
 */

// Dependencies supplied by the surrounding kernel translation.

unsafe fn __do_compat_cache_op(mut start: libc::c_ulong, end: libc::c_ulong) -> libc::c_long {
    let mut ret: libc::c_long;

    loop {
        let chunk = if PAGE_SIZE < end.wrapping_sub(start) {
            PAGE_SIZE
        } else {
            end.wrapping_sub(start)
        };

        if fatal_signal_pending(current) {
            return 0;
        }

        if cpus_have_final_cap(ARM64_WORKAROUND_1542419) {
            /*
             * The workaround requires an inner-shareable tlbi.
             * We pick the reserved-ASID to minimise the impact.
             */
            __tlbi(aside1is, 0 as libc::c_ulong);
            __tlbi_sync_s1ish((*current).mm);
        }

        ret = caches_clean_inval_user_pou(start, start.wrapping_add(chunk));
        if ret != 0 {
            return ret;
        }

        cond_resched();
        start = start.wrapping_add(chunk);
        if start >= end {
            break;
        }
    }

    0
}

unsafe fn do_compat_cache_op(
    start: libc::c_ulong,
    end: libc::c_ulong,
    flags: libc::c_int,
) -> libc::c_long {
    if end < start || flags != 0 {
        return -EINVAL;
    }

    if !access_ok(start as *const libc::c_void, end.wrapping_sub(start)) {
        return -EFAULT;
    }

    __do_compat_cache_op(start, end)
}

/*
 * Handle all unrecognised system calls.
 */
pub unsafe fn compat_arm_syscall(regs: *mut pt_regs, scno: libc::c_int) -> libc::c_long {
    let mut addr: libc::c_ulong;

    match scno {
        /*
         * Flush a region from virtual address 'r0' to virtual address 'r1'
         * _exclusive_.  There is no alignment requirement on either address;
         * user space does not need to know the hardware cache layout.
         *
         * r2 contains flags.  It should ALWAYS be passed as ZERO until it
         * is defined to be something else.  For now we ignore it, but may
         * the fires of hell burn in your belly if you break this rule. ;)
         *
         * (at a later date, we may want to allow this call to not flush
         * various aspects of the cache.  Passing '0' will guarantee that
         * everything necessary gets flushed to maintain consistency in
         * the specified region).
         */
        __ARM_NR_compat_cacheflush => {
            return do_compat_cache_op((*regs).regs[0], (*regs).regs[1], (*regs).regs[2] as libc::c_int);
        }

        __ARM_NR_compat_set_tls => {
            (*current).thread.uw.tp_value = (*regs).regs[0];

            /*
             * Protect against register corruption from context switch.
             * See comment in tls_thread_flush.
             */
            barrier();
            write_sysreg((*regs).regs[0], tpidrro_el0);
            return 0;
        }

        _ => {
            /*
             * Calls 0xf0xxx..0xf07ff are defined to return -ENOSYS
             * if not implemented, rather than raising SIGILL. This
             * way the calling program can gracefully determine whether
             * a feature is supported.
             */
            if scno < __ARM_NR_COMPAT_END {
                return -ENOSYS;
            }
        }
    }

    addr = instruction_pointer(regs) - if compat_thumb_mode(regs) { 2 } else { 4 };

    arm64_notify_die(
        "Oops - bad compat syscall(2)",
        regs,
        SIGILL,
        ILL_ILLTRP,
        addr,
        0,
    );
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
