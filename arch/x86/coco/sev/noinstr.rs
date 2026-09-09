// SPDX-License-Identifier: GPL-2.0-only
/*
 * AMD Memory Encryption Support
 *
 * Copyright (C) 2019 SUSE
 *
 * Author: Joerg Roedel <jroedel@suse.de>
 */

// C preprocessor condition/configuration and kernel dependencies are supplied
// by the surrounding translation unit.

#[inline(always)]
unsafe fn on_vc_stack(regs: *const pt_regs) -> bool {
    let sp = (*regs).sp;

    /* User-mode RSP is not trusted */
    if user_mode(regs) {
        return false;
    }

    /* SYSCALL gap still has user-mode RSP */
    if ip_within_syscall_gap(regs) {
        return false;
    }

    sp >= __this_cpu_ist_bottom_va(VC) && sp < __this_cpu_ist_top_va(VC)
}

/*
 * This function handles the case when an NMI is raised in the #VC
 * exception handler entry code, before the #VC handler has switched off
 * its IST stack. In this case, the IST entry for #VC must be adjusted,
 * so that any nested #VC exception will not overwrite the stack
 * contents of the interrupted #VC handler.
 *
 * The IST entry is adjusted unconditionally so that it can be also be
 * unconditionally adjusted back in __sev_es_ist_exit(). Otherwise a
 * nested sev_es_ist_exit() call may adjust back the IST entry too
 * early.
 *
 * The __sev_es_ist_enter() and __sev_es_ist_exit() functions always run
 * on the NMI IST stack, as they are only called from NMI handling code
 * right now.
 */
pub unsafe fn __sev_es_ist_enter(regs: *const pt_regs) {
    let old_ist: usize;
    let mut new_ist: usize;

    /* Read old IST entry */
    new_ist = old_ist = __this_cpu_read(cpu_tss_rw.x86_tss.ist[IST_INDEX_VC]);

    /*
     * If NMI happened while on the #VC IST stack, set the new IST
     * value below regs->sp, so that the interrupted stack frame is
     * not overwritten by subsequent #VC exceptions.
     */
    if on_vc_stack(regs) {
        new_ist = (*regs).sp;
    }

    /*
     * Reserve additional 8 bytes and store old IST value so this
     * adjustment can be unrolled in __sev_es_ist_exit().
     */
    new_ist -= core::mem::size_of::<usize>();
    *(new_ist as *mut usize) = old_ist;

    /* Set new IST entry */
    this_cpu_write(cpu_tss_rw.x86_tss.ist[IST_INDEX_VC], new_ist);
}

pub unsafe fn __sev_es_ist_exit() {
    let ist = __this_cpu_read(cpu_tss_rw.x86_tss.ist[IST_INDEX_VC]);

    /* Read IST entry */
    if WARN_ON(ist == __this_cpu_ist_top_va(VC)) {
        return;
    }

    /* Read back old IST entry and write it to the TSS */
    this_cpu_write(
        cpu_tss_rw.x86_tss.ist[IST_INDEX_VC],
        *(ist as *const usize),
    );
}

pub unsafe fn __sev_es_nmi_complete() {
    let mut state: ghcb_state = core::mem::zeroed();
    let ghcb: *mut ghcb;

    ghcb = __sev_get_ghcb(&mut state);

    vc_ghcb_invalidate(ghcb);
    ghcb_set_sw_exit_code(ghcb, SVM_VMGEXIT_NMI_COMPLETE);
    ghcb_set_sw_exit_info_1(ghcb, 0);
    ghcb_set_sw_exit_info_2(ghcb, 0);

    sev_es_wr_ghcb_msr(__pa_nodebug(ghcb));
    VMGEXIT!();

    __sev_put_ghcb(&mut state);
}

/*
 * Nothing shall interrupt this code path while holding the per-CPU
 * GHCB. The backup GHCB is only for NMIs interrupting this path.
 *
 * Callers must disable local interrupts around it.
 */
pub unsafe fn __sev_get_ghcb(state: *mut ghcb_state) -> *mut ghcb {
    let data: *mut sev_es_runtime_data;
    let ghcb: *mut ghcb;

    WARN_ON(!irqs_disabled());

    if !sev_cfg.ghcbs_initialized {
        (*state).ghcb = core::ptr::null_mut();
        return boot_ghcb;
    }

    data = this_cpu_read(runtime_data);
    ghcb = &mut (*data).ghcb_page;

    if unlikely((*data).ghcb_active) {
        /* GHCB is already in use - save its contents */

        if unlikely((*data).backup_ghcb_active) {
            /* Backup-GHCB is also already in use. There is no way
             * to continue here so just kill the machine. To make
             * panic() work, mark GHCBs inactive so that messages
             * can be printed out. */
            (*data).ghcb_active = false;
            (*data).backup_ghcb_active = false;

            instrumentation_begin();
            panic!("Unable to handle #VC exception! GHCB and Backup GHCB are already in use");
            instrumentation_end();
        }

        /* Mark backup_ghcb active before writing to it */
        (*data).backup_ghcb_active = true;

        (*state).ghcb = &mut (*data).backup_ghcb;

        /* Backup GHCB content */
        *(*state).ghcb = *ghcb;
    } else {
        (*state).ghcb = core::ptr::null_mut();
        (*data).ghcb_active = true;
    }

    ghcb
}

pub unsafe fn __sev_put_ghcb(state: *mut ghcb_state) {
    let data: *mut sev_es_runtime_data;
    let ghcb: *mut ghcb;

    WARN_ON(!irqs_disabled());

    if !sev_cfg.ghcbs_initialized {
        return;
    }

    data = this_cpu_read(runtime_data);
    ghcb = &mut (*data).ghcb_page;

    if !(*state).ghcb.is_null() {
        /* Restore GHCB from Backup */
        *ghcb = *(*state).ghcb;
        (*data).backup_ghcb_active = false;
        (*state).ghcb = core::ptr::null_mut();
    } else {
        /*
         * Invalidate the GHCB so a VMGEXIT instruction issued
         * from userspace won't appear to be valid.
         */
        vc_ghcb_invalidate(ghcb);
        (*data).ghcb_active = false;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
