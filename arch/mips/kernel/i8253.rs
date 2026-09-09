// SPDX-License-Identifier: GPL-2.0
/*
 * i8253.c  8253/PIT functions
 *
 */

// Declarations supplied by the kernel headers included by the C source.
extern "C" {
    static mut i8253_clockevent: clock_event_device;

    fn clockevent_i8253_init(force: bool);
    fn request_irq(
        irq: u32,
        handler: unsafe extern "C" fn(i32, *mut core::ffi::c_void) -> irqreturn_t,
        flags: usize,
        name: *const core::ffi::c_char,
        dev_id: *mut core::ffi::c_void,
    ) -> i32;
    fn num_possible_cpus() -> i32;
    fn clockevent_state_periodic(dev: *const clock_event_device) -> bool;
    fn clocksource_i8253_init() -> i32;
}

unsafe extern "C" fn timer_interrupt(
    _irq: i32,
    _dev_id: *mut core::ffi::c_void,
) -> irqreturn_t {
    unsafe {
        ((*(&raw mut i8253_clockevent)).event_handler)(&raw mut i8253_clockevent);
    }

    IRQ_HANDLED
}

pub unsafe fn setup_pit_timer() {
    let flags: usize = IRQF_NOBALANCING | IRQF_TIMER;

    unsafe {
        clockevent_i8253_init(true);
        if request_irq(
            0,
            timer_interrupt,
            flags,
            c"timer".as_ptr(),
            core::ptr::null_mut(),
        ) != 0
        {
            pr_err!("Failed to request irq 0 (timer)\n");
        }
    }
}

unsafe fn init_pit_clocksource() -> i32 {
    if unsafe { num_possible_cpus() } > 1
        || !unsafe { clockevent_state_periodic(&raw const i8253_clockevent) }
    {
        return 0;
    }

    unsafe { clocksource_i8253_init() }
}

// arch_initcall(init_pit_clocksource);


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
