// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2006-2007 PA Semi, Inc
 *
 * Maintained by: Olof Johansson <olof@lixom.net>
 */

// C dependencies: linux/kernel.h, linux/string.h, linux/irq.h,
// asm/machdep.h, asm/reg.h, asm/smp.h, and pasemi.h.

use core::ffi::{c_char, c_int};

#[repr(C)]
pub struct pt_regs {
    pub msr: usize,
    pub link: usize,
}

#[repr(C)]
struct SleepMode {
    name: *mut c_char,
    entry: Option<unsafe extern "C" fn()>,
}

unsafe extern "C" {
    fn idle_spin();
    fn idle_doze();
    fn regs_set_return_ip(regs: *mut pt_regs, ip: usize);
    fn set_dec(value: c_int);
    fn restore_astate(cpu: c_int);
    fn hard_smp_processor_id() -> c_int;
    fn regs_set_recoverable(regs: *mut pt_regs);
    fn strcmp(left: *const c_char, right: *const c_char) -> c_int;
}

// Supplied by asm/machdep.h.
#[repr(C)]
struct PpcMd {
    system_reset_exception: Option<unsafe extern "C" fn(*mut pt_regs) -> c_int>,
    power_save: Option<unsafe extern "C" fn()>,
}

unsafe extern "C" {
    static mut ppc_md: PpcMd;
}

static mut MODES: [SleepMode; 2] = [
    SleepMode {
        name: b"spin\0" as *const u8 as *mut c_char,
        entry: Some(idle_spin),
    },
    SleepMode {
        name: b"doze\0" as *const u8 as *mut c_char,
        entry: Some(idle_doze),
    },
];

static mut CURRENT_MODE: c_int = 0;

unsafe extern "C" fn pasemi_system_reset_exception(regs: *mut pt_regs) -> c_int {
    /* If we were woken up from power savings, we need to return
     * to the calling function, since nip is not saved across
     * all modes.
     */
    if ((*regs).msr & SRR1_WAKEMASK) != 0 {
        regs_set_return_ip(regs, (*regs).link);
    }

    match (*regs).msr & SRR1_WAKEMASK {
        SRR1_WAKEDEC => {
            set_dec(1);
        }
        SRR1_WAKEEE => {
            /*
             * Handle these when interrupts get re-enabled and we take
             * them as regular exceptions. We are in an NMI context
             * and can't handle these here.
             */
        }
        _ => {
            /* do system reset */
            return 0;
        }
    }

    /* Set higher astate since we come out of power savings at 0 */
    restore_astate(hard_smp_processor_id());

    /* everything handled */
    regs_set_recoverable(regs);
    1
}

unsafe extern "C" fn pasemi_idle_init() -> c_int {
    // #ifndef CONFIG_PPC_PASEMI_CPUFREQ
    // pr_warn("No cpufreq driver, powersavings modes disabled\\n");
    // current_mode = 0;
    // #endif

    ppc_md.system_reset_exception = Some(pasemi_system_reset_exception);
    ppc_md.power_save = MODES[CURRENT_MODE as usize].entry;
    // pr_info("Using PA6T idle loop (%s)\\n", modes[current_mode].name);

    0
}

// machine_late_initcall(pasemi, pasemi_idle_init);

unsafe extern "C" fn idle_param(p: *mut c_char) -> c_int {
    let mut i: usize = 0;
    while i < MODES.len() {
        if strcmp(MODES[i].name, p) == 0 {
            CURRENT_MODE = i as c_int;
            break;
        }
        i += 1;
    }
    0
}

// early_param("idle", idle_param);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
