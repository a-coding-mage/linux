// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  arch/arm/mach-vt8500/timer.c
 *
 *  Copyright (C) 2012 Tony Prisk <linux@prisktech.co.nz>
 *  Copyright (C) 2010 Alexey Charkov <alchark@gmail.com>
 */

/*
 * This file is copied and modified from the original timer.c provided by
 * Alexey Charkov. Minor changes have been made for Device Tree Support.
 */

// Linux kernel dependencies supplied by other translation units.
use core::ffi::c_void;

const VT8500_TIMER_OFFSET: u32 = 0x0100;
const VT8500_TIMER_HZ: u32 = 3000000;
const TIMER_MATCH_VAL: usize = 0x0000;
const TIMER_COUNT_VAL: usize = 0x0010;
const TIMER_STATUS_VAL: usize = 0x0014;
const TIMER_IER_VAL: usize = 0x001c; // interrupt enable
const TIMER_CTRL_VAL: usize = 0x0020;
const TIMER_AS_VAL: usize = 0x0024; // access status
const TIMER_COUNT_R_ACTIVE: u32 = 1 << 5; // not ready for read
const TIMER_COUNT_W_ACTIVE: u32 = 1 << 4; // not ready for write
const TIMER_MATCH_W_ACTIVE: u32 = 1 << 0; // not ready for write

const MIN_OSCR_DELTA: i32 = 16;

// msecs_to_loops(t) (loops_per_jiffy / 1000 * HZ * t)
#[inline]
unsafe fn msecs_to_loops(t: i32) -> i32 {
    loops_per_jiffy / 1000 * HZ * t
}

#[repr(C)]
pub struct Clocksource {
    pub name: *const u8,
    pub rating: i32,
    pub read: Option<unsafe extern "C" fn(*mut Clocksource) -> u64>,
    pub mask: u64,
    pub flags: u32,
}

#[repr(C)]
pub struct ClockEventDevice {
    pub name: *const u8,
    pub features: u32,
    pub rating: i32,
    pub set_next_event:
        Option<unsafe extern "C" fn(usize, *mut ClockEventDevice) -> i32>,
    pub set_state_shutdown: Option<unsafe extern "C" fn(*mut ClockEventDevice) -> i32>,
    pub set_state_oneshot: Option<unsafe extern "C" fn(*mut ClockEventDevice) -> i32>,
    pub cpumask: *const c_void,
    pub event_handler: Option<unsafe extern "C" fn(*mut ClockEventDevice)>,
}

#[repr(C)]
pub struct DeviceNode {
    _private: [u8; 0],
}

type IrqreturnT = i32;

extern "C" {
    static mut regbase: *mut u8;
    static loops_per_jiffy: i32;
    static HZ: i32;
    static mut clocksource: Clocksource;
    static mut clockevent: ClockEventDevice;

    fn writel(value: u32, address: *mut u8);
    fn readl(address: *mut u8) -> u32;
    fn cpu_relax();
    fn of_iomap(np: *mut DeviceNode, index: i32) -> *mut u8;
    fn irq_of_parse_and_map(np: *mut DeviceNode, index: i32) -> i32;
    fn clocksource_register_hz(cs: *mut Clocksource, hz: u32) -> i32;
    fn cpumask_of(cpu: i32) -> *const c_void;
    fn request_irq(
        irq: i32,
        handler: unsafe extern "C" fn(i32, *mut c_void) -> IrqreturnT,
        flags: u32,
        name: *const u8,
        dev_id: *mut c_void,
    ) -> i32;
    fn clockevents_config_and_register(
        evt: *mut ClockEventDevice,
        freq: u32,
        min_delta: u32,
        max_delta: u32,
    );
}

const CLOCKSOURCE_MASK_32: u64 = 0xffff_ffff;
const CLOCK_SOURCE_IS_CONTINUOUS: u32 = 1;
const CLOCK_EVT_FEAT_ONESHOT: u32 = 1;
const IRQ_HANDLED: IrqreturnT = 1;
const IRQF_TIMER: u32 = 0;
const IRQF_IRQPOLL: u32 = 0;
const ETIME: i32 = 62;
const ENXIO: i32 = 6;
const EINVAL: i32 = 22;

