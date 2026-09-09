// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2024 Rivos Inc.
 */

use core::ffi::{c_char, c_int, c_void};

// Declarations supplied by the kernel and architecture dependencies.
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_attribute {
    _private: [u8; 0],
}

extern "C" {
    fn cpu_mitigations_off() -> c_int;
    fn disable_xtheadvector();
    fn sysfs_emit(buf: *mut c_char, fmt: *const c_char, ...) -> isize;
}

// `mitigation_state`, `UNAFFECTED`, `MITIGATED`, and `VULNERABLE` are defined
// by the architecture dependencies. The build-time CONFIG condition is kept
// as a Cargo feature check representing CONFIG_RISCV_ISA_XTHEADVECTOR.
extern "C" {
    type mitigation_state;
}

static mut ghostwrite_state: mitigation_state = UNAFFECTED;

pub unsafe fn ghostwrite_set_vulnerable() {
    ghostwrite_state = VULNERABLE;
}

/*
 * Vendor extension alternatives will use the value set at the time of boot
 * alternative patching, thus this must be called before boot alternatives are
 * patched (and after extension probing) to be effective.
 *
 * Returns true if mitgated, false otherwise.
 */
pub unsafe fn ghostwrite_enable_mitigation() -> bool {
    if cfg!(feature = "CONFIG_RISCV_ISA_XTHEADVECTOR")
        && ghostwrite_state == VULNERABLE
        && cpu_mitigations_off() == 0
    {
        disable_xtheadvector();
        ghostwrite_state = MITIGATED;
        return true;
    }

    false
}

pub unsafe fn ghostwrite_get_state() -> mitigation_state {
    ghostwrite_state
}

pub unsafe fn cpu_show_ghostwrite(
    _dev: *mut device,
    _attr: *mut device_attribute,
    buf: *mut c_char,
) -> isize {
    if cfg!(feature = "CONFIG_RISCV_ISA_XTHEADVECTOR") {
        match ghostwrite_state {
            UNAFFECTED => sysfs_emit(buf, b"Not affected\0".as_ptr() as *const c_char),
            MITIGATED => sysfs_emit(
                buf,
                b"Mitigation: xtheadvector disabled\0".as_ptr() as *const c_char,
            ),
            VULNERABLE => sysfs_emit(buf, b"Vulnerable\0".as_ptr() as *const c_char),
            _ => sysfs_emit(buf, b"Vulnerable\0".as_ptr() as *const c_char),
        }
    } else {
        sysfs_emit(buf, b"Not affected\0".as_ptr() as *const c_char)
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
