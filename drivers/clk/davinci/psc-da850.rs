// SPDX-License-Identifier: GPL-2.0
/*
 * PSC clock descriptions for TI DA850/OMAP-L138/AM18XX
 *
 * Copyright (C) 2018 David Lechner <david@lechnology.com>
 */

// The following types, constants, macros, and functions are supplied by the
// surrounding PSC implementation and kernel bindings.
use core::ffi::c_void;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk_bulk_data {
    pub id: *const core::ffi::c_char,
}

#[repr(C)]
pub struct davinci_lpsc_clk_info {
    _private: [u8; 0],
}

#[repr(C)]
pub struct davinci_psc_init_data {
    pub parent_clks: *mut clk_bulk_data,
    pub num_parent_clks: usize,
    pub psc_init: Option<unsafe extern "C" fn(*mut device, *mut c_void) -> i32>,
}

extern "C" {
    fn davinci_psc_register_clocks(
        dev: *mut device,
        info: *const davinci_lpsc_clk_info,
        count: u32,
        base: *mut c_void,
    ) -> i32;
    fn of_davinci_psc_clk_init(
        dev: *mut device,
        info: *const davinci_lpsc_clk_info,
        count: u32,
        base: *mut c_void,
    ) -> i32;
}

// LPSC_CLKDEV* declarations are generated clock lookup-data objects.
extern "C" {
    static emifa_clkdev: c_void;
    static spi0_clkdev: c_void;
    static mmcsd0_clkdev: c_void;
    static uart0_clkdev: c_void;
    static arm_clkdev: c_void;
    static dsp_clkdev: c_void;
    static usb0_clkdev: c_void;
    static usb1_clkdev: c_void;
    static gpio_clkdev: c_void;
    static emac_clkdev: c_void;
    static mcasp0_clkdev: c_void;
    static sata_clkdev: c_void;
    static vpif_clkdev: c_void;
    static spi1_clkdev: c_void;
    static i2c1_clkdev: c_void;
    static uart1_clkdev: c_void;
    static uart2_clkdev: c_void;
    static mcbsp0_clkdev: c_void;
    static mcbsp1_clkdev: c_void;
    static lcdc_clkdev: c_void;
    static ehrpwm_clkdev: c_void;
    static mmcsd1_clkdev: c_void;
    static ecap_clkdev: c_void;
}

// REVISIT: used dev_id instead of con_id.

macro_rules! LPSC { ($($item:tt)*) => { davinci_lpsc_clk_info { _private: [] } }; }

static da850_psc0_info: [davinci_lpsc_clk_info; 16] = [
    LPSC!(0, 0, tpcc0, pll0_sysclk2, NULL, LPSC_ALWAYS_ENABLED),
    LPSC!(1, 0, tptc0, pll0_sysclk2, NULL, LPSC_ALWAYS_ENABLED),
    LPSC!(2, 0, tptc1, pll0_sysclk2, NULL, LPSC_ALWAYS_ENABLED),
    LPSC!(3, 0, emifa, async1, emifa_clkdev, 0),
    LPSC!(4, 0, spi0, pll0_sysclk2, spi0_clkdev, 0),
    LPSC!(5, 0, mmcsd0, pll0_sysclk2, mmcsd0_clkdev, 0),
    LPSC!(6, 0, aintc, pll0_sysclk4, NULL, LPSC_ALWAYS_ENABLED),
    LPSC!(7, 0, arm_rom, pll0_sysclk2, NULL, LPSC_ALWAYS_ENABLED),
    LPSC!(9, 0, uart0, pll0_sysclk2, uart0_clkdev, 0),
    LPSC!(13, 0, pruss, pll0_sysclk2, NULL, 0),
    LPSC!(14, 0, arm, pll0_sysclk6, arm_clkdev, LPSC_ALWAYS_ENABLED | LPSC_SET_RATE_PARENT),
    LPSC!(15, 1, dsp, pll0_sysclk1, dsp_clkdev, LPSC_FORCE | LPSC_LOCAL_RESET),
    LPSC!(0, 0, 0, 0, 0, 0), LPSC!(0, 0, 0, 0, 0, 0), LPSC!(0, 0, 0, 0, 0, 0), LPSC!(0, 0, 0, 0, 0, 0),
];

