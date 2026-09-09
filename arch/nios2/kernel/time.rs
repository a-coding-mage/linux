/*
 * Copyright (C) 2013-2014 Altera Corporation
 * Copyright (C) 2010 Tobias Klauser <tklauser@distanz.ch>
 * Copyright (C) 2004 Microtronix Datacom Ltd.
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License. See the file "COPYING" in the main directory of this archive
 * for more details.
 */

// Kernel dependencies supplied by the surrounding Nios2 kernel.

const ALTR_TIMER_COMPATIBLE: &str = "altr,timer-1.0";

const ALTERA_TIMER_STATUS_REG: u32 = 0;
const ALTERA_TIMER_CONTROL_REG: u32 = 4;
const ALTERA_TIMER_PERIODL_REG: u32 = 8;
const ALTERA_TIMER_PERIODH_REG: u32 = 12;
const ALTERA_TIMER_SNAPL_REG: u32 = 16;
const ALTERA_TIMER_SNAPH_REG: u32 = 20;

const ALTERA_TIMER_CONTROL_ITO_MSK: u16 = 0x1;
const ALTERA_TIMER_CONTROL_CONT_MSK: u16 = 0x2;
const ALTERA_TIMER_CONTROL_START_MSK: u16 = 0x4;
const ALTERA_TIMER_CONTROL_STOP_MSK: u16 = 0x8;

#[repr(C)]
struct Nios2Timer {
    base: *mut core::ffi::c_void,
    freq: usize,
}

#[repr(C)]
struct Nios2ClockeventDev {
    timer: Nios2Timer,
    ced: clock_event_device,
}

#[repr(C)]
struct Nios2Clocksource {
    timer: Nios2Timer,
    cs: clocksource,
}

unsafe fn to_nios2_clkevent(evt: *mut clock_event_device) -> *mut Nios2ClockeventDev {
    container_of!(evt, Nios2ClockeventDev, ced)
}

unsafe fn to_nios2_clksource(cs: *mut clocksource) -> *mut Nios2Clocksource {
    container_of!(cs, Nios2Clocksource, cs)
}

unsafe fn timer_readw(timer: *mut Nios2Timer, offs: u32) -> u16 {
    readw((*timer).base.cast::<u8>().add(offs as usize).cast())
}

unsafe fn timer_writew(timer: *mut Nios2Timer, val: u16, offs: u32) {
    writew(val, (*timer).base.cast::<u8>().add(offs as usize).cast())
}

unsafe fn read_timersnapshot(timer: *mut Nios2Timer) -> usize {
    timer_writew(timer, 0, ALTERA_TIMER_SNAPL_REG);
    ((timer_readw(timer, ALTERA_TIMER_SNAPH_REG) as usize) << 16)
        | timer_readw(timer, ALTERA_TIMER_SNAPL_REG) as usize
}

unsafe extern "C" fn nios2_timer_read(cs: *mut clocksource) -> u64 {
    let nios2_cs = to_nios2_clksource(cs);
    let mut flags: usize = 0;
    local_irq_save(&mut flags);
    let count = read_timersnapshot(&mut (*nios2_cs).timer);
    local_irq_restore(flags);
    // Counter is counting down
    !(count as u32) as u64
}

static mut nios2_cs: Nios2Clocksource = Nios2Clocksource {
    timer: Nios2Timer { base: core::ptr::null_mut(), freq: 0 },
    cs: clocksource {
        name: "nios2-clksrc",
        rating: 250,
        read: Some(nios2_timer_read),
        mask: CLOCKSOURCE_MASK(32),
        flags: CLOCK_SOURCE_IS_CONTINUOUS,
    },
};

#[no_mangle]
pub unsafe extern "C" fn get_cycles() -> cycles_t {
    // Only read timer if it has been initialized
    if !nios2_cs.timer.base.is_null() {
        nios2_timer_read(&mut nios2_cs.cs)
    } else {
        0
    }
}

