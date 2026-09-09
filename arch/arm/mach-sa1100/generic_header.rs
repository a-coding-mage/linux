/* SPDX-License-Identifier: GPL-2.0 */
/*
 * linux/arch/arm/mach-sa1100/generic.h
 *
 * Author: Nicolas Pitre
 */

// Dependencies supplied by the surrounding kernel translation.

extern "C" {
    pub fn sa1100_timer_init();
    // C __init attribute.
    pub fn sa1100_map_io();
    // C __init attribute.
    pub fn sa1100_init_irq();
    // C __init attribute.
    pub fn sa1100_init_gpio();
    pub fn sa11x0_restart(mode: reboot_mode, cmd: *const core::ffi::c_char);
    pub fn sa11x0_init_late();
}

// #define SET_BANK(__nr,__start,__size) \
//     mi->bank[__nr].start = (__start), \
//     mi->bank[__nr].size = (__size)
#[macro_export]
macro_rules! SET_BANK {
    ($mi:expr, $nr:expr, $start:expr, $size:expr) => {{
        $mi.bank[$nr].start = $start;
        $mi.bank[$nr].size = $size;
    }};
}

extern "C" {
    pub fn sa1110_mb_enable();
    pub fn sa1110_mb_disable();

    pub static mut sa11x0_freq_table: [cpufreq_frequency_table];
    pub fn sa11x0_getspeed(cpu: core::ffi::c_uint) -> core::ffi::c_uint;
}

pub struct flash_platform_data;
pub struct resource;

extern "C" {
    pub fn sa11x0_register_mtd(
        flash: *mut flash_platform_data,
        res: *mut resource,
        nr: core::ffi::c_int,
    );
}

pub struct mcp_plat_data;
extern "C" {
    pub fn sa11x0_ppc_configure_mcp();
    pub fn sa11x0_register_mcp(data: *mut mcp_plat_data);
}

pub struct sa1100fb_mach_info;
extern "C" {
    pub fn sa11x0_register_lcd(inf: *mut sa1100fb_mach_info);
}

#[cfg(CONFIG_PM)]
extern "C" {
    pub fn sa11x0_pm_init() -> core::ffi::c_int;
}

#[cfg(not(CONFIG_PM))]
#[inline]
pub fn sa11x0_pm_init() -> core::ffi::c_int {
    0
}

extern "C" {
    pub fn sa11xx_clk_init() -> core::ffi::c_int;
}

pub struct gpiod_lookup_table;
extern "C" {
    pub fn sa11x0_register_pcmcia(socket: core::ffi::c_int, table: *mut gpiod_lookup_table);
}

pub struct software_node;
extern "C" {
    pub static sa1100_gpiochip_node: software_node;
}

pub struct fixed_voltage_config;
pub struct regulator_consumer_supply;
extern "C" {
    pub fn sa11x0_register_fixed_regulator(
        n: core::ffi::c_int,
        cfg: *mut fixed_voltage_config,
        supplies: *mut regulator_consumer_supply,
        num_supplies: core::ffi::c_uint,
        uses_gpio: bool,
    ) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
