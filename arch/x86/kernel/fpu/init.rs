// SPDX-License-Identifier: GPL-2.0-only
/*
 * x86 FPU boot time init code:
 */

// Dependencies supplied by the corresponding kernel headers and source files.

/*
 * Initialize the registers found in all CPUs, CR0 and CR4:
 */
unsafe fn fpu__init_cpu_generic() {
    let mut cr0: c_ulong;
    let mut cr4_mask: c_ulong = 0;

    if boot_cpu_has(X86_FEATURE_FXSR) {
        cr4_mask |= X86_CR4_OSFXSR;
    }
    if boot_cpu_has(X86_FEATURE_XMM) {
        cr4_mask |= X86_CR4_OSXMMEXCPT;
    }
    if cr4_mask != 0 {
        cr4_set_bits(cr4_mask);
    }

    cr0 = read_cr0();
    cr0 &= !(X86_CR0_TS | X86_CR0_EM); /* clear TS and EM */
    if !boot_cpu_has(X86_FEATURE_FPU) {
        cr0 |= X86_CR0_EM;
    }
    write_cr0(cr0);

    /* Flush out any pending x87 state: */
    core::arch::asm!("fninit");
}

/*
 * Enable all supported FPU features. Called when a CPU is brought online:
 */
pub unsafe fn fpu__init_cpu() {
    fpu__init_cpu_generic();
    fpu__init_cpu_xstate();

    /* Start allowing kernel-mode FPU: */
    this_cpu_write(kernel_fpu_allowed, true);
}

unsafe fn fpu__probe_without_cpuid() -> bool {
    let mut cr0: c_ulong;
    let mut fsw: u16 = 0xffff;
    let mut fcw: u16 = 0xffff;

    cr0 = read_cr0();
    cr0 &= !(X86_CR0_TS | X86_CR0_EM);
    write_cr0(cr0);

    core::arch::asm!(
        "fninit; fnstsw {0}; fnstcw {1}",
        inout(reg) fsw,
        inout(reg) fcw,
    );

    pr_info!("x86/fpu: Probing for FPU: FSW=0x{:04x} FCW=0x{:04x}\n", fsw, fcw);

    fsw == 0 && (fcw & 0x103f) == 0x003f
}

unsafe fn fpu__init_system_early_generic() {
    set_thread_flag(TIF_NEED_FPU_LOAD);

    if !boot_cpu_has(X86_FEATURE_CPUID)
        && !test_bit(X86_FEATURE_FPU, cpu_caps_cleared as *mut c_ulong)
    {
        if fpu__probe_without_cpuid() {
            setup_force_cpu_cap(X86_FEATURE_FPU);
        } else {
            setup_clear_cpu_cap(X86_FEATURE_FPU);
        }
    }

    if !test_cpu_cap(&boot_cpu_data, X86_FEATURE_FPU) {
        pr_emerg!("x86/fpu: Giving up, no FPU found and no math emulation present\n");
        loop {
            core::arch::asm!("hlt");
        }
    }
}

/*
 * Boot time FPU feature detection code:
 */
pub static mut mxcsr_feature_mask: u32 = 0xffff_ffff;

unsafe fn fpu__init_system_mxcsr() {
    let mut mask: u32 = 0;

    if boot_cpu_has(X86_FEATURE_FXSR) {
        /* Static because GCC does not get 16-byte stack alignment right: */
        static mut fxregs: fxregs_state = fxregs_state::default();

        core::arch::asm!("fxsave [{0}]", in(reg) &raw mut fxregs);

        mask = fxregs.mxcsr_mask;

        /*
         * If zero then use the default features mask,
         * which has all features set, except the
         * denormals-are-zero feature bit:
         */
        if mask == 0 {
            mask = 0x0000_ffbf;
        }
    }
    mxcsr_feature_mask &= mask;
}

/*
 * Once per bootup FPU initialization sequences that will run on most x86 CPUs:
 */
unsafe fn fpu__init_system_generic() {
    /*
     * Set up the legacy init FPU context. Will be updated when the
     * CPU supports XSAVE[S].
     */
    fpstate_init_user(&raw mut init_fpstate);

    fpu__init_system_mxcsr();
}

/* Enforce that `MEMBER` is the last field of `TYPE`. */
// CHECK_MEMBER_AT_END_OF(TYPE, MEMBER) is a build-time C layout assertion.

/*
 * We append the `struct fpu` to the task_struct:
 */
unsafe fn fpu__init_task_struct_size() {
    let mut task_size: c_int = core::mem::size_of::<task_struct>() as c_int;

    task_size += core::mem::size_of::<fpu>() as c_int;

    /* Subtract off the static size of the register state. */
    task_size -= core::mem::size_of::<fpregs_state>() as c_int;

    /* Add back the dynamically-calculated register state size. */
    task_size += fpu_kernel_cfg.default_size as c_int;

    /* `struct fpu` is dynamically sized, so `state` must be at its end. */
    arch_task_struct_size = task_size;
}

/*
 * Set up the user and kernel xstate sizes based on the legacy FPU context size.
 *
 * We set this up first, and later it will be overwritten by
 * fpu__init_system_xstate() if the CPU knows about xstates.
 */
unsafe fn fpu__init_system_xstate_size_legacy() {
    let size: usize;

    /* The size configuration might be overwritten later. */
    if !cpu_feature_enabled(X86_FEATURE_FPU) {
        size = core::mem::size_of::<swregs_state>();
    } else if cpu_feature_enabled(X86_FEATURE_FXSR) {
        size = core::mem::size_of::<fxregs_state>();
        fpu_user_cfg.legacy_features = XFEATURE_MASK_FPSSE;
    } else {
        size = core::mem::size_of::<fregs_state>();
        fpu_user_cfg.legacy_features = XFEATURE_MASK_FP;
    }

    fpu_kernel_cfg.max_size = size;
    fpu_kernel_cfg.default_size = size;
    fpu_user_cfg.max_size = size;
    fpu_user_cfg.default_size = size;
    guest_default_cfg.size = size;
}

/*
 * Called on the boot CPU once per system bootup, to set up the initial
 * FPU state that is later cloned into all processes:
 */
pub unsafe fn fpu__init_system() {
    fpu__init_system_early_generic();

    /* The FPU has to be operational for later FPU init activities. */
    fpu__init_cpu();

    fpu__init_system_generic();
    fpu__init_system_xstate_size_legacy();
    fpu__init_system_xstate(fpu_kernel_cfg.max_size);
    fpu__init_task_struct_size();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
