// SPDX-License-Identifier: GPL-2.0-only
/*
 * arch/arm/mach-spear3xx/spear300.c
 *
 * SPEAr300 machine source file
 *
 * Copyright (C) 2009-2012 ST Microelectronics
 * Viresh Kumar <vireshk@kernel.org>
 */

// #define pr_fmt(fmt) "SPEAr300: " fmt
// External declarations are supplied by the corresponding kernel dependencies.

/* DMAC platform data's slave info */
#[allow(non_upper_case_globals)]
pub static mut spear300_dma_info: [pl08x_channel_data; 26] = [
    pl08x_channel_data { bus_id: b"uart0_rx\0".as_ptr() as *const i8, min_signal: 2, max_signal: 2, muxval: 0, periph_buses: PL08X_AHB1 },
    pl08x_channel_data { bus_id: b"uart0_tx\0".as_ptr() as *const i8, min_signal: 3, max_signal: 3, muxval: 0, periph_buses: PL08X_AHB1 },
    pl08x_channel_data { bus_id: b"ssp0_rx\0".as_ptr() as *const i8, min_signal: 8, max_signal: 8, muxval: 0, periph_buses: PL08X_AHB1 },
    pl08x_channel_data { bus_id: b"ssp0_tx\0".as_ptr() as *const i8, min_signal: 9, max_signal: 9, muxval: 0, periph_buses: PL08X_AHB1 },
    pl08x_channel_data { bus_id: b"i2c_rx\0".as_ptr() as *const i8, min_signal: 10, max_signal: 10, muxval: 0, periph_buses: PL08X_AHB1 },
    pl08x_channel_data { bus_id: b"i2c_tx\0".as_ptr() as *const i8, min_signal: 11, max_signal: 11, muxval: 0, periph_buses: PL08X_AHB1 },
    pl08x_channel_data { bus_id: b"irda\0".as_ptr() as *const i8, min_signal: 12, max_signal: 12, muxval: 0, periph_buses: PL08X_AHB1 },
    pl08x_channel_data { bus_id: b"adc\0".as_ptr() as *const i8, min_signal: 13, max_signal: 13, muxval: 0, periph_buses: PL08X_AHB1 },
    pl08x_channel_data { bus_id: b"to_jpeg\0".as_ptr() as *const i8, min_signal: 14, max_signal: 14, muxval: 0, periph_buses: PL08X_AHB1 },
    pl08x_channel_data { bus_id: b"from_jpeg\0".as_ptr() as *const i8, min_signal: 15, max_signal: 15, muxval: 0, periph_buses: PL08X_AHB1 },
    pl08x_channel_data { bus_id: b"ras0_rx\0".as_ptr() as *const i8, min_signal: 0, max_signal: 0, muxval: 1, periph_buses: PL08X_AHB1 },
    pl08x_channel_data { bus_id: b"ras0_tx\0".as_ptr() as *const i8, min_signal: 1, max_signal: 1, muxval: 1, periph_buses: PL08X_AHB1 },
    pl08x_channel_data { bus_id: b"ras1_rx\0".as_ptr() as *const i8, min_signal: 2, max_signal: 2, muxval: 1, periph_buses: PL08X_AHB1 },
    pl08x_channel_data { bus_id: b"ras1_tx\0".as_ptr() as *const i8, min_signal: 3, max_signal: 3, muxval: 1, periph_buses: PL08X_AHB1 },
    pl08x_channel_data { bus_id: b"ras2_rx\0".as_ptr() as *const i8, min_signal: 4, max_signal: 4, muxval: 1, periph_buses: PL08X_AHB1 },
    pl08x_channel_data { bus_id: b"ras2_tx\0".as_ptr() as *const i8, min_signal: 5, max_signal: 5, muxval: 1, periph_buses: PL08X_AHB1 },
    pl08x_channel_data { bus_id: b"ras3_rx\0".as_ptr() as *const i8, min_signal: 6, max_signal: 6, muxval: 1, periph_buses: PL08X_AHB1 },
    pl08x_channel_data { bus_id: b"ras3_tx\0".as_ptr() as *const i8, min_signal: 7, max_signal: 7, muxval: 1, periph_buses: PL08X_AHB1 },
    pl08x_channel_data { bus_id: b"ras4_rx\0".as_ptr() as *const i8, min_signal: 8, max_signal: 8, muxval: 1, periph_buses: PL08X_AHB1 },
    pl08x_channel_data { bus_id: b"ras4_tx\0".as_ptr() as *const i8, min_signal: 9, max_signal: 9, muxval: 1, periph_buses: PL08X_AHB1 },
    pl08x_channel_data { bus_id: b"ras5_rx\0".as_ptr() as *const i8, min_signal: 10, max_signal: 10, muxval: 1, periph_buses: PL08X_AHB1 },
    pl08x_channel_data { bus_id: b"ras5_tx\0".as_ptr() as *const i8, min_signal: 11, max_signal: 11, muxval: 1, periph_buses: PL08X_AHB1 },
    pl08x_channel_data { bus_id: b"ras6_rx\0".as_ptr() as *const i8, min_signal: 12, max_signal: 12, muxval: 1, periph_buses: PL08X_AHB1 },
    pl08x_channel_data { bus_id: b"ras6_tx\0".as_ptr() as *const i8, min_signal: 13, max_signal: 13, muxval: 1, periph_buses: PL08X_AHB1 },
    pl08x_channel_data { bus_id: b"ras7_rx\0".as_ptr() as *const i8, min_signal: 14, max_signal: 14, muxval: 1, periph_buses: PL08X_AHB1 },
    pl08x_channel_data { bus_id: b"ras7_tx\0".as_ptr() as *const i8, min_signal: 15, max_signal: 15, muxval: 1, periph_buses: PL08X_AHB1 },
];

/* Add SPEAr300 auxdata to pass platform data */
// static struct of_dev_auxdata spear300_auxdata_lookup[] __initdata = {
//     OF_DEV_AUXDATA("arm,pl022", SPEAR3XX_ICM1_SSP_BASE, NULL, &pl022_plat_data),
//     OF_DEV_AUXDATA("arm,pl080", SPEAR_ICM3_DMA_BASE, NULL, &pl080_plat_data),
//     {},
// };

unsafe fn spear300_dt_init() {
    pl080_plat_data.slave_channels = spear300_dma_info.as_mut_ptr();
    pl080_plat_data.num_slave_channels = spear300_dma_info.len();
    of_platform_default_populate(core::ptr::null(), spear300_auxdata_lookup, core::ptr::null_mut());
}

static spear300_dt_board_compat: [*const i8; 3] = [
    b"st,spear300\0".as_ptr() as *const i8,
    b"st,spear300-evb\0".as_ptr() as *const i8,
    core::ptr::null(),
];

unsafe fn spear300_map_io() {
    spear3xx_map_io();
}

// DT_MACHINE_START(SPEAR300_DT, "ST SPEAr300 SoC with Flattened Device Tree")
//     .map_io = spear300_map_io,
//     .init_time = spear3xx_timer_init,
//     .init_machine = spear300_dt_init,
//     .restart = spear_restart,
//     .dt_compat = spear300_dt_board_compat,
// MACHINE_END

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
