// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright STMicroelectronics, 2007.
 */

// Dependency intent from the original Linux headers:
// linux/types.h, linux/init.h, linux/io.h, asm/mach/arch.h,
// asm/mach/map.h, and asm/mach-types.h.

/*
 * These are the only hard-coded address offsets we still have to use.
 */
pub const NOMADIK_FSMC_BASE: usize = 0x10100000; /* FSMC registers */
pub const NOMADIK_SDRAMC_BASE: usize = 0x10110000; /* SDRAM Controller */
pub const NOMADIK_CLCDC_BASE: usize = 0x10120000; /* CLCD Controller */
pub const NOMADIK_MDIF_BASE: usize = 0x10120000; /* MDIF */
pub const NOMADIK_DMA0_BASE: usize = 0x10130000; /* DMA0 Controller */
pub const NOMADIK_IC_BASE: usize = 0x10140000; /* Vectored Irq Controller */
pub const NOMADIK_DMA1_BASE: usize = 0x10150000; /* DMA1 Controller */
pub const NOMADIK_USB_BASE: usize = 0x10170000; /* USB-OTG conf reg base */
pub const NOMADIK_CRYP_BASE: usize = 0x10180000; /* Crypto processor */
pub const NOMADIK_SHA1_BASE: usize = 0x10190000; /* SHA-1 Processor */
pub const NOMADIK_XTI_BASE: usize = 0x101A0000; /* XTI */
pub const NOMADIK_RNG_BASE: usize = 0x101B0000; /* Random number generator */
pub const NOMADIK_SRC_BASE: usize = 0x101E0000; /* SRC base */
pub const NOMADIK_WDOG_BASE: usize = 0x101E1000; /* Watchdog */
pub const NOMADIK_MTU0_BASE: usize = 0x101E2000; /* Multiple Timer 0 */
pub const NOMADIK_MTU1_BASE: usize = 0x101E3000; /* Multiple Timer 1 */
pub const NOMADIK_GPIO0_BASE: usize = 0x101E4000; /* GPIO0 */
pub const NOMADIK_GPIO1_BASE: usize = 0x101E5000; /* GPIO1 */
pub const NOMADIK_GPIO2_BASE: usize = 0x101E6000; /* GPIO2 */
pub const NOMADIK_GPIO3_BASE: usize = 0x101E7000; /* GPIO3 */
pub const NOMADIK_RTC_BASE: usize = 0x101E8000; /* Real Time Clock base */
pub const NOMADIK_PMU_BASE: usize = 0x101E9000; /* Power Management Unit */
pub const NOMADIK_OWM_BASE: usize = 0x101EA000; /* One wire master */
pub const NOMADIK_SCR_BASE: usize = 0x101EF000; /* Secure Control registers */
pub const NOMADIK_MSP2_BASE: usize = 0x101F0000; /* MSP 2 interface */
pub const NOMADIK_MSP1_BASE: usize = 0x101F1000; /* MSP 1 interface */
pub const NOMADIK_UART2_BASE: usize = 0x101F2000; /* UART 2 interface */
pub const NOMADIK_SSIRx_BASE: usize = 0x101F3000; /* SSI 8-ch rx interface */
pub const NOMADIK_SSITx_BASE: usize = 0x101F4000; /* SSI 8-ch tx interface */
pub const NOMADIK_MSHC_BASE: usize = 0x101F5000; /* Memory Stick(Pro) Host */
pub const NOMADIK_SDI_BASE: usize = 0x101F6000; /* SD-card/MM-Card */
pub const NOMADIK_I2C1_BASE: usize = 0x101F7000; /* I2C1 interface */
pub const NOMADIK_I2C0_BASE: usize = 0x101F8000; /* I2C0 interface */
pub const NOMADIK_MSP0_BASE: usize = 0x101F9000; /* MSP 0 interface */
pub const NOMADIK_FIRDA_BASE: usize = 0x101FA000; /* FIrDA interface */
pub const NOMADIK_UART1_BASE: usize = 0x101FB000; /* UART 1 interface */
pub const NOMADIK_SSP_BASE: usize = 0x101FC000; /* SSP interface */
pub const NOMADIK_UART0_BASE: usize = 0x101FD000; /* UART 0 interface */
pub const NOMADIK_SGA_BASE: usize = 0x101FE000; /* SGA interface */
pub const NOMADIK_L2CC_BASE: usize = 0x10210000; /* L2 Cache controller */
pub const NOMADIK_UART1_VBASE: usize = 0xF01FB000;

#[repr(C)]
pub struct MapDesc {
    pub virtual_: usize,
    pub pfn: usize,
    pub length: usize,
    pub type_: usize,
}

// This is needed for LL-debug/earlyprintk/debug-macro.S
static mut CPU8815_IO_DESC: [MapDesc; 1] = [MapDesc {
    virtual_: NOMADIK_UART1_VBASE,
    pfn: NOMADIK_UART1_BASE >> 12,
    length: 0x1000,
    type_: 0,
}];

extern "C" {
    fn iotable_init(desc: *mut MapDesc, size: usize);
    fn ioremap(addr: usize, size: usize) -> *mut core::ffi::c_void;
    fn writel(value: u32, addr: *mut core::ffi::c_void);
}

unsafe fn cpu8815_map_io() {
    iotable_init(CPU8815_IO_DESC.as_mut_ptr(), CPU8815_IO_DESC.len());
}

#[repr(C)]
pub enum RebootMode {
    Unknown,
}

unsafe fn cpu8815_restart(_mode: RebootMode, _cmd: *const core::ffi::c_char) {
    let srcbase = ioremap(NOMADIK_SRC_BASE, 0x1000);

    /* FIXME: use egpio when implemented */

    /* Write anything to Reset status register */
    writel(1, srcbase.add(0x18));
}

static CPU8815_BOARD_COMPAT: [Option<&'static core::ffi::CStr>; 3] = [
    None,
    None,
    None,
];

// DT_MACHINE_START(NOMADIK_DT, "Nomadik STn8815")
//     .l2c_aux_val = 0,
//     .l2c_aux_mask = ~0,
//     .map_io = cpu8815_map_io,
//     .restart = cpu8815_restart,
//     .dt_compat = cpu8815_board_compat,
// MACHINE_END

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
