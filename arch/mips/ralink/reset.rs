// SPDX-License-Identifier: GPL-2.0-only
/*
 *
 * Copyright (C) 2008-2009 Gabor Juhos <juhosg@openwrt.org>
 * Copyright (C) 2008 Imre Kaloz <kaloz@openwrt.org>
 * Copyright (C) 2013 John Crispin <john@phrozen.org>
 */

// C dependencies supplied by the surrounding kernel sources.
unsafe extern "C" {
    fn rt_sysc_m32(mask: u32, value: u32, reg: u32);
    fn rt_sysc_w32(value: u32, reg: u32);
    fn mdelay(ms: u32);
    fn local_irq_disable();
    static mut _machine_restart: Option<unsafe extern "C" fn(*mut core::ffi::c_char)>;
}

/* Reset Control */
const SYSC_REG_RESET_CTRL: u32 = 0x034;

const RSTCTL_RESET_PCI: u32 = 1u32 << 26;
const RSTCTL_RESET_SYSTEM: u32 = 1u32 << 0;

unsafe extern "C" fn ralink_restart(_command: *mut core::ffi::c_char) {
    // Preserved from IS_ENABLED(CONFIG_PCI); the build configuration supplies this condition.
    #[cfg(feature = "CONFIG_PCI")]
    {
        rt_sysc_m32(0, RSTCTL_RESET_PCI, SYSC_REG_RESET_CTRL);
        mdelay(50);
    }

    local_irq_disable();
    rt_sysc_w32(RSTCTL_RESET_SYSTEM, SYSC_REG_RESET_CTRL);
    core::hint::unreachable_unchecked();
}

unsafe extern "C" fn mips_reboot_setup() -> i32 {
    _machine_restart = Some(ralink_restart);

    0
}

// C registration equivalent: arch_initcall(mips_reboot_setup);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
