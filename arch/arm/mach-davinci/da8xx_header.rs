/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Chip specific defines for DA8XX/OMAP L1XX SoC
 *
 * Author: Mark A. Greer <mgreer@mvista.com>
 *
 * 2007, 2009-2010 (c) MontaVista Software, Inc.
 */

// C header dependencies: linux/dma-mapping.h, linux/platform_device.h,
// linux/videodev2.h, linux/reboot.h, linux/regmap.h, hardware.h, pm.h, and
// media/davinci/vpif_types.h provide the symbols referenced below.

use core::ffi::c_void;

extern "C" {
    pub static mut da8xx_syscfg0_base: *mut c_void;
    pub static mut da8xx_syscfg1_base: *mut c_void;
}

/*
 * The cp_intc interrupt controller for the da8xx isn't in the same
 * chunk of physical memory space as the other registers (like it is
 * on the davincis) so it needs to be mapped separately.  It will be
 * mapped early on when the I/O space is mapped and we'll put it just
 * before the I/O space in the processor's virtual memory space.
 */
pub const DA8XX_CP_INTC_BASE: u32 = 0xfffee000;
pub const DA8XX_CP_INTC_SIZE: usize = SZ_8K;
pub const DA8XX_CP_INTC_VIRT: usize = IO_VIRT - DA8XX_CP_INTC_SIZE - SZ_4K;

pub const DA8XX_SYSCFG0_BASE: usize = IO_PHYS + 0x14000;
#[inline]
pub unsafe fn DA8XX_SYSCFG0_VIRT(x: usize) -> *mut c_void {
    (da8xx_syscfg0_base as *mut u8).add(x) as *mut c_void
}
pub const DA8XX_JTAG_ID_REG: usize = 0x18;
pub const DA8XX_HOST1CFG_REG: usize = 0x44;
pub const DA8XX_CHIPSIG_REG: usize = 0x174;
pub const DA8XX_CFGCHIP0_REG: usize = 0x17c;
pub const DA8XX_CFGCHIP1_REG: usize = 0x180;
pub const DA8XX_CFGCHIP2_REG: usize = 0x184;
pub const DA8XX_CFGCHIP3_REG: usize = 0x188;
pub const DA8XX_CFGCHIP4_REG: usize = 0x18c;

pub const DA8XX_SYSCFG1_BASE: usize = IO_PHYS + 0x22C000;
#[inline]
pub unsafe fn DA8XX_SYSCFG1_VIRT(x: usize) -> *mut c_void {
    (da8xx_syscfg1_base as *mut u8).add(x) as *mut c_void
}
pub const DA8XX_DEEPSLEEP_REG: usize = 0x8;
pub const DA8XX_PWRDN_REG: usize = 0x18;

pub const DA8XX_PSC0_BASE: u32 = 0x01c10000;
pub const DA8XX_PLL0_BASE: u32 = 0x01c11000;
pub const DA8XX_TIMER64P0_BASE: u32 = 0x01c20000;
pub const DA8XX_TIMER64P1_BASE: u32 = 0x01c21000;
pub const DA8XX_VPIF_BASE: u32 = 0x01e17000;
pub const DA8XX_GPIO_BASE: u32 = 0x01e26000;
pub const DA8XX_PSC1_BASE: u32 = 0x01e27000;

pub const DA8XX_DSP_L2_RAM_BASE: u32 = 0x11800000;
pub const DA8XX_DSP_L1P_RAM_BASE: u32 = DA8XX_DSP_L2_RAM_BASE + 0x600000;
pub const DA8XX_DSP_L1D_RAM_BASE: u32 = DA8XX_DSP_L2_RAM_BASE + 0x700000;

pub const DA8XX_AEMIF_CS2_BASE: u32 = 0x60000000;
pub const DA8XX_AEMIF_CS3_BASE: u32 = 0x62000000;
pub const DA8XX_AEMIF_CTL_BASE: u32 = 0x68000000;
pub const DA8XX_SHARED_RAM_BASE: u32 = 0x80000000;
pub const DA8XX_ARM_RAM_BASE: u32 = 0xffff0000;

#[repr(C)]
pub struct vpif_display_config {
    _private: [u8; 0],
}
#[repr(C)]
pub struct vpif_capture_config {
    _private: [u8; 0],
}
#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

extern "C" {
    pub fn da850_init();
    pub fn da850_register_vpif_display(
        display_config: *mut vpif_display_config,
    ) -> i32;
    pub fn da850_register_vpif_capture(
        capture_config: *mut vpif_capture_config,
    ) -> i32;
    pub fn da8xx_get_cfgchip() -> *mut regmap;
    pub fn da8xx_get_mem_ctlr() -> *mut c_void;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
