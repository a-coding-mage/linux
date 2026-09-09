// SPDX-License-Identifier: GPL-2.0
/*
 * m5441x.c -- support for Coldfire m5441x processors
 *
 * (C) Copyright Steven King <sfking@fdwdc.com>
 */

// Linux kernel headers and their symbols are supplied by the surrounding tree.

#[repr(C)]
pub struct Clk {
    pub slot: u32,
}

#[repr(C)]
pub struct ClkLookup {
    pub dev_id: *const i8,
    pub con_id: *const i8,
    pub clk: *mut Clk,
}

#[repr(C)]
pub struct ClkOps {
    pub enable: Option<unsafe extern "C" fn(*mut Clk)>,
    pub disable: Option<unsafe extern "C" fn(*mut Clk)>,
}

#[repr(C)]
pub struct Resource {
    pub start: usize,
    pub end: usize,
    pub flags: usize,
}

#[repr(C)]
pub struct PlatformDevice {
    _opaque: [u8; 0],
}

extern "C" {
    static mut MCFSDHC_CLK: usize;
    static mut MCFGPIO_PAR_UART0: usize;
    static mut MCFGPIO_PAR_UART1: usize;
    static mut MCFGPIO_PAR_UART2: usize;
    static mut MCFGPIO_PAR_FEC: usize;
    static mut MCF_RSR: usize;
    static mut mach_sched_init: Option<unsafe extern "C" fn()>;

    fn mcf_read32(address: usize) -> u32;
    fn mcf_write32(value: u32, address: usize);
    fn mcf_write8(value: u8, address: usize);
    fn hw_timer_init();
    fn __clk_init_enabled(clk: *mut Clk);
    fn __clk_init_disabled(clk: *mut Clk);
    fn clkdev_add_table(table: *mut ClkLookup, size: usize);
    fn platform_device_register_simple(
        name: *const i8,
        id: i32,
        resources: *mut Resource,
        num_resources: usize,
    ) -> *mut PlatformDevice;
    fn ptr_err_or_zero(device: *mut PlatformDevice) -> i32;
}

// DEFINE_CLK declarations. The actual clock objects are provided by the kernel clock framework.
extern "C" {
    static mut __clk_0_2: Clk; static mut __clk_0_8: Clk; static mut __clk_0_9: Clk;
    static mut __clk_0_14: Clk; static mut __clk_0_15: Clk; static mut __clk_0_17: Clk;
    static mut __clk_0_18: Clk; static mut __clk_0_19: Clk; static mut __clk_0_20: Clk;
    static mut __clk_0_22: Clk; static mut __clk_0_23: Clk; static mut __clk_0_24: Clk;
    static mut __clk_0_25: Clk; static mut __clk_0_26: Clk; static mut __clk_0_27: Clk;
    static mut __clk_0_28: Clk; static mut __clk_0_29: Clk; static mut __clk_0_30: Clk;
    static mut __clk_0_31: Clk; static mut __clk_0_32: Clk; static mut __clk_0_33: Clk;
    static mut __clk_0_34: Clk; static mut __clk_0_35: Clk; static mut __clk_0_36: Clk;
    static mut __clk_0_37: Clk; static mut __clk_0_38: Clk; static mut __clk_0_39: Clk;
    static mut __clk_0_42: Clk; static mut __clk_0_43: Clk; static mut __clk_0_44: Clk;
    static mut __clk_0_45: Clk; static mut __clk_0_46: Clk; static mut __clk_0_47: Clk;
    static mut __clk_0_48: Clk; static mut __clk_0_49: Clk; static mut __clk_0_50: Clk;
    static mut __clk_0_51: Clk; static mut __clk_0_53: Clk; static mut __clk_0_54: Clk;
    static mut __clk_0_55: Clk; static mut __clk_0_56: Clk; static mut __clk_0_63: Clk;
    static mut __clk_1_2: Clk; static mut __clk_1_4: Clk; static mut __clk_1_5: Clk;
    static mut __clk_1_6: Clk; static mut __clk_1_7: Clk; static mut __clk_1_24: Clk;
    static mut __clk_1_25: Clk; static mut __clk_1_26: Clk; static mut __clk_1_27: Clk;
    static mut __clk_1_28: Clk; static mut __clk_1_29: Clk; static mut __clk_1_34: Clk;
    static mut __clk_1_36: Clk; static mut __clk_1_37: Clk;
    static mut __clk_2_0: Clk; static mut __clk_2_1: Clk; static mut __clk_2_2: Clk;
}

