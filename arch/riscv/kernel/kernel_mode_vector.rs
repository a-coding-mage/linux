// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2012 ARM Ltd.
 * Author: Catalin Marinas <catalin.marinas@arm.com>
 * Copyright (C) 2017 Linaro Ltd. <ard.biesheuvel@linaro.org>
 * Copyright (C) 2021 SiFive
 */

// C dependencies: linux/compiler.h, linux/irqflags.h, linux/percpu.h,
// linux/preempt.h, linux/types.h, linux/kvm_types.h, asm/vector.h,
// asm/kvm_vcpu_vector.h, asm/switch_to.h, asm/simd.h, and, when enabled,
// asm/asm-prototypes.h.

static mut KVM_FLUSH_VECTOR_CTX_CALLBACK: Option<unsafe extern "C" fn()> = None;

pub unsafe extern "C" fn kvm_riscv_register_vctx_callback(
    func: Option<unsafe extern "C" fn()>,
) {
    if WARN_ON_ONCE(rcu_access_pointer(unsafe { KVM_FLUSH_VECTOR_CTX_CALLBACK })) {
        return;
    }

    rcu_assign_pointer(unsafe { &mut KVM_FLUSH_VECTOR_CTX_CALLBACK }, func);
}

pub unsafe extern "C" fn kvm_riscv_unregister_vctx_callback() {
    rcu_assign_pointer(unsafe { &mut KVM_FLUSH_VECTOR_CTX_CALLBACK }, None);
    synchronize_rcu();
}

#[inline]
unsafe fn riscv_v_start(flags: u32) {
    let orig: i32;

    orig = riscv_v_flags();
    BUG_ON((orig & flags as i32) != 0);
    riscv_v_flags_set(orig | flags as i32);
    barrier();
}

#[inline]
unsafe fn riscv_v_stop(flags: u32) {
    let orig: i32;

    barrier();
    orig = riscv_v_flags();
    BUG_ON((orig & flags as i32) == 0);
    riscv_v_flags_set(orig & !(flags as i32));
}

/*
 * Claim ownership of the CPU vector context for use by the calling context.
 *
 * The caller may freely manipulate the vector context metadata until
 * put_cpu_vector_context() is called.
 */
pub unsafe extern "C" fn get_cpu_vector_context() {
    /*
     * disable softirqs so it is impossible for softirqs to nest
     * get_cpu_vector_context() when kernel is actively using Vector.
     */
    if !IS_ENABLED(CONFIG_PREEMPT_RT) {
        if !irqs_disabled() {
            local_bh_disable();
        }
    } else {
        preempt_disable();
    }

    riscv_v_start(RISCV_KERNEL_MODE_V);
}

/*
 * Release the CPU vector context.
 *
 * Must be called from a context in which get_cpu_vector_context() was
 * previously called, with no call to put_cpu_vector_context() in the
 * meantime.
 */
pub unsafe extern "C" fn put_cpu_vector_context() {
    riscv_v_stop(RISCV_KERNEL_MODE_V);

    if !IS_ENABLED(CONFIG_PREEMPT_RT) {
        if !irqs_disabled() {
            local_bh_enable();
        }
    } else {
        preempt_enable();
    }
}

unsafe fn __riscv_flush_vector_context() {
    let vcpu_flush_v_callback: Option<unsafe extern "C" fn()>;

    if riscv_v_flags() & RISCV_V_VCPU_CTX as i32 != 0 {
        rcu_read_lock();
        vcpu_flush_v_callback = rcu_dereference(unsafe { KVM_FLUSH_VECTOR_CTX_CALLBACK });
        vcpu_flush_v_callback.unwrap()();
        rcu_read_unlock();
        return;
    }

    riscv_v_vstate_save(&mut (*current).thread.vstate, task_pt_regs(current));
    riscv_v_vstate_set_restore(current, task_pt_regs(current));
}

#[cfg(CONFIG_RISCV_ISA_V_PREEMPTIVE)]
#[inline(always)]
unsafe fn riscv_v_flags_ptr() -> *mut u32 {
    &mut (*current).thread.riscv_v_flags
}

#[cfg(CONFIG_RISCV_ISA_V_PREEMPTIVE)]
#[inline]
unsafe fn riscv_preempt_v_set_dirty() {
    *riscv_v_flags_ptr() |= RISCV_PREEMPT_V_DIRTY;
}

#[cfg(CONFIG_RISCV_ISA_V_PREEMPTIVE)]
#[inline]
unsafe fn riscv_preempt_v_reset_flags() {
    *riscv_v_flags_ptr() &= !(RISCV_PREEMPT_V_DIRTY | RISCV_PREEMPT_V_NEED_RESTORE);
}

#[cfg(CONFIG_RISCV_ISA_V_PREEMPTIVE)]
#[inline]
unsafe fn riscv_v_ctx_depth_inc() {
    *riscv_v_flags_ptr() += RISCV_V_CTX_UNIT_DEPTH;
}

#[cfg(CONFIG_RISCV_ISA_V_PREEMPTIVE)]
#[inline]
unsafe fn riscv_v_ctx_depth_dec() {
    *riscv_v_flags_ptr() -= RISCV_V_CTX_UNIT_DEPTH;
}

