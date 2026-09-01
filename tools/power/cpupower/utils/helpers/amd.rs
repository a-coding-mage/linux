// SPDX-License-Identifier: GPL-2.0
// Translated from C for x86 targets:
// #if defined(__i386__) || defined(__x86_64__)

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use core::ffi::{c_char, c_int, c_uchar, c_uint, c_ulong, c_ulonglong, c_void};

const MSR_AMD_PSTATE_STATUS: c_uint = 0xc0010063;
const MSR_AMD_PSTATE: c_uint = 0xc0010064;
const MSR_AMD_PSTATE_LIMIT: c_uint = 0xc0010061;

#[repr(C)]
#[derive(Copy, Clone)]
pub union core_pstate {
    pub val: c_ulonglong,
}

impl core_pstate {
    unsafe fn pstate_fid(self) -> c_uint {
        (unsafe { self.val } & 0x3f) as c_uint
    }

    unsafe fn pstate_did(self) -> c_uint {
        ((unsafe { self.val } >> 6) & 0x7) as c_uint
    }

    unsafe fn pstatedef_fid(self) -> c_uint {
        (unsafe { self.val } & 0xff) as c_uint
    }

    unsafe fn pstatedef_did(self) -> c_uint {
        ((unsafe { self.val } >> 8) & 0x3f) as c_uint
    }

    unsafe fn pstatedef_en(self) -> c_uint {
        ((unsafe { self.val } >> 63) & 0x1) as c_uint
    }

    unsafe fn pstatedef2_fid(self) -> c_uint {
        (unsafe { self.val } & 0xfff) as c_uint
    }
}

#[repr(C)]
pub struct cpupower_cpu_info_t {
    pub family: c_uint,
    pub caps: c_uint,
}