static mut M5411X_CLK_LOOKUP: [ClkLookup; 0] = [];
static mut ENABLE_CLKS: [*mut Clk; 18] = [
    unsafe { &mut __clk_0_8 }, unsafe { &mut __clk_0_9 }, unsafe { &mut __clk_0_15 },
    unsafe { &mut __clk_0_17 }, unsafe { &mut __clk_0_18 }, unsafe { &mut __clk_0_19 },
    unsafe { &mut __clk_0_20 }, unsafe { &mut __clk_0_23 }, unsafe { &mut __clk_0_24 },
    unsafe { &mut __clk_0_25 }, unsafe { &mut __clk_0_26 }, unsafe { &mut __clk_0_27 },
    unsafe { &mut __clk_0_33 }, unsafe { &mut __clk_0_36 }, unsafe { &mut __clk_0_48 },
    unsafe { &mut __clk_0_51 }, unsafe { &mut __clk_1_36 }, unsafe { &mut __clk_1_37 },
];
static mut DISABLE_CLKS: [*mut Clk; 33] = [
    unsafe { &mut __clk_0_14 }, unsafe { &mut __clk_0_22 }, unsafe { &mut __clk_0_23 },
    unsafe { &mut __clk_0_28 }, unsafe { &mut __clk_0_29 }, unsafe { &mut __clk_0_30 },
    unsafe { &mut __clk_0_31 }, unsafe { &mut __clk_0_32 }, unsafe { &mut __clk_0_34 },
    unsafe { &mut __clk_0_35 }, unsafe { &mut __clk_0_37 }, unsafe { &mut __clk_0_38 },
    unsafe { &mut __clk_0_39 }, unsafe { &mut __clk_0_44 }, unsafe { &mut __clk_0_45 },
    unsafe { &mut __clk_0_47 }, unsafe { &mut __clk_0_49 }, unsafe { &mut __clk_0_50 },
    unsafe { &mut __clk_0_53 }, unsafe { &mut __clk_0_54 }, unsafe { &mut __clk_0_55 },
    unsafe { &mut __clk_0_56 }, unsafe { &mut __clk_1_2 }, unsafe { &mut __clk_1_4 },
    unsafe { &mut __clk_1_5 }, unsafe { &mut __clk_1_6 }, unsafe { &mut __clk_1_7 },
    unsafe { &mut __clk_1_24 }, unsafe { &mut __clk_1_25 }, unsafe { &mut __clk_1_26 },
    unsafe { &mut __clk_1_27 }, unsafe { &mut __clk_1_28 }, unsafe { &mut __clk_1_29 },
];

unsafe extern "C" fn __clk_enable2(clk: *mut Clk) {
    mcf_write32(mcf_read32(MCFSDHC_CLK) | (1u32 << (*clk).slot), MCFSDHC_CLK);
}

unsafe extern "C" fn __clk_disable2(clk: *mut Clk) {
    mcf_write32(mcf_read32(MCFSDHC_CLK) & !(1u32 << (*clk).slot), MCFSDHC_CLK);
}

#[no_mangle]
pub static mut clk_ops2: ClkOps = ClkOps {
    enable: Some(__clk_enable2),
    disable: Some(__clk_disable2),
};

unsafe fn m5441x_clk_init() {
    for clk in ENABLE_CLKS.iter() {
        __clk_init_enabled(*clk);
    }
    /* make sure these clocks are disabled */
    for clk in DISABLE_CLKS.iter() {
        __clk_init_disabled(*clk);
    }
    clkdev_add_table(M5411X_CLK_LOOKUP.as_mut_ptr(), M5411X_CLK_LOOKUP.len());
}

unsafe fn m5441x_uarts_init() {
    mcf_write8(0x0f, MCFGPIO_PAR_UART0);
    mcf_write8(0x00, MCFGPIO_PAR_UART1);
    mcf_write8(0x00, MCFGPIO_PAR_UART2);
}

unsafe fn m5441x_fec_init() {
    mcf_write8(0x03, MCFGPIO_PAR_FEC);
}

/*
 * Reset Controller Module status register. Exposed to userspace as
 * /sys/devices/platform/mcf-rcm-reset/power_on_reason by the mcf-rcm-reset
 * driver (drivers/power/reset/mcf-rcm-reset.c).
 */
static mut M5441X_RCM_RESOURCE: [Resource; 1] = [Resource {
    start: 0,
    end: 0,
    flags: 0x00000200, // IORESOURCE_MEM
}];

unsafe extern "C" fn m5441x_rcm_init() -> i32 {
    let pdev = platform_device_register_simple(
        b"mcf-rcm-reset\0".as_ptr() as *const i8,
        -1,
        M5441X_RCM_RESOURCE.as_mut_ptr(),
        M5441X_RCM_RESOURCE.len(),
    );
    ptr_err_or_zero(pdev)
}

pub unsafe extern "C" fn config_BSP(_commandp: *mut i8, _size: i32) {
    m5441x_clk_init();
    mach_sched_init = Some(hw_timer_init);
    m5441x_uarts_init();
    m5441x_fec_init();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
