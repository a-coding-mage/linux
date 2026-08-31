// SPDX-License-Identifier: GPL-2.0
//
// Translated from power/cpupower/utils/helpers/cpuid.c.
// C include dependencies: stdio.h, errno.h, string.h, unistd.h, stdlib.h,
// helpers/helpers.h, and on x86 cpuid.h.

use core::ffi::{c_char, c_int, c_uint};
use core::ptr;

const X86_VENDOR_UNKNOWN: c_uint = 0;
const X86_VENDOR_INTEL: c_uint = 1;
const X86_VENDOR_AMD: c_uint = 2;
const X86_VENDOR_HYGON: c_uint = 3;
const X86_VENDOR_MAX: usize = 4;

const EINVAL: c_int = 22;
const EIO: c_int = 5;
const ENODEV: c_int = 19;

// These capability bit definitions are supplied by helpers/helpers.h in C.
// Keep the original names; the translated repository is expected to provide
// their exact Rust definitions.
use crate::{
    CPUPOWER_CAP_AMD_CPB, CPUPOWER_CAP_AMD_CPB_MSR, CPUPOWER_CAP_AMD_HW_PSTATE,
    CPUPOWER_CAP_AMD_PSTATE, CPUPOWER_CAP_AMD_PSTATEDEF, CPUPOWER_CAP_AMD_RDPRU,
    CPUPOWER_CAP_APERF, CPUPOWER_CAP_HAS_TURBO_RATIO, CPUPOWER_CAP_INTEL_IDA,
    CPUPOWER_CAP_INV_TSC, CPUPOWER_CAP_IS_SNB, CPUPOWER_CAP_PERF_BIAS,
};

#[repr(C)]
pub struct cpupower_cpu_info {
    pub vendor: c_uint,
    pub family: c_uint,
    pub model: c_uint,
    pub stepping: c_uint,
    pub caps: u64,
}

