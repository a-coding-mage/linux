// SPDX-License-Identifier: GPL-2.0-only
/*
 * Linux performance counter support for MIPS.
 *
 * Copyright (C) 2010 MIPS Technologies, Inc.
 * Author: Deng-Cheng Zhu
 *
 * This code is based on the implementation for ARM, which is in turn
 * based on the sparc64 perf event code and the x86 code. Performance
 * counter access is based on the MIPS Oprofile code. And the callchain
 * support references the code of MIPS stacktrace.c.
 */

// C dependencies supplied by other translation units are intentionally not
// implemented here.

/* Callchain handling code. */

/*
 * Leave userspace callchain empty for now. When we find a way to trace
 * the user stack callchains, we will add it here.
 */

unsafe fn save_raw_perf_callchain(
    entry: *mut perf_callchain_entry_ctx,
    reg29: c_ulong,
) {
    let mut sp = reg29 as *mut c_ulong;
    let mut addr: c_ulong;

    while !kstack_end(sp) {
        addr = *sp;
        sp = sp.add(1);
        if __kernel_text_address(addr) {
            perf_callchain_store(entry, addr);
            if (*entry).nr >= (*entry).max_stack {
                break;
            }
        }
    }
}

pub unsafe fn perf_callchain_kernel(
    entry: *mut perf_callchain_entry_ctx,
    regs: *mut pt_regs,
) {
    let mut sp = (*regs).regs[29];
    #[cfg(CONFIG_KALLSYMS)]
    {
        let mut ra = (*regs).regs[31];
        let mut pc = (*regs).cp0_epc;

        if raw_show_trace || !__kernel_text_address(pc) {
            let stack_page = task_stack_page(current) as c_ulong;
            if stack_page != 0
                && sp >= stack_page
                && sp <= stack_page.wrapping_add(THREAD_SIZE).wrapping_sub(32)
            {
                save_raw_perf_callchain(entry, sp);
            }
            return;
        }
        loop {
            perf_callchain_store(entry, pc);
            if (*entry).nr >= (*entry).max_stack {
                break;
            }
            pc = unwind_stack(current, &mut sp, pc, &mut ra);
            if pc == 0 {
                break;
            }
        }
    }
    #[cfg(not(CONFIG_KALLSYMS))]
    {
        save_raw_perf_callchain(entry, sp);
    }
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
