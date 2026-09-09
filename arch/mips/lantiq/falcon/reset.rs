// SPDX-License-Identifier: GPL-2.0-only
/*
 *
 * Copyright (C) 2012 Thomas Langer <thomas.langer@lantiq.com>
 * Copyright (C) 2012 John Crispin <john@phrozen.org>
 */

use core::ffi::c_char;

// Declarations supplied by the platform and kernel dependencies.
unsafe extern "C" {
    static mut _machine_restart: Option<unsafe extern "C" fn(*mut c_char)>;
    static mut _machine_halt: Option<unsafe extern "C" fn()>;
    static mut pm_power_off: Option<unsafe extern "C" fn()>;

    fn local_irq_disable();
    fn ltq_w32(value: u32, address: *mut core::ffi::c_void);
}

// Dummy implementation. Used to allow platform code to find out what
// source was booted from.
#[no_mangle]
pub extern "C" fn ltq_boot_select() -> u8 {
    BS_SPI
}

// KSEG1 is supplied by the platform headers.
const BOOT_REG_BASE: usize = KSEG1 | 0x1f200000;
const BOOT_PW1_REG: usize = BOOT_REG_BASE | 0x20;
const BOOT_PW2_REG: usize = BOOT_REG_BASE | 0x24;
const BOOT_PW1: u32 = 0x4c545100;
const BOOT_PW2: u32 = 0x0051544c;

const WDT_REG_BASE: usize = KSEG1 | 0x1f8803f0;
const WDT_PW1: u32 = 0x00be0000;
const WDT_PW2: u32 = 0x00dc0000;

unsafe fn machine_restart(_command: *mut c_char) {
    local_irq_disable();

    /* reboot magic */
    ltq_w32(BOOT_PW1, BOOT_PW1_REG as *mut core::ffi::c_void); /* 'LTQ\0' */
    ltq_w32(BOOT_PW2, BOOT_PW2_REG as *mut core::ffi::c_void); /* '\0QTL' */
    ltq_w32(0, BOOT_REG_BASE as *mut core::ffi::c_void); /* reset Bootreg RVEC */

    /* watchdog magic */
    ltq_w32(WDT_PW1, WDT_REG_BASE as *mut core::ffi::c_void);
    ltq_w32(
        WDT_PW2 |
            (0x3 << 26) | /* PWL */
            (0x2 << 24) | /* CLKDIV */
            (0x1 << 31) | /* enable */
            1, /* reload */
        WDT_REG_BASE as *mut core::ffi::c_void,
    );
    unreachable!();
}

unsafe fn machine_halt() {
    local_irq_disable();
    unreachable!();
}

unsafe fn machine_power_off() {
    local_irq_disable();
    unreachable!();
}

unsafe fn mips_reboot_setup() -> i32 {
    _machine_restart = Some(machine_restart);
    _machine_halt = Some(machine_halt);
    pm_power_off = Some(machine_power_off);
    0
}

// Equivalent of arch_initcall(mips_reboot_setup); registration is supplied
// by the kernel initialization framework.
const _: unsafe fn() -> i32 = mips_reboot_setup;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
