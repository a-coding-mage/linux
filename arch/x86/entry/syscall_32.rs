// SPDX-License-Identifier: GPL-2.0-only
/* 32-bit system call dispatch */

// Kernel and architecture dependencies are supplied by the surrounding translation unit.

#[cfg(CONFIG_IA32_EMULATION)]
pub const __SYSCALL_WITH_COMPAT: () = ();

/* The asm/syscalls_32.h X-macro include supplies these declarations and cases. */

#[cfg(CONFIG_X86_32)]
#[no_mangle]
pub static sys_call_table: [sys_call_ptr_t; 0] = [];

/* The unsigned int @nr argument is intentional as it creates denser code in a 64-bit build */
#[inline(never)]
unsafe fn ia32_sys_call(regs: *const pt_regs, nr: u32) -> c_long {
    match nr {
        /* asm/syscalls_32.h expands to: case nr: return __ia32_##sym(regs); */
        _ => __ia32_sys_ni_syscall(regs),
    }
}

#[inline(always)]
unsafe fn syscall_32_enter(regs: *mut pt_regs) -> c_long {
    if cfg!(CONFIG_IA32_EMULATION) {
        (*current_thread_info()).status |= TS_COMPAT;
    }

    (*regs).orig_ax as i32 as c_long
}

#[cfg(CONFIG_IA32_EMULATION)]
#[no_mangle]
pub static mut __ia32_enabled: bool = !cfg!(CONFIG_IA32_EMULATION_DEFAULT_DISABLED);

#[cfg(CONFIG_IA32_EMULATION)]
unsafe fn ia32_emulation_override_cmdline(arg: *mut c_char) -> c_int {
    kstrtobool(arg, &mut __ia32_enabled)
}

#[cfg(CONFIG_IA32_EMULATION)]
/* early_param("ia32_emulation", ia32_emulation_override_cmdline); */

/*
 * Invoke a 32-bit syscall.  Called with IRQs on in CT_STATE_KERNEL.
 */
#[inline(always)]
unsafe fn do_syscall_32_irqs_on(regs: *mut pt_regs, mut nr: c_ulong) {
    if likely(nr < IA32_NR_syscalls as c_ulong) {
        nr = array_index_nospec(nr, IA32_NR_syscalls as c_ulong);
        (*regs).ax = ia32_sys_call(regs, nr as u32) as c_ulong;
    }
}

#[cfg(CONFIG_IA32_EMULATION)]
#[inline(always)]
unsafe fn int80_is_external() -> bool {
    let offs: c_uint = (0x80 / 32) * 0x10;
    let bit: u32 = 1u32 << (0x80 % 32);

    /* The local APIC on XENPV guests is fake */
    if cpu_feature_enabled(X86_FEATURE_XENPV) {
        return false;
    }

    /* If vector 0x80 is set in the APIC ISR then this is an external interrupt. */
    apic_read(APIC_ISR + offs) & bit != 0
}

#[cfg(CONFIG_IA32_EMULATION)]
#[no_mangle]
pub unsafe extern "C" fn do_int80_emulation(regs: *mut pt_regs) {
    let mut nr: c_long;

    /* Kernel does not use INT $0x80! */
    if unlikely(!user_mode(regs)) {
        irqentry_enter(regs);
        instrumentation_begin();
        panic("Unexpected external interrupt 0x80\n".as_ptr() as *const c_char);
    }

    enter_from_user_mode_randomize_stack(regs);
    instrumentation_begin();

    /* Validate that this is a soft interrupt to the extent possible */
    if unlikely(int80_is_external()) {
        panic("Unexpected external interrupt 0x80\n".as_ptr() as *const c_char);
    }

    (*regs).orig_ax = (*regs).ax & GENMASK(31, 0) as c_ulong;
    (*regs).ax = (-ENOSYS) as c_ulong;
    nr = syscall_32_enter(regs);

    local_irq_enable();
    if likely(syscall_enter_from_user_mode_work(regs, &mut nr)) {
        do_syscall_32_irqs_on(regs, nr as c_ulong);
    }
    instrumentation_end();
    syscall_exit_to_user_mode(regs);
}

#[cfg(all(CONFIG_IA32_EMULATION, CONFIG_X86_FRED))]
#[no_mangle]
pub unsafe extern "C" fn int80_emulation(regs: *mut pt_regs) {
    let mut nr: c_long;
    enter_from_user_mode_randomize_stack(regs);
    instrumentation_begin();
    (*regs).orig_ax = (*regs).ax & GENMASK(31, 0) as c_ulong;
    (*regs).ax = (-ENOSYS) as c_ulong;
    nr = syscall_32_enter(regs);
    local_irq_enable();
    if likely(syscall_enter_from_user_mode_work(regs, &mut nr)) {
        do_syscall_32_irqs_on(regs, nr as c_ulong);
    }
    instrumentation_end();
    syscall_exit_to_user_mode(regs);
}

#[cfg(not(CONFIG_IA32_EMULATION))]
#[no_mangle]
pub unsafe extern "C" fn do_int80_syscall_32(regs: *mut pt_regs) {
    let mut nr = syscall_32_enter(regs);
    /* A ptrace-provided value is truncated by the int return semantics. */
    if likely(syscall_enter_from_user_mode_randomize_stack(regs, &mut nr)) {
        instrumentation_begin();
        do_syscall_32_irqs_on(regs, nr as c_ulong);
        instrumentation_end();
    }
    syscall_exit_to_user_mode(regs);
}

unsafe fn __do_fast_syscall_32(regs: *mut pt_regs) -> bool {
    let mut nr = syscall_32_enter(regs);
    let res: c_int;
    enter_from_user_mode_randomize_stack(regs);
    instrumentation_begin();
    local_irq_enable();
    /* Fetch EBP from where the vDSO stashed it. */
    if cfg!(CONFIG_X86_64) {
        res = __get_user(&mut (*regs).bp as *mut _ as *mut u32,
            ( (*regs).sp as u32) as usize as *mut u32);
    } else {
        res = get_user(&mut (*regs).bp as *mut _ as *mut u32,
            ( (*regs).sp as u32) as usize as *mut u32);
    }
    if res != 0 {
        (*regs).ax = (-EFAULT) as c_ulong;
        local_irq_disable();
        instrumentation_end();
        irqentry_exit_to_user_mode(regs);
        return false;
    }
    if likely(syscall_enter_from_user_mode_work(regs, &mut nr)) {
        do_syscall_32_irqs_on(regs, nr as c_ulong);
    }
    instrumentation_end();
    syscall_exit_to_user_mode(regs);
    true
}

#[no_mangle]
pub unsafe extern "C" fn do_fast_syscall_32(regs: *mut pt_regs) -> bool {
    let landing_pad = (*current).mm.context.vdso as c_ulong + vdso32_image.sym_int80_landing_pad;
    (*regs).ip = landing_pad;
    if !__do_fast_syscall_32(regs) { return false; }
    if cpu_feature_enabled(X86_FEATURE_XENPV) { return false; }
    if unlikely((*regs).ip != landing_pad) { return false; }
    if unlikely((*regs).cs != __USER32_CS || (*regs).ss != __USER_DS) { return false; }
    if unlikely((*regs).flags & (X86_EFLAGS_RF | X86_EFLAGS_TF | X86_EFLAGS_VM) != 0) { return false; }
    true
}

#[no_mangle]
pub unsafe extern "C" fn do_SYSENTER_32(regs: *mut pt_regs) -> bool {
    (*regs).sp = (*regs).bp;
    (*regs).flags |= X86_EFLAGS_IF;
    do_fast_syscall_32(regs)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
