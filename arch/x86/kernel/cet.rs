// SPDX-License-Identifier: GPL-2.0

// Dependencies supplied by the surrounding kernel translation unit:
// linux/ptrace.h, asm/bugs.h, asm/msr.h, and asm/traps.h.

#[repr(u32)]
enum CpErrorCode {
    CP_EC = (1 << 15) - 1,
    CP_RET = 1,
    CP_IRET = 2,
    CP_ENDBR = 3,
    CP_RSTRORSSP = 4,
    CP_SETSSBSY = 5,
    CP_ENCL = 1 << 15,
}

static CP_ERR: [&str; 6] = [
    "unknown",
    "near ret",
    "far/iret",
    "endbranch",
    "rstorssp",
    "setssbsy",
];

unsafe fn cp_err_string(error_code: usize) -> &'static str {
    let mut cpec = error_code & (CpErrorCode::CP_EC as usize);

    if cpec >= CP_ERR.len() {
        cpec = 0;
    }
    CP_ERR[cpec]
}

unsafe fn do_unexpected_cp(regs: *mut pt_regs, error_code: usize) {
    WARN_ONCE(
        1,
        "Unexpected %s #CP, error_code: %s\n",
        if user_mode(regs) { "user mode" } else { "kernel mode" },
        cp_err_string(error_code),
    );
}

static mut cpf_rate: DEFINE_RATELIMIT_STATE!(DEFAULT_RATELIMIT_INTERVAL, DEFAULT_RATELIMIT_BURST);

unsafe fn do_user_cp_fault(regs: *mut pt_regs, error_code: usize) {
    let tsk: *mut task_struct;
    let mut ssp: usize = 0;

    /*
     * An exception was just taken from userspace. Since interrupts are disabled
     * here, no scheduling should have messed with the registers yet and they
     * will be whatever is live in userspace. So read the SSP before enabling
     * interrupts so locking the fpregs to do it later is not required.
     */
    rdmsrq(MSR_IA32_PL3_SSP, &mut ssp);

    cond_local_irq_enable(regs);

    tsk = current;
    (*tsk).thread.error_code = error_code;
    (*tsk).thread.trap_nr = X86_TRAP_CP;

    /* Ratelimit to prevent log spamming. */
    if show_unhandled_signals && unhandled_signal(tsk, SIGSEGV) && ratelimit(&mut cpf_rate) {
        pr_emerg!(
            "{}[{}] control protection ip:{:x} sp:{:x} ssp:{:x} error:{:x}({}){}",
            (*tsk).comm,
            task_pid_nr(tsk),
            (*regs).ip,
            (*regs).sp,
            ssp,
            error_code,
            cp_err_string(error_code),
            if error_code & (CpErrorCode::CP_ENCL as usize) != 0 { " in enclave" } else { "" },
        );
        print_vma_addr(KERN_CONT, (*regs).ip);
        pr_cont!("\n");
    }

    force_sig_fault(SIGSEGV, SEGV_CPERR, core::ptr::null_mut());
    cond_local_irq_disable(regs);
}

static mut ibt_fatal: bool = true;

/*
 * By definition, all missing-ENDBRANCH #CPs are a result of WFE && !ENDBR.
 *
 * For the kernel IBT no ENDBR selftest where #CPs are deliberately triggered,
 * the WFE state of the interrupted context needs to be cleared to let execution
 * continue. Otherwise when the CPU resumes from the instruction that just
 * caused the previous #CP, another missing-ENDBRANCH #CP is raised and the CPU
 * enters a dead loop.
 *
 * This is not a problem with IDT because it doesn't preserve WFE and IRET doesn't
 * set WFE. But FRED provides space on the entry stack (in an expanded CS area)
 * to save and restore the WFE state, thus the WFE state is no longer clobbered,
 * so software must clear it.
 */
unsafe fn ibt_clear_fred_wfe(regs: *mut pt_regs) {
    /*
     * No need to do any FRED checks.
     *
     * For IDT event delivery, the high-order 48 bits of CS are pushed
     * as 0s into the stack, and later IRET ignores these bits.
     *
     * For FRED, a test to check if fred_cs.wfe is set would be dropped
     * by compilers.
     */
    (*regs).fred_cs.wfe = 0;
}

unsafe fn do_kernel_cp_fault(regs: *mut pt_regs, error_code: usize) {
    if (error_code & (CpErrorCode::CP_EC as usize)) != CpErrorCode::CP_ENDBR as usize {
        do_unexpected_cp(regs, error_code);
        return;
    }

    if unlikely((*regs).ip == ibt_selftest_noendbr as usize) {
        (*regs).ax = 0;
        ibt_clear_fred_wfe(regs);
        return;
    }

    pr_err!("Missing ENDBR: %pS\n", instruction_pointer(regs));
    if !ibt_fatal {
        printk!(KERN_DEFAULT, CUT_HERE);
        __warn(file!(), line!(), (*regs).ip as *mut core::ffi::c_void, TAINT_WARN, regs, core::ptr::null_mut());
        ibt_clear_fred_wfe(regs);
        return;
    }
    BUG!();
}

unsafe fn ibt_setup(str_: *mut core::ffi::c_char) -> i32 {
    if strcmp(str_, c"off".as_ptr()) == 0 {
        setup_clear_cpu_cap(X86_FEATURE_IBT);
    }

    if strcmp(str_, c"warn".as_ptr()) == 0 {
        ibt_fatal = false;
    }

    1
}

__setup!(c"ibt=", ibt_setup);

unsafe fn exc_control_protection(regs: *mut pt_regs, error_code: usize) {
    if user_mode(regs) {
        if cpu_feature_enabled(X86_FEATURE_USER_SHSTK) {
            do_user_cp_fault(regs, error_code);
        } else {
            do_unexpected_cp(regs, error_code);
        }
    } else if cpu_feature_enabled(X86_FEATURE_IBT) {
        do_kernel_cp_fault(regs, error_code);
    } else {
        do_unexpected_cp(regs, error_code);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
