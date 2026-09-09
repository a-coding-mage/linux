// SPDX-License-Identifier: GPL-2.0
/*
 * MStar timer driver
 *
 * Copyright (C) 2021 Daniel Palmer
 * Copyright (C) 2021 Romain Perier
 */

// Linux kernel dependencies are supplied by the surrounding repository.

const TIMER_NAME: &str = "msc313e_timer";

const MSC313E_REG_CTRL: usize = 0x00;
const MSC313E_REG_CTRL_TIMER_EN: u16 = 1 << 0;
const MSC313E_REG_CTRL_TIMER_TRIG: u16 = 1 << 1;
const MSC313E_REG_CTRL_TIMER_INT_EN: u16 = 1 << 8;
const MSC313E_REG_TIMER_MAX_LOW: usize = 0x08;
const MSC313E_REG_TIMER_MAX_HIGH: usize = 0x0c;
const MSC313E_REG_COUNTER_LOW: usize = 0x10;
const MSC313E_REG_COUNTER_HIGH: usize = 0x14;
const MSC313E_REG_TIMER_DIVIDE: usize = 0x18;

const MSC313E_CLK_DIVIDER: u32 = 9;
const TIMER_SYNC_TICKS: u32 = 3;

#[cfg(CONFIG_ARM)]
struct Msc313eDelay {
    base: *mut core::ffi::c_void,
    delay: delay_timer,
}

#[cfg(CONFIG_ARM)]
static mut MSC313E_DELAY: Msc313eDelay = Msc313eDelay {
    base: core::ptr::null_mut(),
    delay: delay_timer {},
};

static mut MSC313E_CLKSRC: *mut core::ffi::c_void = core::ptr::null_mut();

unsafe fn msc313e_timer_stop(base: *mut core::ffi::c_void) {
    writew(0, base.add(MSC313E_REG_CTRL));
}

unsafe fn msc313e_timer_start(base: *mut core::ffi::c_void, periodic: bool) {
    let mut reg: u16 = readw(base.add(MSC313E_REG_CTRL));
    if periodic {
        reg |= MSC313E_REG_CTRL_TIMER_EN;
    } else {
        reg |= MSC313E_REG_CTRL_TIMER_TRIG;
    }
    writew(reg | MSC313E_REG_CTRL_TIMER_INT_EN, base.add(MSC313E_REG_CTRL));
}

unsafe fn msc313e_timer_setup(base: *mut core::ffi::c_void, delay: usize) {
    let mut flags: usize = 0;
    local_irq_save(&mut flags);
    writew((delay >> 16) as u16, base.add(MSC313E_REG_TIMER_MAX_HIGH));
    writew((delay & 0xffff) as u16, base.add(MSC313E_REG_TIMER_MAX_LOW));
    local_irq_restore(flags);
}

unsafe fn msc313e_timer_current_value(base: *mut core::ffi::c_void) -> usize {
    let mut flags: usize = 0;
    local_irq_save(&mut flags);
    let l: u16 = readw(base.add(MSC313E_REG_COUNTER_LOW));
    let h: u16 = readw(base.add(MSC313E_REG_COUNTER_HIGH));
    local_irq_restore(flags);
    (((h as u32) << 16) | l as u32) as usize
}

unsafe extern "C" fn msc313e_timer_clkevt_shutdown(evt: *mut clock_event_device) -> i32 {
    let timer = to_timer_of(evt);
    msc313e_timer_stop(timer_of_base(timer));
    0
}

unsafe extern "C" fn msc313e_timer_clkevt_set_oneshot(evt: *mut clock_event_device) -> i32 {
    let timer = to_timer_of(evt);
    msc313e_timer_stop(timer_of_base(timer));
    msc313e_timer_start(timer_of_base(timer), false);
    0
}

unsafe extern "C" fn msc313e_timer_clkevt_set_periodic(evt: *mut clock_event_device) -> i32 {
    let timer = to_timer_of(evt);
    msc313e_timer_stop(timer_of_base(timer));
    msc313e_timer_setup(timer_of_base(timer), timer_of_period(timer));
    msc313e_timer_start(timer_of_base(timer), true);
    0
}

unsafe extern "C" fn msc313e_timer_clkevt_next_event(evt: usize, clkevt: *mut clock_event_device) -> i32 {
    let timer = to_timer_of(clkevt);
    msc313e_timer_stop(timer_of_base(timer));
    msc313e_timer_setup(timer_of_base(timer), evt);
    msc313e_timer_start(timer_of_base(timer), false);
    0
}

unsafe extern "C" fn msc313e_timer_clkevt_irq(_irq: i32, dev_id: *mut core::ffi::c_void) -> irqreturn_t {
    let evt = dev_id as *mut clock_event_device;
    ((*evt).event_handler)(evt);
    IRQ_HANDLED
}

