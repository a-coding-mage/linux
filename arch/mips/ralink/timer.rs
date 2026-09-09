// SPDX-License-Identifier: GPL-2.0-only
/*
 * Ralink RT2880 timer
 * Author: John Crispin
 *
 * Copyright (C) 2013 John Crispin <john@phrozen.org>
 */

// Kernel headers and asm/mach-ralink/ralink_regs.h are supplied externally.

const TIMER_REG_TMRSTAT: u8 = 0x00;
const TIMER_REG_TMR0LOAD: u8 = 0x10;
const TIMER_REG_TMR0CTL: u8 = 0x18;

const TMRSTAT_TMR0INT: u32 = 1 << 0;
const TMR0CTL_ENABLE: u32 = 1 << 7;
const TMR0CTL_MODE_PERIODIC: u32 = 1 << 4;
const TMR0CTL_PRESCALER: u32 = 1;
const TMR0CTL_PRESCALE_VAL: u32 = 0xf - TMR0CTL_PRESCALER;
const TMR0CTL_PRESCALE_DIV: u32 = 65536 / (1 << TMR0CTL_PRESCALER);

#[repr(C)]
struct RtTimer {
    dev: *mut Device,
    membase: *mut u8,
    irq: i32,
    timer_freq: usize,
    timer_div: usize,
}

#[repr(C)]
struct Device {
    _private: [u8; 0],
}

#[repr(C)]
struct PlatformDevice {
    _private: [u8; 0],
}

#[repr(C)]
struct Clk {
    _private: [u8; 0],
}

#[repr(C)]
struct OfDeviceId {
    compatible: *const u8,
}

#[repr(C)]
struct PlatformDriver {
    probe: Option<unsafe extern "C" fn(*mut PlatformDevice) -> i32>,
    driver: Driver,
}

#[repr(C)]
struct Driver {
    name: *const u8,
    of_match_table: *const OfDeviceId,
    suppress_bind_attrs: bool,
}

type IrqReturn = i32;
const IRQ_HANDLED: IrqReturn = 1;
const GFP_KERNEL: u32 = 0;

extern "C" {
    fn __raw_writel(value: u32, addr: *mut u8);
    fn __raw_readl(addr: *mut u8) -> u32;
    fn request_irq(irq: i32, handler: unsafe extern "C" fn(i32, *mut core::ffi::c_void) -> IrqReturn,
                   flags: u32, name: *const u8, dev: *mut core::ffi::c_void) -> i32;
    fn dev_name(dev: *mut Device) -> *const u8;
    fn dev_err(dev: *mut Device, fmt: *const u8, ...);
    fn dev_info(dev: *mut Device, fmt: *const u8, ...);
    fn devm_kzalloc(dev: *mut Device, size: usize, flags: u32) -> *mut RtTimer;
    fn platform_get_irq(pdev: *mut PlatformDevice, index: u32) -> i32;
    fn devm_platform_get_and_ioremap_resource(pdev: *mut PlatformDevice, index: u32,
                                               resource: *mut core::ffi::c_void) -> *mut u8;
    fn devm_clk_get(dev: *mut Device, id: *const u8) -> *mut Clk;
    fn clk_get_rate(clk: *mut Clk) -> usize;
    fn platform_set_drvdata(pdev: *mut PlatformDevice, data: *mut RtTimer);
    fn is_err(ptr: *const core::ffi::c_void) -> bool;
    fn ptr_err(ptr: *const core::ffi::c_void) -> i32;
}

#[inline]
unsafe fn rt_timer_w32(rt: *mut RtTimer, reg: u8, val: u32) {
    __raw_writel(val, (*rt).membase.add(reg as usize));
}

#[inline]
unsafe fn rt_timer_r32(rt: *mut RtTimer, reg: u8) -> u32 {
    __raw_readl((*rt).membase.add(reg as usize))
}