unsafe extern "C" {
    static base_cpu: c_int;

    fn cpupower_amd_pstate_enabled() -> c_int;

    fn fopen(path: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fclose(stream: *mut FILE) -> c_int;
    fn feof(stream: *mut FILE) -> c_int;
    fn fgets(s: *mut c_char, size: c_int, stream: *mut FILE) -> *mut c_char;
    fn sscanf(s: *const c_char, format: *const c_char, ...) -> c_int;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
}

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

static CPU_VENDOR_UNKNOWN: &[u8] = b"Unknown\0";
static CPU_VENDOR_INTEL: &[u8] = b"GenuineIntel\0";
static CPU_VENDOR_AMD: &[u8] = b"AuthenticAMD\0";
static CPU_VENDOR_HYGON: &[u8] = b"HygonGenuine\0";

static cpu_vendor_table: [*const c_char; X86_VENDOR_MAX] = [
    CPU_VENDOR_UNKNOWN.as_ptr() as *const c_char,
    CPU_VENDOR_INTEL.as_ptr() as *const c_char,
    CPU_VENDOR_AMD.as_ptr() as *const c_char,
    CPU_VENDOR_HYGON.as_ptr() as *const c_char,
];

// defined(__i386__) || defined(__x86_64__)

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub fn cpuid_eax(op: c_uint) -> c_uint {
    #[cfg(target_arch = "x86")]
    use core::arch::x86::__cpuid;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::__cpuid;

    unsafe { __cpuid(op).eax }
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub fn cpuid_ebx(op: c_uint) -> c_uint {
    #[cfg(target_arch = "x86")]
    use core::arch::x86::__cpuid;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::__cpuid;

    unsafe { __cpuid(op).ebx }
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub fn cpuid_ecx(op: c_uint) -> c_uint {
    #[cfg(target_arch = "x86")]
    use core::arch::x86::__cpuid;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::__cpuid;

    unsafe { __cpuid(op).ecx }
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub fn cpuid_edx(op: c_uint) -> c_uint {
    #[cfg(target_arch = "x86")]
    use core::arch::x86::__cpuid;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::__cpuid;

    unsafe { __cpuid(op).edx }
}

/*
 * get_cpu_info
 *
 * Extract CPU vendor, family, model, stepping info from /proc/cpuinfo
 *
 * Returns 0 on success or a negativ error code
 *
 * TBD: Should there be a cpuid alternative for this if /proc is not mounted?
 */
pub unsafe extern "C" fn get_cpu_info(cpu_info: *mut cpupower_cpu_info) -> c_int {
    let fp: *mut FILE;
    let mut value: [c_char; 64] = [0; 64];
    let mut proc_: c_uint = 0;
    let mut x: c_uint;
    let unknown: c_uint = 0x00ff_ffff;
    let cpuid_level: c_uint;
    let ext_cpuid_level: c_uint;

    let mut ret: c_int = -EINVAL;

    (*cpu_info).vendor = X86_VENDOR_UNKNOWN;
    (*cpu_info).family = unknown;
    (*cpu_info).model = unknown;
    (*cpu_info).stepping = unknown;
    (*cpu_info).caps = 0;

    fp = fopen(b"/proc/cpuinfo\0".as_ptr() as *const c_char, b"r\0".as_ptr() as *const c_char);
    if fp.is_null() {
        return -EIO;
    }

    while feof(fp) == 0 {
        if fgets(value.as_mut_ptr(), 64, fp).is_null() {
            continue;
        }
        value[63 - 1] = b'\0' as c_char;

        if strncmp(
            value.as_ptr(),
            b"processor\t: \0".as_ptr() as *const c_char,
            12,
        ) == 0
        {
            sscanf(
                value.as_ptr(),
                b"processor\t: %u\0".as_ptr() as *const c_char,
                &mut proc_ as *mut c_uint,
            );
        }

        if proc_ != base_cpu as c_uint {
            continue;
        }

        /* Get CPU vendor */
        if strncmp(value.as_ptr(), b"vendor_id\0".as_ptr() as *const c_char, 9) == 0 {
            x = 1;
            while x < X86_VENDOR_MAX as c_uint {
                if !strstr(value.as_ptr(), cpu_vendor_table[x as usize]).is_null() {
                    (*cpu_info).vendor = x;
                }
                x = x.wrapping_add(1);
            }
        /* Get CPU family, etc. */
        } else if strncmp(
            value.as_ptr(),
            b"cpu family\t: \0".as_ptr() as *const c_char,
            13,
        ) == 0
        {
            sscanf(
                value.as_ptr(),
                b"cpu family\t: %u\0".as_ptr() as *const c_char,
                &mut (*cpu_info).family as *mut c_uint,
            );
        } else if strncmp(value.as_ptr(), b"model\t\t: \0".as_ptr() as *const c_char, 9) == 0 {
            sscanf(
                value.as_ptr(),
                b"model\t\t: %u\0".as_ptr() as *const c_char,
                &mut (*cpu_info).model as *mut c_uint,
            );
        } else if strncmp(
            value.as_ptr(),
            b"stepping\t: \0".as_ptr() as *const c_char,
            10,
        ) == 0
        {
            sscanf(
                value.as_ptr(),
                b"stepping\t: %u\0".as_ptr() as *const c_char,
                &mut (*cpu_info).stepping as *mut c_uint,
            );

            /* Exit -> all values must have been set */
            if (*cpu_info).vendor == X86_VENDOR_UNKNOWN
                || (*cpu_info).family == unknown
                || (*cpu_info).model == unknown
                || (*cpu_info).stepping == unknown
            {
                ret = -EINVAL;
                fclose(fp);
                return finish_cpu_info(cpu_info, ret);
            }

            ret = 0;
            fclose(fp);
            return finish_cpu_info(cpu_info, ret);
        }
    }
    ret = -ENODEV;
    fclose(fp);
    finish_cpu_info(cpu_info, ret)
}

unsafe fn finish_cpu_info(cpu_info: *mut cpupower_cpu_info, ret: c_int) -> c_int {
    let cpuid_level: c_uint;
    let ext_cpuid_level: c_uint;

    /* Get some useful CPU capabilities from cpuid */
    if (*cpu_info).vendor != X86_VENDOR_AMD
        && (*cpu_info).vendor != X86_VENDOR_HYGON
        && (*cpu_info).vendor != X86_VENDOR_INTEL
    {
        return ret;
    }

    cpuid_level = cpuid_eax(0);
    ext_cpuid_level = cpuid_eax(0x80000000);

    /* Invariant TSC */
    if ext_cpuid_level >= 0x80000007 && (cpuid_edx(0x80000007) & (1 << 8)) != 0 {
        (*cpu_info).caps |= CPUPOWER_CAP_INV_TSC;
    }

    /* Aperf/Mperf registers support */
    if cpuid_level >= 6 && (cpuid_ecx(6) & 0x1) != 0 {
        (*cpu_info).caps |= CPUPOWER_CAP_APERF;
    }

    /* AMD or Hygon Boost state enable/disable register */
    if (*cpu_info).vendor == X86_VENDOR_AMD || (*cpu_info).vendor == X86_VENDOR_HYGON {
        if ext_cpuid_level >= 0x80000007 {
            if (cpuid_edx(0x80000007) & (1 << 9)) != 0 {
                (*cpu_info).caps |= CPUPOWER_CAP_AMD_CPB;

                if (*cpu_info).family >= 0x17 {
                    (*cpu_info).caps |= CPUPOWER_CAP_AMD_CPB_MSR;
                }
            }

            if (cpuid_edx(0x80000007) & (1 << 7)) != 0 && (*cpu_info).family != 0x14 {
                /* HW pstate was not implemented in family 0x14 */
                (*cpu_info).caps |= CPUPOWER_CAP_AMD_HW_PSTATE;

                if (*cpu_info).family >= 0x17 {
                    (*cpu_info).caps |= CPUPOWER_CAP_AMD_PSTATEDEF;
                }
            }
        }

        if ext_cpuid_level >= 0x80000008 && (cpuid_ebx(0x80000008) & (1 << 4)) != 0 {
            (*cpu_info).caps |= CPUPOWER_CAP_AMD_RDPRU;
        }

        if cpupower_amd_pstate_enabled() != 0 {
            (*cpu_info).caps |= CPUPOWER_CAP_AMD_PSTATE;

            /*
             * If AMD P-State is enabled, the firmware will treat
             * AMD P-State function as high priority.
             */
            (*cpu_info).caps &= !CPUPOWER_CAP_AMD_CPB;
            (*cpu_info).caps &= !CPUPOWER_CAP_AMD_CPB_MSR;
            (*cpu_info).caps &= !CPUPOWER_CAP_AMD_HW_PSTATE;
            (*cpu_info).caps &= !CPUPOWER_CAP_AMD_PSTATEDEF;
        }
    }

    if (*cpu_info).vendor == X86_VENDOR_INTEL {
        if cpuid_level >= 6 && (cpuid_eax(6) & (1 << 1)) != 0 {
            (*cpu_info).caps |= CPUPOWER_CAP_INTEL_IDA;
        }
    }

    if (*cpu_info).vendor == X86_VENDOR_INTEL {
        /* Intel's perf-bias MSR support */
        if cpuid_level >= 6 && (cpuid_ecx(6) & (1 << 3)) != 0 {
            (*cpu_info).caps |= CPUPOWER_CAP_PERF_BIAS;
        }

        /* Intel's Turbo Ratio Limit support */
        if (*cpu_info).family == 6 {
            match (*cpu_info).model {
                0x1A => {
                    /*
                     * Core i7, Xeon 5500 series
                     * Bloomfield, Gainstown NHM-EP
                     */
                    (*cpu_info).caps |= CPUPOWER_CAP_HAS_TURBO_RATIO;
                }
                0x1E => {
                    /*
                     * Core i7 and i5 Processor
                     * Clarksfield, Lynnfield, Jasper Forest
                     */
                    (*cpu_info).caps |= CPUPOWER_CAP_HAS_TURBO_RATIO;
                }
                0x1F => {
                    /* Core i7 and i5 Processor - Nehalem */
                    (*cpu_info).caps |= CPUPOWER_CAP_HAS_TURBO_RATIO;
                }
                0x25 => {
                    /*
                     * Westmere Client
                     * Clarkdale, Arrandale
                     */
                    (*cpu_info).caps |= CPUPOWER_CAP_HAS_TURBO_RATIO;
                }
                0x2C => {
                    /* Westmere EP - Gulftown */
                    (*cpu_info).caps |= CPUPOWER_CAP_HAS_TURBO_RATIO;
                }
                0x2A => {
                    /* SNB */
                    (*cpu_info).caps |= CPUPOWER_CAP_HAS_TURBO_RATIO;
                    (*cpu_info).caps |= CPUPOWER_CAP_IS_SNB;
                }
                0x2D => {
                    /* SNB Xeon */
                    (*cpu_info).caps |= CPUPOWER_CAP_HAS_TURBO_RATIO;
                    (*cpu_info).caps |= CPUPOWER_CAP_IS_SNB;
                }
                0x3A => {
                    /* IVB */
                    (*cpu_info).caps |= CPUPOWER_CAP_HAS_TURBO_RATIO;
                    (*cpu_info).caps |= CPUPOWER_CAP_IS_SNB;
                }
                0x3E => {
                    /* IVB Xeon */
                    (*cpu_info).caps |= CPUPOWER_CAP_HAS_TURBO_RATIO;
                    (*cpu_info).caps |= CPUPOWER_CAP_IS_SNB;
                }
                0x2E => {
                    /* Nehalem-EX Xeon - Beckton */
                }
                0x2F => {
                    /* Westmere-EX Xeon - Eagleton */
                }
                _ => {}
            }
        }
    }

    /*
        printf("ID: %u - Extid: 0x%x - Caps: 0x%llx\n",
            cpuid_level, ext_cpuid_level, cpu_info->caps);
    */
    ret
}
