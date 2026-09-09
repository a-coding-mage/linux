/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2012 ARM Ltd.
 */

/*
 * The C header includes <asm/stack_pointer.h> and <asm/ptrace.h>; the names
 * supplied by those headers are intentionally left as external dependencies.
 */

/* CONFIG_PERF_EVENTS */
#[cfg(feature = "CONFIG_PERF_EVENTS")]
macro_rules! perf_arch_bpf_user_pt_regs {
    ($regs:expr) => {
        unsafe { &mut (*$regs).user_regs }
    };
}

macro_rules! perf_arch_fetch_caller_regs {
    ($regs:expr, $ip:expr) => {{
        unsafe {
            (*$regs).pc = $ip;
            (*$regs).regs[29] = {
                let frame_address: usize;
                core::arch::asm!("mov {0}, x29", out(reg) frame_address);
                frame_address as _
            };
            (*$regs).sp = current_stack_pointer;
            (*$regs).pstate = PSR_MODE_EL1h;
        }
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
