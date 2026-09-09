// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2007 Atmel Corporation
 */

use core::ffi::{c_char, c_int, c_ulong, c_void};

// Linux kernel headers supplying these declarations are external dependencies.
#[repr(C)]
pub struct notifier_block {
    pub notifier_call:
        Option<unsafe extern "C" fn(*mut notifier_block, c_ulong, *mut c_void) -> c_int>,
}

#[repr(C)]
pub struct die_args {
    pub regs: *mut c_void,
}

unsafe extern "C" {
    fn register_die_notifier(nb: *mut notifier_block);
    fn show_state();
    fn show_regs(regs: *mut c_void);
    fn mdelay(ms: c_ulong);
    fn printk(fmt: *const c_char, ...);
    fn strchr(s: *mut c_char, c: c_int) -> *mut c_char;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
}

const NMI_SHOW_STATE: c_ulong = 1 << 0;
const NMI_SHOW_REGS: c_ulong = 1 << 1;
const NMI_DIE: c_ulong = 1 << 2;
const NMI_DEBOUNCE: c_ulong = 1 << 3;

const DIE_NMI: c_ulong = 0;
const NOTIFY_DONE: c_int = 0;
const NOTIFY_OK: c_int = 1;
const NOTIFY_BAD: c_int = 0x8002;

static mut nmi_actions: c_ulong = 0;

unsafe extern "C" fn nmi_debug_notify(
    _self: *mut notifier_block,
    val: c_ulong,
    data: *mut c_void,
) -> c_int {
    let args = data as *mut die_args;

    if val != DIE_NMI {
        return NOTIFY_DONE;
    }

    if nmi_actions & NMI_SHOW_STATE != 0 {
        show_state();
    }
    if nmi_actions & NMI_SHOW_REGS != 0 {
        show_regs((*args).regs);
    }
    if nmi_actions & NMI_DEBOUNCE != 0 {
        mdelay(10);
    }
    if nmi_actions & NMI_DIE != 0 {
        return NOTIFY_BAD;
    }

    NOTIFY_OK
}

static mut nmi_debug_nb: notifier_block = notifier_block {
    notifier_call: Some(nmi_debug_notify),
};

unsafe extern "C" fn nmi_debug_setup(str_: *mut c_char) -> c_int {
    let mut p: *mut c_char;
    let mut sep: *mut c_char;

    register_die_notifier(&raw mut nmi_debug_nb);

    if *str_ != b'=' as c_char {
        return 1;
    }

    p = str_.add(1);
    while *p != 0 {
        sep = strchr(p, b',' as c_int);
        if !sep.is_null() {
            *sep = 0;
        }
        if strcmp(p, c"state".as_ptr()) == 0 {
            nmi_actions |= NMI_SHOW_STATE;
        } else if strcmp(p, c"regs".as_ptr()) == 0 {
            nmi_actions |= NMI_SHOW_REGS;
        } else if strcmp(p, c"debounce".as_ptr()) == 0 {
            nmi_actions |= NMI_DEBOUNCE;
        } else if strcmp(p, c"die".as_ptr()) == 0 {
            nmi_actions |= NMI_DIE;
        } else {
            printk(c"NMI: Unrecognized action `%s'\n".as_ptr(), p);
        }
        if sep.is_null() {
            break;
        }
        p = sep.add(1);
    }

    1
}

// __setup("nmi_debug", nmi_debug_setup);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
