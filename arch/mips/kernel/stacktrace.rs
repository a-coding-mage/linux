// SPDX-License-Identifier: GPL-2.0-only
/*
 * Stack trace management functions
 *
 *  Copyright (C) 2006 Atsushi Nemoto <anemo@mba.ocn.ne.jp>
 */

#[repr(C)]
pub struct StackTrace {
    pub skip: ::core::ffi::c_int,
    pub nr_entries: ::core::ffi::c_uint,
    pub max_entries: ::core::ffi::c_uint,
    pub entries: *mut ::core::ffi::c_ulong,
}

#[repr(C)]
pub struct PtRegs {
    pub regs: [::core::ffi::c_ulong; 32],
    pub cp0_epc: ::core::ffi::c_ulong,
}

#[repr(C)]
pub struct ThreadStruct {
    pub reg29: ::core::ffi::c_ulong,
    pub reg31: ::core::ffi::c_ulong,
}

#[repr(C)]
pub struct TaskStruct {
    pub thread: ThreadStruct,
}

extern "C" {
    static mut current: *mut TaskStruct;
    static mut raw_show_trace: bool;
    static THREAD_SIZE: ::core::ffi::c_ulong;

    fn kstack_end(sp: *mut ::core::ffi::c_ulong) -> bool;
    fn __kernel_text_address(addr: ::core::ffi::c_ulong) -> bool;
    fn in_sched_functions(addr: ::core::ffi::c_ulong) -> bool;
    fn task_stack_page(tsk: *mut TaskStruct) -> *mut ::core::ffi::c_void;
    fn unwind_stack(
        tsk: *mut TaskStruct,
        sp: *mut ::core::ffi::c_ulong,
        pc: ::core::ffi::c_ulong,
        ra: *mut ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_ulong;
    fn prepare_frametrace(regs: *mut PtRegs);
    fn WARN_ON(condition: bool) -> bool;
}

/*
 * Save stack-backtrace addresses into a stack_trace buffer:
 */
unsafe fn save_raw_context_stack(
    trace: *mut StackTrace,
    reg29: ::core::ffi::c_ulong,
    savesched: ::core::ffi::c_int,
) {
    let mut sp = reg29 as *mut ::core::ffi::c_ulong;
    let mut addr: ::core::ffi::c_ulong;

    while !kstack_end(sp) {
        addr = *sp;
        sp = sp.add(1);
        if __kernel_text_address(addr)
            && (savesched != 0 || !in_sched_functions(addr))
        {
            if (*trace).skip > 0 {
                (*trace).skip -= 1;
            } else {
                *(*trace).entries.add((*trace).nr_entries as usize) = addr;
                (*trace).nr_entries += 1;
            }
            if (*trace).nr_entries >= (*trace).max_entries {
                break;
            }
        }
    }
}

unsafe fn save_context_stack(
    trace: *mut StackTrace,
    tsk: *mut TaskStruct,
    regs: *mut PtRegs,
    savesched: ::core::ffi::c_int,
) {
    let mut sp = (*regs).regs[29];

    #[cfg(CONFIG_KALLSYMS)]
    {
        let mut ra = (*regs).regs[31];
        let mut pc = (*regs).cp0_epc;

        if raw_show_trace || !__kernel_text_address(pc) {
            let stack_page = task_stack_page(tsk) as ::core::ffi::c_ulong;
            if stack_page != 0
                && sp >= stack_page
                && sp <= stack_page + THREAD_SIZE - 32
            {
                save_raw_context_stack(trace, sp, savesched);
            }
            return;
        }
        loop {
            if savesched != 0 || !in_sched_functions(pc) {
                if (*trace).skip > 0 {
                    (*trace).skip -= 1;
                } else {
                    *(*trace).entries.add((*trace).nr_entries as usize) = pc;
                    (*trace).nr_entries += 1;
                }
                if (*trace).nr_entries >= (*trace).max_entries {
                    break;
                }
            }
            pc = unwind_stack(tsk, &mut sp, pc, &mut ra);
            if pc == 0 {
                break;
            }
        }
    }

    #[cfg(not(CONFIG_KALLSYMS))]
    {
        save_raw_context_stack(trace, sp, savesched);
    }
}

/*
 * Save stack-backtrace addresses into a stack_trace buffer.
 */
#[no_mangle]
pub unsafe extern "C" fn save_stack_trace(trace: *mut StackTrace) {
    save_stack_trace_tsk(current, trace);
}

#[no_mangle]
pub unsafe extern "C" fn save_stack_trace_tsk(
    tsk: *mut TaskStruct,
    trace: *mut StackTrace,
) {
    let mut dummyregs = ::core::mem::MaybeUninit::<PtRegs>::uninit();
    let regs = dummyregs.as_mut_ptr();

    WARN_ON((*trace).nr_entries != 0 || (*trace).max_entries == 0);

    if tsk != current {
        (*regs).regs[29] = (*tsk).thread.reg29;
        (*regs).regs[31] = 0;
        (*regs).cp0_epc = (*tsk).thread.reg31;
    } else {
        prepare_frametrace(regs);
    }
    save_context_stack(trace, tsk, regs, (tsk == current) as ::core::ffi::c_int);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
