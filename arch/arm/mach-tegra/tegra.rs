// SPDX-License-Identifier: GPL-2.0-only
/*
 * NVIDIA Tegra SoC device tree board support
 *
 * Copyright (C) 2011, 2013, NVIDIA Corporation
 * Copyright (C) 2010 Secret Lab Technologies, Ltd.
 * Copyright (C) 2010 Google, Inc.
 */

use core::ffi::{c_char, c_int, c_void};

// Declarations supplied by the surrounding kernel translation unit.
extern "C" {
    fn of_register_trusted_foundations();
    fn tegra_cpu_reset_handler_init();
    fn call_firmware_op(op: Option<unsafe extern "C" fn()>);
    fn l2x0_init();
    fn tegra_init_irq();
    fn irqchip_init();
    fn tegra_soc_device_register() -> *mut c_void;
    fn of_platform_default_populate(
        root: *const c_void,
        matches: *const c_void,
        parent: *mut c_void,
    );
    fn of_machine_is_compatible(compatible: *const c_char) -> bool;
    fn platform_device_register_simple(
        name: *const c_char,
        id: c_int,
        res: *const c_void,
        num: u32,
    );
    fn psci_smp_available() -> bool;
    fn tegra_map_common_io();
    static tegra_smp_ops: c_void;
}

/*
 * Storage for debug-macro.S's state.
 *
 * This must be in .data not .bss so that it gets initialized each time the
 * kernel is loaded. The data is declared here rather than debug-macro.S so
 * that multiple inclusions of debug-macro.S point at the same data.
 */
#[no_mangle]
pub static mut tegra_uart_config: [u32; 3] = [
    /* Debug UART initialization required */
    1,
    /* Debug UART physical address */
    0,
    /* Debug UART virtual address */
    0,
];

unsafe extern "C" fn tegra_init_early() {
    of_register_trusted_foundations();
    tegra_cpu_reset_handler_init();
    call_firmware_op(Some(l2x0_init));
}

unsafe extern "C" fn tegra_dt_init_irq() {
    tegra_init_irq();
    irqchip_init();
}

unsafe extern "C" fn tegra_dt_init() {
    let parent = tegra_soc_device_register();
    of_platform_default_populate(core::ptr::null(), core::ptr::null(), parent);
}

unsafe extern "C" fn tegra_dt_init_late() {
    // IS_ENABLED(CONFIG_ARCH_TEGRA_2x_SOC) is a build-time configuration condition.
    if cfg!(feature = "CONFIG_ARCH_TEGRA_2x_SOC")
        && of_machine_is_compatible(b"nvidia,tegra20\0".as_ptr() as *const c_char)
    {
        platform_device_register_simple(b"tegra20-cpufreq\0".as_ptr() as *const c_char, -1, core::ptr::null(), 0);
    }

    // IS_ENABLED(CONFIG_ARM_TEGRA_CPUIDLE) is a build-time configuration condition.
    if cfg!(feature = "CONFIG_ARM_TEGRA_CPUIDLE") && !psci_smp_available() {
        platform_device_register_simple(b"tegra-cpuidle\0".as_ptr() as *const c_char, -1, core::ptr::null(), 0);
    }

    // IS_ENABLED(CONFIG_ARCH_TEGRA_3x_SOC) is a build-time configuration condition.
    if cfg!(feature = "CONFIG_ARCH_TEGRA_3x_SOC")
        && of_machine_is_compatible(b"nvidia,tegra30\0".as_ptr() as *const c_char)
    {
        platform_device_register_simple(b"tegra20-cpufreq\0".as_ptr() as *const c_char, -1, core::ptr::null(), 0);
    }
}

static tegra_dt_board_compat: [*const c_char; 5] = [
    b"nvidia,tegra124\0".as_ptr() as *const c_char,
    b"nvidia,tegra114\0".as_ptr() as *const c_char,
    b"nvidia,tegra30\0".as_ptr() as *const c_char,
    b"nvidia,tegra20\0".as_ptr() as *const c_char,
    core::ptr::null(),
];

// DT_MACHINE_START(TEGRA_DT, "NVIDIA Tegra SoC (Flattened Device Tree)")
// .l2c_aux_val = 0x3c400000, .l2c_aux_mask = 0xc20fc3ff,
// .smp = smp_ops(tegra_smp_ops), .map_io = tegra_map_common_io,
// .init_early = tegra_init_early, .init_irq = tegra_dt_init_irq,
// .init_machine = tegra_dt_init, .init_late = tegra_dt_init_late,
// .dt_compat = tegra_dt_board_compat, MACHINE_END

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
