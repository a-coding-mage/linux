// SPDX-License-Identifier: GPL-2.0-only
/*
 * linux/arch/arm/mach-omap1/mcbsp.c
 *
 * Copyright (C) 2008 Instituto Nokia de Tecnologia
 * Contact: Eduardo Valentin <eduardo.valentin@indt.org.br>
 *
 * Multichannel mode not supported.
 */
// Linux header dependencies are supplied by the surrounding translation.

const DPS_RSTCT2_PER_EN: u16 = 1 << 0;
const DSP_RSTCT2_WD_PER_EN: u16 = 1 << 1;

#[repr(C)]
pub struct clk { _private: [u8; 0] }
#[repr(C)]
pub struct resource {
    pub name: *const core::ffi::c_char,
    pub start: usize,
    pub end: usize,
    pub flags: usize,
}
#[repr(C)]
pub struct omap_mcbsp_ops {
    pub request: Option<unsafe extern "C" fn(u32)>,
    pub free: Option<unsafe extern "C" fn(u32)>,
}
#[repr(C)]
pub struct omap_mcbsp_platform_data {
    pub ops: *mut omap_mcbsp_ops,
    pub reg_size: u32,
    pub reg_step: u32,
}
#[repr(C)]
pub struct device { pub platform_data: *mut core::ffi::c_void }
#[repr(C)]
pub struct platform_device { pub dev: device }

extern "C" {
    static mut DSP_RSTCT2: *mut u16;
    fn clk_get(dev: *mut core::ffi::c_void, id: *const core::ffi::c_char) -> *mut clk;
    fn clk_prepare_enable(clk: *mut clk) -> i32;
    fn clk_disable_unprepare(clk: *mut clk);
    fn clk_put(clk: *mut clk);
    fn __raw_readw(addr: *mut u16) -> u16;
    fn __raw_writew(value: u16, addr: *mut u16);
    fn kzalloc_objs<T>(count: usize) -> *mut T;
    fn printk(fmt: *const core::ffi::c_char, ...);
    fn platform_device_alloc(name: *const core::ffi::c_char, id: i32) -> *mut platform_device;
    fn platform_device_add_resources(dev: *mut platform_device, res: *mut resource, count: i32) -> i32;
    fn platform_device_add(dev: *mut platform_device) -> i32;
    fn platform_device_put(dev: *mut platform_device);
    fn cpu_class_is_omap1() -> bool;
    fn cpu_is_omap15xx() -> bool;
    fn cpu_is_omap16xx() -> bool;
}

static mut dsp_use: i32 = 0;
static mut api_clk: *mut clk = core::ptr::null_mut();
static mut dsp_clk: *mut clk = core::ptr::null_mut();
static mut omap_mcbsp_devices: *mut *mut platform_device = core::ptr::null_mut();

unsafe extern "C" fn omap1_mcbsp_request(id: u32) {
    if id == 0 || id == 2 {
        dsp_use += 1;
        if dsp_use == 1 {
            api_clk = clk_get(core::ptr::null_mut(), b"api_ck\0".as_ptr() as _);
            dsp_clk = clk_get(core::ptr::null_mut(), b"dsp_ck\0".as_ptr() as _);
            if !api_clk.is_null() && !dsp_clk.is_null() {
                clk_prepare_enable(api_clk);
                clk_prepare_enable(dsp_clk);
                __raw_writew(__raw_readw(DSP_RSTCT2) | DPS_RSTCT2_PER_EN |
                    DSP_RSTCT2_WD_PER_EN, DSP_RSTCT2);
            }
        }
    }
}

unsafe extern "C" fn omap1_mcbsp_free(id: u32) {
    if id == 0 || id == 2 {
        dsp_use -= 1;
        if dsp_use == 0 {
            if !api_clk.is_null() { clk_disable_unprepare(api_clk); clk_put(api_clk); }
            if !dsp_clk.is_null() { clk_disable_unprepare(dsp_clk); clk_put(dsp_clk); }
        }
    }
}

