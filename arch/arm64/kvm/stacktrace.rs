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

/* C headers and assembly-provided symbols are supplied by the surrounding translation. */

unsafe fn stackinfo_get_overflow() -> stack_info {
    let stacktrace_info = this_cpu_ptr_nvhe_sym(kvm_stacktrace_info);
    let low = stacktrace_info.overflow_stack_base as usize;
    let high = low + OVERFLOW_STACK_SIZE;
    stack_info { low, high }
}

unsafe fn stackinfo_get_overflow_kern_va() -> stack_info {
    let low = this_cpu_ptr_nvhe_sym(overflow_stack) as usize;
    let high = low + OVERFLOW_STACK_SIZE;
    stack_info { low, high }
}

unsafe fn stackinfo_get_hyp() -> stack_info {
    let stacktrace_info = this_cpu_ptr_nvhe_sym(kvm_stacktrace_info);
    let low = stacktrace_info.stack_base as usize;
    let high = low + NVHE_STACK_SIZE;
    stack_info { low, high }
}

unsafe fn stackinfo_get_hyp_kern_va() -> stack_info {
    let low = (*this_cpu_ptr(&kvm_arm_hyp_stack_base)) as usize;
    let high = low + NVHE_STACK_SIZE;
    stack_info { low, high }
}

/*
 * kvm_nvhe_stack_kern_va - Convert KVM nVHE HYP stack addresses to a kernel VAs
 *
 * The nVHE hypervisor stack is mapped in the flexible 'private' VA range, to
 * allow for guard pages below the stack. Consequently, the fixed offset address
 * translation macros won't work here.
 *
 * The kernel VA is calculated as an offset from the kernel VA of the hypervisor
 * stack base.
 *
 * Returns true on success and updates @addr to its corresponding kernel VA;
 * otherwise returns false.
 */
unsafe fn kvm_nvhe_stack_kern_va(addr: *mut usize, size: usize) -> bool {
    let mut stack_hyp = stackinfo_get_hyp();
    let mut stack_kern = stackinfo_get_hyp_kern_va();
    if stackinfo_on_stack(&stack_hyp, *addr, size) {
        *addr = *addr - stack_hyp.low + stack_kern.low;
        return true;
    }

    stack_hyp = stackinfo_get_overflow();
    stack_kern = stackinfo_get_overflow_kern_va();
    if stackinfo_on_stack(&stack_hyp, *addr, size) {
        *addr = *addr - stack_hyp.low + stack_kern.low;
        return true;
    }

    false
}

/* Convert a KVN nVHE HYP frame record address to a kernel VA */
unsafe fn kvm_nvhe_stack_kern_record_va(addr: *mut usize) -> bool {
    kvm_nvhe_stack_kern_va(addr, 16)
}

unsafe fn unwind_next(state: *mut unwind_state) -> i32 {
    /* The FP is in the hypervisor VA space. Convert it to the kernel VA space. */
    if !kvm_nvhe_stack_kern_record_va(&mut (*state).fp) {
        return -EINVAL;
    }
    unwind_next_frame_record(state)
}

unsafe fn unwind(state: *mut unwind_state, consume_entry: stack_trace_consume_fn, cookie: *mut core::ffi::c_void) {
    loop {
        if !consume_entry(cookie, (*state).pc) {
            break;
        }
        if unwind_next(state) < 0 {
            break;
        }
    }
}

/* Symbolize and print an nVHE backtrace entry. */
unsafe fn kvm_nvhe_dump_backtrace_entry(arg: *mut core::ffi::c_void, mut where_: usize) -> bool {
    let va_mask = GENMASK_ULL(__hyp_va_bits - 1, 0);
    let hyp_offset = arg as usize;
    where_ = (where_ & va_mask) + hyp_offset;
    kvm_err(" [<%016lx>] %pB\n", where_, (where_ + kaslr_offset()) as *mut core::ffi::c_void);
    true
}

unsafe fn kvm_nvhe_dump_backtrace_start() {
    kvm_err("nVHE call trace:\n");
}

unsafe fn kvm_nvhe_dump_backtrace_end() {
    kvm_err("---[ end nVHE call trace ]---\n");
}

/* Dump the non-protected nVHE backtrace. */
unsafe fn hyp_dump_backtrace(hyp_offset: usize) {
    let stacks = [stackinfo_get_overflow_kern_va(), stackinfo_get_hyp_kern_va()];
    let mut state = unwind_state {
        stacks: stacks.as_ptr(),
        nr_stacks: stacks.len(),
        ..core::mem::zeroed()
    };
    let stacktrace_info = this_cpu_ptr_nvhe_sym(kvm_stacktrace_info);
    kvm_nvhe_unwind_init(&mut state, stacktrace_info.fp, stacktrace_info.pc);
    kvm_nvhe_dump_backtrace_start();
    unwind(&mut state, kvm_nvhe_dump_backtrace_entry, hyp_offset as *mut _);
    kvm_nvhe_dump_backtrace_end();
}

#[cfg(CONFIG_PKVM_STACKTRACE)]
unsafe fn pkvm_dump_backtrace(hyp_offset: usize) {
    let stacktrace = this_cpu_ptr_nvhe_sym(pkvm_stacktrace) as *const usize;
    kvm_nvhe_dump_backtrace_start();
    let mut i = 0;
    while i < ARRAY_SIZE(kvm_nvhe_sym(pkvm_stacktrace)) && *stacktrace.add(i) != 0 {
        kvm_nvhe_dump_backtrace_entry(hyp_offset as *mut _, *stacktrace.add(i));
        i += 1;
    }
    kvm_nvhe_dump_backtrace_end();
}

#[cfg(not(CONFIG_PKVM_STACKTRACE))]
unsafe fn pkvm_dump_backtrace(_hyp_offset: usize) {
    kvm_err("Cannot dump pKVM nVHE stacktrace: !CONFIG_PKVM_STACKTRACE\n");
}

/* Dump KVM nVHE hypervisor backtrace. */
pub unsafe fn kvm_nvhe_dump_backtrace(hyp_offset: usize) {
    if is_protected_kvm_enabled() {
        pkvm_dump_backtrace(hyp_offset);
    } else {
        hyp_dump_backtrace(hyp_offset);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
