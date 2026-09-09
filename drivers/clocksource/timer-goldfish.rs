// SPDX-License-Identifier: GPL-2.0

// Dependencies supplied by the Linux kernel and the goldfish timer headers.

#[repr(C)]
pub struct goldfish_timer {
    pub cs: clocksource,
    pub ced: clock_event_device,
    pub res: resource,
    pub base: *mut core::ffi::c_void,
}

unsafe fn ced_to_gf(ced: *mut clock_event_device) -> *mut goldfish_timer {
    (ced as *mut u8).sub(core::mem::offset_of!(goldfish_timer, ced)) as *mut goldfish_timer
}

unsafe fn cs_to_gf(cs: *mut clocksource) -> *mut goldfish_timer {
    (cs as *mut u8).sub(core::mem::offset_of!(goldfish_timer, cs)) as *mut goldfish_timer
}

unsafe fn goldfish_timer_read(cs: *mut clocksource) -> u64 {
    let timerdrv = &*cs_to_gf(cs);
    let base = timerdrv.base as *mut u8;
    let time_low: u32;
    let time_high: u32;
    let ticks: u64;

    /*
     * time_low: get low bits of current time and update time_high
     * time_high: get high bits of time at last time_low read
     */
    time_low = gf_ioread32(base.add(TIMER_TIME_LOW) as *mut core::ffi::c_void);
    time_high = gf_ioread32(base.add(TIMER_TIME_HIGH) as *mut core::ffi::c_void);

    ticks = ((time_high as u64) << 32) | time_low as u64;

    ticks
}

unsafe fn goldfish_timer_set_oneshot(evt: *mut clock_event_device) -> i32 {
    let timerdrv = &*ced_to_gf(evt);
    let base = timerdrv.base as *mut u8;

    gf_iowrite32(0, base.add(TIMER_ALARM_HIGH) as *mut core::ffi::c_void);
    gf_iowrite32(0, base.add(TIMER_ALARM_LOW) as *mut core::ffi::c_void);
    gf_iowrite32(1, base.add(TIMER_IRQ_ENABLED) as *mut core::ffi::c_void);

    0
}

unsafe fn goldfish_timer_shutdown(evt: *mut clock_event_device) -> i32 {
    let timerdrv = &*ced_to_gf(evt);
    let base = timerdrv.base as *mut u8;

    gf_iowrite32(0, base.add(TIMER_IRQ_ENABLED) as *mut core::ffi::c_void);

    0
}

unsafe fn goldfish_timer_next_event(delta: usize, evt: *mut clock_event_device) -> i32 {
    let timerdrv = &*ced_to_gf(evt);
    let base = timerdrv.base as *mut u8;
    let mut now: u64;

    now = goldfish_timer_read(&timerdrv.cs);
    now = now.wrapping_add(delta as u64);

    gf_iowrite32((now >> 32) as u32, base.add(TIMER_ALARM_HIGH) as *mut core::ffi::c_void);
    gf_iowrite32(now as u32, base.add(TIMER_ALARM_LOW) as *mut core::ffi::c_void);

    0
}

unsafe fn goldfish_timer_irq(_irq: i32, dev_id: *mut core::ffi::c_void) -> irqreturn_t {
    let timerdrv = &mut *(dev_id as *mut goldfish_timer);
    let evt = &mut timerdrv.ced;
    let base = timerdrv.base as *mut u8;

    gf_iowrite32(1, base.add(TIMER_CLEAR_INTERRUPT) as *mut core::ffi::c_void);

    (evt.event_handler)(evt);

    IRQ_HANDLED
}

pub unsafe fn goldfish_timer_init(irq: i32, base: *mut core::ffi::c_void) -> i32 {
    let timerdrv = kzalloc_obj::<goldfish_timer>();
    if timerdrv.is_null() {
        return -ENOMEM;
    }

    (*timerdrv).base = base;

    (*timerdrv).ced = clock_event_device {
        name: "goldfish_timer\\0".as_ptr() as *const i8,
        features: CLOCK_EVT_FEAT_ONESHOT,
        set_state_shutdown: Some(goldfish_timer_shutdown),
        set_state_oneshot: Some(goldfish_timer_set_oneshot),
        set_next_event: Some(goldfish_timer_next_event),
    };

    (*timerdrv).res = resource {
        name: "goldfish_timer\\0".as_ptr() as *const i8,
        start: base as usize,
        end: base as usize + 0xfff,
    };

    let mut ret = request_resource(&iomem_resource, &mut (*timerdrv).res);
    if ret != 0 {
        pr_err!("Cannot allocate '%s' resource\\n", (*timerdrv).res.name);
        return ret;
    }

    (*timerdrv).cs = clocksource {
        name: "goldfish_timer\\0".as_ptr() as *const i8,
        rating: 400,
        read: Some(goldfish_timer_read),
        mask: CLOCKSOURCE_MASK(64),
        flags: 0,
        max_idle_ns: LONG_MAX,
    };

    clocksource_register_hz(&mut (*timerdrv).cs, NSEC_PER_SEC);

    ret = request_irq(irq, Some(goldfish_timer_irq), IRQF_TIMER,
                      "goldfish_timer\\0".as_ptr() as *const i8, timerdrv as *mut core::ffi::c_void);
    if ret != 0 {
        pr_err!("Couldn't register goldfish-timer interrupt\\n");
        return ret;
    }

    clockevents_config_and_register(&mut (*timerdrv).ced, NSEC_PER_SEC, 1, 0xffffffff);

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
