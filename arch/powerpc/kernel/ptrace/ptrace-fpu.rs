// SPDX-License-Identifier: GPL-2.0-or-later

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced here rather than reimplemented.

pub unsafe fn ptrace_get_fpr(
    child: *mut task_struct,
    index: i32,
    data: *mut c_ulong,
) -> i32 {
    #[cfg(CONFIG_PPC_FPU_REGS)]
    let fpidx: u32 = (index - PT_FPR0) as u32;

    if index > PT_FPSCR {
        return -(EIO as i32);
    }

    #[cfg(CONFIG_PPC_FPU_REGS)]
    {
        flush_fp_to_thread(child);
        if fpidx < (PT_FPSCR - PT_FPR0) as u32 {
            // On 32-bit the index we are passed refers to 32-bit words
            #[cfg(CONFIG_PPC32)]
            {
                *data = (*((*child).thread.fp_state.fpr as *mut u32).add(fpidx as usize))
                    as c_ulong;
            }
            #[cfg(not(CONFIG_PPC32))]
            {
                core::ptr::copy_nonoverlapping(
                    &((*child).thread.TS_FPR(fpidx)),
                    data,
                    1,
                );
            }
        } else {
            *data = (*child).thread.fp_state.fpscr;
        }
    }

    #[cfg(not(CONFIG_PPC_FPU_REGS))]
    {
        *data = 0;
    }

    0
}

pub unsafe fn ptrace_put_fpr(
    child: *mut task_struct,
    index: i32,
    data: c_ulong,
) -> i32 {
    #[cfg(CONFIG_PPC_FPU_REGS)]
    let fpidx: u32 = (index - PT_FPR0) as u32;

    if index > PT_FPSCR {
        return -(EIO as i32);
    }

    #[cfg(CONFIG_PPC_FPU_REGS)]
    {
        flush_fp_to_thread(child);
        if fpidx < (PT_FPSCR - PT_FPR0) as u32 {
            // On 32-bit the index we are passed refers to 32-bit words
            #[cfg(CONFIG_PPC32)]
            {
                *((*child).thread.fp_state.fpr as *mut u32).add(fpidx as usize) = data as u32;
            }
            #[cfg(not(CONFIG_PPC32))]
            {
                core::ptr::copy_nonoverlapping(
                    &data,
                    &mut ((*child).thread.TS_FPR(fpidx)),
                    1,
                );
            }
        } else {
            (*child).thread.fp_state.fpscr = data;
        }
    }

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
