// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *
 * Copyright (C) 2007 Lemote, Inc. & Institute of Computing Technology
 * Author: Fuxin Zhang, zhangfx@lemote.com
 * Copyright (C) 2009 Lemote, Inc.
 * Author: Zhangjin Wu, wuzhangjin@gmail.com
 */

// Translated from the Linux MIPS reset implementation. The following names
// are supplied by the surrounding kernel environment.
use core::ffi::c_void;

extern "C" {
    static mut _machine_restart: Option<unsafe extern "C" fn(*mut i8)>;
    static mut _machine_halt: Option<unsafe extern "C" fn()>;
    static mut pm_power_off: Option<unsafe extern "C" fn()>;
    static mut cpu_wait: Option<unsafe extern "C" fn()>;

    fn ioremap(addr: usize, size: usize) -> *mut c_void;
    fn mach_prepare_reboot();
    fn mach_prepare_shutdown();
    fn pr_notice(fmt: *const i8, ...);
}

// Supplied by <loongson.h>.
const LOONGSON_BOOT_BASE: usize = 0; // build-time platform constant

#[inline]
unsafe fn loongson_reboot() {
    #[cfg(not(CONFIG_CPU_JUMP_WORKAROUNDS))]
    {
        let func = ioremap(LOONGSON_BOOT_BASE, 4) as *mut (); 
        let func: unsafe extern "C" fn() = core::mem::transmute(func);
        func();
    }

    #[cfg(CONFIG_CPU_JUMP_WORKAROUNDS)]
    {
        let func = ioremap(LOONGSON_BOOT_BASE, 4) as *mut ();
        core::arch::asm!(
            ".set\tnoat",
            "jr\t{func}",
            ".set\tat",
            func = in(reg) func,
            options(noreturn)
        );
    }
}

unsafe extern "C" fn loongson_restart(_command: *mut i8) {
    /* do preparation for reboot */
    mach_prepare_reboot();

    /* reboot via jumping to boot base address */
    loongson_reboot();
}

unsafe extern "C" fn loongson_poweroff() {
    mach_prepare_shutdown();

    /*
     * It needs a wait loop here, but mips/kernel/reset.c already calls
     * a generic delay loop, machine_hang(), so simply return.
     */
    return;
}

unsafe extern "C" fn loongson_halt() {
    pr_notice(b"\n\n** You can safely turn off the power now **\n\n\0".as_ptr() as *const i8);
    loop {
        if let Some(wait) = cpu_wait {
            wait();
        }
    }
}

unsafe extern "C" fn mips_reboot_setup() -> i32 {
    _machine_restart = Some(loongson_restart);
    _machine_halt = Some(loongson_halt);
    pm_power_off = Some(loongson_poweroff);

    0
}

// Equivalent of the Linux arch_initcall(mips_reboot_setup) registration.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
