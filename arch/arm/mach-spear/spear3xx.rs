// SPDX-License-Identifier: GPL-2.0-only
/*
 * arch/arm/mach-spear3xx/spear3xx.c
 *
 * SPEAr3XX machines common source file
 *
 * Copyright (C) 2009-2012 ST Microelectronics
 * Viresh Kumar <vireshk@kernel.org>
 */

// Dependencies supplied by the surrounding kernel translation unit:
// linux/amba/pl022.h, linux/amba/pl080.h, linux/clk.h,
// linux/clk/spear.h, linux/io.h, asm/mach/map.h, pl080.h, generic.h,
// spear.h, and misc_regs.h.

#[repr(C)]
pub struct Pl022SspController {
    pub bus_id: i32,
    pub enable_dma: i32,
    pub dma_filter: Option<unsafe extern "C" fn() -> bool>,
    pub dma_tx_param: *const core::ffi::c_char,
    pub dma_rx_param: *const core::ffi::c_char,
}

#[repr(C)]
pub struct Pl08xPlatformData {
    pub memcpy_burst_size: u32,
    pub memcpy_bus_width: u32,
    pub memcpy_prot_buff: bool,
    pub memcpy_prot_cache: bool,
    pub lli_buses: u32,
    pub mem_buses: u32,
    pub get_xfer_signal: Option<unsafe extern "C" fn() -> i32>,
    pub put_xfer_signal: Option<unsafe extern "C" fn(i32)>,
}

#[repr(C)]
pub struct MapDesc {
    pub virtual_: usize,
    pub pfn: usize,
    pub length: usize,
    pub type_: u32,
}

pub enum Clk {}

extern "C" {
    pub fn pl08x_filter_id() -> bool;
    pub fn pl080_get_signal() -> i32;
    pub fn pl080_put_signal(signal: i32);
    pub fn __phys_to_pfn(phys: usize) -> usize;
    pub fn iotable_init(desc: *mut MapDesc, size: usize);
    pub fn spear3xx_clk_init(misc_base: usize, soc_config_base: usize);
    pub fn clk_get_sys(dev_id: *const core::ffi::c_char, con_id: *const core::ffi::c_char) -> *mut Clk;
    pub fn clk_get(dev_id: *const core::ffi::c_char, con_id: *const core::ffi::c_char) -> *mut Clk;
    pub fn IS_ERR(ptr: *mut Clk) -> bool;
    pub fn clk_set_parent(clk: *mut Clk, parent: *mut Clk) -> i32;
    pub fn clk_put(clk: *mut Clk);
    pub fn spear_setup_of_timer();
    pub fn bug();
    pub fn pr_err(fmt: *const core::ffi::c_char, ...);
}

// ssp device registration
#[no_mangle]
pub static mut pl022_plat_data: Pl022SspController = Pl022SspController {
    bus_id: 0,
    enable_dma: 1,
    dma_filter: Some(pl08x_filter_id),
    dma_tx_param: b"ssp0_tx\0".as_ptr() as *const core::ffi::c_char,
    dma_rx_param: b"ssp0_rx\0".as_ptr() as *const core::ffi::c_char,
};

// dmac device registration
#[no_mangle]
pub static mut pl080_plat_data: Pl08xPlatformData = Pl08xPlatformData {
    memcpy_burst_size: PL08X_BURST_SZ_16,
    memcpy_bus_width: PL08X_BUS_WIDTH_32_BITS,
    memcpy_prot_buff: true,
    memcpy_prot_cache: true,
    lli_buses: PL08X_AHB1,
    mem_buses: PL08X_AHB1,
    get_xfer_signal: Some(pl080_get_signal),
    put_xfer_signal: Some(pl080_put_signal),
};

/*
 * Following will create 16MB static virtual/physical mappings
 * PHYSICAL             VIRTUAL
 * 0xD0000000            0xFD000000
 * 0xFC000000            0xFC000000
 */
#[no_mangle]
pub static mut spear3xx_io_desc: [MapDesc; 2] = [
    MapDesc {
        virtual_: VA_SPEAR_ICM1_2_BASE as usize,
        pfn: 0, // __phys_to_pfn(SPEAR_ICM1_2_BASE)
        length: SZ_16M,
        type_: MT_DEVICE,
    },
    MapDesc {
        virtual_: VA_SPEAR_ICM3_SMI_CTRL_BASE as usize,
        pfn: 0, // __phys_to_pfn(SPEAR_ICM3_SMI_CTRL_BASE)
        length: SZ_16M,
        type_: MT_DEVICE,
    },
];

/* This will create static memory mapping for selected devices */
pub unsafe extern "C" fn spear3xx_map_io() {
    iotable_init(spear3xx_io_desc.as_mut_ptr(), spear3xx_io_desc.len());
}

pub unsafe extern "C" fn spear3xx_timer_init() {
    let pclk_name = *b"pll3_clk\0";
    let mut gpt_clk: *mut Clk;
    let mut pclk: *mut Clk;

    spear3xx_clk_init(MISC_BASE as usize, VA_SPEAR320_SOC_CONFIG_BASE as usize);

    /* get the system timer clock */
    gpt_clk = clk_get_sys(b"gpt0\0".as_ptr() as *const _, core::ptr::null());
    if IS_ERR(gpt_clk) {
        pr_err(b"%s:couldn't get clk for gpt\n\0".as_ptr() as *const _, b"spear3xx_timer_init\0".as_ptr());
        bug();
    }

    /* get the suitable parent clock for timer*/
    pclk = clk_get(core::ptr::null(), pclk_name.as_ptr() as *const _);
    if IS_ERR(pclk) {
        pr_err(b"%s:couldn't get %s as parent for gpt\n\0".as_ptr() as *const _, b"spear3xx_timer_init\0".as_ptr(), pclk_name.as_ptr());
        bug();
    }

    clk_set_parent(gpt_clk, pclk);
    clk_put(gpt_clk);
    clk_put(pclk);

    spear_setup_of_timer();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
