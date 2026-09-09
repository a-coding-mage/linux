// SPDX-License-Identifier: GPL-2.0-only
/*
 * linux/arch/arm/mach-spear13xx/hotplug.c
 *
 * Copyright (C) 2012 ST Microelectronics Ltd.
 * Deepak Sikri <deepak.sikri@st.com>
 *
 * based upon linux/arch/arm/mach-realview/hotplug.c
 */

// Dependencies supplied by the surrounding kernel translation.
extern "C" {
    static mut spear_pen_release: ::core::ffi::c_uint;
    fn wfi();
    fn pr_warn(fmt: *const ::core::ffi::c_char, ...);
}

// CR_C is supplied by the ARM CP15 definitions.
const CR_C: u32 = 0;

#[inline]
unsafe fn cpu_enter_lowpower() {
    let mut v: u32;

    core::arch::asm!(
        "mcr p15, 0, {zero}, c7, c5, 0",
        "dsb",
        // Turn off coherency
        "mrc p15, 0, {value}, c1, c0, 1",
        "bic {value}, {value}, #0x20",
        "mcr p15, 0, {value}, c1, c0, 1",
        "mrc p15, 0, {value}, c1, c0, 0",
        "bic {value}, {value}, {cr_c}",
        "mcr p15, 0, {value}, c1, c0, 0",
        zero = in(reg) 0u32,
        value = lateout(reg) v,
        cr_c = in(reg) CR_C,
        options(nostack, preserves_flags),
    );
}

#[inline]
unsafe fn cpu_leave_lowpower() {
    let mut v: u32;

    core::arch::asm!(
        "mrc p15, 0, {value}, c1, c0, 0",
        "orr {value}, {value}, {cr_c}",
        "mcr p15, 0, {value}, c1, c0, 0",
        "mrc p15, 0, {value}, c1, c0, 1",
        "orr {value}, {value}, #0x20",
        "mcr p15, 0, {value}, c1, c0, 1",
        value = lateout(reg) v,
        cr_c = in(reg) CR_C,
        options(nostack, preserves_flags),
    );
}

#[inline]
unsafe fn spear13xx_do_lowpower(cpu: u32, spurious: *mut i32) {
    loop {
        wfi();

        if spear_pen_release == cpu {
            // OK, proper wakeup, we're done
            break;
        }

        // Getting here means that we have come out of WFI without having
        // been woken up - this shouldn't happen. Just note it happening -
        // when we're woken, we can report its occurrence.
        *spurious += 1;
    }
}

/*
 * platform-specific code to shutdown a CPU
 *
 * Called with IRQs disabled
 */
#[no_mangle]
pub unsafe extern "C" fn spear13xx_cpu_die(cpu: u32) {
    let mut spurious: i32 = 0;

    // we're ready for shutdown now, so do it
    cpu_enter_lowpower();
    spear13xx_do_lowpower(cpu, &mut spurious);

    // bring this CPU back into the world of cache coherency, and then
    // restore interrupts
    cpu_leave_lowpower();

    if spurious != 0 {
        // The kernel's pr_warn formatting facility is supplied externally.
        pr_warn(
            b"CPU%u: %u spurious wakeup calls\n\0".as_ptr() as *const _,
            cpu,
            spurious as u32,
        );
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