static da850_psc1_info: [davinci_lpsc_clk_info; 32] = [
    LPSC!(0, 0, tpcc1, pll0_sysclk2, NULL, LPSC_ALWAYS_ENABLED),
    LPSC!(1, 0, usb0, pll0_sysclk2, usb0_clkdev, 0),
    LPSC!(2, 0, usb1, pll0_sysclk4, usb1_clkdev, 0),
    LPSC!(3, 0, gpio, pll0_sysclk4, gpio_clkdev, 0),
    LPSC!(5, 0, emac, pll0_sysclk4, emac_clkdev, 0),
    LPSC!(6, 0, ddr, pll0_sysclk2, NULL, LPSC_ALWAYS_ENABLED),
    LPSC!(7, 0, mcasp0, async3, mcasp0_clkdev, 0), LPSC!(8, 0, sata, pll0_sysclk2, sata_clkdev, LPSC_FORCE),
    LPSC!(9, 0, vpif, pll0_sysclk2, vpif_clkdev, 0), LPSC!(10, 0, spi1, async3, spi1_clkdev, 0),
    LPSC!(11, 0, i2c1, pll0_sysclk4, i2c1_clkdev, 0), LPSC!(12, 0, uart1, async3, uart1_clkdev, 0),
    LPSC!(13, 0, uart2, async3, uart2_clkdev, 0), LPSC!(14, 0, mcbsp0, async3, mcbsp0_clkdev, 0),
    LPSC!(15, 0, mcbsp1, async3, mcbsp1_clkdev, 0), LPSC!(16, 0, lcdc, pll0_sysclk2, lcdc_clkdev, 0),
    LPSC!(17, 0, ehrpwm, async3, ehrpwm_clkdev, 0), LPSC!(18, 0, mmcsd1, pll0_sysclk2, mmcsd1_clkdev, 0),
    LPSC!(20, 0, ecap, async3, ecap_clkdev, 0), LPSC!(21, 0, tptc2, pll0_sysclk2, NULL, LPSC_ALWAYS_ENABLED),
    LPSC!(0, 0, 0, 0, 0, 0), LPSC!(0, 0, 0, 0, 0, 0), LPSC!(0, 0, 0, 0, 0, 0), LPSC!(0, 0, 0, 0, 0, 0),
    LPSC!(0, 0, 0, 0, 0, 0), LPSC!(0, 0, 0, 0, 0, 0), LPSC!(0, 0, 0, 0, 0, 0), LPSC!(0, 0, 0, 0, 0, 0),
    LPSC!(0, 0, 0, 0, 0, 0), LPSC!(0, 0, 0, 0, 0, 0), LPSC!(0, 0, 0, 0, 0, 0), LPSC!(0, 0, 0, 0, 0, 0),
];

pub unsafe extern "C" fn da850_psc0_init(dev: *mut device, base: *mut c_void) -> i32 {
    davinci_psc_register_clocks(dev, da850_psc0_info.as_ptr(), 16, base)
}

pub unsafe extern "C" fn of_da850_psc0_init(dev: *mut device, base: *mut c_void) -> i32 {
    of_davinci_psc_clk_init(dev, da850_psc0_info.as_ptr(), 16, base)
}

pub static mut da850_psc0_parent_clks: [clk_bulk_data; 5] = [
    clk_bulk_data { id: b"pll0_sysclk1\0".as_ptr() as *const _ },
    clk_bulk_data { id: b"pll0_sysclk2\0".as_ptr() as *const _ },
    clk_bulk_data { id: b"pll0_sysclk4\0".as_ptr() as *const _ },
    clk_bulk_data { id: b"pll0_sysclk6\0".as_ptr() as *const _ },
    clk_bulk_data { id: b"async1\0".as_ptr() as *const _ },
];

pub static da850_psc0_init_data: davinci_psc_init_data = davinci_psc_init_data {
    parent_clks: unsafe { da850_psc0_parent_clks.as_ptr() as *mut _ },
    num_parent_clks: 5,
    psc_init: Some(da850_psc0_init),
};

pub static of_da850_psc0_init_data: davinci_psc_init_data = davinci_psc_init_data {
    parent_clks: unsafe { da850_psc0_parent_clks.as_ptr() as *mut _ },
    num_parent_clks: 5,
    psc_init: Some(of_da850_psc0_init),
};

pub unsafe extern "C" fn da850_psc1_init(dev: *mut device, base: *mut c_void) -> i32 {
    davinci_psc_register_clocks(dev, da850_psc1_info.as_ptr(), 32, base)
}

pub unsafe extern "C" fn of_da850_psc1_init(dev: *mut device, base: *mut c_void) -> i32 {
    of_davinci_psc_clk_init(dev, da850_psc1_info.as_ptr(), 32, base)
}

pub static mut da850_psc1_parent_clks: [clk_bulk_data; 3] = [
    clk_bulk_data { id: b"pll0_sysclk2\0".as_ptr() as *const _ },
    clk_bulk_data { id: b"pll0_sysclk4\0".as_ptr() as *const _ },
    clk_bulk_data { id: b"async3\0".as_ptr() as *const _ },
];

pub static da850_psc1_init_data: davinci_psc_init_data = davinci_psc_init_data {
    parent_clks: unsafe { da850_psc1_parent_clks.as_ptr() as *mut _ },
    num_parent_clks: 3,
    psc_init: Some(da850_psc1_init),
};

pub static of_da850_psc1_init_data: davinci_psc_init_data = davinci_psc_init_data {
    parent_clks: unsafe { da850_psc1_parent_clks.as_ptr() as *mut _ },
    num_parent_clks: 3,
    psc_init: Some(of_da850_psc1_init),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
