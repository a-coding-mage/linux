// SPDX-License-Identifier: GPL-2.0-only
/*
 * ksyms_common.c: A split of kernel/kallsyms.c
 * Contains a few generic function definations independent of config KALLSYMS.
 */

// Dependencies: linux/kallsyms.h, linux/security.h.

use core::ptr::{addr_of, read_volatile};
use kernel::bindings::{cred, kptr_restrict};
use kernel::ffi::c_int;

// `struct cred` embeds zero-sized tracking types; only pointers cross here.
#[allow(improper_ctypes)]
extern "C" {
    #[cfg(CONFIG_PERF_EVENTS)]
    static sysctl_perf_event_paranoid: c_int;
    fn lupos_ksyms_capable_syslog_noaudit(cred: *const cred) -> bool;
}

fn kallsyms_for_perf() -> c_int {
    #[cfg(CONFIG_PERF_EVENTS)]
    {
        // SAFETY: a plain read of the always-valid sysctl integer.
        if unsafe { read_volatile(addr_of!(sysctl_perf_event_paranoid)) } <= 1 {
            return 1;
        }
    }
    0
}

/*
 * We show kallsyms information even to normal users if we've enabled
 * kernel profiling and are explicitly not paranoid (so kptr_restrict
 * is clear, and sysctl_perf_event_paranoid isn't set).
 *
 * Otherwise, require CAP_SYSLOG (assuming kptr_restrict isn't set to
 * block even that).
 */
/// Returns whether `cred` may see kallsyms addresses.
///
/// # Safety
/// `cred` points to valid credentials.
#[no_mangle]
pub unsafe extern "C" fn kallsyms_show_value(cred: *const cred) -> bool {
    // SAFETY: kptr_restrict is an always-valid sysctl integer; the
    // capability check is the native security_capable() call.
    unsafe {
        match read_volatile(addr_of!(kptr_restrict)) {
            0 => {
                if kallsyms_for_perf() != 0 {
                    return true;
                }
                // fallthrough
                lupos_ksyms_capable_syslog_noaudit(cred)
            }
            1 => lupos_ksyms_capable_syslog_noaudit(cred),
            _ => false,
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
