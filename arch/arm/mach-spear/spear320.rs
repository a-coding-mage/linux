// SPDX-License-Identifier: GPL-2.0-only
/*
 * arch/arm/mach-spear3xx/spear320.c
 *
 * SPEAr320 machine source file
 *
 * Copyright (C) 2009-2012 ST Microelectronics
 * Viresh Kumar <vireshk@kernel.org>
 */

// Kernel headers and symbols referenced below are supplied by the surrounding
// kernel translation units.

use core::ffi::{c_char, c_void};

const SPEAR320_UART1_BASE: usize = 0xA3000000;
const SPEAR320_UART2_BASE: usize = 0xA4000000;
const SPEAR320_SSP0_BASE: usize = 0xA5000000;
const SPEAR320_SSP1_BASE: usize = 0xA6000000;

#[repr(C)]
pub struct pl08x_channel_data {
    pub bus_id: *const c_char,
    pub min_signal: i32,
    pub max_signal: i32,
    pub muxval: i32,
    pub periph_buses: u32,
}

#[repr(C)]
pub struct pl022_ssp_controller {
    pub bus_id: i32,
    pub enable_dma: i32,
    pub dma_filter: Option<unsafe extern "C" fn(*mut c_void, *mut c_void) -> bool>,
    pub dma_tx_param: *const c_char,
    pub dma_rx_param: *const c_char,
}

#[repr(C)]
pub struct amba_pl011_data {
    pub dma_filter: Option<unsafe extern "C" fn(*mut c_void, *mut c_void) -> bool>,
    pub dma_tx_param: *const c_char,
    pub dma_rx_param: *const c_char,
}

#[repr(C)]
pub struct of_dev_auxdata {
    pub compatible: *const c_char,
    pub phys_addr: usize,
    pub name: *const c_char,
    pub platform_data: *mut c_void,
}

#[repr(C)]
pub struct map_desc {
    pub virtual_addr: usize,
    pub pfn: usize,
    pub length: usize,
    pub type_: usize,
}

unsafe extern "C" {
    static mut pl080_plat_data: c_void;
    static mut pl022_plat_data: c_void;
    unsafe fn pl08x_filter_id(*mut c_void, *mut c_void) -> bool;
    unsafe fn of_platform_default_populate(*mut c_void, *mut of_dev_auxdata, *mut c_void);
    unsafe fn iotable_init(*mut map_desc, usize);
    unsafe fn spear3xx_map_io();
    unsafe fn spear3xx_timer_init();
    unsafe fn spear_restart(*mut c_void);
}

const AHB1: u32 = 1;
const AHB2: u32 = 2;

macro_rules! cstr { ($s:literal) => { concat!($s, "\0").as_ptr() as *const c_char }; }

#[no_mangle]
pub static mut spear320_dma_info: [pl08x_channel_data; 26] = [
    pl08x_channel_data { bus_id: cstr!("uart0_rx"), min_signal: 2, max_signal: 2, muxval: 0, periph_buses: AHB1 },
    pl08x_channel_data { bus_id: cstr!("uart0_tx"), min_signal: 3, max_signal: 3, muxval: 0, periph_buses: AHB1 },
    pl08x_channel_data { bus_id: cstr!("ssp0_rx"), min_signal: 8, max_signal: 8, muxval: 0, periph_buses: AHB1 },
    pl08x_channel_data { bus_id: cstr!("ssp0_tx"), min_signal: 9, max_signal: 9, muxval: 0, periph_buses: AHB1 },
    pl08x_channel_data { bus_id: cstr!("i2c0_rx"), min_signal: 10, max_signal: 10, muxval: 0, periph_buses: AHB1 },
    pl08x_channel_data { bus_id: cstr!("i2c0_tx"), min_signal: 11, max_signal: 11, muxval: 0, periph_buses: AHB1 },
    pl08x_channel_data { bus_id: cstr!("irda"), min_signal: 12, max_signal: 12, muxval: 0, periph_buses: AHB1 },
    pl08x_channel_data { bus_id: cstr!("adc"), min_signal: 13, max_signal: 13, muxval: 0, periph_buses: AHB1 },
    pl08x_channel_data { bus_id: cstr!("to_jpeg"), min_signal: 14, max_signal: 14, muxval: 0, periph_buses: AHB1 },
    pl08x_channel_data { bus_id: cstr!("from_jpeg"), min_signal: 15, max_signal: 15, muxval: 0, periph_buses: AHB1 },
    pl08x_channel_data { bus_id: cstr!("ssp1_rx"), min_signal: 0, max_signal: 0, muxval: 1, periph_buses: AHB2 },
    pl08x_channel_data { bus_id: cstr!("ssp1_tx"), min_signal: 1, max_signal: 1, muxval: 1, periph_buses: AHB2 },
    pl08x_channel_data { bus_id: cstr!("ssp2_rx"), min_signal: 2, max_signal: 2, muxval: 1, periph_buses: AHB2 },
    pl08x_channel_data { bus_id: cstr!("ssp2_tx"), min_signal: 3, max_signal: 3, muxval: 1, periph_buses: AHB2 },
    pl08x_channel_data { bus_id: cstr!("uart1_rx"), min_signal: 4, max_signal: 4, muxval: 1, periph_buses: AHB2 },
    pl08x_channel_data { bus_id: cstr!("uart1_tx"), min_signal: 5, max_signal: 5, muxval: 1, periph_buses: AHB2 },
    pl08x_channel_data { bus_id: cstr!("uart2_rx"), min_signal: 6, max_signal: 6, muxval: 1, periph_buses: AHB2 },
    pl08x_channel_data { bus_id: cstr!("uart2_tx"), min_signal: 7, max_signal: 7, muxval: 1, periph_buses: AHB2 },
    pl08x_channel_data { bus_id: cstr!("i2c1_rx"), min_signal: 8, max_signal: 8, muxval: 1, periph_buses: AHB2 },
    pl08x_channel_data { bus_id: cstr!("i2c1_tx"), min_signal: 9, max_signal: 9, muxval: 1, periph_buses: AHB2 },
    pl08x_channel_data { bus_id: cstr!("i2c2_rx"), min_signal: 10, max_signal: 10, muxval: 1, periph_buses: AHB2 },
    pl08x_channel_data { bus_id: cstr!("i2c2_tx"), min_signal: 11, max_signal: 11, muxval: 1, periph_buses: AHB2 },
    pl08x_channel_data { bus_id: cstr!("i2s_rx"), min_signal: 12, max_signal: 12, muxval: 1, periph_buses: AHB2 },
    pl08x_channel_data { bus_id: cstr!("i2s_tx"), min_signal: 13, max_signal: 13, muxval: 1, periph_buses: AHB2 },
    pl08x_channel_data { bus_id: cstr!("rs485_rx"), min_signal: 14, max_signal: 14, muxval: 1, periph_buses: AHB2 },
    pl08x_channel_data { bus_id: cstr!("rs485_tx"), min_signal: 15, max_signal: 15, muxval: 1, periph_buses: AHB2 },
];

