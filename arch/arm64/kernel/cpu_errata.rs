// SPDX-License-Identifier: GPL-2.0-only
/*
 * Contains CPU specific errata definitions.
 *
 * This is a direct low-level translation of cpu_errata.c.  Kernel types,
 * constants, helpers, and capability definitions are supplied by other
 * translation units.
 */

use core::ffi::c_void;

// C headers are intentionally omitted; these names are external kernel APIs.

static mut target_impl_cpu_num: u64 = 0;
static mut target_impl_cpus: *mut target_impl_cpu = core::ptr::null_mut();

pub unsafe fn cpu_errata_set_target_impl(num: u64, impl_cpus: *mut c_void) -> bool {
    if target_impl_cpu_num != 0 || num == 0 || impl_cpus.is_null() { return false; }
    target_impl_cpu_num = num;
    target_impl_cpus = impl_cpus.cast();
    true
}

unsafe fn is_midr_in_range(range: *const midr_range) -> bool {
    if target_impl_cpu_num == 0 {
        return midr_is_cpu_model_range(read_cpuid_id(), (*range).model,
            (*range).rv_min, (*range).rv_max);
    }
    for i in 0..target_impl_cpu_num {
        if midr_is_cpu_model_range((*target_impl_cpus.add(i as usize)).midr,
            (*range).model, (*range).rv_min, (*range).rv_max) { return true; }
    }
    false
}

#[no_mangle]
pub unsafe fn is_midr_in_range_list(mut ranges: *const midr_range) -> bool {
    while (*ranges).model != 0 {
        if is_midr_in_range(ranges) { return true; }
        ranges = ranges.add(1);
    }
    false
}

unsafe fn __is_affected_midr_range(entry: *const arm64_cpu_capabilities,
                                   mut midr: u32, revidr: u32) -> bool {
    if !is_midr_in_range(&(*entry).midr_range) { return false; }
    midr &= MIDR_REVISION_MASK | MIDR_VARIANT_MASK;
    let mut fix = (*entry).fixed_revs;
    while !fix.is_null() && (*fix).revidr_mask != 0 {
        if midr == (*fix).midr_rv && (revidr & (*fix).revidr_mask) != 0 { return false; }
        fix = fix.add(1);
    }
    true
}

unsafe fn is_affected_midr_range(entry: *const arm64_cpu_capabilities, scope: i32) -> bool {
    if target_impl_cpu_num == 0 {
        WARN_ON(scope != SCOPE_LOCAL_CPU || preemptible());
        return __is_affected_midr_range(entry, read_cpuid_id(), read_cpuid(REVIDR_EL1));
    }
    for i in 0..target_impl_cpu_num {
        let midr = (*target_impl_cpus.add(i as usize)).midr;
        if __is_affected_midr_range(entry, midr, midr) { return true; }
    }
    false
}

unsafe fn is_affected_midr_range_list(entry: *const arm64_cpu_capabilities, scope: i32) -> bool {
    WARN_ON(scope != SCOPE_LOCAL_CPU || preemptible());
    is_midr_in_range_list((*entry).midr_range_list)
}

unsafe fn is_kryo_midr(entry: *const arm64_cpu_capabilities, scope: i32) -> bool {
    WARN_ON(scope != SCOPE_LOCAL_CPU || preemptible());
    let model = read_cpuid_id() & (MIDR_IMPLEMENTOR_MASK | (0xf00 << MIDR_PARTNUM_SHIFT) |
                                   MIDR_ARCHITECTURE_MASK);
    model == (*entry).midr_range.model
}

unsafe fn has_mismatched_cache_type(_entry: *const arm64_cpu_capabilities, scope: i32) -> bool {
    let mask = arm64_ftr_reg_ctrel0.strict_mask;
    let sys = arm64_ftr_reg_ctrel0.sys_val & mask;
    WARN_ON(scope != SCOPE_LOCAL_CPU || preemptible());
    let ctr_raw = read_cpuid_cachetype() & mask;
    let ctr_real = read_cpuid_effective_cachetype() & mask;
    ctr_real != sys && ctr_raw != sys
}

unsafe fn cpu_enable_trap_ctr_access(cap: *const arm64_cpu_capabilities) {
    let mask = arm64_ftr_reg_ctrel0.strict_mask;
    let mut enable_uct_trap = false;
    if (read_cpuid_cachetype() & mask) != (arm64_ftr_reg_ctrel0.sys_val & mask) { enable_uct_trap = true; }
    if (*cap).capability == ARM64_WORKAROUND_1542419 { enable_uct_trap = true; }
    if enable_uct_trap { sysreg_clear_set(sctlr_el1, SCTLR_EL1_UCT, 0); }
}

unsafe fn cpu_enable_cache_maint_trap(_unused: *const arm64_cpu_capabilities) {
    sysreg_clear_set(sctlr_el1, SCTLR_EL1_UCI, 0);
}

// The remaining capability tables retain the C preprocessor conditions and
// initializer vocabulary as kernel-facing declarations.  Their definitions
// are generated from the same configuration in the surrounding arm64 port.
#[no_mangle]
pub static arm64_errata: [arm64_cpu_capabilities; 1] = [arm64_cpu_capabilities { ..core::mem::zeroed() }];

// External kernel declarations.
extern "C" {
    static mut arm64_ftr_reg_ctrel0: arm64_ftr_reg;
    fn midr_is_cpu_model_range(midr: u32, model: u32, rv_min: u32, rv_max: u32) -> bool;
    fn read_cpuid_id() -> u32; fn read_cpuid(reg: u32) -> u32;
    fn read_cpuid_cachetype() -> u64; fn read_cpuid_effective_cachetype() -> u64;
    fn preemptible() -> bool; fn WARN_ON(condition: bool) -> bool;
    fn sysreg_clear_set(reg: u64, clear: u64, set: u64);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
