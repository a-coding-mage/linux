// SPDX-License-Identifier: GPL-2.0-only
/*
 * linux/arch/arm/kernel/xscale-cp0.c
 *
 * XScale DSP and iWMMXt coprocessor context switching and handling
 */

// C dependencies: linux/types.h, linux/kernel.h, linux/signal.h,
// linux/sched.h, linux/init.h, linux/io.h, asm/thread_notify.h,
// asm/cputype.h

// asm("\t.arch armv5te\n");

#[inline]
unsafe fn dsp_save_state(state: *mut u32) {
    core::arch::asm!(
        "mrrc p0, 0, {0}, {1}, c0",
        out(reg) *state.add(0),
        out(reg) *state.add(1),
    );
}

#[inline]
unsafe fn dsp_load_state(state: *const u32) {
    core::arch::asm!(
        "mcrr p0, 0, {0}, {1}, c0",
        in(reg) *state.add(0),
        in(reg) *state.add(1),
    );
}

unsafe fn dsp_do(_self: *mut notifier_block, cmd: usize, t: *mut core::ffi::c_void) -> i32 {
    let thread = t as *mut thread_info;

    match cmd {
        THREAD_NOTIFY_FLUSH => {
            (*thread).cpu_context.extra[0] = 0;
            (*thread).cpu_context.extra[1] = 0;
        }

        THREAD_NOTIFY_SWITCH => {
            dsp_save_state(current_thread_info().cpu_context.extra.as_mut_ptr());
            dsp_load_state((*thread).cpu_context.extra.as_ptr());
        }

        _ => {}
    }

    NOTIFY_DONE
}

static mut dsp_notifier_block: notifier_block = notifier_block {
    notifier_call: Some(dsp_do),
};

// Preserved from #ifdef CONFIG_IWMMXT.
#[cfg(CONFIG_IWMMXT)]
unsafe fn iwmmxt_do(_self: *mut notifier_block, cmd: usize, t: *mut core::ffi::c_void) -> i32 {
    let thread = t as *mut thread_info;

    match cmd {
        THREAD_NOTIFY_FLUSH => {
            /*
             * flush_thread() zeroes thread->fpstate, so no need
             * to do anything here.
             *
             * FALLTHROUGH: Ensure we don't try to overwrite our newly
             * initialised state information on the first fault.
             */
        }

        THREAD_NOTIFY_EXIT => {
            iwmmxt_task_release(thread);
        }

        THREAD_NOTIFY_SWITCH => {
            iwmmxt_task_switch(thread);
        }

        _ => {}
    }

    NOTIFY_DONE
}

#[cfg(CONFIG_IWMMXT)]
static mut iwmmxt_notifier_block: notifier_block = notifier_block {
    notifier_call: Some(iwmmxt_do),
};

unsafe fn xscale_cp_access_read() -> u32 {
    let value: u32;

    core::arch::asm!(
        "mrc p15, 0, {0}, c15, c1, 0",
        out(reg) value,
    );

    value
}

unsafe fn xscale_cp_access_write(value: u32) {
    let temp: u32;

    core::arch::asm!(
        "mcr p15, 0, {1}, c15, c1, 0",
        "mrc p15, 0, {0}, c15, c1, 0",
        "mov {0}, {0}",
        "sub pc, pc, #4",
        out(reg) temp,
        in(reg) value,
    );
}

/*
 * Detect whether we have a MAC coprocessor (40 bit register) or an
 * iWMMXt coprocessor (64 bit registers) by loading 00000100:00000000
 * into a coprocessor register and reading it back, and checking
 * whether the upper word survived intact.
 */
unsafe fn cpu_has_iwmmxt() -> i32 {
    let lo: u32;
    let hi: u32;

    /*
     * This sequence is interpreted by the DSP coprocessor as:
     *\tmar\tacc0, %2, %3
     *\tmra\t%0, %1, acc0
     *
     * And by the iWMMXt coprocessor as:
     *\ttmcrr\twR0, %2, %3
     *\ttmrrc\t%0, %1, wR0
     */
    core::arch::asm!(
        "mcrr p0, 0, {2}, {3}, c0",
        "mrrc p0, 0, {0}, {1}, c0",
        out(reg) lo,
        out(reg) hi,
        in(reg) 0u32,
        in(reg) 0x100u32,
    );

    (hi != 0) as i32
}

/*
 * If we detect that the CPU has iWMMXt (and CONFIG_IWMMXT=y), we
 * disable CP0/CP1 on boot, and let call_fpe() and the iWMMXt lazy
 * switch code handle iWMMXt context switching.  If on the other
 * hand the CPU has a DSP coprocessor, we keep access to CP0 enabled
 * all the time, and save/restore acc0 on context switch in non-lazy
 * fashion.
 */
unsafe fn xscale_cp0_init() -> i32 {
    let mut cp_access: u32;

    /* do not attempt to probe iwmmxt on non-xscale family CPUs */
    if !cpu_is_xscale_family() {
        return 0;
    }

    cp_access = xscale_cp_access_read() & !3;
    xscale_cp_access_write(cp_access | 1);

    if cpu_has_iwmmxt() != 0 {
        // Preserved from #ifndef CONFIG_IWMMXT / #else.
        #[cfg(not(CONFIG_IWMMXT))]
        pr_warn!("CAUTION: XScale iWMMXt coprocessor detected, but kernel support is missing.\n");
        #[cfg(CONFIG_IWMMXT)]
        {
            pr_info!("XScale iWMMXt coprocessor detected.\n");
            elf_hwcap |= HWCAP_IWMMXT;
            thread_register_notifier(&raw mut iwmmxt_notifier_block);
            register_iwmmxt_undef_handler();
        }
    } else {
        pr_info!("XScale DSP coprocessor detected.\n");
        thread_register_notifier(&raw mut dsp_notifier_block);
        cp_access |= 1;
    }

    xscale_cp_access_write(cp_access);

    0
}

// late_initcall(xscale_cp0_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