unsafe extern "C" fn msc313e_timer_clksrc_read(cs: *mut clocksource) -> u64 {
    (msc313e_timer_current_value(MSC313E_CLKSRC) as u64) & (*cs).mask
}

#[cfg(CONFIG_ARM)]
unsafe extern "C" fn msc313e_read_delay_timer_read() -> usize {
    msc313e_timer_current_value(MSC313E_DELAY.base)
}

unsafe extern "C" fn msc313e_timer_sched_clock_read() -> u64 {
    msc313e_timer_current_value(MSC313E_CLKSRC) as u64
}

static mut MSC313E_CLKEVT: clock_event_device = clock_event_device {
    name: TIMER_NAME.as_ptr() as *const i8,
    rating: 300,
    features: CLOCK_EVT_FEAT_PERIODIC | CLOCK_EVT_FEAT_ONESHOT,
    set_state_shutdown: Some(msc313e_timer_clkevt_shutdown),
    set_state_periodic: Some(msc313e_timer_clkevt_set_periodic),
    set_state_oneshot: Some(msc313e_timer_clkevt_set_oneshot),
    tick_resume: Some(msc313e_timer_clkevt_shutdown),
    set_next_event: Some(msc313e_timer_clkevt_next_event),
    ..unsafe { core::mem::zeroed() }
};

unsafe extern "C" fn msc313e_clkevt_init(np: *mut device_node) -> i32 {
    let mut ret: i32;
    let to = kzalloc_obj::<timer_of>();
    if to.is_null() {
        return -ENOMEM;
    }
    (*to).flags = TIMER_OF_IRQ | TIMER_OF_CLOCK | TIMER_OF_BASE;
    (*to).of_irq.handler = Some(msc313e_timer_clkevt_irq);
    ret = timer_of_init(np, to);
    if ret != 0 { return ret; }
    if of_device_is_compatible(np, c_str!("sstar,ssd20xd-timer")) {
        (*to).of_clk.rate = clk_get_rate((*to).of_clk.clk) / MSC313E_CLK_DIVIDER as u64;
        (*to).of_clk.period = div_round_up((*to).of_clk.rate, HZ);
        writew((MSC313E_CLK_DIVIDER - 1) as u16, timer_of_base(to).add(MSC313E_REG_TIMER_DIVIDE));
    }
    MSC313E_CLKEVT.cpumask = cpu_possible_mask;
    MSC313E_CLKEVT.irq = (*to).of_irq.irq;
    (*to).clkevt = MSC313E_CLKEVT;
    clockevents_config_and_register(&mut (*to).clkevt, timer_of_rate(to), TIMER_SYNC_TICKS, 0xffffffff);
    0
}

unsafe extern "C" fn msc313e_clksrc_init(np: *mut device_node) -> i32 {
    let mut to: timer_of = core::mem::zeroed();
    let ret: i32;
    to.flags = TIMER_OF_BASE | TIMER_OF_CLOCK;
    ret = timer_of_init(np, &mut to);
    if ret != 0 { return ret; }
    MSC313E_CLKSRC = timer_of_base(&mut to);
    let mut reg = readw(MSC313E_CLKSRC.add(MSC313E_REG_CTRL));
    reg |= MSC313E_REG_CTRL_TIMER_EN;
    writew(reg, MSC313E_CLKSRC.add(MSC313E_REG_CTRL));
    #[cfg(CONFIG_ARM)]
    {
        MSC313E_DELAY.base = timer_of_base(&mut to);
        MSC313E_DELAY.delay.read_current_timer = Some(msc313e_read_delay_timer_read);
        MSC313E_DELAY.delay.freq = timer_of_rate(&mut to);
        register_current_timer_delay(&mut MSC313E_DELAY.delay);
    }
    sched_clock_register(Some(msc313e_timer_sched_clock_read), 32, timer_of_rate(&mut to));
    clocksource_mmio_init(timer_of_base(&mut to), TIMER_NAME.as_ptr() as *const i8, timer_of_rate(&mut to), 300, 32, Some(msc313e_timer_clksrc_read))
}

unsafe extern "C" fn msc313e_timer_init(np: *mut device_node) -> i32 {
    static mut NUM_CALLED: i32 = 0;
    let ret = if NUM_CALLED == 0 { msc313e_clksrc_init(np) } else { msc313e_clkevt_init(np) };
    if ret != 0 { return ret; }
    NUM_CALLED += 1;
    0
}

TIMER_OF_DECLARE!(msc313, "mstar,msc313e-timer", msc313e_timer_init);
TIMER_OF_DECLARE!(ssd20xd, "sstar,ssd20xd-timer", msc313e_timer_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
