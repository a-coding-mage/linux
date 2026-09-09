// SPDX-License-Identifier: GPL-2.0-only
/*
 * ksyms_common.c: A split of kernel/kallsyms.c
 * Contains a few generic function definations independent of config KALLSYMS.
 */

// External kernel dependencies supplied by other translation units.
extern "C" {
    static mut kptr_restrict: i32;
    static mut init_user_ns: UserNamespace;
    fn security_capable(
        cred: *const Cred,
        ns: *const UserNamespace,
        cap: i32,
        opts: u32,
    ) -> i32;
}

#[repr(C)]
pub struct Cred {
    _private: [u8; 0],
}

#[repr(C)]
pub struct UserNamespace {
    _private: [u8; 0],
}

const CAP_SYSLOG: i32 = 34;
const CAP_OPT_NOAUDIT: u32 = 0;

#[inline]
unsafe fn kallsyms_for_perf() -> i32 {
    // CONFIG_PERF_EVENTS controls this declaration and branch at build time.
    #[cfg(CONFIG_PERF_EVENTS)]
    {
        extern "C" {
            static mut sysctl_perf_event_paranoid: i32;
        }

        if sysctl_perf_event_paranoid <= 1 {
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
pub unsafe fn kallsyms_show_value(cred: *const Cred) -> bool {
    match kptr_restrict {
        0 => {
            if kallsyms_for_perf() != 0 {
                return true;
            }
            // fallthrough
            if security_capable(
                cred,
                &init_user_ns as *const UserNamespace,
                CAP_SYSLOG,
                CAP_OPT_NOAUDIT,
            ) == 0
            {
                return true;
            }
            false
        }
        1 => {
            if security_capable(
                cred,
                &init_user_ns as *const UserNamespace,
                CAP_SYSLOG,
                CAP_OPT_NOAUDIT,
            ) == 0
            {
                return true;
            }
            false
        }
        _ => false,
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
