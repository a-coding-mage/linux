/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  (C) 2010,2011       Thomas Renninger <trenn@suse.de>, Novell Inc.
 *
 * Miscellaneous helpers which do not fit or are worth
 * to put into separate headers
 */

use std::os::raw::{c_char, c_int, c_uint, c_ulong, c_ulonglong};

/* Dependencies from C headers:
 * - helpers/bitmask.h: struct bitmask
 * - pci/pci.h on x86: struct pci_access, struct pci_dev
 * - cpupower.h
 */
#[repr(C)]
pub struct bitmask {
    _private: [u8; 0],
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[repr(C)]
pub struct pci_access {
    _private: [u8; 0],
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[repr(C)]
pub struct pci_dev {
    _private: [u8; 0],
}

/* Internationalization ****************************/
/* C macros translated as small helpers. With the NLS cfg enabled, gettext is
 * an external dependency; otherwise strings are returned unchanged.
 */
#[cfg(feature = "NLS")]
unsafe extern "C" {
    pub fn gettext(msgid: *const c_char) -> *mut c_char;
}

#[cfg(feature = "NLS")]
#[inline]
pub unsafe fn r#_(string: *const c_char) -> *mut c_char {
    unsafe { gettext(string) }
}

#[cfg(not(feature = "NLS"))]
#[inline]
pub unsafe fn r#_(string: *const c_char) -> *const c_char {
    string
}

#[inline]
pub unsafe fn N_(string: *const c_char) -> *const c_char {
    string
}
/* Internationalization ****************************/

unsafe extern "C" {
    pub static mut run_as_root: c_int;
    pub static mut base_cpu: c_int;
    pub static mut cpus_chosen: *mut bitmask;
}

/* Global verbose (-d) stuff *********************************/
/*
 * define DEBUG via global Makefile variable
 * Debug output is sent to stderr, do:
 * cpupower monitor 2>/tmp/debug
 * to split debug output away from normal output
 */
#[cfg(feature = "DEBUG")]
unsafe extern "C" {
    pub static mut be_verbose: c_int;
}

/* C debug macro dprint(fmt, ...) is intentionally represented only by the
 * DEBUG dependency above; Rust has no source-level equivalent for __func__ and
 * C varargs macro expansion in this header alone.
 */
#[cfg(not(feature = "DEBUG"))]
#[inline]
pub unsafe fn dprint(_fmt: *const c_char) {}

#[cfg(not(feature = "DEBUG"))]
unsafe extern "C" {
    pub static mut be_verbose: c_int;
}
/* Global verbose (-v) stuff *********************************/

/* cpuid and cpuinfo helpers  **************************/
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum cpupower_cpu_vendor {
    X86_VENDOR_UNKNOWN = 0,
    X86_VENDOR_INTEL = 1,
    X86_VENDOR_AMD = 2,
    X86_VENDOR_HYGON = 3,
    X86_VENDOR_MAX = 4,
}

pub const CPUPOWER_CAP_INV_TSC: c_ulonglong = 0x00000001;
pub const CPUPOWER_CAP_APERF: c_ulonglong = 0x00000002;
pub const CPUPOWER_CAP_AMD_CPB: c_ulonglong = 0x00000004;
pub const CPUPOWER_CAP_PERF_BIAS: c_ulonglong = 0x00000008;
pub const CPUPOWER_CAP_HAS_TURBO_RATIO: c_ulonglong = 0x00000010;
pub const CPUPOWER_CAP_IS_SNB: c_ulonglong = 0x00000020;
pub const CPUPOWER_CAP_INTEL_IDA: c_ulonglong = 0x00000040;
pub const CPUPOWER_CAP_AMD_RDPRU: c_ulonglong = 0x00000080;
pub const CPUPOWER_CAP_AMD_HW_PSTATE: c_ulonglong = 0x00000100;
pub const CPUPOWER_CAP_AMD_PSTATEDEF: c_ulonglong = 0x00000200;
pub const CPUPOWER_CAP_AMD_CPB_MSR: c_ulonglong = 0x00000400;
pub const CPUPOWER_CAP_AMD_PSTATE: c_ulonglong = 0x00000800;

pub const CPUPOWER_AMD_CPBDIS: c_uint = 0x02000000;

pub const MAX_HW_PSTATES: c_int = 10;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct cpupower_cpu_info {
    pub vendor: cpupower_cpu_vendor,
    pub family: c_uint,
    pub model: c_uint,
    pub stepping: c_uint,
    /* CPU capabilities read out from cpuid */
    pub caps: c_ulonglong,
}

/* get_cpu_info
 *
 * Extract CPU vendor, family, model, stepping info from /proc/cpuinfo
 *
 * Returns 0 on success or a negative error code
 * Only used on x86, below global's struct values are zero/unknown on
 * other archs
 */
unsafe extern "C" {
    pub fn get_cpu_info(cpu_info: *mut cpupower_cpu_info) -> c_int;
    pub static mut cpupower_cpu_info: cpupower_cpu_info;
}

/* cpuid and cpuinfo helpers  **************************/

unsafe extern "C" {
    pub fn cpufreq_has_generic_boost_support(active: *mut bool) -> c_int;
    pub fn cpupower_set_generic_turbo_boost(turbo_boost: c_int) -> c_int;
}

