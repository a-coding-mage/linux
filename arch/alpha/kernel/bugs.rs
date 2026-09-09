// Dependencies supplied by the surrounding kernel sources:
// asm/hwrpb.h, linux/device.h, and linux/cpu.h
//
// The C source is guarded by CONFIG_SYSFS; retain that build-time condition
// here as a Rust configuration condition.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

#[cfg(feature = "CONFIG_SYSFS")]
use core::ffi::{c_char, c_int, c_void};

#[cfg(feature = "CONFIG_SYSFS")]
type ssize_t = isize;

#[cfg(feature = "CONFIG_SYSFS")]
#[repr(C)]
pub struct percpu_struct {
    pub type_: u64,
}

#[cfg(feature = "CONFIG_SYSFS")]
#[repr(C)]
pub struct hwrpb_struct {
    pub processor_offset: u64,
}

#[cfg(feature = "CONFIG_SYSFS")]
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[cfg(feature = "CONFIG_SYSFS")]
#[repr(C)]
pub struct device_attribute {
    _private: [u8; 0],
}

#[cfg(feature = "CONFIG_SYSFS")]
unsafe extern "C" {
    static mut hwrpb: *mut hwrpb_struct;
    fn sprintf(buf: *mut c_char, format: *const c_char, ...) -> c_int;
}

#[cfg(feature = "CONFIG_SYSFS")]
unsafe extern "C" {
    pub static EV6_CPU: u64;
    pub static EV67_CPU: u64;
    pub static EV69_CPU: u64;
}

#[cfg(feature = "CONFIG_SYSFS")]
unsafe fn cpu_is_ev6_or_later() -> c_int {
    let cpu: *mut percpu_struct;
    let cputype: u64;

    cpu = (hwrpb as *mut u8)
        .add((*hwrpb).processor_offset as usize) as *mut percpu_struct;
    cputype = (*cpu).type_ & 0xffff_ffff;
    /* Include all of EV6, EV67, EV68, EV7, EV79 and EV69. */
    ((cputype == EV6_CPU) || ((cputype >= EV67_CPU) && (cputype <= EV69_CPU))) as c_int
}

#[cfg(feature = "CONFIG_SYSFS")]
pub unsafe extern "C" fn cpu_show_meltdown(
    _dev: *mut device,
    _attr: *mut device_attribute,
    buf: *mut c_char,
) -> ssize_t {
    if cpu_is_ev6_or_later() != 0 {
        sprintf(buf, c"Vulnerable\n".as_ptr(),) as ssize_t
    } else {
        sprintf(buf, c"Not affected\n".as_ptr(),) as ssize_t
    }
}

#[cfg(feature = "CONFIG_SYSFS")]
pub unsafe extern "C" fn cpu_show_spectre_v1(
    _dev: *mut device,
    _attr: *mut device_attribute,
    buf: *mut c_char,
) -> ssize_t {
    if cpu_is_ev6_or_later() != 0 {
        sprintf(buf, c"Vulnerable\n".as_ptr(),) as ssize_t
    } else {
        sprintf(buf, c"Not affected\n".as_ptr(),) as ssize_t
    }
}

#[cfg(feature = "CONFIG_SYSFS")]
pub unsafe extern "C" fn cpu_show_spectre_v2(
    _dev: *mut device,
    _attr: *mut device_attribute,
    buf: *mut c_char,
) -> ssize_t {
    if cpu_is_ev6_or_later() != 0 {
        sprintf(buf, c"Vulnerable\n".as_ptr(),) as ssize_t
    } else {
        sprintf(buf, c"Not affected\n".as_ptr(),) as ssize_t
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
