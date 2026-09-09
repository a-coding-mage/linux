// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Performance counter callchain support - powerpc architecture code
 *
 * Copyright © 2009 Paul Mackerras, IBM Corporation.
 */

// Dependencies supplied by the surrounding kernel translation unit.

/*
 * Is sp valid as the address of the next kernel stack frame after prev_sp?
 * The next frame may be in a different stack area but should not go
 * back down in the same stack area.
 */
unsafe fn valid_next_sp(sp: ::core::ffi::c_ulong, prev_sp: ::core::ffi::c_ulong) -> ::core::ffi::c_int {
    if sp & 0xf != 0 {
        return 0; // must be 16-byte aligned
    }
    if validate_sp(sp, current) == 0 {
        return 0;
    }
    if sp >= prev_sp.wrapping_add(STACK_FRAME_MIN_SIZE) {
        return 1;
    }
    /*
     * sp could decrease when we jump off an interrupt stack
     * back to the regular process stack.
     */
    if (sp & !(THREAD_SIZE.wrapping_sub(1)))
        != (prev_sp & !(THREAD_SIZE.wrapping_sub(1)))
    {
        return 1;
    }
    0
}

#[no_sanitize(address)]
pub unsafe fn perf_callchain_kernel(
    entry: *mut perf_callchain_entry_ctx,
    mut regs: *mut pt_regs,
) {
    let mut sp: ::core::ffi::c_ulong;
    let mut next_sp: ::core::ffi::c_ulong;
    let mut next_ip: ::core::ffi::c_ulong;
    let mut lr: ::core::ffi::c_ulong;
    let mut level: ::core::ffi::c_long = 0;
    let mut fp: *mut ::core::ffi::c_ulong;

    lr = (*regs).link;
    sp = (*regs).gpr[1];
    perf_callchain_store(entry, perf_arch_instruction_pointer(regs));

    if validate_sp(sp, current) == 0 {
        return;
    }

    loop {
        fp = sp as *mut ::core::ffi::c_ulong;
        next_sp = *fp.add(0);

        if next_sp == sp.wrapping_add(STACK_INT_FRAME_SIZE)
            && validate_sp_size(sp, current, STACK_INT_FRAME_SIZE) != 0
            && *fp.add(STACK_INT_FRAME_MARKER_LONGS as usize) == STACK_FRAME_REGS_MARKER
        {
            /*
             * This looks like an interrupt frame for an
             * interrupt that occurred in the kernel
             */
            regs = (sp.wrapping_add(STACK_INT_FRAME_REGS)) as *mut pt_regs;
            next_ip = (*regs).nip;
            lr = (*regs).link;
            level = 0;
            perf_callchain_store_context(entry, PERF_CONTEXT_KERNEL);
        } else {
            if level == 0 {
                next_ip = lr;
            } else {
                next_ip = *fp.add(STACK_FRAME_LR_SAVE as usize);
            }

            /*
             * We can't tell which of the first two addresses
             * we get are valid, but we can filter out the
             * obviously bogus ones here.  We replace them
             * with 0 rather than removing them entirely so
             * that userspace can tell which is which.
             */
            if (level == 1 && next_ip == lr)
                || (level <= 1 && kernel_text_address(next_ip) == 0)
            {
                next_ip = 0;
            }

            level = level.wrapping_add(1);
        }

        perf_callchain_store(entry, next_ip);
        if valid_next_sp(next_sp, sp) == 0 {
            return;
        }
        sp = next_sp;
    }
}

pub unsafe fn perf_callchain_user(
    entry: *mut perf_callchain_entry_ctx,
    regs: *mut pt_regs,
) {
    perf_callchain_store(entry, perf_arch_instruction_pointer(regs));

    if (*current).mm.is_null() {
        return;
    }

    if is_32bit_task() == 0 {
        perf_callchain_user_64(entry, regs);
    } else {
        perf_callchain_user_32(entry, regs);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
