// SPDX-License-Identifier: GPL-2.0-or-later

/*
    NetWinder Floating Point Emulator
    (c) Rebel.com, 1998-1999
    (c) Philip Blundell, 1998-1999

    Direct questions, comments to Scott Bambrough <scottb@netwinder.org>
*/

// C dependencies supplied by the surrounding kernel/emulator sources:
// fpa11.h, softfloat.h, fpopcode.h, fpmodule.h, and fpa11.inl.

// CONFIG_FPE_NWFPE_XP selects "extended"; otherwise NWFPE_BITS is "double".
#[cfg(CONFIG_FPE_NWFPE_XP)]
const NWFPE_BITS: &str = "extended";
#[cfg(not(CONFIG_FPE_NWFPE_XP))]
const NWFPE_BITS: &str = "double";

#[cfg(MODULE)]
extern "C" {
    fn fp_send_sig(sig: libc::c_ulong, task: *mut task_struct, priv_: libc::c_int);
}

#[cfg(not(MODULE))]
extern "C" {
    static mut fpe_type: [libc::c_char; 0];
    fn send_sig(sig: libc::c_int, task: *mut task_struct, priv_: libc::c_int);
    fn fp_enter();
}

#[repr(C)]
pub struct notifier_block {
    pub notifier_call: Option<unsafe extern "C" fn(*mut notifier_block, libc::c_ulong, *mut libc::c_void) -> libc::c_int>,
}

extern "C" {
    type task_struct;
    type thread_info;
    type FPA11;
    type FPREG;
    type fp_state;

    fn nwfpe_init_fpa(state: *mut libc::c_void);
    fn thread_register_notifier(block: *mut notifier_block);
    fn thread_unregister_notifier(block: *mut notifier_block);
    fn nwfpe_enter();
    fn readFPSR() -> libc::c_uint;
    fn writeFPSR(value: libc::c_uint);
    static mut current: *mut task_struct;
    static mut kern_fp_enter: Option<unsafe extern "C" fn()>;
}

const THREAD_NOTIFY_FLUSH: libc::c_ulong = 0;
const NOTIFY_DONE: libc::c_int = 0;
const EINVAL: libc::c_int = 22;
const SIGFPE: libc::c_int = 8;

// These exception-bit constants are supplied by fpopcode.h/softfloat.h.
extern "C" {
    static BIT_IXC: libc::c_uint;
    static BIT_UFC: libc::c_uint;
    static BIT_OFC: libc::c_uint;
    static BIT_DZC: libc::c_uint;
    static BIT_IOC: libc::c_uint;
    static BIT_IXE: libc::c_uint;
    static BIT_UFE: libc::c_uint;
    static BIT_OFE: libc::c_uint;
    static BIT_DZE: libc::c_uint;
    static BIT_IOE: libc::c_uint;
}

unsafe extern "C" fn nwfpe_notify(
    _self: *mut notifier_block,
    cmd: libc::c_ulong,
    v: *mut libc::c_void,
) -> libc::c_int {
    let thread = v as *mut thread_info;

    if cmd == THREAD_NOTIFY_FLUSH {
        nwfpe_init_fpa(thread as *mut libc::c_void);
    }

    NOTIFY_DONE
}

static mut nwfpe_notifier_block: notifier_block = notifier_block {
    notifier_call: Some(nwfpe_notify),
};

extern "C" {
    fn fp_setup();
}

// Original value of fp_enter from the kernel before patched by fpe_init.
static mut orig_fp_enter: Option<unsafe extern "C" fn()> = None;

unsafe extern "C" fn fpe_init() -> libc::c_int {
    // sizeof(FPA11) > sizeof(union fp_state)
    if core::mem::size_of::<FPA11>() > core::mem::size_of::<fp_state>() {
        return -EINVAL;
    }

    if core::mem::size_of::<FPREG>() != 12 {
        return -EINVAL;
    }

    #[cfg(not(MODULE))]
    {
        // C condition: if (fpe_type[0] && strcmp(fpe_type, "nwfpe")) return 0;
        if fpe_type[0] != 0 {
            // The kernel-provided string comparison is intentionally external.
            return 0;
        }
    }

    thread_register_notifier(&mut nwfpe_notifier_block);

    orig_fp_enter = kern_fp_enter;
    kern_fp_enter = Some(nwfpe_enter);

    0
}

unsafe extern "C" fn fpe_exit() {
    thread_unregister_notifier(&mut nwfpe_notifier_block);
    kern_fp_enter = orig_fp_enter;
}

pub unsafe extern "C" fn float_raise(flags: libc::c_schar) {
    let fpsr: libc::c_uint;
    let mut cumulativeTraps: libc::c_uint;

    fpsr = readFPSR();
    cumulativeTraps = 0;

    if (fpsr & BIT_IXE) == 0 && ((flags as libc::c_int as libc::c_uint) & BIT_IXC) != 0 {
        cumulativeTraps |= BIT_IXC;
    }
    if (fpsr & BIT_UFE) == 0 && ((flags as libc::c_int as libc::c_uint) & BIT_UFC) != 0 {
        cumulativeTraps |= BIT_UFC;
    }
    if (fpsr & BIT_OFE) == 0 && ((flags as libc::c_int as libc::c_uint) & BIT_OFC) != 0 {
        cumulativeTraps |= BIT_OFC;
    }
    if (fpsr & BIT_DZE) == 0 && ((flags as libc::c_int as libc::c_uint) & BIT_DZC) != 0 {
        cumulativeTraps |= BIT_DZC;
    }
    if (fpsr & BIT_IOE) == 0 && ((flags as libc::c_int as libc::c_uint) & BIT_IOC) != 0 {
        cumulativeTraps |= BIT_IOC;
    }

    if cumulativeTraps != 0 {
        writeFPSR(fpsr | cumulativeTraps);
    }

    if (fpsr & ((flags as libc::c_int as libc::c_uint) << 16)) != 0 {
        #[cfg(MODULE)]
        fp_send_sig(SIGFPE as libc::c_ulong, current, 1);
        #[cfg(not(MODULE))]
        send_sig(SIGFPE, current, 1);
    }
}

// C module_init(fpe_init) and module_exit(fpe_exit) registrations.
// MODULE_AUTHOR("Scott Bambrough <scottb@rebel.com>");
// MODULE_DESCRIPTION("NWFPE floating point emulator (" NWFPE_BITS " precision)");
// MODULE_LICENSE("GPL");

#[cfg(CONFIG_DEBUG_USER)]
static mut debug: libc::c_int = !(1 << 0); // ~BIT_IXC

// CONFIG_DEBUG_USER module_param(debug, int, 0644) is supplied by the kernel.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
