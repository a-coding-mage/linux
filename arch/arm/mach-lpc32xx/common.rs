// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * arch/arm/mach-lpc32xx/common.c
 *
 * Author: Kevin Wells <kevin.wells@nxp.com>
 *
 * Copyright (C) 2010 NXP Semiconductors
 */

// C includes and symbols supplied by the surrounding kernel are external dependencies.

extern "C" {
    fn __raw_readl(addr: *const core::ffi::c_void) -> u32;
    fn __raw_writel(value: u32, addr: *mut core::ffi::c_void);
    fn io_p2v(addr: usize) -> *mut core::ffi::c_void;
    fn iotable_init(desc: *mut map_desc, num: usize);
    fn printk(fmt: *const core::ffi::c_char, ...);
    static mut system_serial_low: u32;
    static mut system_serial_high: u32;
}

// Constants and types below are supplied by lpc32xx.h and common.h.
extern "C" {
    static LPC32XX_CLKPWR_DEVID: unsafe extern "C" fn(usize) -> *mut core::ffi::c_void;
}

type DmaAddr = usize;

#[repr(C)]
struct map_desc {
    virtual_: usize,
    pfn: usize,
    length: usize,
    type_: usize,
}

const LPC32XX_IRAM_BANK_SIZE: u32 = SZ_128K;
static mut iram_size: u32 = 0;

/*
 * Returns the unique ID for the device
 */
pub unsafe fn lpc32xx_get_uid(devid: *mut u32) {
    let mut i: i32 = 0;
    while i < 4 {
        *devid.add(i as usize) = __raw_readl(LPC32XX_CLKPWR_DEVID((i << 2) as usize));
        i += 1;
    }
}

/*
 * Detects and returns IRAM size for the device variation
 */
pub unsafe fn lpc32xx_return_iram(
    mapbase: *mut *mut core::ffi::c_void,
    dmaaddr: *mut DmaAddr,
) -> u32 {
    if iram_size == 0 {
        let (savedval1, savedval2): (u32, u32);
        let iramptr1: *mut core::ffi::c_void;
        let iramptr2: *mut core::ffi::c_void;

        iramptr1 = io_p2v(LPC32XX_IRAM_BASE as usize);
        iramptr2 = io_p2v((LPC32XX_IRAM_BASE + LPC32XX_IRAM_BANK_SIZE) as usize);
        savedval1 = __raw_readl(iramptr1);
        savedval2 = __raw_readl(iramptr2);

        if savedval1 == savedval2 {
            __raw_writel(savedval2.wrapping_add(1), iramptr2);
            if __raw_readl(iramptr1) == savedval2.wrapping_add(1) {
                iram_size = LPC32XX_IRAM_BANK_SIZE;
            } else {
                iram_size = LPC32XX_IRAM_BANK_SIZE.wrapping_mul(2);
            }
            __raw_writel(savedval2, iramptr2);
        } else {
            iram_size = LPC32XX_IRAM_BANK_SIZE.wrapping_mul(2);
        }
    }
    if !dmaaddr.is_null() {
        *dmaaddr = LPC32XX_IRAM_BASE as DmaAddr;
    }
    if !mapbase.is_null() {
        *mapbase = io_p2v(LPC32XX_IRAM_BASE as usize);
    }

    iram_size
}

pub unsafe fn lpc32xx_set_phy_interface_mode(mode: phy_interface_t) {
    let mut tmp = __raw_readl(LPC32XX_CLKPWR_MACCLK_CTRL);
    tmp &= !LPC32XX_CLKPWR_MACCTRL_PINS_MSK;
    if mode == PHY_INTERFACE_MODE_MII {
        tmp |= LPC32XX_CLKPWR_MACCTRL_USE_MII_PINS;
    } else {
        tmp |= LPC32XX_CLKPWR_MACCTRL_USE_RMII_PINS;
    }
    __raw_writel(tmp, LPC32XX_CLKPWR_MACCLK_CTRL);
}

static mut lpc32xx_io_desc: [map_desc; 4] = [
    map_desc { virtual_: IO_ADDRESS(LPC32XX_AHB0_START), pfn: __phys_to_pfn(LPC32XX_AHB0_START), length: LPC32XX_AHB0_SIZE, type_: MT_DEVICE },
    map_desc { virtual_: IO_ADDRESS(LPC32XX_AHB1_START), pfn: __phys_to_pfn(LPC32XX_AHB1_START), length: LPC32XX_AHB1_SIZE, type_: MT_DEVICE },
    map_desc { virtual_: IO_ADDRESS(LPC32XX_FABAPB_START), pfn: __phys_to_pfn(LPC32XX_FABAPB_START), length: LPC32XX_FABAPB_SIZE, type_: MT_DEVICE },
    map_desc { virtual_: IO_ADDRESS(LPC32XX_IRAM_BASE), pfn: __phys_to_pfn(LPC32XX_IRAM_BASE), length: LPC32XX_IRAM_BANK_SIZE * 2, type_: MT_DEVICE },
];

pub unsafe fn lpc32xx_map_io() {
    iotable_init(lpc32xx_io_desc.as_mut_ptr(), lpc32xx_io_desc.len());
}

pub unsafe fn lpc32xx_check_uid() {
    let mut uid = [0u32; 4];
    lpc32xx_get_uid(uid.as_mut_ptr());

    printk(b"LPC32XX unique ID: %08x%08x%08x%08x\0".as_ptr() as *const _, uid[3], uid[2], uid[1], uid[0]);

    if system_serial_low == 0 && system_serial_high == 0 {
        system_serial_low = uid[0];
        system_serial_high = uid[1];
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
