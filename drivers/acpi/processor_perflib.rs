// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * processor_perflib.c - ACPI Processor P-States Library ($Revision: 71 $)
 *
 *  Copyright (C) 2001, 2002 Andy Grover <andrew.grover@intel.com>
 *  Copyright (C) 2001, 2002 Paul Diefenbaugh <paul.s.diefenbaugh@intel.com>
 *  Copyright (C) 2004       Dominik Brodowski <linux@brodo.de>
 *  Copyright (C) 2004  Anil S Keshavamurthy <anil.s.keshavamurthy@intel.com>
 *                       - Added processor hotplug support
 */

// Kernel dependencies supplied by other translation units.

const ACPI_PROCESSOR_FILE_PERFORMANCE: &str = "performance";

/* _PPC support is implemented as a CPUfreq policy notifier. */
static mut ignore_ppc: i32 = -1;
static mut acpi_processor_ppc_in_use: bool = false;

unsafe fn acpi_processor_get_platform_limit(pr: *mut acpi_processor) -> i32 {
    let mut status: acpi_status = 0;
    let mut ppc: u64 = 0;
    let mut qos_value: s32;
    let index: i32;
    let ret: i32;

    if pr.is_null() { return -EINVAL; }
    status = acpi_evaluate_integer((*pr).handle, "_PPC", core::ptr::null_mut(), &mut ppc);
    if status != AE_NOT_FOUND {
        acpi_processor_ppc_in_use = true;
        if ACPI_FAILURE(status) {
            acpi_evaluation_failure_warn((*pr).handle, "_PPC", status);
            return -ENODEV;
        }
    }
    index = ppc as i32;
    if (*pr).performance_platform_limit == index ||
       ppc >= (*(*pr).performance).state_count as u64 { return 0; }
    pr_debug!("CPU {}: _PPC is {} - frequency {} limited\n", (*pr).id, index,
              if index != 0 { "is" } else { "is not" });
    (*pr).performance_platform_limit = index;
    if !freq_qos_request_active(&mut (*pr).perflib_req) { return 0; }
    if index == 0 { qos_value = FREQ_QOS_MAX_DEFAULT_VALUE; }
    else { qos_value = (*(*(*pr).performance).states.add(index as usize)).core_frequency * 1000; }
    ret = freq_qos_update_request(&mut (*pr).perflib_req, qos_value);
    if ret < 0 { pr_warn!("Failed to update perflib freq constraint: CPU{} ({})\n", (*pr).id, ret); }
    0
}

const ACPI_PROCESSOR_NOTIFY_PERFORMANCE: u32 = 0x80;
unsafe fn acpi_processor_ppc_ost(handle: acpi_handle, status: i32) {
    if acpi_has_method(handle, "_OST") { acpi_evaluate_ost(handle, ACPI_PROCESSOR_NOTIFY_PERFORMANCE, status, core::ptr::null_mut()); }
}

pub unsafe fn acpi_processor_ppc_has_changed(pr: *mut acpi_processor, event_flag: i32) {
    if ignore_ppc != 0 || (*pr).performance.is_null() {
        if event_flag != 0 { acpi_processor_ppc_ost((*pr).handle, 1); }
        return;
    }
    let ret = acpi_processor_get_platform_limit(pr);
    if event_flag != 0 { acpi_processor_ppc_ost((*pr).handle, if ret < 0 { 1 } else { 0 }); }
    if ret >= 0 { cpufreq_update_limits((*pr).id); }
}

pub unsafe fn acpi_processor_get_bios_limit(cpu: i32, limit: *mut u32) -> i32 {
    let pr = per_cpu(processors, cpu);
    if pr.is_null() || (*pr).performance.is_null() || (*(*pr).performance).state_count == 0 { return -ENODEV; }
    *limit = (*(*(*pr).performance).states.add((*pr).performance_platform_limit as usize)).core_frequency * 1000;
    0
}

pub unsafe fn acpi_processor_ignore_ppc_init() { if ignore_ppc < 0 { ignore_ppc = 0; } }

pub unsafe fn acpi_processor_ppc_init(policy: *mut cpufreq_policy) {
    if ignore_ppc == 1 { return; }
    for_each_cpu!(cpu, (*policy).related_cpus, {
        let pr = per_cpu(processors, cpu);
        if pr.is_null() { continue; }
        (*pr).performance_platform_limit = 0;
        let ret = freq_qos_add_request(&mut (*policy).constraints, &mut (*pr).perflib_req, FREQ_QOS_MAX, FREQ_QOS_MAX_DEFAULT_VALUE);
        if ret < 0 { pr_err!("Failed to add freq constraint for CPU{} ({})\n", cpu, ret); }
        if (*pr).performance.is_null() { continue; }
        let ret = acpi_processor_get_platform_limit(pr);
        if ret != 0 { pr_err!("Failed to update freq constraint for CPU{} ({})\n", cpu, ret); }
    });
}