static mut omap1_mcbsp_ops: omap_mcbsp_ops = omap_mcbsp_ops {
    request: Some(omap1_mcbsp_request), free: Some(omap1_mcbsp_free),
};

// The following platform resource tables preserve the original hardware constants.
const OMAP7XX_MCBSP1_BASE: usize = 0xfffb1000;
const OMAP7XX_MCBSP2_BASE: usize = 0xfffb1800;
const OMAP1510_MCBSP1_BASE: usize = 0xe1011800;
const OMAP1510_MCBSP2_BASE: usize = 0xfffb1000;
const OMAP1510_MCBSP3_BASE: usize = 0xe1017000;
const OMAP1610_MCBSP1_BASE: usize = 0xe1011800;
const OMAP1610_MCBSP2_BASE: usize = 0xfffb1000;
const OMAP1610_MCBSP3_BASE: usize = 0xe1017000;

// IRQ and resource-flag symbols are external kernel dependencies.
extern "C" {
    static IORESOURCE_MEM: usize; static IORESOURCE_IRQ: usize; static IORESOURCE_DMA: usize;
    static INT_McBSP1RX: usize; static INT_McBSP1TX: usize;
    static INT_1510_SPI_RX: usize; static INT_1510_SPI_TX: usize;
    static INT_McBSP3RX: usize; static INT_McBSP3TX: usize;
    static INT_1610_McBSP2_RX: usize; static INT_1610_McBSP2_TX: usize;
}

static mut omap15xx_mcbsp_res: [[resource; 6]; 3] = [[resource { name: core::ptr::null(), start: 0, end: 0, flags: 0 }; 6]; 3];
static mut omap16xx_mcbsp_res: [[resource; 6]; 3] = [[resource { name: core::ptr::null(), start: 0, end: 0, flags: 0 }; 6]; 3];
static mut omap15xx_mcbsp_pdata: [omap_mcbsp_platform_data; 3] = [omap_mcbsp_platform_data { ops: core::ptr::null_mut(), reg_size: 0, reg_step: 0 }; 3];
static mut omap16xx_mcbsp_pdata: [omap_mcbsp_platform_data; 3] = [omap_mcbsp_platform_data { ops: core::ptr::null_mut(), reg_size: 0, reg_step: 0 }; 3];

unsafe fn omap_mcbsp_register_board_cfg(res: *mut resource, res_count: i32,
    config: *mut omap_mcbsp_platform_data, size: i32) {
    omap_mcbsp_devices = kzalloc_objs(size as usize);
    if omap_mcbsp_devices.is_null() { return; }
    for i in 0..size {
        let new_mcbsp = platform_device_alloc(b"omap-mcbsp\0".as_ptr() as _, i + 1);
        if new_mcbsp.is_null() { continue; }
        platform_device_add_resources(new_mcbsp, res.add((i * res_count) as usize), res_count);
        (*config.add(i as usize)).reg_size = 2;
        (*config.add(i as usize)).reg_step = 2;
        (*new_mcbsp).dev.platform_data = config.add(i as usize) as *mut _;
        if platform_device_add(new_mcbsp) != 0 { platform_device_put(new_mcbsp); continue; }
        *omap_mcbsp_devices.add(i as usize) = new_mcbsp;
    }
}

unsafe extern "C" fn omap1_mcbsp_init() -> i32 {
    if !cpu_class_is_omap1() { return -19; }
    if cpu_is_omap15xx() { omap_mcbsp_register_board_cfg(omap15xx_mcbsp_res as _, 6, omap15xx_mcbsp_pdata.as_mut_ptr(), 3); }
    if cpu_is_omap16xx() { omap_mcbsp_register_board_cfg(omap16xx_mcbsp_res as _, 6, omap16xx_mcbsp_pdata.as_mut_ptr(), 3); }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
