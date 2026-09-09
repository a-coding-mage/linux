// SPDX-License-Identifier: GPL-2.0-only
/*
 * arch/arm/mach-spear3xx/spear310.c
 *
 * SPEAr310 machine source file
 *
 * Copyright (C) 2009-2012 ST Microelectronics
 * Viresh Kumar <vireshk@kernel.org>
 */

// C headers and symbols supplied by the surrounding kernel translation unit.

const SPEAR310_UART1_BASE: usize = 0xB2000000;
const SPEAR310_UART2_BASE: usize = 0xB2080000;
const SPEAR310_UART3_BASE: usize = 0xB2100000;
const SPEAR310_UART4_BASE: usize = 0xB2180000;
const SPEAR310_UART5_BASE: usize = 0xB2200000;

/* DMAC platform data's slave info */
pub static mut spear310_dma_info: [pl08x_channel_data; 26] = [
    pl08x_channel_data { bus_id: b"uart0_rx\0".as_ptr(), min_signal: 2, max_signal: 2, muxval: 0, periph_buses: PL08X_AHB1 },
    pl08x_channel_data { bus_id: b"uart0_tx\0".as_ptr(), min_signal: 3, max_signal: 3, muxval: 0, periph_buses: PL08X_AHB1 },
    pl08x_channel_data { bus_id: b"ssp0_rx\0".as_ptr(), min_signal: 8, max_signal: 8, muxval: 0, periph_buses: PL08X_AHB1 },
    pl08x_channel_data { bus_id: b"ssp0_tx\0".as_ptr(), min_signal: 9, max_signal: 9, muxval: 0, periph_buses: PL08X_AHB1 },
    pl08x_channel_data { bus_id: b"i2c_rx\0".as_ptr(), min_signal: 10, max_signal: 10, muxval: 0, periph_buses: PL08X_AHB1 },
    pl08x_channel_data { bus_id: b"i2c_tx\0".as_ptr(), min_signal: 11, max_signal: 11, muxval: 0, periph_buses: PL08X_AHB1 },
    pl08x_channel_data { bus_id: b"irda\0".as_ptr(), min_signal: 12, max_signal: 12, muxval: 0, periph_buses: PL08X_AHB1 },
    pl08x_channel_data { bus_id: b"adc\0".as_ptr(), min_signal: 13, max_signal: 13, muxval: 0, periph_buses: PL08X_AHB1 },
    pl08x_channel_data { bus_id: b"to_jpeg\0".as_ptr(), min_signal: 14, max_signal: 14, muxval: 0, periph_buses: PL08X_AHB1 },
    pl08x_channel_data { bus_id: b"from_jpeg\0".as_ptr(), min_signal: 15, max_signal: 15, muxval: 0, periph_buses: PL08X_AHB1 },
    pl08x_channel_data { bus_id: b"uart1_rx\0".as_ptr(), min_signal: 0, max_signal: 0, muxval: 1, periph_buses: PL08X_AHB1 },
    pl08x_channel_data { bus_id: b"uart1_tx\0".as_ptr(), min_signal: 1, max_signal: 1, muxval: 1, periph_buses: PL08X_AHB1 },
    pl08x_channel_data { bus_id: b"uart2_rx\0".as_ptr(), min_signal: 2, max_signal: 2, muxval: 1, periph_buses: PL08X_AHB1 },
    pl08x_channel_data { bus_id: b"uart2_tx\0".as_ptr(), min_signal: 3, max_signal: 3, muxval: 1, periph_buses: PL08X_AHB1 },
    pl08x_channel_data { bus_id: b"uart3_rx\0".as_ptr(), min_signal: 4, max_signal: 4, muxval: 1, periph_buses: PL08X_AHB1 },
    pl08x_channel_data { bus_id: b"uart3_tx\0".as_ptr(), min_signal: 5, max_signal: 5, muxval: 1, periph_buses: PL08X_AHB1 },
    pl08x_channel_data { bus_id: b"uart4_rx\0".as_ptr(), min_signal: 6, max_signal: 6, muxval: 1, periph_buses: PL08X_AHB1 },
    pl08x_channel_data { bus_id: b"uart4_tx\0".as_ptr(), min_signal: 7, max_signal: 7, muxval: 1, periph_buses: PL08X_AHB1 },
    pl08x_channel_data { bus_id: b"uart5_rx\0".as_ptr(), min_signal: 8, max_signal: 8, muxval: 1, periph_buses: PL08X_AHB1 },
    pl08x_channel_data { bus_id: b"uart5_tx\0".as_ptr(), min_signal: 9, max_signal: 9, muxval: 1, periph_buses: PL08X_AHB1 },
    pl08x_channel_data { bus_id: b"ras5_rx\0".as_ptr(), min_signal: 10, max_signal: 10, muxval: 1, periph_buses: PL08X_AHB1 },
    pl08x_channel_data { bus_id: b"ras5_tx\0".as_ptr(), min_signal: 11, max_signal: 11, muxval: 1, periph_buses: PL08X_AHB1 },
    pl08x_channel_data { bus_id: b"ras6_rx\0".as_ptr(), min_signal: 12, max_signal: 12, muxval: 1, periph_buses: PL08X_AHB1 },
    pl08x_channel_data { bus_id: b"ras6_tx\0".as_ptr(), min_signal: 13, max_signal: 13, muxval: 1, periph_buses: PL08X_AHB1 },
    pl08x_channel_data { bus_id: b"ras7_rx\0".as_ptr(), min_signal: 14, max_signal: 14, muxval: 1, periph_buses: PL08X_AHB1 },
    pl08x_channel_data { bus_id: b"ras7_tx\0".as_ptr(), min_signal: 15, max_signal: 15, muxval: 1, periph_buses: PL08X_AHB1 },
];

