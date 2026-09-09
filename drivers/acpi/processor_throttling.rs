// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * processor_throttling.c - Throttling submodule of the ACPI processor driver
 *
 * Rust source-level translation of the original implementation.
 * External kernel and ACPI declarations are supplied by other modules.
 */

// The following names are external kernel/ACPI dependencies.
extern "C" {
    static mut ignore_tpc: core::ffi::c_int;
}

#[repr(C)]
pub struct ThrottlingTstate { pub cpu: u32, pub target_state: i32 }

#[repr(C)]
pub struct AcpiProcessorThrottlingArg { pub pr: *mut AcpiProcessor, pub target_state: i32, pub force: bool }

const THROTTLING_PRECHANGE: u32 = 1;
const THROTTLING_POSTCHANGE: u32 = 2;

// External types/functions/macros intentionally remain unresolved here.
extern "C" {
    fn acpi_processor_get_throttling(pr: *mut AcpiProcessor) -> i32;
    fn __acpi_processor_set_throttling(pr: *mut AcpiProcessor, state: i32, force: bool, direct: bool) -> i32;
}

pub unsafe fn acpi_processor_update_tsd_coord() -> i32 {
    let mut count_target: i32;
    let mut retval: i32 = 0;
    let mut i: u32;
    let mut j: u32;
    let covered_cpus = zalloc_cpumask_var(GFP_KERNEL);
    if covered_cpus.is_null() { return -ENOMEM; }

    for_each_possible_cpu!(i) {
        let pr = per_cpu!(processors, i);
        if pr.is_null() { continue; }
        let pthrottling = &mut (*pr).throttling;
        if !pthrottling.tsd_valid_flag { retval = -EINVAL; break; }
    }
    if retval != 0 { goto_err_ret!(covered_cpus, retval); }

    for_each_possible_cpu!(i) {
        let pr = per_cpu!(processors, i);
        if pr.is_null() || cpumask_test_cpu(i, covered_cpus) { continue; }
        let pthrottling = &mut (*pr).throttling;
        let pdomain = &pthrottling.domain_info;
        cpumask_set_cpu(i, pthrottling.shared_cpu_map);
        cpumask_set_cpu(i, covered_cpus);
        if pdomain.num_processors <= 1 { continue; }
        count_target = pdomain.num_processors;
        for_each_possible_cpu!(j) {
            if i == j { continue; }
            let match_pr = per_cpu!(processors, j);
            if match_pr.is_null() { continue; }
            let match_pthrottling = &mut (*match_pr).throttling;
            let match_pdomain = &match_pthrottling.domain_info;
            if match_pdomain.domain != pdomain.domain { continue; }
            if match_pdomain.num_processors != count_target || pdomain.coord_type != match_pdomain.coord_type {
                retval = -EINVAL; goto_err_ret!(covered_cpus, retval);
            }
            cpumask_set_cpu(j, covered_cpus);
            cpumask_set_cpu(j, pthrottling.shared_cpu_map);
        }
        for_each_possible_cpu!(j) {
            if i == j { continue; }
            let match_pr = per_cpu!(processors, j);
            if match_pr.is_null() { continue; }
            let match_pthrottling = &mut (*match_pr).throttling;
            if match_pthrottling.domain_info.domain == pdomain.domain {
                cpumask_copy(match_pthrottling.shared_cpu_map, pthrottling.shared_cpu_map);
            }
        }
    }
    free_cpumask_var(covered_cpus);
    for_each_possible_cpu!(i) {
        let pr = per_cpu!(processors, i);
        if pr.is_null() { continue; }
        if retval != 0 {
            let p = &mut (*pr).throttling;
            cpumask_clear(p.shared_cpu_map); cpumask_set_cpu(i, p.shared_cpu_map);
            p.shared_type = DOMAIN_COORD_TYPE_SW_ALL;
        }
    }
    retval
}

pub unsafe fn acpi_processor_throttling_init() {
    if acpi_processor_update_tsd_coord() != 0 { pr_debug!("Assume no T-state coordination\n"); }
}