pub unsafe fn acpi_processor_ppc_exit(policy: *mut cpufreq_policy) {
    for_each_cpu!(cpu, (*policy).related_cpus, { let pr = per_cpu(processors, cpu); if !pr.is_null() { freq_qos_remove_request(&mut (*pr).perflib_req); } });
}

// The following interfaces preserve the CONFIG_X86 implementation and depend on
// the kernel ACPI, cpufreq, MSR, CPU-mask, and processor declarations.
#[cfg(CONFIG_X86)]
static mut performance_mutex: mutex = DEFINE_MUTEX!();

#[cfg(CONFIG_X86)]
pub unsafe fn acpi_processor_get_performance_info(pr: *mut acpi_processor) -> i32 {
    let mut result = 0;
    if pr.is_null() || (*pr).performance.is_null() || (*pr).handle.is_null() { return -EINVAL; }
    if !acpi_has_method((*pr).handle, "_PCT") { acpi_handle_debug((*pr).handle, "ACPI-based processor performance control unavailable\n"); return -ENODEV; }
    result = acpi_processor_get_performance_control(pr);
    if result != 0 { return result; }
    result = acpi_processor_get_performance_states(pr);
    if result != 0 { return result; }
    if ignore_ppc != 1 { result = acpi_processor_get_platform_limit(pr); }
    result
}

#[cfg(CONFIG_X86)]
pub unsafe fn acpi_processor_pstate_control() -> i32 {
    if acpi_gbl_FADT.smi_command == 0 || acpi_gbl_FADT.pstate_control == 0 { return 0; }
    let status = acpi_os_write_port(acpi_gbl_FADT.smi_command, acpi_gbl_FADT.pstate_control as u32, 8);
    if ACPI_SUCCESS(status) { 1 } else { -EIO }
}

#[cfg(CONFIG_X86)]
pub unsafe fn acpi_processor_get_psd(handle: acpi_handle, pdomain: *mut acpi_psd_package) -> i32 {
    let mut buffer = acpi_buffer { length: ACPI_ALLOCATE_BUFFER, pointer: core::ptr::null_mut() };
    let mut format = acpi_buffer { length: core::mem::size_of::<&str>(), pointer: "NNNNN" as *const _ as *mut _ };
    let mut state = acpi_buffer { length: core::mem::size_of::<acpi_psd_package>(), pointer: pdomain as *mut _ };
    let status = acpi_evaluate_object(handle, "_PSD", core::ptr::null_mut(), &mut buffer);
    if ACPI_FAILURE(status) { return -ENODEV; }
    let psd = buffer.pointer as *mut acpi_object;
    let mut result = 0;
    if psd.is_null() || (*psd).type_ != ACPI_TYPE_PACKAGE || (*psd).package.count != 1 { pr_err!("Invalid _PSD data\n"); result = -EFAULT; }
    else if ACPI_FAILURE(acpi_extract_package((*psd).package.elements, &mut format, &mut state)) { pr_err!("Invalid _PSD data\n"); result = -EFAULT; }
    else if (*pdomain).num_entries != ACPI_PSD_REV0_ENTRIES || (*pdomain).revision != ACPI_PSD_REV0_REVISION ||
            ((*pdomain).coord_type != DOMAIN_COORD_TYPE_SW_ALL && (*pdomain).coord_type != DOMAIN_COORD_TYPE_SW_ANY && (*pdomain).coord_type != DOMAIN_COORD_TYPE_HW_ALL) { result = -EFAULT; }
    kfree(buffer.pointer); result
}

#[cfg(CONFIG_X86)]
pub unsafe fn acpi_processor_notify_smm(calling_module: *mut module) -> i32 {
    static mut is_done: i32 = 0;
    if !acpi_processor_cpufreq_init { return -EBUSY; }
    if !try_module_get(calling_module) { return -EINVAL; }
    if is_done != 0 { let result = if is_done < 0 { is_done } else { 0 }; module_put(calling_module); return result; }
    let result = acpi_processor_pstate_control();
    if result <= 0 { is_done = if result != 0 { result } else { 1 }; module_put(calling_module); return result; }
    is_done = 1;
    if acpi_processor_ppc_in_use { return 0; }
    module_put(calling_module); 0
}

// Registration routines are direct kernel per-CPU operations; their declarations
// remain exported with the same names and signatures in the surrounding crate.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
