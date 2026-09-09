/*
 * linux/arch/arm/mach-omap1/timer32k.c
 *
 * OMAP 32K Timer
 *
 * Translated from the original C implementation.
 */

const OMAP1_32K_TIMER_BASE: usize = 0xfffb9000;
const OMAP1_32KSYNC_TIMER_BASE: usize = 0xfffbc400;
const OMAP1_32K_TIMER_CR: usize = 0x08;
const OMAP1_32K_TIMER_TVR: usize = 0x00;
const OMAP1_32K_TIMER_TCR: usize = 0x04;
const OMAP_32K_TICKS_PER_SEC: u64 = 32768;

// HZ is supplied by the kernel configuration.
const OMAP_32K_TIMER_TICK_PERIOD: u64 = (OMAP_32K_TICKS_PER_SEC / HZ) - 1;

const OMAP2_32KSYNCNT_REV_OFF: usize = 0x0;
const OMAP2_32KSYNCNT_REV_SCHEME: u32 = 0x3 << 30;
const OMAP2_32KSYNCNT_CR_OFF_LOW: usize = 0x10;
const OMAP2_32KSYNCNT_CR_OFF_HIGH: usize = 0x30;

// External kernel types and functions are provided by the surrounding tree.
extern "C" {
    static HZ: u64;
    fn omap_writew(val: i32, reg: usize);
    fn request_irq(irq: i32, handler: unsafe extern "C" fn(i32, *mut core::ffi::c_void) -> irqreturn_t,
                   flags: u32, name: *const core::ffi::c_char,
                   dev_id: *mut core::ffi::c_void) -> i32;
    fn cpumask_of(cpu: u32) -> *const core::ffi::c_void;
    fn clockevents_config_and_register(dev: *mut clock_event_device, freq: u32,
                                       min_delta: u32, max_delta: u32);
    fn readl_relaxed(addr: *const u8) -> u32;
    fn clocksource_cyc2ns(cycles: u64, mult: u32, shift: u32) -> u64;
    fn timespec64_add_ns(ts: *mut timespec64, ns: u64);
    fn clocks_calc_mult_shift(mult: *mut u32, shift: *mut u32, from: u32, to: u32, maxsec: u32);
    fn clocksource_mmio_init(reg: *const u8, name: *const core::ffi::c_char, hz: u32,
                             rating: u32, mask: u32, read: *const core::ffi::c_void) -> i32;
    fn sched_clock_register(read: unsafe extern "C" fn() -> u64, bits: u32, rate: u32);
    fn register_persistent_clock(read: unsafe extern "C" fn(*mut timespec64));
    fn ioremap(addr: usize, size: usize) -> *mut u8;
    fn clk_get(dev: *mut core::ffi::c_void, name: *const core::ffi::c_char) -> *mut clk;
    fn IS_ERR(clk: *mut clk) -> bool;
    fn clk_prepare_enable(clk: *mut clk) -> i32;
    fn cpu_is_omap16xx() -> bool;
}

#[repr(C)] pub struct clk { _private: [u8; 0] }
#[repr(C)] pub struct timespec64 { _private: [u8; 0] }
#[repr(C)] pub struct clock_event_device {
    pub name: *const core::ffi::c_char,
    pub features: u32,
    pub set_next_event: Option<unsafe extern "C" fn(u64, *mut clock_event_device) -> i32>,
    pub set_state_shutdown: Option<unsafe extern "C" fn(*mut clock_event_device) -> i32>,
    pub set_state_periodic: Option<unsafe extern "C" fn(*mut clock_event_device) -> i32>,
    pub set_state_oneshot: Option<unsafe extern "C" fn(*mut clock_event_device) -> i32>,
    pub tick_resume: Option<unsafe extern "C" fn(*mut clock_event_device) -> i32>,
    pub cpumask: *const core::ffi::c_void,
    pub event_handler: Option<unsafe extern "C" fn(*mut clock_event_device)>,
}
pub type irqreturn_t = i32;
pub const IRQ_HANDLED: irqreturn_t = 1;
pub const CLOCK_EVT_FEAT_PERIODIC: u32 = 1;
pub const CLOCK_EVT_FEAT_ONESHOT: u32 = 2;
pub const IRQF_TIMER: u32 = 0;
pub const IRQF_IRQPOLL: u32 = 0;
pub const INT_OS_TIMER: i32 = 0;
pub const SZ_1K: usize = 1024;
pub const NSEC_PER_SEC: u32 = 1_000_000_000;
pub const ENODEV: i32 = 19;

