// SPDX-License-Identifier: GPL-2.0
/*
 * arch/arm64/kvm/fpsimd.c: Guest/host FPSIMD context coordination helpers
 *
 * Copyright 2018 Arm Limited
 * Author: Dave Martin <Dave.Martin@arm.com>
 */

// Dependencies supplied by the surrounding kernel translation.

/*
 * Prepare vcpu for saving the host's FPSIMD state and loading the guest's.
 * The actual loading is done by the FPSIMD access trap taken to hyp.
 *
 * Here, we just set the correct metadata to indicate that the FPSIMD
 * state in the cpu regs (if any) belongs to current on the host.
 */
pub unsafe fn kvm_arch_vcpu_load_fp(vcpu: *mut kvm_vcpu) {
    BUG_ON(!(*current).mm);

    if !system_supports_fpsimd() {
        return;
    }

    /*
     * Avoid needless save/restore of the guest's common
     * FPSIMD/SVE/SME regs during transitions between L1/L2.
     *
     * These transitions only happens in a non-preemptible context
     * where the host regs have already been saved and unbound. The
     * live registers are either free or owned by the guest.
     */
    if vcpu_get_flag(vcpu, IN_NESTED_ERET) || vcpu_get_flag(vcpu, IN_NESTED_EXCEPTION) {
        WARN_ON_ONCE(host_owns_fp_regs());
        return;
    }

    /*
     * Ensure that any host FPSIMD/SVE/SME state is saved and unbound such
     * that the host kernel is responsible for restoring this state upon
     * return to userspace, and the hyp code doesn't need to save anything.
     *
     * When the host may use SME, fpsimd_save_and_flush_cpu_state() ensures
     * that PSTATE.{SM,ZA} == {0,0}.
     */
    fpsimd_save_and_flush_cpu_state();
    *host_data_ptr(fp_owner) = FP_STATE_FREE;

    WARN_ON_ONCE(system_supports_sme() && read_sysreg_s(SYS_SVCR));
}

/*
 * Called just before entering the guest once we are no longer preemptible
 * and interrupts are disabled. If we have managed to run anything using
 * FP while we were preemptible (such as off the back of an interrupt),
 * then neither the host nor the guest own the FP hardware (and it was the
 * responsibility of the code that used FP to save the existing state).
 */
pub unsafe fn kvm_arch_vcpu_ctxflush_fp(_vcpu: *mut kvm_vcpu) {
    if test_thread_flag(TIF_FOREIGN_FPSTATE) {
        *host_data_ptr(fp_owner) = FP_STATE_FREE;
    }
}

/*
 * Called just after exiting the guest. If the guest FPSIMD state
 * was loaded, update the host's context tracking data mark the CPU
 * FPSIMD regs as dirty and belonging to vcpu so that they will be
 * written back if the kernel clobbers them due to kernel-mode NEON
 * before re-entry into the guest.
 */
pub unsafe fn kvm_arch_vcpu_ctxsync_fp(vcpu: *mut kvm_vcpu) {
    let mut fp_state: cpu_fp_state;

    WARN_ON_ONCE(!irqs_disabled());

    if guest_owns_fp_regs() {
        /*
         * Currently we do not support SME guests so SVCR is
         * always 0 and we just need a variable to point to.
         */
        fp_state.st = &mut (*vcpu).arch.ctxt.fp_regs;
        fp_state.sve_state = (*vcpu).arch.sve_state;
        fp_state.sve_vl = (*vcpu).arch.sve_max_vl;
        fp_state.sme_state = core::ptr::null_mut();
        fp_state.svcr = __ctxt_sys_reg(&mut (*vcpu).arch.ctxt, SVCR);
        fp_state.fpmr = __ctxt_sys_reg(&mut (*vcpu).arch.ctxt, FPMR);
        fp_state.fp_type = &mut (*vcpu).arch.fp_type;

        if vcpu_has_sve(vcpu) {
            fp_state.to_save = FP_STATE_SVE;
        } else {
            fp_state.to_save = FP_STATE_FPSIMD;
        }

        fpsimd_bind_state_to_cpu(&mut fp_state);

        clear_thread_flag(TIF_FOREIGN_FPSTATE);
    }
}

/*
 * Write back the vcpu FPSIMD regs if they are dirty, and invalidate the
 * cpu FPSIMD regs so that they can't be spuriously reused if this vcpu
 * disappears and another task or vcpu appears that recycles the same
 * struct fpsimd_state.
 */
pub unsafe fn kvm_arch_vcpu_put_fp(vcpu: *mut kvm_vcpu) {
    let mut flags: ulong;

    /*
     * See comment in kvm_arch_vcpu_load_fp(). Note that we also rely on
     * the guest's max VL to have been set by fpsimd_lazy_switch_to_host()
     * so that any intervening kernel-mode SIMD (NEON or otherwise)
     * operation sees the full guest state that needs saving.
     */
    if vcpu_get_flag(vcpu, IN_NESTED_ERET) || vcpu_get_flag(vcpu, IN_NESTED_EXCEPTION) {
        WARN_ON_ONCE(host_owns_fp_regs());
        return;
    }

    local_irq_save(&mut flags);

    if guest_owns_fp_regs() {
        /*
         * Flush (save and invalidate) the fpsimd/sve state so that if
         * the host tries to use fpsimd/sve, it's not using stale data
         * from the guest.
         *
         * Flushing the state sets the TIF_FOREIGN_FPSTATE bit for the
         * context unconditionally, in both nVHE and VHE. This allows
         * the kernel to restore the fpsimd/sve state, including ZCR_EL1
         * when needed.
         */
        fpsimd_save_and_flush_cpu_state();
    }

    local_irq_restore(flags);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
