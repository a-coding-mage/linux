// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2018 Hangzhou C-SKY Microsystems co.,ltd.

// Dependencies supplied by the surrounding kernel translation unit.

pub static mut cpu_data: [cpuinfo_csky; NR_CPUS] = [/* supplied type */];

#[cfg(CONFIG_STACKPROTECTOR)]
pub static mut __stack_chk_guard: c_ulong = 0;

unsafe extern "C" {
    fn ret_from_fork();
    fn ret_from_kernel_thread();
}

/*
 * Some archs flush debug and FPU info here
 */
pub unsafe fn flush_thread() {}

pub unsafe fn copy_thread(
    p: *mut task_struct,
    args: *const kernel_clone_args,
) -> c_int {
    let clone_flags: u64 = (*args).flags;
    let usp: c_ulong = (*args).stack;
    let tls: c_ulong = (*args).tls;
    let childstack: *mut switch_stack;
    let childregs: *mut pt_regs = task_pt_regs(p);

    #[cfg(CONFIG_CPU_HAS_FPU)]
    save_to_user_fp(&mut (*p).thread.user_fp);

    childstack = (childregs as *mut switch_stack).offset(-1);
    core::ptr::write_bytes(
        childstack as *mut u8,
        0,
        core::mem::size_of::<switch_stack>(),
    );

    /* setup thread.sp for switch_to !!! */
    (*p).thread.sp = childstack as c_ulong;

    if unlikely((*args).fn_ != 0) {
        core::ptr::write_bytes(
            childregs as *mut u8,
            0,
            core::mem::size_of::<pt_regs>(),
        );
        (*childstack).r15 = ret_from_kernel_thread as *const () as c_ulong;
        (*childstack).r10 = (*args).fn_arg as c_ulong;
        (*childstack).r9 = (*args).fn_ as c_ulong;
        (*childregs).sr = mfcr("psr");
    } else {
        *childregs = *current_pt_regs();
        if usp != 0 {
            (*childregs).usp = usp;
        }
        if clone_flags & CLONE_SETTLS != 0 {
            (*task_thread_info(p)).tp_value = tls;
            (*childregs).tls = tls;
        }

        (*childregs).a0 = 0;
        (*childstack).r15 = ret_from_fork as *const () as c_ulong;
    }

    0
}

/* Fill in the fpu structure for a core dump.  */
pub unsafe fn elf_core_copy_task_fpregs(
    t: *mut task_struct,
    fpu: *mut elf_fpregset_t,
) -> c_int {
    core::ptr::copy_nonoverlapping(
        &(*current).thread.user_fp as *const _,
        fpu,
        1,
    );
    1
}

pub unsafe fn dump_task_regs(
    tsk: *mut task_struct,
    pr_regs: *mut elf_gregset_t,
) -> c_int {
    let regs: *mut pt_regs = task_pt_regs(tsk);

    /* NOTE: usp is error value. */
    // ELF_CORE_COPY_REGS((*pr_regs), regs)

    1
}

#[cfg(not(CONFIG_CPU_PM_NONE))]
pub unsafe fn arch_cpu_idle() {
    #[cfg(CONFIG_CPU_PM_WAIT)]
    core::arch::asm!("wait");

    #[cfg(CONFIG_CPU_PM_DOZE)]
    core::arch::asm!("doze");

    #[cfg(CONFIG_CPU_PM_STOP)]
    core::arch::asm!("stop");
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
