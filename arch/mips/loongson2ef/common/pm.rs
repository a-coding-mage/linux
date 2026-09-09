// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * loongson-specific suspend support
 *
 *  Copyright (C) 2009 Lemote Inc.
 *  Author: Wu Zhangjin <wuzhangjin@gmail.com>
 */

// Translated from the Linux kernel implementation.  The declarations below
// are supplied by the corresponding platform and architecture dependencies.

extern "C" {
    fn local_irq_disable();
    fn local_irq_enable();
    fn inb(port: u16) -> u8;
    fn outb(value: u8, port: u16);
    fn readl(addr: *mut u32) -> u32;
    fn writel(value: u32, addr: *mut u32);
    fn mmiowb();
    fn suspend_set_ops(ops: *const platform_suspend_ops);
    fn __write_64bit_c0_register(reg: u32, sel: u32, value: u64);

    static mut LOONGSON_INTEN: u32;
    static mut LOONGSON_INTENCLR: u32;
    static mut LOONGSON_INTENSET: u32;
    static mut LOONGSON_CHIPCFG: u32;
    static mut PIC_SLAVE_IMR: u16;
    static mut PIC_MASTER_IMR: u16;
}

const PM_SUSPEND_ON: suspend_state_t = 0;
const PM_SUSPEND_STANDBY: suspend_state_t = 1;
const PM_SUSPEND_MEM: suspend_state_t = 3;

type suspend_state_t = i32;

#[repr(C)]
struct platform_suspend_ops {
    valid: Option<unsafe extern "C" fn(state: suspend_state_t) -> i32>,
    begin: Option<unsafe extern "C" fn(state: suspend_state_t) -> i32>,
    prepare: Option<unsafe extern "C" fn() -> i32>,
    prepare_late: Option<unsafe extern "C" fn() -> i32>,
    enter: Option<unsafe extern "C" fn(state: suspend_state_t) -> i32>,
    wake: Option<unsafe extern "C" fn()>,
    finish: Option<unsafe extern "C" fn()>,
    end: Option<unsafe extern "C" fn()>,
    recover: Option<unsafe extern "C" fn()>,
};

static mut cached_master_mask: u32 = 0; // i8259A
static mut cached_slave_mask: u32 = 0;
static mut cached_bonito_irq_mask: u32 = 0; // bonito

pub unsafe extern "C" fn arch_suspend_disable_irqs() {
    /* disable all mips events */
    local_irq_disable();

    // CONFIG_I8259 conditionally enables the i8259A event handling here.
    /* disable all events of i8259A */
    cached_slave_mask = inb(PIC_SLAVE_IMR) as u32;
    cached_master_mask = inb(PIC_MASTER_IMR) as u32;

    outb(0xff, PIC_SLAVE_IMR);
    inb(PIC_SLAVE_IMR);
    outb(0xff, PIC_MASTER_IMR);
    inb(PIC_MASTER_IMR);

    /* disable all events of bonito */
    cached_bonito_irq_mask = LOONGSON_INTEN;
    LOONGSON_INTENCLR = 0xffff;
    let _ = LOONGSON_INTENCLR;
}

pub unsafe extern "C" fn arch_suspend_enable_irqs() {
    /* enable all mips events */
    local_irq_enable();
    // CONFIG_I8259 conditionally enables restoration of cached i8259A events.
    /* only enable the cached events of i8259A */
    outb(cached_slave_mask as u8, PIC_SLAVE_IMR);
    outb(cached_master_mask as u8, PIC_MASTER_IMR);

    /* enable all cached events of bonito */
    LOONGSON_INTENSET = cached_bonito_irq_mask;
    let _ = LOONGSON_INTENSET;
}

/*
 * Setup the board-specific events for waking up loongson from wait mode
 */
#[no_mangle]
pub unsafe extern "C" fn setup_wakeup_events() {
}

/*
 * Check wakeup events
 */
#[no_mangle]
pub unsafe extern "C" fn wakeup_loongson() -> i32 {
    1
}

/*
 * If the events are really what we want to wakeup the CPU, wake it up
 * otherwise put the CPU asleep again.
 */
unsafe fn wait_for_wakeup_events() {
    while wakeup_loongson() == 0 {
        writel(readl(&raw mut LOONGSON_CHIPCFG) & !0x7, &raw mut LOONGSON_CHIPCFG);
    }
}

/*
 * Stop all perf counters
 *
 * $24 is the control register of Loongson perf counter
 */
#[inline]
unsafe fn stop_perf_counters() {
    __write_64bit_c0_register(24, 0, 0);
}

unsafe fn loongson_suspend_enter() {
    let cached_cpu_freq: u32;

    /* setup wakeup events via enabling the IRQs */
    setup_wakeup_events();

    stop_perf_counters();

    cached_cpu_freq = readl(&raw mut LOONGSON_CHIPCFG);

    /* Put CPU into wait mode */
    writel(readl(&raw mut LOONGSON_CHIPCFG) & !0x7, &raw mut LOONGSON_CHIPCFG);

    /* wait for the given events to wakeup cpu from wait mode */
    wait_for_wakeup_events();

    writel(cached_cpu_freq, &raw mut LOONGSON_CHIPCFG);

    mmiowb();
}

#[no_mangle]
pub unsafe extern "C" fn mach_suspend() {
}

#[no_mangle]
pub unsafe extern "C" fn mach_resume() {
}

unsafe extern "C" fn loongson_pm_enter(_state: suspend_state_t) -> i32 {
    mach_suspend();

    /* processor specific suspend */
    loongson_suspend_enter();

    mach_resume();

    0
}

unsafe extern "C" fn loongson_pm_valid_state(state: suspend_state_t) -> i32 {
    match state {
        PM_SUSPEND_ON | PM_SUSPEND_STANDBY | PM_SUSPEND_MEM => 1,
        _ => 0,
    }
}

static loongson_pm_ops: platform_suspend_ops = platform_suspend_ops {
    valid: Some(loongson_pm_valid_state),
    begin: None,
    prepare: None,
    prepare_late: None,
    enter: Some(loongson_pm_enter),
    wake: None,
    finish: None,
    end: None,
    recover: None,
};

unsafe extern "C" fn loongson_pm_init() -> i32 {
    suspend_set_ops(&raw const loongson_pm_ops);

    0
}

// arch_initcall(loongson_pm_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
