// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Idle daemon for PowerPC.  Idle daemon will handle any action
 * that needs to be taken when the system becomes idle.
 *
 * Originally written by Cort Dougan (cort@cs.nmt.edu).
 * Subsequent 32-bit hacking by Tom Rini, Armin Kuster,
 * Paul Mackerras and others.
 *
 * iSeries supported added by Mike Corrigan <mikejc@us.ibm.com>
 *
 * Additional shared processor, SMT, and firmware support
 *    Copyright (c) 2003 Dave Engebretsen <engebret@us.ibm.com>
 *
 * 32-bit and 64-bit versions merged by Paul Mackerras <paulus@samba.org>
 */

// Dependencies supplied by the surrounding kernel translation.

extern "C" {
    static mut ppc_md: PowerpcMachineDesc;

    fn irqs_disabled() -> bool;
    fn raw_local_irq_disable();
    fn cpu_has_feature(feature: u64) -> bool;
    fn prep_irq_for_idle() -> bool;
    fn power4_idle_nap();
    fn register_sysctl(name: *const ::core::ffi::c_char,
                        table: *const CtlTable) -> *mut CtlTable;
    fn proc_dointvec();
}

#[no_mangle]
pub static mut cpuidle_disable: ::core::ffi::c_ulong = IDLE_NO_OVERRIDE;

#[repr(C)]
struct PowerpcMachineDesc {
    power_save: Option<unsafe extern "C" fn()>,
}

#[repr(C)]
struct CtlTable {
    procname: *const ::core::ffi::c_char,
    data: *mut ::core::ffi::c_void,
    maxlen: ::core::ffi::c_ulong,
    mode: ::core::ffi::c_uint,
    proc_handler: Option<unsafe extern "C" fn()>,
}

// These constants and platform helpers are provided by the kernel headers.
const IDLE_NO_OVERRIDE: ::core::ffi::c_ulong = 0;
const IDLE_POWERSAVE_OFF: ::core::ffi::c_ulong = 1;
const CPU_FTR_CAN_NAP: u64 = 0;
const CPU_FTR_ALTIVEC: u64 = 0;

extern "C" {
    fn ppc64_runlatch_off();
    fn ppc64_runlatch_on();
    fn HMT_low();
    fn HMT_very_low();
    fn HMT_medium();
}

#[no_mangle]
pub unsafe extern "C" fn powersave_off(_arg: *mut ::core::ffi::c_char) -> i32 {
    ppc_md.power_save = None;
    cpuidle_disable = IDLE_POWERSAVE_OFF;
    1
}

#[no_mangle]
pub unsafe extern "C" fn arch_cpu_idle() {
    ppc64_runlatch_off();

    if let Some(power_save) = ppc_md.power_save {
        power_save();
        // Some power_save functions return with interrupts enabled, some don't.
        if !irqs_disabled() {
            raw_local_irq_disable();
        }
    } else {
        // Go into low thread priority and possibly low power mode.
        HMT_low();
        HMT_very_low();
    }

    HMT_medium();
    ppc64_runlatch_on();
}

#[no_mangle]
pub static mut powersave_nap: i32 = 0;

#[cfg(CONFIG_PPC_970_NAP)]
#[no_mangle]
pub unsafe extern "C" fn power4_idle() {
    if !cpu_has_feature(CPU_FTR_CAN_NAP) {
        return;
    }

    if powersave_nap == 0 {
        return;
    }

    if !prep_irq_for_idle() {
        return;
    }

    if cpu_has_feature(CPU_FTR_ALTIVEC) {
        // PPC_DSSALL " ; sync" with a memory clobber.
        ::core::arch::asm!("dssall; sync", options(nostack));
    }

    power4_idle_nap();

    // power4_idle_nap returns with interrupts enabled (soft and hard).
    // Our caller can cope with either interrupts disabled or enabled upon return.
}

#[cfg(CONFIG_SYSCTL)]
static powersave_nap_ctl_table: [CtlTable; 1] = [CtlTable {
    procname: b"powersave-nap\0".as_ptr() as *const ::core::ffi::c_char,
    data: unsafe { &raw mut powersave_nap as *mut ::core::ffi::c_void },
    maxlen: ::core::mem::size_of::<i32>() as ::core::ffi::c_ulong,
    mode: 0o644,
    proc_handler: Some(proc_dointvec),
}];

#[cfg(CONFIG_SYSCTL)]
unsafe extern "C" fn register_powersave_nap_sysctl() -> i32 {
    register_sysctl(b"kernel\0".as_ptr() as *const ::core::ffi::c_char,
                    powersave_nap_ctl_table.as_ptr());
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
