// SPDX-License-Identifier: GPL-2.0-only
/*
 * Joshua Henderson <joshua.henderson@microchip.com>
 * Copyright (C) 2015 Microchip Technology Inc.  All rights reserved.
 */

use core::ffi::c_void;

// Supplied by the Linux/MIPS platform dependencies.
extern "C" {
    static PIC32_BASE_RESET: usize;
    fn ioremap(offset: usize, size: usize) -> *mut c_void;
    fn pic32_syskey_unlock();
    fn __raw_writel(value: u32, address: *mut c_void);
    fn __raw_readl(address: *mut c_void) -> u32;
    fn local_irq_disable();

    static mut _machine_restart: Option<unsafe extern "C" fn(command: *mut i8)>;
    static mut _machine_halt: Option<unsafe extern "C" fn()>;
    static mut pm_power_off: Option<unsafe extern "C" fn()>;
}

const PIC32_RSWRST: usize = 0x10;

unsafe fn pic32_halt() -> ! {
    loop {
        // The original assembly selects the r4000 architecture, executes wait,
        // and restores the assembler state.
        core::arch::asm!("wait", options(nomem, nostack, preserves_flags));
    }
}

unsafe extern "C" fn pic32_machine_restart(_command: *mut i8) {
    let reg = ioremap(PIC32_BASE_RESET + PIC32_RSWRST, core::mem::size_of::<u32>());

    pic32_syskey_unlock();

    /* magic write/read */
    __raw_writel(1, reg);
    let _ = __raw_readl(reg);

    pic32_halt();
}

unsafe extern "C" fn pic32_machine_halt() {
    local_irq_disable();

    pic32_halt();
}

unsafe extern "C" fn mips_reboot_setup() -> i32 {
    _machine_restart = Some(pic32_machine_restart);
    _machine_halt = Some(pic32_machine_halt);
    pm_power_off = Some(pic32_machine_halt);

    0
}

// Corresponds to the Linux arch_initcall(mips_reboot_setup) registration.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