unsafe extern "C" fn rt_timer_irq(_irq: i32, _rt: *mut core::ffi::c_void) -> IrqReturn {
    let rt = _rt as *mut RtTimer;

    rt_timer_w32(rt, TIMER_REG_TMR0LOAD, ((*rt).timer_freq / (*rt).timer_div) as u32);
    rt_timer_w32(rt, TIMER_REG_TMRSTAT, TMRSTAT_TMR0INT);

    IRQ_HANDLED
}

unsafe fn rt_timer_request(rt: *mut RtTimer) -> i32 {
    let err = request_irq((*rt).irq, rt_timer_irq, 0, dev_name((*rt).dev), rt as *mut _);
    if err != 0 {
        dev_err((*rt).dev, b"failed to request irq\0".as_ptr());
    } else {
        let t = TMR0CTL_MODE_PERIODIC | TMR0CTL_PRESCALE_VAL;
        rt_timer_w32(rt, TIMER_REG_TMR0CTL, t);
    }
    err
}

unsafe fn rt_timer_config(rt: *mut RtTimer, divisor: usize) -> i32 {
    if (*rt).timer_freq < divisor {
        (*rt).timer_div = (*rt).timer_freq;
    } else {
        (*rt).timer_div = divisor;
    }
    rt_timer_w32(rt, TIMER_REG_TMR0LOAD, ((*rt).timer_freq / (*rt).timer_div) as u32);
    0
}

unsafe fn rt_timer_enable(rt: *mut RtTimer) -> i32 {
    rt_timer_w32(rt, TIMER_REG_TMR0LOAD, ((*rt).timer_freq / (*rt).timer_div) as u32);
    let mut t = rt_timer_r32(rt, TIMER_REG_TMR0CTL);
    t |= TMR0CTL_ENABLE;
    rt_timer_w32(rt, TIMER_REG_TMR0CTL, t);
    0
}

unsafe extern "C" fn rt_timer_probe(pdev: *mut PlatformDevice) -> i32 {
    let rt = devm_kzalloc(pdev as *mut Device, core::mem::size_of::<RtTimer>(), GFP_KERNEL);
    if rt.is_null() {
        dev_err(pdev as *mut Device, b"failed to allocate memory\0".as_ptr());
        return -12;
    }
    (*rt).irq = platform_get_irq(pdev, 0);
    if (*rt).irq < 0 { return (*rt).irq; }
    (*rt).membase = devm_platform_get_and_ioremap_resource(pdev, 0, core::ptr::null_mut());
    if is_err((*rt).membase as *const _) { return ptr_err((*rt).membase as *const _); }
    let clk = devm_clk_get(pdev as *mut Device, core::ptr::null());
    if is_err(clk as *const _) {
        dev_err(pdev as *mut Device, b"failed get clock rate\0".as_ptr());
        return ptr_err(clk as *const _);
    }
    (*rt).timer_freq = clk_get_rate(clk) / TMR0CTL_PRESCALE_DIV as usize;
    if (*rt).timer_freq == 0 { return -22; }
    (*rt).dev = pdev as *mut Device;
    platform_set_drvdata(pdev, rt);
    rt_timer_request(rt);
    rt_timer_config(rt, 2);
    rt_timer_enable(rt);
    dev_info((*rt).dev, b"maximum frequency is %luHz\n\0".as_ptr(), (*rt).timer_freq);
    0
}

static RT_TIMER_MATCH: [OfDeviceId; 2] = [
    OfDeviceId { compatible: b"ralink,rt2880-timer\0".as_ptr() },
    OfDeviceId { compatible: core::ptr::null() },
];

static RT_TIMER_DRIVER: PlatformDriver = PlatformDriver {
    probe: Some(rt_timer_probe),
    driver: Driver {
        name: b"rt-timer\0".as_ptr(),
        of_match_table: RT_TIMER_MATCH.as_ptr(),
        suppress_bind_attrs: true,
    },
};

// builtin_platform_driver(rt_timer_driver);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