static mut spear320_ssp_data: [pl022_ssp_controller; 2] = [
    pl022_ssp_controller { bus_id: 1, enable_dma: 1, dma_filter: Some(pl08x_filter_id), dma_tx_param: cstr!("ssp1_tx"), dma_rx_param: cstr!("ssp1_rx") },
    pl022_ssp_controller { bus_id: 2, enable_dma: 1, dma_filter: Some(pl08x_filter_id), dma_tx_param: cstr!("ssp2_tx"), dma_rx_param: cstr!("ssp2_rx") },
];

static mut spear320_uart_data: [amba_pl011_data; 2] = [
    amba_pl011_data { dma_filter: Some(pl08x_filter_id), dma_tx_param: cstr!("uart1_tx"), dma_rx_param: cstr!("uart1_rx") },
    amba_pl011_data { dma_filter: Some(pl08x_filter_id), dma_tx_param: cstr!("uart2_tx"), dma_rx_param: cstr!("uart2_rx") },
];

static mut spear320_auxdata_lookup: [of_dev_auxdata; 7] = [
    of_dev_auxdata { compatible: cstr!("arm,pl022"), phys_addr: 0, name: core::ptr::null(), platform_data: unsafe { &mut pl022_plat_data as *mut _ as *mut c_void } },
    of_dev_auxdata { compatible: cstr!("arm,pl080"), phys_addr: 0, name: core::ptr::null(), platform_data: unsafe { &mut pl080_plat_data as *mut _ as *mut c_void } },
    of_dev_auxdata { compatible: cstr!("arm,pl022"), phys_addr: SPEAR320_SSP0_BASE, name: core::ptr::null(), platform_data: unsafe { &mut spear320_ssp_data[0] as *mut _ as *mut c_void } },
    of_dev_auxdata { compatible: cstr!("arm,pl022"), phys_addr: SPEAR320_SSP1_BASE, name: core::ptr::null(), platform_data: unsafe { &mut spear320_ssp_data[1] as *mut _ as *mut c_void } },
    of_dev_auxdata { compatible: cstr!("arm,pl011"), phys_addr: SPEAR320_UART1_BASE, name: core::ptr::null(), platform_data: unsafe { &mut spear320_uart_data[0] as *mut _ as *mut c_void } },
    of_dev_auxdata { compatible: cstr!("arm,pl011"), phys_addr: SPEAR320_UART2_BASE, name: core::ptr::null(), platform_data: unsafe { &mut spear320_uart_data[1] as *mut _ as *mut c_void } },
    of_dev_auxdata { compatible: core::ptr::null(), phys_addr: 0, name: core::ptr::null(), platform_data: core::ptr::null_mut() },
];

unsafe fn spear320_dt_init() {
    // pl080_plat_data.slave_channels = spear320_dma_info;
    // pl080_plat_data.num_slave_channels = ARRAY_SIZE(spear320_dma_info);
    of_platform_default_populate(core::ptr::null_mut(), spear320_auxdata_lookup.as_mut_ptr(), core::ptr::null_mut());
}

static spear320_dt_board_compat: [*const c_char; 4] = [cstr!("st,spear320"), cstr!("st,spear320-evb"), cstr!("st,spear320-hmi"), core::ptr::null()];

static mut spear320_io_desc: [map_desc; 1] = [map_desc { virtual_addr: 0, pfn: 0, length: 16 * 1024 * 1024, type_: 0 }];

unsafe fn spear320_map_io() {
    iotable_init(spear320_io_desc.as_mut_ptr(), 1);
    spear3xx_map_io();
}

// DT_MACHINE_START(SPEAR320_DT, "ST SPEAr320 SoC with Flattened Device Tree")
// .map_io = spear320_map_io, .init_time = spear3xx_timer_init,
// .init_machine = spear320_dt_init, .restart = spear_restart,
// .dt_compat = spear320_dt_board_compat, MACHINE_END

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