unsafe extern "C" fn vt8500_timer_read(_cs: *mut Clocksource) -> u64 {
    let mut loops = msecs_to_loops(10);
    writel(3, regbase.add(TIMER_CTRL_VAL));
    while (readl(regbase.add(TIMER_AS_VAL)) & TIMER_COUNT_R_ACTIVE) != 0 && {
        loops -= 1;
        loops != 0
    } {
        cpu_relax();
    }
    readl(regbase.add(TIMER_COUNT_VAL)) as u64
}

#[no_mangle]
pub static mut CLOCKSOURCE: Clocksource = Clocksource {
    name: b"vt8500_timer\0".as_ptr(),
    rating: 200,
    read: Some(vt8500_timer_read),
    mask: CLOCKSOURCE_MASK_32,
    flags: CLOCK_SOURCE_IS_CONTINUOUS,
};

unsafe extern "C" fn vt8500_timer_set_next_event(
    cycles: usize,
    _evt: *mut ClockEventDevice,
) -> i32 {
    let mut loops = msecs_to_loops(10);
    let alarm = (vt8500_timer_read(&mut CLOCKSOURCE) as usize).wrapping_add(cycles);
    while (readl(regbase.add(TIMER_AS_VAL)) & TIMER_MATCH_W_ACTIVE) != 0 && {
        loops -= 1;
        loops != 0
    } {
        cpu_relax();
    }
    writel(alarm as u32, regbase.add(TIMER_MATCH_VAL));

    if (alarm.wrapping_sub(vt8500_timer_read(&mut CLOCKSOURCE) as usize) as i32) <= MIN_OSCR_DELTA {
        return -ETIME;
    }

    writel(1, regbase.add(TIMER_IER_VAL));
    0
}

unsafe extern "C" fn vt8500_shutdown(_evt: *mut ClockEventDevice) -> i32 {
    writel(readl(regbase.add(TIMER_CTRL_VAL)) | 1, regbase.add(TIMER_CTRL_VAL));
    writel(0, regbase.add(TIMER_IER_VAL));
    0
}

#[no_mangle]
pub static mut CLOCKEVENT: ClockEventDevice = ClockEventDevice {
    name: b"vt8500_timer\0".as_ptr(),
    features: CLOCK_EVT_FEAT_ONESHOT,
    rating: 200,
    set_next_event: Some(vt8500_timer_set_next_event),
    set_state_shutdown: Some(vt8500_shutdown),
    set_state_oneshot: Some(vt8500_shutdown),
    cpumask: core::ptr::null(),
    event_handler: None,
};

unsafe extern "C" fn vt8500_timer_interrupt(irq: i32, dev_id: *mut c_void) -> IrqreturnT {
    let _ = irq;
    let evt = dev_id as *mut ClockEventDevice;
    writel(0xf, regbase.add(TIMER_STATUS_VAL));
    if let Some(handler) = (*evt).event_handler {
        handler(evt);
    }
    IRQ_HANDLED
}

unsafe extern "C" fn vt8500_timer_init(np: *mut DeviceNode) -> i32 {
    let timer_base = of_iomap(np, 0);
    regbase = timer_base;
    if regbase.is_null() {
        return -ENXIO;
    }

    let timer_irq = irq_of_parse_and_map(np, 0);
    if timer_irq == 0 {
        return -EINVAL;
    }

    writel(1, regbase.add(TIMER_CTRL_VAL));
    writel(0xf, regbase.add(TIMER_STATUS_VAL));
    writel(!0, regbase.add(TIMER_MATCH_VAL));

    let ret = clocksource_register_hz(&mut CLOCKSOURCE, VT8500_TIMER_HZ);
    if ret != 0 {
        return ret;
    }

    CLOCKEVENT.cpumask = cpumask_of(0);
    let ret = request_irq(
        timer_irq,
        vt8500_timer_interrupt,
        IRQF_TIMER | IRQF_IRQPOLL,
        b"vt8500_timer\0".as_ptr(),
        &mut CLOCKEVENT as *mut _ as *mut c_void,
    );
    if ret != 0 {
        return ret;
    }

    clockevents_config_and_register(&mut CLOCKEVENT, VT8500_TIMER_HZ, (MIN_OSCR_DELTA * 2) as u32, 0xf0000000);
    0
}

// TIMER_OF_DECLARE(vt8500, "via,vt8500-timer", vt8500_timer_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
