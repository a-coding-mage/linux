// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * loongson-specific suspend support
 *
 *  Copyright (C) 2009 Lemote Inc.
 *  Author: Wu Zhangjin <wuzhangjin@gmail.com>
 */

// Declarations supplied by the Linux and Loongson dependencies.
extern "C" {
    fn loongson_lefi_sleep(sleep_addr: ::core::ffi::c_ulong);
}

unsafe fn lefi_pm_enter(state: suspend_state_t) -> ::core::ffi::c_int {
    match state {
        PM_SUSPEND_MEM => {
            pm_set_suspend_via_firmware();
            loongson_lefi_sleep(loongson_sysconf.suspend_addr);
            pm_set_resume_via_firmware();
            0
        }
        _ => -EINVAL,
    }
}

unsafe fn lefi_pm_valid_state(state: suspend_state_t) -> ::core::ffi::c_int {
    match state {
        PM_SUSPEND_MEM => {
            if loongson_sysconf.suspend_addr != 0 {
                1
            } else {
                0
            }
        }
        _ => 0,
    }
}

static lefi_pm_ops: platform_suspend_ops = platform_suspend_ops {
    valid: Some(lefi_pm_valid_state),
    enter: Some(lefi_pm_enter),
};

unsafe fn loongson_pm_init() -> ::core::ffi::c_int {
    if loongson_sysconf.fw_interface == LOONGSON_LEFI {
        suspend_set_ops(&lefi_pm_ops);
    }

    0
}

// arch_initcall(loongson_pm_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
