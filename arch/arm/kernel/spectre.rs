// SPDX-License-Identifier: GPL-2.0-only
// Dependencies supplied by the corresponding Linux kernel headers.

use core::ffi::{c_char, c_int, c_void};

pub struct device;
pub struct device_attribute;

extern "C" {
    fn sprintf(buf: *mut c_char, format: *const c_char, ...) -> c_int;

    #[cfg(CONFIG_BPF_SYSCALL)]
    static mut sysctl_unprivileged_bpf_disabled: bool;
}

// CONFIG_BPF_SYSCALL controls this branch at build time in the kernel.
unsafe fn _unprivileged_ebpf_enabled() -> bool {
    #[cfg(CONFIG_BPF_SYSCALL)]
    {
        return !sysctl_unprivileged_bpf_disabled;
    }

    #[cfg(not(CONFIG_BPF_SYSCALL))]
    {
        false
    }
}

pub unsafe extern "C" fn cpu_show_spectre_v1(
    _dev: *mut device,
    _attr: *mut device_attribute,
    buf: *mut c_char,
) -> isize {
    sprintf(
        buf,
        b"Mitigation: __user pointer sanitization\n\0".as_ptr() as *const c_char,
    ) as isize
}

static mut spectre_v2_state: u32 = 0;
static mut spectre_v2_methods: u32 = 0;

pub unsafe extern "C" fn spectre_v2_update_state(state: u32, method: u32) {
    if state > spectre_v2_state {
        spectre_v2_state = state;
    }
    spectre_v2_methods |= method;
}

pub unsafe extern "C" fn cpu_show_spectre_v2(
    _dev: *mut device,
    _attr: *mut device_attribute,
    buf: *mut c_char,
) -> isize {
    let method: *const c_char;

    if spectre_v2_state == SPECTRE_UNAFFECTED {
        return sprintf(buf, b"%s\n\0".as_ptr() as *const c_char,
                       b"Not affected\0".as_ptr() as *const c_char) as isize;
    }

    if spectre_v2_state != SPECTRE_MITIGATED {
        return sprintf(buf, b"%s\n\0".as_ptr() as *const c_char,
                       b"Vulnerable\0".as_ptr() as *const c_char) as isize;
    }

    if _unprivileged_ebpf_enabled() {
        return sprintf(
            buf,
            b"Vulnerable: Unprivileged eBPF enabled\n\0".as_ptr() as *const c_char,
        ) as isize;
    }

    method = match spectre_v2_methods {
        SPECTRE_V2_METHOD_BPIALL => b"Branch predictor hardening\0".as_ptr() as *const c_char,
        SPECTRE_V2_METHOD_ICIALLU => b"I-cache invalidation\0".as_ptr() as *const c_char,
        SPECTRE_V2_METHOD_SMC | SPECTRE_V2_METHOD_HVC => b"Firmware call\0".as_ptr() as *const c_char,
        SPECTRE_V2_METHOD_LOOP8 => b"History overwrite\0".as_ptr() as *const c_char,
        _ => b"Multiple mitigations\0".as_ptr() as *const c_char,
    };

    sprintf(buf, b"Mitigation: %s\n\0".as_ptr() as *const c_char, method) as isize
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