/* uart devices plat data */
static mut spear310_uart_data: [amba_pl011_data; 5] = [
    amba_pl011_data { dma_filter: pl08x_filter_id, dma_tx_param: b"uart1_tx\0".as_ptr(), dma_rx_param: b"uart1_rx\0".as_ptr() },
    amba_pl011_data { dma_filter: pl08x_filter_id, dma_tx_param: b"uart2_tx\0".as_ptr(), dma_rx_param: b"uart2_rx\0".as_ptr() },
    amba_pl011_data { dma_filter: pl08x_filter_id, dma_tx_param: b"uart3_tx\0".as_ptr(), dma_rx_param: b"uart3_rx\0".as_ptr() },
    amba_pl011_data { dma_filter: pl08x_filter_id, dma_tx_param: b"uart4_tx\0".as_ptr(), dma_rx_param: b"uart4_rx\0".as_ptr() },
    amba_pl011_data { dma_filter: pl08x_filter_id, dma_tx_param: b"uart5_tx\0".as_ptr(), dma_rx_param: b"uart5_rx\0".as_ptr() },
];

/* Add SPEAr310 auxdata to pass platform data */
static mut spear310_auxdata_lookup: [of_dev_auxdata; 8] = [
    OF_DEV_AUXDATA!(b"arm,pl022\0", SPEAR3XX_ICM1_SSP_BASE, core::ptr::null(), &pl022_plat_data),
    OF_DEV_AUXDATA!(b"arm,pl080\0", SPEAR_ICM3_DMA_BASE, core::ptr::null(), &pl080_plat_data),
    OF_DEV_AUXDATA!(b"arm,pl011\0", SPEAR310_UART1_BASE, core::ptr::null(), &spear310_uart_data[0]),
    OF_DEV_AUXDATA!(b"arm,pl011\0", SPEAR310_UART2_BASE, core::ptr::null(), &spear310_uart_data[1]),
    OF_DEV_AUXDATA!(b"arm,pl011\0", SPEAR310_UART3_BASE, core::ptr::null(), &spear310_uart_data[2]),
    OF_DEV_AUXDATA!(b"arm,pl011\0", SPEAR310_UART4_BASE, core::ptr::null(), &spear310_uart_data[3]),
    OF_DEV_AUXDATA!(b"arm,pl011\0", SPEAR310_UART5_BASE, core::ptr::null(), &spear310_uart_data[4]),
    of_dev_auxdata::default(),
];

unsafe fn spear310_dt_init() {
    pl080_plat_data.slave_channels = spear310_dma_info.as_mut_ptr();
    pl080_plat_data.num_slave_channels = spear310_dma_info.len();
    of_platform_default_populate(core::ptr::null(), spear310_auxdata_lookup.as_ptr(), core::ptr::null());
}

static spear310_dt_board_compat: [*const u8; 3] = [b"st,spear310\0".as_ptr(), b"st,spear310-evb\0".as_ptr(), core::ptr::null()];

unsafe fn spear310_map_io() {
    spear3xx_map_io();
}

// DT_MACHINE_START(SPEAR310_DT, "ST SPEAr310 SoC with Flattened Device Tree")
// .map_io = spear310_map_io, .init_time = spear3xx_timer_init,
// .init_machine = spear310_dt_init, .restart = spear_restart,
// .dt_compat = spear310_dt_board_compat, MACHINE_END

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