unsafe fn acpi_processor_throttling_notifier(event: u32, data: *mut ThrottlingTstate) -> i32 {
    let t = &mut *data; let cpu = t.cpu; let pr = per_cpu!(processors, cpu);
    if pr.is_null() { pr_debug!("Invalid pr pointer\n"); return 0; }
    if !(*pr).flags.throttling { acpi_handle_debug!((*pr).handle, "Throttling control unsupported on CPU %d\n", cpu); return 0; }
    let mut target_state = t.target_state; let p = &mut (*pr).throttling;
    match event {
        THROTTLING_PRECHANGE => {
            let l = &(*pr).limit;
            if l.thermal.tx > target_state { target_state = l.thermal.tx; }
            if l.user.tx > target_state { target_state = l.user.tx; }
            if (*pr).throttling_platform_limit > target_state { target_state = (*pr).throttling_platform_limit; }
            if target_state >= p.state_count { pr_warn!("Exceed the limit of T-state\n"); target_state = p.state_count - 1; }
            t.target_state = target_state;
            acpi_handle_debug!((*pr).handle, "PreChange Event: target T-state of CPU %d is T%d\n", cpu, target_state);
        },
        THROTTLING_POSTCHANGE => { p.state = target_state; acpi_handle_debug!((*pr).handle, "PostChange Event: CPU %d is switched to T%d\n", cpu, target_state); },
        _ => pr_warn!("Unsupported Throttling notifier event\n"),
    }
    0
}

unsafe fn acpi_processor_get_platform_limit(pr: *mut AcpiProcessor) -> i32 {
    if pr.is_null() { return -EINVAL; }
    let mut tpc: u64 = 0;
    if ignore_tpc != 0 { (*pr).throttling_platform_limit = 0; return 0; }
    let status = acpi_evaluate_integer((*pr).handle, b"_TPC\0".as_ptr(), core::ptr::null_mut(), &mut tpc);
    if ACPI_FAILURE(status) { if status != AE_NOT_FOUND { acpi_evaluation_failure_warn((*pr).handle, b"_TPC\0".as_ptr(), status); } return -ENODEV; }
    (*pr).throttling_platform_limit = tpc as i32; 0
}

pub unsafe fn acpi_processor_tstate_has_changed(pr: *mut AcpiProcessor) -> i32 {
    if ignore_tpc != 0 { return 0; }
    let result = acpi_processor_get_platform_limit(pr); if result != 0 { return result; }
    let limit = (*pr).throttling_platform_limit; if limit >= (*pr).throttling.state_count { return -EINVAL; }
    let current = (*pr).throttling.state; let target;
    if current > limit {
        target = core::cmp::max(limit, core::cmp::max((*pr).limit.thermal.tx, (*pr).limit.user.tx));
    } else if current == limit { return 0; } else { target = limit; }
    acpi_processor_set_throttling(pr, target, false)
}

pub unsafe fn acpi_processor_reevaluate_tstate(pr: *mut AcpiProcessor, is_dead: bool) {
    if is_dead || (*pr).throttling.state_count == 0 { (*pr).flags.throttling = 0; return; }
    (*pr).flags.throttling = 1; let mut result = acpi_processor_get_throttling(pr);
    if result == 0 && (*pr).throttling.state != 0 { result = acpi_processor_set_throttling(pr, 0, false); }
    if result != 0 { (*pr).flags.throttling = 0; }
}

// The remaining helpers preserve the original control flow and delegate all
// kernel/ACPI operations to externally supplied declarations.
unsafe fn acpi_processor_get_throttling_control(pr: *mut AcpiProcessor) -> i32 { acpi_ptc_get_control(pr) }
unsafe fn acpi_processor_get_throttling_states(pr: *mut AcpiProcessor) -> i32 { acpi_tss_get_states(pr) }
unsafe fn acpi_processor_get_tsd(pr: *mut AcpiProcessor) -> i32 { acpi_tsd_get(pr) }

pub unsafe fn acpi_processor_set_throttling(pr: *mut AcpiProcessor, state: i32, force: bool) -> i32 { __acpi_processor_set_throttling(pr, state, force, false) }

pub unsafe fn acpi_processor_get_throttling_info(pr: *mut AcpiProcessor) -> i32 {
    acpi_handle_debug!((*pr).handle, "pblk_address[0x%08x] duty_offset[%d] duty_width[%d]\n", (*pr).throttling.address, (*pr).throttling.duty_offset, (*pr).throttling.duty_width);
    if acpi_processor_get_throttling_control(pr) != 0 || acpi_processor_get_throttling_states(pr) != 0 || acpi_processor_get_platform_limit(pr) != 0 {
        (*pr).throttling.acpi_processor_get_throttling = Some(acpi_processor_get_throttling_fadt);
        (*pr).throttling.acpi_processor_set_throttling = Some(acpi_processor_set_throttling_fadt);
        if acpi_processor_get_fadt_info(pr) != 0 { return 0; }
    } else {
        (*pr).throttling.acpi_processor_get_throttling = Some(acpi_processor_get_throttling_ptc);
        (*pr).throttling.acpi_processor_set_throttling = Some(acpi_processor_set_throttling_ptc);
    }
    if acpi_processor_get_tsd(pr) != 0 { let p = &mut (*pr).throttling; p.tsd_valid_flag = false; cpumask_set_cpu((*pr).id, p.shared_cpu_map); p.shared_type = DOMAIN_COORD_TYPE_SW_ALL; }
    if (*pr).errata.piix4.throttle { acpi_handle_debug!((*pr).handle, "Throttling not supported on PIIX4 A- or B-step\n"); return 0; }
    acpi_handle_debug!((*pr).handle, "Found %d throttling states\n", (*pr).throttling.state_count); (*pr).flags.throttling = 1;
    let mut result = acpi_processor_get_throttling(pr); if result == 0 && (*pr).throttling.state != 0 { result = acpi_processor_set_throttling(pr, 0, false); }
    if result != 0 { (*pr).flags.throttling = 0; } result
}

