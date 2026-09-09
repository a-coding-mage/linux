/*
 * arch/arm/include/asm/hardware/locomo.h
 *
 * This file contains the definitions for the LoCoMo G/A Chip
 *
 * (C) Copyright 2004 John Lenz
 *
 * May be copied or modified under the terms of the GNU General Public
 * License.  See linux/COPYING for more information.
 *
 * Based on sa1111.h
 */

#[inline]
pub unsafe fn locomo_writel(val: u16, addr: *mut u16) { core::ptr::write_volatile(addr, val); }
#[inline]
pub unsafe fn locomo_readl(addr: *const u16) -> u16 { core::ptr::read_volatile(addr) }

pub const LOCOMO_VER: u32 = 0x00;
pub const LOCOMO_ST: u32 = 0x04;
pub const LOCOMO_C32K: u32 = 0x08;
pub const LOCOMO_ICR: u32 = 0x0C;
pub const LOCOMO_MCSX0: u32 = 0x10;
pub const LOCOMO_MCSX1: u32 = 0x14;
pub const LOCOMO_MCSX2: u32 = 0x18;
pub const LOCOMO_MCSX3: u32 = 0x1c;
pub const LOCOMO_ASD: u32 = 0x20;
pub const LOCOMO_HSD: u32 = 0x28;
pub const LOCOMO_HSC: u32 = 0x2c;
pub const LOCOMO_TADC: u32 = 0x30;
pub const LOCOMO_LTC: u32 = 0xd8;
pub const LOCOMO_LTINT: u32 = 0xdc;
pub const LOCOMO_DAC: u32 = 0xe0;
pub const LOCOMO_DAC_SCLOEB: u32 = 0x08;
pub const LOCOMO_DAC_TEST: u32 = 0x04;
pub const LOCOMO_DAC_SDA: u32 = 0x02;
pub const LOCOMO_DAC_SDAOEB: u32 = 0x01;
pub const LOCOMO_SPI: u32 = 0x60;
pub const LOCOMO_SPIMD: u32 = 0x00;
pub const LOCOMO_SPICT: u32 = 0x04;
pub const LOCOMO_SPIST: u32 = 0x08;
pub const LOCOMO_SPI_TEND: u32 = 1 << 3;
pub const LOCOMO_SPI_REND: u32 = 1 << 2;
pub const LOCOMO_SPI_RFW: u32 = 1 << 1;
pub const LOCOMO_SPI_RFR: u32 = 1;
pub const LOCOMO_SPIIS: u32 = 0x10;
pub const LOCOMO_SPIWE: u32 = 0x14;
pub const LOCOMO_SPIIE: u32 = 0x18;
pub const LOCOMO_SPIIR: u32 = 0x1c;
pub const LOCOMO_SPITD: u32 = 0x20;
pub const LOCOMO_SPIRD: u32 = 0x24;
pub const LOCOMO_SPITS: u32 = 0x28;
pub const LOCOMO_SPIRS: u32 = 0x2C;
pub const LOCOMO_GPD: u32 = 0x90;
pub const LOCOMO_GPE: u32 = 0x94;
pub const LOCOMO_GPL: u32 = 0x98;
pub const LOCOMO_GPO: u32 = 0x9c;
pub const LOCOMO_GRIE: u32 = 0xa0;
pub const LOCOMO_GFIE: u32 = 0xa4;
pub const LOCOMO_GIS: u32 = 0xa8;
pub const LOCOMO_GWE: u32 = 0xac;
pub const LOCOMO_GIE: u32 = 0xb0;
pub const LOCOMO_GIR: u32 = 0xb4;
#[inline] pub const fn LOCOMO_GPIO(nb: u32) -> u32 { 0x01 << nb }
pub const LOCOMO_GPIO_RTS: u32 = LOCOMO_GPIO(0);
pub const LOCOMO_GPIO_CTS: u32 = LOCOMO_GPIO(1);
pub const LOCOMO_GPIO_DSR: u32 = LOCOMO_GPIO(2);
pub const LOCOMO_GPIO_DTR: u32 = LOCOMO_GPIO(3);
pub const LOCOMO_GPIO_LCD_VSHA_ON: u32 = LOCOMO_GPIO(4);
pub const LOCOMO_GPIO_LCD_VSHD_ON: u32 = LOCOMO_GPIO(5);
pub const LOCOMO_GPIO_LCD_VEE_ON: u32 = LOCOMO_GPIO(6);
pub const LOCOMO_GPIO_LCD_MOD: u32 = LOCOMO_GPIO(7);
pub const LOCOMO_GPIO_DAC_ON: u32 = LOCOMO_GPIO(8);
pub const LOCOMO_GPIO_FL_VR: u32 = LOCOMO_GPIO(9);
pub const LOCOMO_GPIO_DAC_SDATA: u32 = LOCOMO_GPIO(10);
pub const LOCOMO_GPIO_DAC_SCK: u32 = LOCOMO_GPIO(11);
pub const LOCOMO_GPIO_DAC_SLOAD: u32 = LOCOMO_GPIO(12);
pub const LOCOMO_GPIO_CARD_DETECT: u32 = LOCOMO_GPIO(13);
pub const LOCOMO_GPIO_WRITE_PROT: u32 = LOCOMO_GPIO(14);
pub const LOCOMO_GPIO_CARD_POWER: u32 = LOCOMO_GPIO(15);