#[cfg(CONFIG_RISCV_ISA_V_PREEMPTIVE)]
#[inline]
unsafe fn riscv_v_ctx_get_depth() -> u32 {
    *riscv_v_flags_ptr() & RISCV_V_CTX_DEPTH_MASK
}

#[cfg(CONFIG_RISCV_ISA_V_PREEMPTIVE)]
unsafe fn riscv_v_stop_kernel_context() -> i32 {
    if riscv_v_ctx_get_depth() != 0 || !riscv_preempt_v_started(current) {
        return 1;
    }

    riscv_preempt_v_clear_dirty(current);
    riscv_v_stop(RISCV_PREEMPT_V);
    0
}

#[cfg(CONFIG_RISCV_ISA_V_PREEMPTIVE)]
unsafe fn riscv_v_start_kernel_context() -> i32 {
    let kvstate: *mut __riscv_v_ext_state;

    kvstate = &mut (*current).thread.kernel_vstate;
    if (*kvstate).datap.is_null() {
        return -ENOENT;
    }

    if riscv_preempt_v_started(current) {
        WARN_ON(riscv_v_ctx_get_depth() == 0);
        get_cpu_vector_context();
        if riscv_preempt_v_dirty(current) {
            __riscv_v_vstate_save(kvstate, (*kvstate).datap);
            riscv_preempt_v_clear_dirty(current);
        }
        riscv_preempt_v_set_restore(current);
        return 0;
    }

    /* Transfer the ownership of V from user to kernel, then save */
    get_cpu_vector_context();
    __riscv_flush_vector_context();
    put_cpu_vector_context();
    /*
     *  A voluntary context switch caused by put_cpu_vector_context() can
     *  raise the NEED_RESTORE flag if preempt_v starts too early due to a
     *  failed risv_v_is_on() check.
     *
     *  This causes the next context_nesting_end pollute the v-reg from the
     *  stale context memory in kernel-mode vector.
     */
    riscv_v_start(RISCV_PREEMPT_V);
    0
}

#[cfg(CONFIG_RISCV_ISA_V_PREEMPTIVE)]
pub unsafe extern "C" fn riscv_v_context_nesting_start(regs: *mut pt_regs) {
    let depth: i32;

    if !riscv_preempt_v_started(current) {
        return;
    }

    depth = riscv_v_ctx_get_depth() as i32;
    if depth == 0 && __riscv_v_vstate_check((*regs).status, DIRTY) {
        riscv_preempt_v_set_dirty();
    }

    riscv_v_ctx_depth_inc();
}

#[cfg(CONFIG_RISCV_ISA_V_PREEMPTIVE)]
pub unsafe extern "C" fn riscv_v_context_nesting_end(regs: *mut pt_regs) {
    let vstate: *mut __riscv_v_ext_state = &mut (*current).thread.kernel_vstate;
    let depth: u32;

    WARN_ON(!irqs_disabled());

    if !riscv_preempt_v_started(current) {
        return;
    }

    riscv_v_ctx_depth_dec();
    depth = riscv_v_ctx_get_depth();
    if depth == 0 {
        if riscv_preempt_v_restore(current) {
            __riscv_v_vstate_restore(vstate, (*vstate).datap);
            __riscv_v_vstate_clean(regs);
            riscv_preempt_v_reset_flags();
        }
    }
}

#[cfg(not(CONFIG_RISCV_ISA_V_PREEMPTIVE))]
unsafe fn riscv_v_start_kernel_context() -> i32 { -ENOENT }

#[cfg(not(CONFIG_RISCV_ISA_V_PREEMPTIVE))]
unsafe fn riscv_v_stop_kernel_context() -> i32 { -ENOENT }

/*
 * kernel_vector_begin(): obtain the CPU vector registers for use by the calling
 * context
 *
 * Must not be called unless may_use_simd() returns true.
 * Task context in the vector registers is saved back to memory as necessary.
 *
 * A matching call to kernel_vector_end() must be made before returning from the
 * calling context.
 *
 * The caller may freely use the vector registers until kernel_vector_end() is
 * called.
 */
pub unsafe extern "C" fn kernel_vector_begin() {
    if WARN_ON(!(has_vector() || has_xtheadvector())) {
        return;
    }

    BUG_ON(!may_use_simd());

    if riscv_v_start_kernel_context() != 0 {
        get_cpu_vector_context();
        __riscv_flush_vector_context();
    }

    riscv_v_enable();
}

/*
 * kernel_vector_end(): give the CPU vector registers back to the current task
 *
 * Must be called from a context in which kernel_vector_begin() was previously
 * called, with no call to kernel_vector_end() in the meantime.
 *
 * The caller must not use the vector registers after this function is called,
 * unless kernel_vector_begin() is called again in the meantime.
 */
pub unsafe extern "C" fn kernel_vector_end() {
    if WARN_ON(!(has_vector() || has_xtheadvector())) {
        return;
    }

    riscv_v_disable();

    if riscv_v_stop_kernel_context() != 0 {
        put_cpu_vector_context();
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