/* X86 ONLY ****************************************/
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
unsafe extern "C" {
    /* Read/Write msr ****************************/
    pub fn read_msr(cpu: c_int, idx: c_uint, val: *mut c_ulonglong) -> c_int;
    pub fn write_msr(cpu: c_int, idx: c_uint, val: c_ulonglong) -> c_int;

    pub fn cpupower_intel_set_perf_bias(cpu: c_uint, val: c_uint) -> c_int;
    pub fn cpupower_intel_get_perf_bias(cpu: c_uint) -> c_int;
    pub fn msr_intel_get_turbo_ratio(cpu: c_uint) -> c_ulonglong;

    pub fn cpupower_set_epp(cpu: c_uint, epp: *mut c_char) -> c_int;
    pub fn cpupower_set_amd_pstate_mode(mode: *mut c_char) -> c_int;

    /* Read/Write msr ****************************/

    /* PCI stuff ****************************/
    pub fn amd_pci_get_num_boost_states(active: *mut c_int, states: *mut c_int) -> c_int;
    pub fn pci_acc_init(
        pacc: *mut *mut pci_access,
        domain: c_int,
        bus: c_int,
        slot: c_int,
        func: c_int,
        vendor: c_int,
        dev: c_int,
    ) -> *mut pci_dev;
    pub fn pci_slot_func_init(
        pacc: *mut *mut pci_access,
        slot: c_int,
        func: c_int,
    ) -> *mut pci_dev;

    /* PCI stuff ****************************/

    /* AMD HW pstate decoding **************************/
    pub fn decode_pstates(
        cpu: c_uint,
        boost_states: c_int,
        pstates: *mut c_ulong,
        no: *mut c_int,
    ) -> c_int;

    /* AMD HW pstate decoding **************************/

    pub fn cpufreq_has_x86_boost_support(
        cpu: c_uint,
        support: *mut c_int,
        active: *mut c_int,
        states: *mut c_int,
    ) -> c_int;
    pub fn cpupower_set_intel_turbo_boost(turbo_boost: c_int) -> c_int;

    /* AMD P-State stuff **************************/
    pub fn cpupower_amd_pstate_enabled() -> bool;
    pub fn amd_pstate_boost_init(cpu: c_uint, support: *mut c_int, active: *mut c_int);
    pub fn amd_pstate_show_perf_and_freq(cpu: c_uint, no_rounding: c_int);

    /* AMD P-State stuff **************************/

    /*
     * CPUID functions returning a single datum
     */
    pub fn cpuid_eax(op: c_uint) -> c_uint;
    pub fn cpuid_ebx(op: c_uint) -> c_uint;
    pub fn cpuid_ecx(op: c_uint) -> c_uint;
    pub fn cpuid_edx(op: c_uint) -> c_uint;
}

/* cpuid and cpuinfo helpers  **************************/
/* X86 ONLY ********************************************/
#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
#[inline]
pub unsafe fn decode_pstates(
    _cpu: c_uint,
    _boost_states: c_int,
    _pstates: *mut c_ulong,
    _no: *mut c_int,
) -> c_int {
    -1
}

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
#[inline]
pub unsafe fn read_msr(_cpu: c_int, _idx: c_uint, _val: *mut c_ulonglong) -> c_int {
    -1
}

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
#[inline]
pub unsafe fn write_msr(_cpu: c_int, _idx: c_uint, _val: c_ulonglong) -> c_int {
    -1
}

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
#[inline]
pub unsafe fn cpupower_intel_set_perf_bias(_cpu: c_uint, _val: c_uint) -> c_int {
    -1
}

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
#[inline]
pub unsafe fn cpupower_intel_get_perf_bias(_cpu: c_uint) -> c_int {
    -1
}

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
#[inline]
pub unsafe fn msr_intel_get_turbo_ratio(_cpu: c_uint) -> c_ulonglong {
    0
}

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
#[inline]
pub unsafe fn cpupower_set_epp(_cpu: c_uint, _epp: *mut c_char) -> c_int {
    -1
}

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
#[inline]
pub unsafe fn cpupower_set_amd_pstate_mode(_mode: *mut c_char) -> c_int {
    -1
}

/* Read/Write msr ****************************/

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
#[inline]
pub unsafe fn cpufreq_has_x86_boost_support(
    _cpu: c_uint,
    _support: *mut c_int,
    _active: *mut c_int,
    _states: *mut c_int,
) -> c_int {
    -1
}

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
#[inline]
pub unsafe fn cpupower_set_intel_turbo_boost(_turbo_boost: c_int) -> c_int {
    -1
}

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
#[inline]
pub unsafe fn cpupower_amd_pstate_enabled() -> bool {
    false
}

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
#[inline]
pub unsafe fn amd_pstate_boost_init(_cpu: c_uint, _support: *mut c_int, _active: *mut c_int) {}

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
#[inline]
pub unsafe fn amd_pstate_show_perf_and_freq(_cpu: c_uint, _no_rounding: c_int) {}

/* cpuid and cpuinfo helpers  **************************/

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
#[inline]
pub unsafe fn cpuid_eax(_op: c_uint) -> c_uint {
    0
}

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
#[inline]
pub unsafe fn cpuid_ebx(_op: c_uint) -> c_uint {
    0
}

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
#[inline]
pub unsafe fn cpuid_ecx(_op: c_uint) -> c_uint {
    0
}

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
#[inline]
pub unsafe fn cpuid_edx(_op: c_uint) -> c_uint {
    0
}

/*
 * CPU State related functions
 */
unsafe extern "C" {
    pub static mut online_cpus: *mut bitmask;
    pub static mut offline_cpus: *mut bitmask;

    pub fn get_cpustate();
    pub fn print_online_cpus();
    pub fn print_offline_cpus();
    pub fn print_speed(speed: c_ulong, no_rounding: c_int);

    pub fn cppc_show_perf_and_freq(cpu: c_uint, no_rounding: c_int);
}