pub const LOCOMO_KEYBOARD: u32 = 0x40;
pub const LOCOMO_KIB: u32 = 0x00;
pub const LOCOMO_KSC: u32 = 0x04;
pub const LOCOMO_KCMD: u32 = 0x08;
pub const LOCOMO_KIC: u32 = 0x0c;
pub const LOCOMO_FRONTLIGHT: u32 = 0xc8;
pub const LOCOMO_ALS: u32 = 0x00;
pub const LOCOMO_ALD: u32 = 0x04;
pub const LOCOMO_ALC_EN: u32 = 0x8000;
pub const LOCOMO_BACKLIGHT: u32 = 0x38;
pub const LOCOMO_TC: u32 = 0x00;
pub const LOCOMO_CPSD: u32 = 0x04;
pub const LOCOMO_AUDIO: u32 = 0x54;
pub const LOCOMO_ACC: u32 = 0x00;
pub const LOCOMO_PAIF: u32 = 0xD0;
pub const LOCOMO_ACC_XON: u32 = 0x80;
pub const LOCOMO_ACC_XEN: u32 = 0x40;
pub const LOCOMO_ACC_XSEL0: u32 = 0x00;
pub const LOCOMO_ACC_XSEL1: u32 = 0x20;
pub const LOCOMO_ACC_MCLKEN: u32 = 0x10;
pub const LOCOMO_ACC_64FSEN: u32 = 0x08;
pub const LOCOMO_ACC_CLKSEL000: u32 = 0x00;
pub const LOCOMO_ACC_CLKSEL001: u32 = 0x01;
pub const LOCOMO_ACC_CLKSEL010: u32 = 0x02;
pub const LOCOMO_ACC_CLKSEL011: u32 = 0x03;
pub const LOCOMO_ACC_CLKSEL100: u32 = 0x04;
pub const LOCOMO_ACC_CLKSEL101: u32 = 0x05;
pub const LOCOMO_PAIF_SCINV: u32 = 0x20;
pub const LOCOMO_PAIF_SCEN: u32 = 0x10;
pub const LOCOMO_PAIF_LRCRST: u32 = 0x08;
pub const LOCOMO_PAIF_LRCEVE: u32 = 0x04;
pub const LOCOMO_PAIF_LRCINV: u32 = 0x02;
pub const LOCOMO_PAIF_LRCEN: u32 = 0x01;
pub const LOCOMO_LED: u32 = 0xe8;
pub const LOCOMO_LPT0: u32 = 0x00;
pub const LOCOMO_LPT1: u32 = 0x04;
pub const LOCOMO_LPT_TOFH: u32 = 0x80;
pub const LOCOMO_LPT_TOFL: u32 = 0x08;
#[inline] pub const fn LOCOMO_LPT_TOH(toh: u32) -> u32 { (toh & 0x7) << 4 }
#[inline] pub const fn LOCOMO_LPT_TOL(tol: u32) -> u32 { tol & 0x7 }

pub const LOCOMO_DEVID_KEYBOARD: u32 = 0;
pub const LOCOMO_DEVID_FRONTLIGHT: u32 = 1;
pub const LOCOMO_DEVID_BACKLIGHT: u32 = 2;
pub const LOCOMO_DEVID_AUDIO: u32 = 3;
pub const LOCOMO_DEVID_LED: u32 = 4;
pub const LOCOMO_DEVID_UART: u32 = 5;
pub const LOCOMO_DEVID_SPI: u32 = 6;

#[repr(C)]
pub struct locomo_dev {
    pub dev: crate::device,
    pub devid: u32,
    pub irq: [u32; 1],
    pub mapbase: *mut core::ffi::c_void,
    pub length: core::ffi::c_ulong,
    pub dma_mask: u64,
}

pub const LOCOMO_DEV: &str = "container_of((_d), struct locomo_dev, dev)";
pub const LOCOMO_DRIVER_NAME: &str = "((_ldev)->dev.driver->name)";

#[repr(C)]
pub struct locomo_driver {
    pub drv: crate::device_driver,
    pub devid: u32,
    pub probe: Option<unsafe extern "C" fn(*mut locomo_dev) -> i32>,
    pub remove: Option<unsafe extern "C" fn(*mut locomo_dev)>,
}

pub const LOCOMO_DRV: &str = "container_of_const((_d), struct locomo_driver, drv)";

unsafe extern "C" {
    pub fn locomolcd_power(on: i32);
    pub fn locomo_driver_register(driver: *mut locomo_driver) -> i32;
    pub fn locomo_driver_unregister(driver: *mut locomo_driver);
    pub fn locomo_gpio_set_dir(dev: *mut crate::device, bits: u32, dir: u32);
    pub fn locomo_gpio_read_level(dev: *mut crate::device, bits: u32) -> i32;
    pub fn locomo_gpio_read_output(dev: *mut crate::device, bits: u32) -> i32;
    pub fn locomo_gpio_write(dev: *mut crate::device, bits: u32, set: u32);
    pub fn locomo_m62332_senddata(ldev: *mut locomo_dev, dac_data: u32, channel: i32);
    pub fn locomo_frontlight_set(dev: *mut locomo_dev, duty: i32, vr: i32, bpwf: i32);
}

#[repr(C)]
pub struct locomo_platform_data {
    pub irq_base: i32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
