// SPDX-License-Identifier: GPL-2.0
/*
 * The idle loop for all SuperH platforms.
 *
 *  Copyright (C) 2002 - 2009  Paul Mundt
 */

// Declarations supplied by the corresponding kernel and architecture dependencies.
extern "C" {
    fn set_bl_bit();
    fn raw_local_irq_enable();
    fn cpu_sleep();
    fn raw_local_irq_disable();
    fn clear_bl_bit();
    fn play_dead() -> !;
    fn local_irq_disable();
    fn set_cpu_online(cpu: i32, online: bool);
    fn smp_processor_id() -> i32;
}

static mut sh_idle: Option<unsafe extern "C" fn()> = None;

pub unsafe extern "C" fn default_idle() {
    set_bl_bit();
    raw_local_irq_enable();
    /* Isn't this racy ? */
    cpu_sleep();
    raw_local_irq_disable();
    clear_bl_bit();
}

pub unsafe extern "C" fn arch_cpu_idle_dead() -> ! {
    play_dead();
}

pub unsafe extern "C" fn arch_cpu_idle() {
    if let Some(idle) = sh_idle {
        idle();
    }
}

pub unsafe extern "C" fn select_idle_routine() {
    /*
     * If a platform has set its own idle routine, leave it alone.
     */
    if sh_idle.is_none() {
        sh_idle = Some(default_idle);
    }
}

pub unsafe extern "C" fn stop_this_cpu(_unused: *mut core::ffi::c_void) -> ! {
    local_irq_disable();
    set_cpu_online(smp_processor_id(), false);

    loop {
        cpu_sleep();
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