#[repr(C)]
pub struct pci_access {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pci_dev {
    _private: [u8; 0],
}

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

unsafe extern "C" {
    static cpupower_cpu_info: cpupower_cpu_info_t;
    static CPUPOWER_CAP_AMD_PSTATEDEF: c_uint;
    static CPUPOWER_CAP_AMD_HW_PSTATE: c_uint;
    static MAX_HW_PSTATES: c_int;
    static NOMINAL_PERF: c_uint;
    static NOMINAL_FREQ: c_uint;
    static LOWEST_NONLINEAR_PERF: c_uint;
    static LOWEST_PERF: c_uint;
    static LOWEST_FREQ: c_uint;
    static mut stderr: *mut FILE;

    fn read_msr(cpu: c_uint, msr: c_uint, val: *mut c_ulonglong) -> c_int;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn pci_slot_func_init(pci_acc: *mut *mut pci_access, slot: c_int, func: c_int) -> *mut pci_dev;
    fn pci_read_byte(device: *mut pci_dev, pos: c_int) -> c_uchar;
    fn pci_cleanup(pacc: *mut pci_access);
    fn cpufreq_get_sysfs_value_from_table(
        cpu: c_uint,
        table: *const *const c_char,
        index: c_uint,
        count: c_uint,
    ) -> c_ulong;
    fn acpi_cppc_get_data(cpu: c_uint, which: c_uint) -> c_ulong;
    fn cpufreq_get_hardware_limits(cpu: c_uint, min: *mut c_ulong, max: *mut c_ulong) -> c_int;
    fn print_speed(speed: c_ulong, no_rounding: c_int);
}

/* ACPI P-States Helper Functions for AMD Processors ***************/

unsafe fn get_did(pstate: core_pstate) -> c_int {
    let t: c_int;

    /* Fam 1Ah onward do not use did */
    if unsafe { cpupower_cpu_info.family } >= 0x1A {
        return 0;
    }

    if (unsafe { cpupower_cpu_info.caps } & unsafe { CPUPOWER_CAP_AMD_PSTATEDEF }) != 0 {
        t = unsafe { pstate.pstatedef_did() } as c_int;
    } else if unsafe { cpupower_cpu_info.family } == 0x12 {
        t = (unsafe { pstate.val } & 0xf) as c_int;
    } else {
        t = unsafe { pstate.pstate_did() } as c_int;
    }

    t
}

unsafe fn get_cof(pstate: core_pstate) -> c_int {
    let mut t: c_int;
    let fid: c_int;
    let did: c_int;
    let mut cof: c_int = 0;

    did = unsafe { get_did(pstate) };
    if (unsafe { cpupower_cpu_info.caps } & unsafe { CPUPOWER_CAP_AMD_PSTATEDEF }) != 0 {
        if unsafe { cpupower_cpu_info.family } >= 0x1A {
            fid = unsafe { pstate.pstatedef2_fid() } as c_int;
            if fid > 0x0f {
                cof = fid * 5;
            }
        } else {
            fid = unsafe { pstate.pstatedef_fid() } as c_int;
            cof = 200 * fid / did;
        }
    } else {
        t = 0x10;
        fid = unsafe { pstate.pstate_fid() } as c_int;
        if unsafe { cpupower_cpu_info.family } == 0x11 {
            t = 0x8;
        }
        cof = (100 * (fid + t)) >> did;
    }
    cof
}

/* Needs:
 * cpu          -> the cpu that gets evaluated
 * boost_states -> how much boost states the machines support
 *
 * Fills up:
 * pstates -> a pointer to an array of size MAX_HW_PSTATES
 *            must be initialized with zeros.
 *            All available  HW pstates (including boost states)
 * no      -> amount of pstates above array got filled up with
 *
 * returns zero on success, -1 on failure
 */
#[no_mangle]
pub unsafe extern "C" fn decode_pstates(
    cpu: c_uint,
    boost_states: c_int,
    pstates: *mut c_ulong,
    no: *mut c_int,
) -> c_int {
    let mut i: c_int;
    let mut psmax: c_int;
    let mut pstate = core_pstate { val: 0 };
    let mut val: c_ulonglong = 0;

    /* Only read out frequencies from HW if HW Pstate is supported,
     * otherwise frequencies are exported via ACPI tables.
     */
    if (unsafe { cpupower_cpu_info.caps } & unsafe { CPUPOWER_CAP_AMD_HW_PSTATE }) == 0 {
        return -1;
    }

    if unsafe { read_msr(cpu, MSR_AMD_PSTATE_LIMIT, &mut val) } != 0 {
        return -1;
    }

    psmax = ((val >> 4) & 0x7) as c_int;
    psmax += boost_states;
    i = 0;
    while i <= psmax {
        if i >= unsafe { MAX_HW_PSTATES } {
            unsafe {
                fprintf(
                    stderr,
                    b"HW pstates [%d] exceeding max [%d]\n\0".as_ptr() as *const c_char,
                    psmax,
                    MAX_HW_PSTATES,
                );
            }
            return -1;
        }
        if unsafe {
            read_msr(
                cpu,
                MSR_AMD_PSTATE.wrapping_add(i as c_uint),
                &mut pstate.val,
            )
        } != 0
        {
            return -1;
        }

        /* The enabled bit (bit 63) is common for all families */
        if unsafe { pstate.pstatedef_en() } == 0 {
            i += 1;
            continue;
        }

        unsafe {
            *pstates.add(i as usize) = get_cof(pstate) as c_ulong;
        }
        i += 1;
    }
    unsafe {
        *no = i;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn amd_pci_get_num_boost_states(
    active: *mut c_int,
    states: *mut c_int,
) -> c_int {
    let mut pci_acc: *mut pci_access = core::ptr::null_mut();
    let device: *mut pci_dev;
    let mut val: c_uchar = 0;

    unsafe {
        *active = 0;
        *states = *active;
    }

    device = unsafe { pci_slot_func_init(&mut pci_acc, 0x18, 4) };

    if device.is_null() {
        return -19;
    }

    val = unsafe { pci_read_byte(device, 0x15c) };
    if (val & 3) != 0 {
        unsafe {
            *active = 1;
        }
    } else {
        unsafe {
            *active = 0;
        }
    }
    unsafe {
        *states = ((val >> 2) & 7) as c_int;
    }

    unsafe {
        pci_cleanup(pci_acc);
    }
    0
}

/* ACPI P-States Helper Functions for AMD Processors ***************/

/* AMD P-State Helper Functions ************************************/
#[repr(C)]
#[derive(Copy, Clone)]
pub enum amd_pstate_value {
    AMD_PSTATE_HIGHEST_PERF = 0,
    AMD_PSTATE_MAX_FREQ = 1,
    AMD_PSTATE_LOWEST_NONLINEAR_FREQ = 2,
    AMD_PSTATE_HW_PREFCORE = 3,
    AMD_PSTATE_PREFCORE_RANKING = 4,
    MAX_AMD_PSTATE_VALUE_READ_FILES = 5,
}

static amd_pstate_value_files: [*const c_char; amd_pstate_value::MAX_AMD_PSTATE_VALUE_READ_FILES as usize] = [
    b"amd_pstate_highest_perf\0".as_ptr() as *const c_char,
    b"amd_pstate_max_freq\0".as_ptr() as *const c_char,
    b"amd_pstate_lowest_nonlinear_freq\0".as_ptr() as *const c_char,
    b"amd_pstate_hw_prefcore\0".as_ptr() as *const c_char,
    b"amd_pstate_prefcore_ranking\0".as_ptr() as *const c_char,
];

unsafe fn amd_pstate_get_data(cpu: c_uint, value: amd_pstate_value) -> c_ulong {
    unsafe {
        cpufreq_get_sysfs_value_from_table(
            cpu,
            amd_pstate_value_files.as_ptr(),
            value as c_uint,
            amd_pstate_value::MAX_AMD_PSTATE_VALUE_READ_FILES as c_uint,
        )
    }
}

#[no_mangle]
pub unsafe extern "C" fn amd_pstate_boost_init(
    cpu: c_uint,
    support: *mut c_int,
    active: *mut c_int,
) {
    let highest_perf: c_ulong;
    let nominal_perf: c_ulong;
    let mut cpuinfo_min: c_ulong = 0;
    let mut cpuinfo_max: c_ulong = 0;
    let amd_pstate_max: c_ulong;

    highest_perf = unsafe { amd_pstate_get_data(cpu, amd_pstate_value::AMD_PSTATE_HIGHEST_PERF) };
    nominal_perf = unsafe { acpi_cppc_get_data(cpu, NOMINAL_PERF) };

    unsafe {
        *support = if highest_perf > nominal_perf { 1 } else { 0 };
    }
    if unsafe { *support } == 0 {
        return;
    }

    unsafe {
        cpufreq_get_hardware_limits(cpu, &mut cpuinfo_min, &mut cpuinfo_max);
    }
    amd_pstate_max = unsafe { amd_pstate_get_data(cpu, amd_pstate_value::AMD_PSTATE_MAX_FREQ) };

    unsafe {
        *active = if cpuinfo_max == amd_pstate_max { 1 } else { 0 };
    }
}

#[no_mangle]
pub unsafe extern "C" fn amd_pstate_show_perf_and_freq(cpu: c_uint, no_rounding: c_int) {
    unsafe {
        printf(b"  amd-pstate limits:\n\0".as_ptr() as *const c_char);
        printf(
            b"    Highest Performance: %lu. Maximum Frequency: \0".as_ptr() as *const c_char,
            amd_pstate_get_data(cpu, amd_pstate_value::AMD_PSTATE_HIGHEST_PERF),
        );
    }
    /*
     * If boost isn't active, the cpuinfo_max doesn't indicate real max
     * frequency. So we read it back from amd-pstate sysfs entry.
     */
    unsafe {
        print_speed(
            amd_pstate_get_data(cpu, amd_pstate_value::AMD_PSTATE_MAX_FREQ),
            no_rounding,
        );
        printf(b".\n\0".as_ptr() as *const c_char);

        printf(
            b"    Nominal Performance: %lu. Nominal Frequency: \0".as_ptr() as *const c_char,
            acpi_cppc_get_data(cpu, NOMINAL_PERF),
        );
        print_speed(acpi_cppc_get_data(cpu, NOMINAL_FREQ) * 1000, no_rounding);
        printf(b".\n\0".as_ptr() as *const c_char);

        printf(
            b"    Lowest Non-linear Performance: %lu. Lowest Non-linear Frequency: \0".as_ptr()
                as *const c_char,
            acpi_cppc_get_data(cpu, LOWEST_NONLINEAR_PERF),
        );
        print_speed(
            amd_pstate_get_data(cpu, amd_pstate_value::AMD_PSTATE_LOWEST_NONLINEAR_FREQ),
            no_rounding,
        );
        printf(b".\n\0".as_ptr() as *const c_char);

        printf(
            b"    Lowest Performance: %lu. Lowest Frequency: \0".as_ptr() as *const c_char,
            acpi_cppc_get_data(cpu, LOWEST_PERF),
        );
        print_speed(acpi_cppc_get_data(cpu, LOWEST_FREQ) * 1000, no_rounding);
        printf(b".\n\0".as_ptr() as *const c_char);

        printf(
            b"    Preferred Core Support: %lu. Preferred Core Ranking: %lu.\n\0".as_ptr()
                as *const c_char,
            amd_pstate_get_data(cpu, amd_pstate_value::AMD_PSTATE_HW_PREFCORE),
            amd_pstate_get_data(cpu, amd_pstate_value::AMD_PSTATE_PREFCORE_RANKING),
        );
    }
}

/* AMD P-State Helper Functions ************************************/
// #endif /* defined(__i386__) || defined(__x86_64__) */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