unsafe fn omap_32k_timer_write(val: i32, reg: usize) { omap_writew(val, OMAP1_32K_TIMER_BASE + reg); }
unsafe fn omap_32k_timer_start(mut load_val: u64) {
    if load_val == 0 { load_val = 1; }
    omap_32k_timer_write(load_val as i32, OMAP1_32K_TIMER_TVR);
    omap_32k_timer_write(0x0f, OMAP1_32K_TIMER_CR);
}
unsafe fn omap_32k_timer_stop() { omap_32k_timer_write(0, OMAP1_32K_TIMER_CR); }
unsafe extern "C" fn omap_32k_timer_interrupt(_irq: i32, _dev_id: *mut core::ffi::c_void) -> irqreturn_t {
    if let Some(handler) = (*(&raw mut CLOCKEVENT_32K_TIMER)).event_handler {
        handler(&raw mut CLOCKEVENT_32K_TIMER);
    }
    IRQ_HANDLED
}
unsafe fn omap_init_32k_timer() {
    let name = b"32KHz timer\0";
    request_irq(INT_OS_TIMER, omap_32k_timer_interrupt, IRQF_TIMER | IRQF_IRQPOLL,
                name.as_ptr() as *const _, core::ptr::null_mut());
    (*(&raw mut CLOCKEVENT_32K_TIMER)).cpumask = cpumask_of(0);
    clockevents_config_and_register(&raw mut CLOCKEVENT_32K_TIMER, OMAP_32K_TICKS_PER_SEC as u32, 1, 0xfffffffe);
}

unsafe extern "C" fn omap_32k_timer_set_next_event(delta: u64, _dev: *mut clock_event_device) -> i32 {
    omap_32k_timer_start(delta); 0
}
unsafe extern "C" fn omap_32k_timer_shutdown(_evt: *mut clock_event_device) -> i32 { omap_32k_timer_stop(); 0 }
unsafe extern "C" fn omap_32k_timer_set_periodic(_evt: *mut clock_event_device) -> i32 {
    omap_32k_timer_stop(); omap_32k_timer_start(OMAP_32K_TIMER_TICK_PERIOD); 0
}

static mut CLOCKEVENT_32K_TIMER: clock_event_device = clock_event_device {
    name: b"32k-timer\0".as_ptr() as *const _,
    features: CLOCK_EVT_FEAT_PERIODIC | CLOCK_EVT_FEAT_ONESHOT,
    set_next_event: Some(omap_32k_timer_set_next_event),
    set_state_shutdown: Some(omap_32k_timer_shutdown),
    set_state_periodic: Some(omap_32k_timer_set_periodic),
    set_state_oneshot: Some(omap_32k_timer_shutdown),
    tick_resume: Some(omap_32k_timer_shutdown), cpumask: core::ptr::null(), event_handler: None,
};

static mut SYNC32K_CNT_REG: *mut u8 = core::ptr::null_mut();
unsafe extern "C" fn omap_32k_read_sched_clock() -> u64 {
    if !SYNC32K_CNT_REG.is_null() { readl_relaxed(SYNC32K_CNT_REG) as u64 } else { 0 }
}
static mut PERSISTENT_TS: timespec64 = timespec64 { _private: [] };
static mut CYCLES: u64 = 0;
static mut PERSISTENT_MULT: u32 = 0;
static mut PERSISTENT_SHIFT: u32 = 0;

unsafe extern "C" fn omap_read_persistent_clock64(ts: *mut timespec64) {
    let last_cycles = CYCLES;
    CYCLES = if !SYNC32K_CNT_REG.is_null() { readl_relaxed(SYNC32K_CNT_REG) as u64 } else { 0 };
    let nsecs = clocksource_cyc2ns(CYCLES.wrapping_sub(last_cycles), PERSISTENT_MULT, PERSISTENT_SHIFT);
    timespec64_add_ns(&mut PERSISTENT_TS, nsecs);
    core::ptr::write(ts, PERSISTENT_TS);
}

unsafe fn omap_init_clocksource_32k(vbase: *mut u8) -> i32 {
    if readl_relaxed(vbase.add(OMAP2_32KSYNCNT_REV_OFF)) & OMAP2_32KSYNCNT_REV_SCHEME != 0 {
        SYNC32K_CNT_REG = vbase.add(OMAP2_32KSYNCNT_CR_OFF_HIGH);
    } else { SYNC32K_CNT_REG = vbase.add(OMAP2_32KSYNCNT_CR_OFF_LOW); }
    clocks_calc_mult_shift(&mut PERSISTENT_MULT, &mut PERSISTENT_SHIFT, 32768, NSEC_PER_SEC, 120000);
    let ret = clocksource_mmio_init(SYNC32K_CNT_REG, b"32k_counter\0".as_ptr() as *const _, 32768, 250, 32, core::ptr::null());
    if ret != 0 { return ret; }
    sched_clock_register(omap_32k_read_sched_clock, 32, 32768);
    register_persistent_clock(omap_read_persistent_clock64);
    0
}

pub unsafe extern "C" fn omap_32k_timer_init() -> i32 {
    let mut ret = -ENODEV;
    if cpu_is_omap16xx() {
        let base = ioremap(OMAP1_32KSYNC_TIMER_BASE, SZ_1K);
        if base.is_null() { return -ENODEV; }
        let sync32k_ick = clk_get(core::ptr::null_mut(), b"omap_32ksync_ick\0".as_ptr() as *const _);
        if !IS_ERR(sync32k_ick) { clk_prepare_enable(sync32k_ick); }
        ret = omap_init_clocksource_32k(base);
    }
    if ret == 0 { omap_init_32k_timer(); }
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
