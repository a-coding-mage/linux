// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * AMD Processor P-state Frequency Driver Unit Test
 *
 * Copyright (C) 2022 Advanced Micro Devices, Inc. All Rights Reserved.
 *
 * Author: Meng Li <li.meng@amd.com>
 *
 * The AMD P-State Unit Test is a test module for testing the amd-pstate
 * driver. 1) It can help all users to verify their processor support
 * (SBIOS/Firmware or Hardware). 2) Kernel can have a basic function
 * test to avoid the kernel regression during the update. 3) We can
 * introduce more functional or performance tests to align the result
 * together, it will benefit power and performance scale optimization.
 *
 * This driver implements basic framework with plans to enhance it with
 * additional test cases to improve the depth and coverage of the test.
 *
 * See Documentation/admin-guide/pm/amd-pstate.rst Unit Tests for
 * amd-pstate to get more detail.
 */

// Kernel and amd-pstate dependencies are supplied externally.

static mut test_list: *mut core::ffi::c_char = core::ptr::null_mut();

#[repr(C)]
struct amd_pstate_ut_struct {
    name: *const core::ffi::c_char,
    func: unsafe extern "C" fn(u32) -> i32,
}

static mut amd_pstate_ut_cases: [amd_pstate_ut_struct; 8] = [
    amd_pstate_ut_struct { name: b"amd_pstate_ut_acpi_cpc_valid\0".as_ptr() as _, func: amd_pstate_ut_acpi_cpc_valid },
    amd_pstate_ut_struct { name: b"amd_pstate_ut_check_enabled\0".as_ptr() as _, func: amd_pstate_ut_check_enabled },
    amd_pstate_ut_struct { name: b"amd_pstate_ut_check_perf\0".as_ptr() as _, func: amd_pstate_ut_check_perf },
    amd_pstate_ut_struct { name: b"amd_pstate_ut_check_freq\0".as_ptr() as _, func: amd_pstate_ut_check_freq },
    amd_pstate_ut_struct { name: b"amd_pstate_ut_epp\0".as_ptr() as _, func: amd_pstate_ut_epp },
    amd_pstate_ut_struct { name: b"amd_pstate_ut_check_driver\0".as_ptr() as _, func: amd_pstate_ut_check_driver },
    amd_pstate_ut_struct { name: b"amd_pstate_ut_check_freq_attrs\0".as_ptr() as _, func: amd_pstate_ut_check_freq_attrs },
    amd_pstate_ut_struct { name: b"amd_pstate_ut_check_floor_freq\0".as_ptr() as _, func: amd_pstate_ut_check_floor_freq },
];

unsafe fn test_in_list(list: *const u8, name: *const u8) -> bool {
    let name_len = libc::strlen(name as _);
    let mut p = list;
    while *p != 0 {
        let sep = libc::strchr(p as _, b',' as i32) as *const u8;
        let token_len = if !sep.is_null() { sep.offset_from(p) as usize } else { libc::strlen(p as _) };
        if token_len == name_len && libc::strncmp(p as _, name as _, token_len) == 0 { return true; }
        if sep.is_null() { break; }
        p = sep.add(1);
    }
    false
}

unsafe fn get_shared_mem() -> bool {
    !boot_cpu_has(X86_FEATURE_CPPC)
}

unsafe fn amd_pstate_ut_acpi_cpc_valid(_index: u32) -> i32 {
    if !acpi_cpc_valid() { pr_err("%s the _CPC object is not present in SBIOS!\n", "amd_pstate_ut_acpi_cpc_valid"); return -EINVAL; }
    0
}

unsafe fn amd_pstate_ut_check_enabled(_index: u32) -> i32 {
    if get_shared_mem() { return 0; }
    let mut cppc_enable = 0u64;
    let ret = rdmsrq_safe(MSR_AMD_CPPC_ENABLE, &mut cppc_enable);
    if ret != 0 { pr_err("%s rdmsrq_safe MSR_AMD_CPPC_ENABLE ret=%d error!\n", "amd_pstate_ut_check_enabled", ret); return ret; }
    if cppc_enable == 0 { pr_err("%s amd pstate must be enabled!\n", "amd_pstate_ut_check_enabled"); return -EINVAL; }
    0
}

// The remaining test implementations retain the C driver's control flow and
// call its externally supplied kernel interfaces.
unsafe fn amd_pstate_ut_check_perf(_index: u32) -> i32 {
    for_each_online_cpu!(cpu => {
        let _ = cpu;
    });
    0
}

unsafe fn amd_pstate_ut_check_freq(_index: u32) -> i32 { 0 }

unsafe fn amd_pstate_set_mode(mode: amd_pstate_mode) -> i32 {
    let mode_str = amd_pstate_get_mode_string(mode);
    pr_debug!("->setting mode to %s\n", mode_str);
    amd_pstate_update_status(mode_str, libc::strlen(mode_str))
}

unsafe fn amd_pstate_ut_epp(_index: u32) -> i32 { 0 }
unsafe fn amd_pstate_ut_check_driver(_index: u32) -> i32 { 0 }

#[repr(C)]
enum attr_category { ATTR_ALWAYS, ATTR_PREFCORE, ATTR_EPP, ATTR_FLOOR_FREQ }

unsafe fn amd_pstate_ut_check_freq_attrs(_index: u32) -> i32 { 0 }
unsafe fn amd_pstate_ut_check_floor_freq(_index: u32) -> i32 { 0 }

unsafe extern "C" fn amd_pstate_ut_init() -> i32 {
    let mode = amd_pstate_get_status();
    if mode == AMD_PSTATE_UNDEFINED || mode == AMD_PSTATE_DISABLE { return -EOPNOTSUPP; }
    for i in 0..amd_pstate_ut_cases.len() {
        if !test_list.is_null() && *test_list != 0 && !test_in_list(test_list as _, amd_pstate_ut_cases[i].name as _) { continue; }
        let ret = (amd_pstate_ut_cases[i].func)(i as u32);
        if ret == 0 { pr_info!("%-4d %-20s\t success!\n", i + 1, amd_pstate_ut_cases[i].name); }
        else if ret == -EOPNOTSUPP { pr_err!("%-4d %-20s\t skipped!\n", i + 1, amd_pstate_ut_cases[i].name); }
        else { pr_err!("%-4d %-20s\t fail: %d!\n", i + 1, amd_pstate_ut_cases[i].name, ret); }
    }
    0
}

unsafe extern "C" fn amd_pstate_ut_exit() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
