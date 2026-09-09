// SPDX-License-Identifier: GPL-2.0
/*
 * 8253/PIT functions
 *
 */

// Dependencies supplied by the surrounding kernel translation.

/*
 * HPET replaces the PIT, when enabled. So we need to know, which of
 * the two timers is used
 */
static mut global_clock_event: *mut clock_event_device = core::ptr::null_mut();

/*
 * Modern chipsets can disable the PIT clock which makes it unusable. It
 * would be possible to enable the clock but the registers are chipset
 * specific and not discoverable. Avoid the whack a mole game.
 *
 * These platforms have discoverable TSC/CPU frequencies but this also
 * requires to know the local APIC timer frequency as it normally is
 * calibrated against the PIT interrupt.
 */
#[allow(non_snake_case)]
unsafe fn use_pit() -> bool
{
    if !boot_cpu_has(X86_FEATURE_TSC) {
        return true;
    }

    /* This also returns true when APIC is disabled */
    apic_needs_pit()
}

#[allow(non_snake_case)]
pub unsafe fn pit_timer_init() -> bool
{
    if !use_pit() {
        /*
         * Don't just ignore the PIT. Ensure it's stopped, because
         * VMMs otherwise steal CPU time just to pointlessly waggle
         * the (masked) IRQ.
         */
        // C scoped_guard(irq) is represented by the corresponding
        // interrupt guard in the surrounding kernel translation.
        let _irq_guard = irq_guard();
        clockevent_i8253_disable();
        return false;
    }
    clockevent_i8253_init(true);
    global_clock_event = &raw mut i8253_clockevent;
    true
}

// This block is compiled only when CONFIG_X86_64 is not enabled.
#[cfg(not(target_pointer_width = "64"))]
unsafe fn init_pit_clocksource() -> i32
{
    /*
     * Several reasons not to register PIT as a clocksource:
     *
     * - On SMP PIT does not scale due to i8253_lock
     * - when HPET is enabled
     * - when local APIC timer is active (PIT is switched off)
     */
    if num_possible_cpus() > 1 || is_hpet_enabled() ||
       !clockevent_state_periodic(&i8253_clockevent)
    {
        return 0;
    }

    clocksource_i8253_init()
}

// C: arch_initcall(init_pit_clocksource);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