unsafe fn nios2_timer_start(timer: *mut Nios2Timer) {
    let mut ctrl = timer_readw(timer, ALTERA_TIMER_CONTROL_REG);
    ctrl |= ALTERA_TIMER_CONTROL_START_MSK;
    timer_writew(timer, ctrl, ALTERA_TIMER_CONTROL_REG);
}

unsafe fn nios2_timer_stop(timer: *mut Nios2Timer) {
    let mut ctrl = timer_readw(timer, ALTERA_TIMER_CONTROL_REG);
    ctrl |= ALTERA_TIMER_CONTROL_STOP_MSK;
    timer_writew(timer, ctrl, ALTERA_TIMER_CONTROL_REG);
}

unsafe fn nios2_timer_config(timer: *mut Nios2Timer, mut period: usize, periodic: bool) {
    // The timer's actual period is one cycle greater than the value stored in the period register.
    period = period.wrapping_sub(1);
    let mut ctrl = timer_readw(timer, ALTERA_TIMER_CONTROL_REG);
    // stop counter
    timer_writew(timer, ctrl | ALTERA_TIMER_CONTROL_STOP_MSK, ALTERA_TIMER_CONTROL_REG);
    // write new count
    timer_writew(timer, period as u16, ALTERA_TIMER_PERIODL_REG);
    timer_writew(timer, (period >> 16) as u16, ALTERA_TIMER_PERIODH_REG);
    ctrl |= ALTERA_TIMER_CONTROL_START_MSK | ALTERA_TIMER_CONTROL_ITO_MSK;
    if periodic { ctrl |= ALTERA_TIMER_CONTROL_CONT_MSK; }
    else { ctrl &= !ALTERA_TIMER_CONTROL_CONT_MSK; }
    timer_writew(timer, ctrl, ALTERA_TIMER_CONTROL_REG);
}

unsafe extern "C" fn nios2_timer_set_next_event(delta: usize, evt: *mut clock_event_device) -> i32 {
    let ced = to_nios2_clkevent(evt);
    nios2_timer_config(&mut (*ced).timer, delta, false); 0
}

unsafe extern "C" fn nios2_timer_shutdown(evt: *mut clock_event_device) -> i32 {
    let ced = to_nios2_clkevent(evt);
    nios2_timer_stop(&mut (*ced).timer); 0
}

unsafe extern "C" fn nios2_timer_set_periodic(evt: *mut clock_event_device) -> i32 {
    let ced = to_nios2_clkevent(evt);
    let period = ((*ced).timer.freq + HZ - 1) / HZ;
    nios2_timer_config(&mut (*ced).timer, period, true); 0
}

unsafe extern "C" fn nios2_timer_resume(evt: *mut clock_event_device) -> i32 {
    let ced = to_nios2_clkevent(evt);
    nios2_timer_start(&mut (*ced).timer); 0
}

pub unsafe extern "C" fn timer_interrupt(_irq: i32, dev_id: *mut core::ffi::c_void) -> irqreturn_t {
    let evt = dev_id.cast::<clock_event_device>();
    let ced = to_nios2_clkevent(evt);
    timer_writew(&mut (*ced).timer, 0, ALTERA_TIMER_STATUS_REG);
    ((*evt).event_handler)(evt);
    IRQ_HANDLED
}

unsafe fn nios2_timer_get_base_and_freq(np: *mut device_node, base: *mut *mut core::ffi::c_void, freq: *mut u32) -> i32 {
    *base = of_iomap(np, 0);
    if (*base).is_null() { pr_crit!("Unable to map reg for %pOFn\n", np); return -ENXIO; }
    if of_property_read_u32(np, "clock-frequency", freq) != 0 { pr_crit!("Unable to get %pOFn clock frequency\n", np); return -EINVAL; }
    0
}

