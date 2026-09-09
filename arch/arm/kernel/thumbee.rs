// SPDX-License-Identifier: GPL-2.0-only
/*
 * arch/arm/kernel/thumbee.c
 *
 * Copyright (C) 2008 ARM Limited
 */

// C dependencies supplied by the surrounding kernel translation.

/*
 * Access to the ThumbEE Handler Base register
 */
#[inline]
unsafe fn teehbr_read() -> ::core::ffi::c_ulong {
    let v: ::core::ffi::c_ulong;
    ::core::arch::asm!(
        "mrc p14, 6, {0}, c1, c0, 0",
        out(reg) v,
    );
    v
}

#[inline]
unsafe fn teehbr_write(v: ::core::ffi::c_ulong) {
    ::core::arch::asm!(
        "mcr p14, 6, {0}, c1, c0, 0",
        in(reg) v,
    );
}

unsafe fn thumbee_notifier(
    self_: *mut notifier_block,
    cmd: ::core::ffi::c_ulong,
    t: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let _ = self_;
    let thread = t as *mut thread_info;

    match cmd {
        THREAD_NOTIFY_FLUSH => {
            teehbr_write(0);
        }
        THREAD_NOTIFY_SWITCH => {
            (*current_thread_info()).thumbee_state = teehbr_read();
            teehbr_write((*thread).thumbee_state);
        }
        _ => {}
    }

    NOTIFY_DONE
}

static mut thumbee_notifier_block: notifier_block = notifier_block {
    notifier_call: Some(thumbee_notifier),
};

unsafe fn thumbee_init() -> ::core::ffi::c_int {
    let mut pfr0: ::core::ffi::c_ulong;
    let cpu_arch: ::core::ffi::c_uint = cpu_architecture();

    if cpu_arch < CPU_ARCH_ARMv7 {
        return 0;
    }

    pfr0 = read_cpuid_ext(CPUID_EXT_PFR0);
    if (pfr0 & 0x0000f000) != 0x00001000 {
        return 0;
    }

    pr_info!("ThumbEE CPU extension supported.\n");
    elf_hwcap |= HWCAP_THUMBEE;
    thread_register_notifier(&mut thumbee_notifier_block);

    0
}

// Corresponds to: late_initcall(thumbee_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
