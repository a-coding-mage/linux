/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * KVM nVHE hypervisor stack tracing support.
 *
 * The unwinder implementation depends on the nVHE mode:
 *
 *   1) Non-protected nVHE mode - the host can directly access the
 *      HYP stack pages and unwind the HYP stack in EL1. This saves having
 *      to allocate shared buffers for the host to read the unwinded
 *      stacktrace.
 *
 *   2) pKVM (protected nVHE) mode - the host cannot directly access
 *      the HYP memory. The stack is unwinded in EL2 and dumped to a shared
 *      buffer where the host can read and print the stacktrace.
 *
 * Copyright (C) 2022 Google LLC
 */

// Dependency corresponding to <asm/stacktrace/common.h>.

/**
 * kvm_nvhe_unwind_init() - Start an unwind from the given nVHE HYP fp and pc
 *
 * @state : unwind_state to initialize
 * @fp    : frame pointer at which to start the unwinding.
 * @pc    : program counter at which to start the unwinding.
 */
#[inline]
pub unsafe fn kvm_nvhe_unwind_init(
    state: *mut unwind_state,
    fp: c_ulong,
    pc: c_ulong,
) {
    unwind_init_common(state);

    (*state).fp = fp;
    (*state).pc = pc;
}

// __KVM_NVHE_HYPERVISOR__ controls whether the conventional host-side
// nVHE unwinder declarations below are present at build time.
#[cfg(not(kvm_nvhe_hypervisor))]
pub mod non_protected {
    // Conventional (non-protected) nVHE HYP stack unwinder
    //
    // In non-protected mode, the unwinding is done from kernel proper context
    // (by the host in EL1).

    // DECLARE_KVM_NVHE_PER_CPU declarations retain per-CPU storage intent.
    unsafe extern "C" {
        pub static mut overflow_stack:
            [c_ulong; OVERFLOW_STACK_SIZE / core::mem::size_of::<c_ulong>()];
        pub static mut kvm_stacktrace_info: kvm_nvhe_stacktrace_info;
        pub static mut kvm_arm_hyp_stack_base: c_ulong;
        pub fn kvm_nvhe_dump_backtrace(hyp_offset: c_ulong);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