static mut nios2_ce: Nios2ClockeventDev = Nios2ClockeventDev {
    timer: Nios2Timer { base: core::ptr::null_mut(), freq: 0 },
    ced: clock_event_device {
        name: "nios2-clkevent", features: CLOCK_EVT_FEAT_PERIODIC | CLOCK_EVT_FEAT_ONESHOT,
        rating: 250, shift: 32, set_next_event: Some(nios2_timer_set_next_event),
        set_state_shutdown: Some(nios2_timer_shutdown), set_state_periodic: Some(nios2_timer_set_periodic),
        set_state_oneshot: Some(nios2_timer_shutdown), tick_resume: Some(nios2_timer_resume),
    },
};

unsafe extern "C" fn nios2_clockevent_init(timer: *mut device_node) -> i32 {
    let mut iobase = core::ptr::null_mut(); let mut freq = 0u32;
    let ret = nios2_timer_get_base_and_freq(timer, &mut iobase, &mut freq); if ret != 0 { return ret; }
    let irq = irq_of_parse_and_map(timer, 0); if irq == 0 { pr_crit!("Unable to parse timer irq\n"); return -EINVAL; }
    nios2_ce.timer.base = iobase; nios2_ce.timer.freq = freq;
    nios2_ce.ced.cpumask = cpumask_of(0); nios2_ce.ced.irq = irq;
    nios2_timer_stop(&mut nios2_ce.timer); timer_writew(&mut nios2_ce.timer, 0, ALTERA_TIMER_STATUS_REG);
    let ret = request_irq(irq, timer_interrupt, IRQF_TIMER, (*timer).name, &mut nios2_ce.ced);
    if ret != 0 { pr_crit!("Unable to setup timer irq\n"); return ret; }
    clockevents_config_and_register(&mut nios2_ce.ced, freq, 1, usize::MAX); 0
}

unsafe extern "C" fn nios2_clocksource_init(timer: *mut device_node) -> i32 {
    let mut iobase = core::ptr::null_mut(); let mut freq = 0u32;
    let ret = nios2_timer_get_base_and_freq(timer, &mut iobase, &mut freq); if ret != 0 { return ret; }
    nios2_cs.timer.base = iobase; nios2_cs.timer.freq = freq;
    let ret = clocksource_register_hz(&mut nios2_cs.cs, freq); if ret != 0 { return ret; }
    timer_writew(&mut nios2_cs.timer, u16::MAX, ALTERA_TIMER_PERIODL_REG);
    timer_writew(&mut nios2_cs.timer, u16::MAX, ALTERA_TIMER_PERIODH_REG);
    let ctrl = ALTERA_TIMER_CONTROL_CONT_MSK | ALTERA_TIMER_CONTROL_START_MSK;
    timer_writew(&mut nios2_cs.timer, ctrl, ALTERA_TIMER_CONTROL_REG);
    lpj_fine = (freq / HZ) as usize; 0
}

unsafe extern "C" fn nios2_time_init(timer: *mut device_node) -> i32 {
    static mut NUM_CALLED: i32 = 0;
    let ret = match NUM_CALLED { 0 => nios2_clockevent_init(timer), 1 => nios2_clocksource_init(timer), _ => 0 };
    NUM_CALLED += 1; ret
}

pub unsafe extern "C" fn read_persistent_clock64(ts: *mut timespec64) {
    (*ts).tv_sec = mktime64(2007, 1, 1, 0, 0, 0); (*ts).tv_nsec = 0;
}

pub unsafe extern "C" fn time_init() {
    let mut np: *mut device_node = core::ptr::null_mut(); let mut count = 0;
    for_each_compatible_node!(np, core::ptr::null_mut(), ALTR_TIMER_COMPATIBLE) { count += 1; }
    if count < 2 { panic!("{} timer is found, it needs 2 timers in system\n", count); }
    timer_probe();
}

TIMER_OF_DECLARE!(nios2_timer, ALTR_TIMER_COMPATIBLE, nios2_time_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