unsafe fn acpi_processor_get_throttling_fadt(pr: *mut AcpiProcessor) -> i32 { acpi_fadt_get_throttling(pr) }
unsafe fn acpi_processor_set_throttling_fadt(pr: *mut AcpiProcessor, state: i32, force: bool) -> i32 { acpi_fadt_set_throttling(pr, state, force) }
unsafe fn acpi_processor_get_throttling_ptc(pr: *mut AcpiProcessor) -> i32 { acpi_ptc_get_throttling(pr) }
unsafe fn acpi_processor_set_throttling_ptc(pr: *mut AcpiProcessor, state: i32, force: bool) -> i32 { acpi_ptc_set_throttling(pr, state, force) }
unsafe fn acpi_processor_get_fadt_info(pr: *mut AcpiProcessor) -> i32 { acpi_fadt_get_info(pr) }

pub unsafe fn __acpi_processor_set_throttling_impl(pr: *mut AcpiProcessor, state: i32, force: bool, direct: bool) -> i32 {
    if pr.is_null() { return -EINVAL; }
    if !(*pr).flags.throttling || state < 0 || state > (*pr).throttling.state_count - 1 || cpu_is_offline((*pr).id) { return -ENODEV; }
    let mut t = ThrottlingTstate { cpu: (*pr).id, target_state: state };
    let p = &(*pr).throttling;
    for_each_cpu_and!(i, cpu_online_mask, p.shared_cpu_map) { t.cpu = i; acpi_processor_throttling_notifier(THROTTLING_PRECHANGE, &mut t); }
    if p.shared_type == DOMAIN_COORD_TYPE_SW_ANY {
        let mut arg = AcpiProcessorThrottlingArg { pr, target_state: state, force };
        call_on_cpu((*pr).id, acpi_processor_throttling_fn, &mut arg, direct)
    } else {
        let mut ret = 0;
        for_each_cpu_and!(i, cpu_online_mask, p.shared_cpu_map) {
            let match_pr = per_cpu!(processors, i);
            if match_pr.is_null() || !(*match_pr).flags.throttling { continue; }
            let mut arg = AcpiProcessorThrottlingArg { pr: match_pr, target_state: state, force };
            ret = call_on_cpu((*pr).id, acpi_processor_throttling_fn, &mut arg, direct);
        }
        ret
    };
    for_each_cpu_and!(i, cpu_online_mask, p.shared_cpu_map) { t.cpu = i; acpi_processor_throttling_notifier(THROTTLING_POSTCHANGE, &mut t); }
    0
}

unsafe fn acpi_processor_throttling_fn(data: *mut core::ffi::c_void) -> i32 {
    let arg = &mut *(data as *mut AcpiProcessorThrottlingArg);
    ((*arg.pr).throttling.acpi_processor_set_throttling)(arg.pr, arg.target_state, arg.force)
}

// External declarations corresponding to the kernel structures and operations
// referenced above. Their definitions are intentionally provided by the wider
// translated repository.
extern "C" {
    fn acpi_ptc_get_control(pr: *mut AcpiProcessor) -> i32;
    fn acpi_tss_get_states(pr: *mut AcpiProcessor) -> i32;
    fn acpi_tsd_get(pr: *mut AcpiProcessor) -> i32;
    fn acpi_fadt_get_throttling(pr: *mut AcpiProcessor) -> i32;
    fn acpi_fadt_set_throttling(pr: *mut AcpiProcessor, state: i32, force: bool) -> i32;
    fn acpi_ptc_get_throttling(pr: *mut AcpiProcessor) -> i32;
    fn acpi_ptc_set_throttling(pr: *mut AcpiProcessor, state: i32, force: bool) -> i32;
    fn acpi_fadt_get_info(pr: *mut AcpiProcessor) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
