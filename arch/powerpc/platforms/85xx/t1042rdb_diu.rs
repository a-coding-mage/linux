// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * T1042 platform DIU operation
 *
 * Copyright 2014 Freescale Semiconductor Inc.
 */

// Linux headers and build-time initialization/module macros are supplied by
// the surrounding kernel translation.

/* DIU Pixel ClockCR offset in scfg */
const CCSR_SCFG_PIXCLKCR: usize = 0x28;

/* DIU Pixel Clock bits of the PIXCLKCR */
const PIXCLKCR_PXCKEN: u32 = 0x80000000;
const PIXCLKCR_PXCKINV: u32 = 0x40000000;
const PIXCLKCR_PXCKDLY: u32 = 0x0000FF00;
const PIXCLKCR_PXCLK_MASK: u32 = 0x00FF0000;

/* Some CPLD register definitions */
const CPLD_DIUCSR: usize = 0x16;
const CPLD_DIUCSR_DVIEN: u8 = 0x80;
const CPLD_DIUCSR_BACKLIGHT: u8 = 0x0f;

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fsl_diu_monitor_port {
    FSL_DIU_PORT_DVI,
    FSL_DIU_PORT_LVDS,
}

extern "C" {
    static mut cpld_node: *mut device_node;
    static mut diu_ops: DiuOps;

    fn of_iomap(node: *mut device_node, index: i32) -> *mut core::ffi::c_void;
    fn of_node_put(node: *mut device_node);
    fn of_find_compatible_node(
        from: *mut device_node,
        ty: *const core::ffi::c_char,
        compatible: *const core::ffi::c_char,
    ) -> *mut device_node;
    fn iounmap(addr: *mut core::ffi::c_void);
    fn fsl_get_sys_freq() -> u64;
    fn pr_err(fmt: *const core::ffi::c_char, ...);
}

#[repr(C)]
pub struct DiuOps {
    pub set_monitor_port: Option<unsafe extern "C" fn(fsl_diu_monitor_port)>,
    pub set_pixel_clock: Option<unsafe extern "C" fn(u32)>,
    pub valid_monitor_port:
        Option<unsafe extern "C" fn(fsl_diu_monitor_port) -> fsl_diu_monitor_port>,
}

unsafe fn clrbits8(addr: *mut u8, mask: u8) {
    let value = core::ptr::read_volatile(addr);
    core::ptr::write_volatile(addr, value & !mask);
}

unsafe fn setbits8(addr: *mut u8, mask: u8) {
    let value = core::ptr::read_volatile(addr);
    core::ptr::write_volatile(addr, value | mask);
}

unsafe fn clrbits32(addr: *mut u32, mask: u32) {
    let value = core::ptr::read_volatile(addr);
    core::ptr::write_volatile(addr, value & !mask);
}

unsafe fn setbits32(addr: *mut u32, mask: u32) {
    let value = core::ptr::read_volatile(addr);
    core::ptr::write_volatile(addr, value | mask);
}

/**
 * t1042rdb_set_monitor_port: switch the output to a different monitor port
 */
unsafe extern "C" fn t1042rdb_set_monitor_port(port: fsl_diu_monitor_port) {
    let cpld_base = of_iomap(cpld_node, 0) as *mut u8;
    if cpld_base.is_null() {
        of_node_put(cpld_node);
        return;
    }

    match port {
        fsl_diu_monitor_port::FSL_DIU_PORT_DVI => {
            /* Enable the DVI(HDMI) port, disable the DFP and the backlight. */
            clrbits8(cpld_base.add(CPLD_DIUCSR), CPLD_DIUCSR_DVIEN);
        }
        fsl_diu_monitor_port::FSL_DIU_PORT_LVDS => {
            /* LVDS also needs backlight enabled, otherwise the display will be blank. */
            /* Enable the DFP port, disable the DVI. */
            setbits8(cpld_base.add(CPLD_DIUCSR), (0x01 << 8) as u8);
            setbits8(cpld_base.add(CPLD_DIUCSR), (0x01 << 4) as u8);
            setbits8(cpld_base.add(CPLD_DIUCSR), CPLD_DIUCSR_BACKLIGHT);
        }
    }

    iounmap(cpld_base as *mut core::ffi::c_void);
    of_node_put(cpld_node);
}

/**
 * t1042rdb_set_pixel_clock: program the DIU's clock
 * @pixclock: pixel clock in ps (pico seconds)
 */
unsafe extern "C" fn t1042rdb_set_pixel_clock(pixclock: u32) {
    let scfg_np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null(),
                                          b"fsl,t1040-scfg\0".as_ptr() as *const _);
    if scfg_np.is_null() {
        return;
    }

    let scfg = of_iomap(scfg_np, 0) as *mut u8;
    of_node_put(scfg_np);
    if scfg.is_null() {
        return;
    }

    /* Convert pixclock into frequency. */
    let temp = 1_000_000_000_000u64 / pixclock as u64;
    let freq = temp;

    /* 'pxclk' is the ratio of the platform clock to the pixel clock. */
    let mut pxclk = ((fsl_get_sys_freq() + freq / 2) / freq) as u32;
    pxclk = pxclk.clamp(2, 255);

    /* Disable the pixel clock, and set it to non-inverted and no delay. */
    clrbits32((scfg.add(CCSR_SCFG_PIXCLKCR)) as *mut u32,
              PIXCLKCR_PXCKEN | PIXCLKCR_PXCKDLY | PIXCLKCR_PXCLK_MASK);

    /* Enable the clock and set the pxclk. */
    setbits32(scfg.add(CCSR_SCFG_PIXCLKCR) as *mut u32,
              PIXCLKCR_PXCKEN | (pxclk << 16));

    iounmap(scfg as *mut core::ffi::c_void);
}

/**
 * t1042rdb_valid_monitor_port: set the monitor port for sysfs
 */
unsafe extern "C" fn t1042rdb_valid_monitor_port(
    port: fsl_diu_monitor_port,
) -> fsl_diu_monitor_port {
    match port {
        fsl_diu_monitor_port::FSL_DIU_PORT_DVI
        | fsl_diu_monitor_port::FSL_DIU_PORT_LVDS => port,
    }
}

unsafe extern "C" fn t1042rdb_diu_init() -> i32 {
    cpld_node = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null(),
                                         b"fsl,t1042rdb-cpld\0".as_ptr() as *const _);
    if cpld_node.is_null() {
        return 0;
    }

    diu_ops.set_monitor_port = Some(t1042rdb_set_monitor_port);
    diu_ops.set_pixel_clock = Some(t1042rdb_set_pixel_clock);
    diu_ops.valid_monitor_port = Some(t1042rdb_valid_monitor_port);

    0
}

// early_initcall(t1042rdb_diu_init);
// MODULE_DESCRIPTION("Freescale T1042 DIU driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
