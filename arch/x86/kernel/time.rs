// SPDX-License-Identifier: GPL-2.0
/*
 *  Copyright (c) 1991,1992,1995  Linus Torvalds
 *  Copyright (c) 1994  Alan Modra
 *  Copyright (c) 1995  Markus Kuhn
 *  Copyright (c) 1996  Ingo Molnar
 *  Copyright (c) 1998  Andrea Arcangeli
 *  Copyright (c) 2002,2006  Vojtech Pavlik
 *  Copyright (c) 2003  Andi Kleen
 */

// Linux and x86 dependencies are supplied by the surrounding translation.

extern "C" {
    fn instruction_pointer(regs: *mut pt_regs) -> c_ulong;
    fn request_irq(
        irq: c_uint,
        handler: Option<unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t>,
        flags: c_ulong,
        name: *const c_char,
        dev_id: *mut c_void,
    ) -> c_int;
    fn hpet_enable() -> c_int;
    fn pit_timer_init() -> c_int;
    fn tsc_init();
    fn use_tpause_delay();
    fn cpu_feature_enabled(feature: c_int) -> bool;
}

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clock_event_device {
    pub event_handler: Option<unsafe extern "C" fn(*mut clock_event_device)>,
}

#[repr(C)]
pub struct clocksource {
    pub vdso_clock_mode: c_int,
    pub mask: u64,
    pub name: *const c_char,
}

#[repr(C)]
pub struct irq_ops {
    pub intr_mode_select: unsafe extern "C" fn(),
    pub intr_mode_init: unsafe extern "C" fn(),
}

#[repr(C)]
pub struct timer_ops {
    pub timer_init: unsafe extern "C" fn(),
}

#[repr(C)]
pub struct x86_init_struct {
    pub irqs: irq_ops,
    pub timers: timer_ops,
}

#[allow(non_upper_case_globals)]
extern "C" {
    static mut global_clock_event: *mut clock_event_device;
    static mut x86_init: x86_init_struct;
    static mut late_time_init: Option<unsafe extern "C" fn()>;
}

type c_int = i32;
type c_uint = u32;
type c_ulong = usize;
type c_char = i8;
type c_void = core::ffi::c_void;
type irqreturn_t = c_int;

const IRQ_HANDLED: irqreturn_t = 1;
const IRQF_NOBALANCING: c_ulong = 0x0000_0008;
const IRQF_IRQPOLL: c_ulong = 0x0000_1000;
const IRQF_TIMER: c_ulong = 0x0000_0080;
const VDSO_CLOCKMODE_NONE: c_int = 0;
const X86_FEATURE_WAITPKG: c_int = 0;

#[inline]
const fn clocksource_mask(bits: u32) -> u64 {
    if bits == 64 { u64::MAX } else { (1u64 << bits) - 1 }
}

#[no_mangle]
pub unsafe extern "C" fn profile_pc(regs: *mut pt_regs) -> c_ulong {
    instruction_pointer(regs)
}

/* Default timer interrupt handler for PIT/HPET */
unsafe extern "C" fn timer_interrupt(_irq: c_int, _dev_id: *mut c_void) -> irqreturn_t {
    let event = global_clock_event;
    if let Some(handler) = (*event).event_handler {
        handler(event);
    }
    IRQ_HANDLED
}

unsafe fn setup_default_timer_irq() {
    let flags: c_ulong = IRQF_NOBALANCING | IRQF_IRQPOLL | IRQF_TIMER;

    /* Unconditionally register the legacy timer interrupt; even
     * without legacy PIC/PIT we need this for the HPET0 in legacy
     * replacement mode. */
    let name = b"timer\0";
    if request_irq(0, Some(timer_interrupt), flags, name.as_ptr() as *const c_char, core::ptr::null_mut()) != 0 {
        // pr_info("Failed to register legacy timer interrupt\n");
    }
}

/* Default timer init function */
pub unsafe extern "C" fn hpet_time_init() {
    if hpet_enable() == 0 {
        if pit_timer_init() == 0 {
            return;
        }
    }

    setup_default_timer_irq();
}

unsafe extern "C" fn x86_late_time_init() {
    /* Before PIT/HPET init, select the interrupt mode. */
    (x86_init.irqs.intr_mode_select)();

    /* Setup the legacy timers */
    (x86_init.timers.timer_init)();

    /* After PIT/HPET timers init, set up the final interrupt mode. */
    (x86_init.irqs.intr_mode_init)();
    tsc_init();

    if cpu_feature_enabled(X86_FEATURE_WAITPKG) {
        use_tpause_delay();
    }
}

/* Initialize TSC and delay the periodic timer init to late x86_late_time_init(). */
pub unsafe extern "C" fn time_init() {
    late_time_init = Some(x86_late_time_init);
}

/* Sanity check the vdso related archdata content. */
pub unsafe extern "C" fn clocksource_arch_init(cs: *mut clocksource) {
    if (*cs).vdso_clock_mode == VDSO_CLOCKMODE_NONE {
        return;
    }

    if (*cs).mask != clocksource_mask(64) {
        // pr_warn("clocksource registered with invalid mask for VDSO. Disabling VDSO support.\n");
        (*cs).vdso_clock_mode = VDSO_CLOCKMODE_NONE;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
