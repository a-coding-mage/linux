// SPDX-License-Identifier: GPL-2.0-only
/*
 * x86 APERF/MPERF KHz calculation for
 * /sys/.../cpufreq/scaling_cur_freq
 *
 * Copyright (C) 2017 Intel Corp.
 * Author: Len Brown <len.brown@intel.com>
 */

// C kernel dependencies are supplied by the surrounding translation unit.

#[repr(C)]
pub struct aperfmperf {
    pub seq: seqcount_t,
    pub last_update: c_ulong,
    pub acnt: u64,
    pub mcnt: u64,
    pub aperf: u64,
    pub mperf: u64,
}

static mut cpu_samples: aperfmperf = aperfmperf {
    seq: SEQCNT_ZERO!(cpu_samples.seq),
    last_update: 0,
    acnt: 0,
    mcnt: 0,
    aperf: 0,
    mperf: 0,
};

unsafe fn init_counter_refs(_data: *mut c_void) {
    let mut aperf: u64 = 0;
    let mut mperf: u64 = 0;
    rdmsrq(MSR_IA32_APERF, &mut aperf);
    rdmsrq(MSR_IA32_MPERF, &mut mperf);
    this_cpu_write!(cpu_samples.aperf, aperf);
    this_cpu_write!(cpu_samples.mperf, mperf);
}

#[cfg(all(target_arch = "x86_64", feature = "SMP"))]
{
    static mut arch_scale_freq_key: static_key_false = DEFINE_STATIC_KEY_FALSE!();
    static mut arch_turbo_freq_ratio: u64 = SCHED_CAPACITY_SCALE;
    static mut arch_max_freq_ratio: u64 = SCHED_CAPACITY_SCALE;

    pub unsafe fn arch_set_max_freq_ratio(turbo_disabled: bool) {
        arch_max_freq_ratio = if turbo_disabled { SCHED_CAPACITY_SCALE } else { arch_turbo_freq_ratio };
    }

    unsafe fn turbo_disabled() -> bool {
        let mut misc_en = 0u64;
        if rdmsrq_safe(MSR_IA32_MISC_ENABLE, &mut misc_en) != 0 { return false; }
        (misc_en & MSR_IA32_MISC_ENABLE_TURBO_DISABLE) != 0
    }

    unsafe fn slv_set_max_freq_ratio(base_freq: *mut u64, turbo_freq: *mut u64) -> bool {
        if rdmsrq_safe(MSR_ATOM_CORE_RATIOS, base_freq) != 0 { return false; }
        if rdmsrq_safe(MSR_ATOM_CORE_TURBO_RATIOS, turbo_freq) != 0 { return false; }
        *base_freq = (*base_freq >> 16) & 0x3f;
        *turbo_freq &= 0x3f;
        true
    }

    // X86_MATCH(vfm) expands to X86_MATCH_VFM_FEATURE(vfm, X86_FEATURE_APERFMPERF, NULL).
    static has_knl_turbo_ratio_limits: [x86_cpu_id; 3] = [
        X86_MATCH!(INTEL_XEON_PHI_KNL), X86_MATCH!(INTEL_XEON_PHI_KNM), x86_cpu_id::default(),
    ];
    static has_skx_turbo_ratio_limits: [x86_cpu_id; 2] = [
        X86_MATCH!(INTEL_SKYLAKE_X), x86_cpu_id::default(),
    ];
    static has_glm_turbo_ratio_limits: [x86_cpu_id; 4] = [
        X86_MATCH!(INTEL_ATOM_GOLDMONT), X86_MATCH!(INTEL_ATOM_GOLDMONT_D),
        X86_MATCH!(INTEL_ATOM_GOLDMONT_PLUS), x86_cpu_id::default(),
    ];

    unsafe fn knl_set_max_freq_ratio(base_freq: *mut u64, turbo_freq: *mut u64, num_delta_fratio: i32) -> bool {
        let mut msr = 0u64;
        if rdmsrq_safe(MSR_PLATFORM_INFO, base_freq) != 0 { return false; }
        *base_freq = (*base_freq >> 8) & 0xff;
        if rdmsrq_safe(MSR_TURBO_RATIO_LIMIT, &mut msr) != 0 { return false; }
        let mut fratio = ((msr >> 8) & 0xff) as i32;
        let mut i = 16;
        let mut found = 0;
        loop {
            if found >= num_delta_fratio { *turbo_freq = fratio as u64; return true; }
            let delta = ((msr >> (i + 5)) & 7) as i32;
            if delta != 0 { found += 1; fratio -= delta; }
            i += 8;
            if i >= 64 { break; }
        }
        true
    }

    unsafe fn skx_set_max_freq_ratio(base_freq: *mut u64, turbo_freq: *mut u64, size: i32) -> bool {
        let (mut ratios, mut counts) = (0u64, 0u64);
        if rdmsrq_safe(MSR_PLATFORM_INFO, base_freq) != 0 { return false; }
        *base_freq = (*base_freq >> 8) & 0xff;
        if rdmsrq_safe(MSR_TURBO_RATIO_LIMIT, &mut ratios) != 0 { return false; }
        if rdmsrq_safe(MSR_TURBO_RATIO_LIMIT1, &mut counts) != 0 { return false; }
        let mut i = 0;
        while i < 64 {
            let group_size = (counts >> i) & 0xff;
            if group_size >= size as u64 { *turbo_freq = (ratios >> i) & 0xff; return true; }
            i += 8;
        }
        false
    }

    unsafe fn core_set_max_freq_ratio(base_freq: *mut u64, turbo_freq: *mut u64) -> bool {
        let mut msr = 0u64;
        if rdmsrq_safe(MSR_PLATFORM_INFO, base_freq) != 0 { return false; }
        if rdmsrq_safe(MSR_TURBO_RATIO_LIMIT, &mut msr) != 0 { return false; }
        *base_freq = (*base_freq >> 8) & 0xff;
        *turbo_freq = (msr >> 24) & 0xff;
        if *turbo_freq == 0 { *turbo_freq = msr & 0xff; }
        true
    }

    unsafe fn intel_set_max_freq_ratio() -> bool {
        let (mut base_freq, mut turbo_freq) = (0u64, 0u64);
        if slv_set_max_freq_ratio(&mut base_freq, &mut turbo_freq) { return intel_set_max_freq_ratio_out(base_freq, turbo_freq); }
        if x86_match_cpu(&has_glm_turbo_ratio_limits) && skx_set_max_freq_ratio(&mut base_freq, &mut turbo_freq, 1) { return intel_set_max_freq_ratio_out(base_freq, turbo_freq); }
        if x86_match_cpu(&has_knl_turbo_ratio_limits) && knl_set_max_freq_ratio(&mut base_freq, &mut turbo_freq, 1) { return intel_set_max_freq_ratio_out(base_freq, turbo_freq); }
        if x86_match_cpu(&has_skx_turbo_ratio_limits) && skx_set_max_freq_ratio(&mut base_freq, &mut turbo_freq, 4) { return intel_set_max_freq_ratio_out(base_freq, turbo_freq); }
        if core_set_max_freq_ratio(&mut base_freq, &mut turbo_freq) { return intel_set_max_freq_ratio_out(base_freq, turbo_freq); }
        false
    }

    unsafe fn intel_set_max_freq_ratio_out(base_freq: u64, turbo_freq: u64) -> bool {
        if base_freq == 0 || turbo_freq == 0 { pr_debug!("Couldn't determine cpu base or turbo frequency, necessary for scale-invariant accounting.\n"); return false; }
        let turbo_ratio = div_u64(turbo_freq * SCHED_CAPACITY_SCALE, base_freq);
        if turbo_ratio == 0 { pr_debug!("Non-zero turbo and base frequencies led to a 0 ratio.\n"); return false; }
        arch_turbo_freq_ratio = turbo_ratio;
        arch_set_max_freq_ratio(turbo_disabled());
        true
    }

    unsafe fn scale_freq_tick(acnt: u64, mcnt: u64) {
        if !arch_scale_freq_invariant() { return; }
        let mut acnt = acnt;
        if check_shl_overflow(&mut acnt, 2 * SCHED_CAPACITY_SHIFT) { return scale_freq_tick_error(); }
        let freq_ratio = if static_branch_unlikely(&arch_hybrid_cap_scale_key) { READ_ONCE!(this_cpu_ptr(arch_cpu_scale).freq_ratio) } else { arch_max_freq_ratio };
        let mut mcnt = mcnt;
        if check_mul_overflow(&mut mcnt, freq_ratio) || mcnt == 0 { return scale_freq_tick_error(); }
        let mut freq_scale = div64_u64(acnt, mcnt);
        if freq_scale == 0 { return scale_freq_tick_error(); }
        if freq_scale > SCHED_CAPACITY_SCALE { freq_scale = SCHED_CAPACITY_SCALE; }
        this_cpu_write!(arch_freq_scale, freq_scale);
    }
    unsafe fn scale_freq_tick_error() { pr_warn!("Scheduler frequency invariance went wobbly, disabling!\n"); schedule_work(&disable_freq_invariance_work); }
}

#[cfg(not(all(target_arch = "x86_64", feature = "SMP")))]
unsafe fn scale_freq_tick(_acnt: u64, _mcnt: u64) {}

// The following scheduler-facing items preserve the corresponding C globals,
// callbacks, and interfaces; kernel allocation, static-key, and per-CPU
// primitives are supplied by the surrounding kernel bindings.
#[cfg(all(target_arch = "x86_64", feature = "SMP"))]
unsafe fn freq_invariance_enable() {
    if static_branch_unlikely(&arch_scale_freq_key) { WARN_ON_ONCE!(1); return; }
    static_branch_enable_cpuslocked(&mut arch_scale_freq_key);
    register_freq_invariance_syscore();
    pr_info!("Estimated ratio of average max frequency by base frequency (times 1024): {}\n", arch_max_freq_ratio);
}

#[cfg(all(target_arch = "x86_64", feature = "SMP"))]
pub unsafe fn freq_invariance_set_perf_ratio(ratio: u64, turbo_disabled: bool) {
    arch_turbo_freq_ratio = ratio;
    arch_set_max_freq_ratio(turbo_disabled);
    freq_invariance_enable();
}

#[cfg(all(target_arch = "x86_64", feature = "SMP"))]
unsafe fn bp_init_freq_invariance() {
    if boot_cpu_data.x86_vendor != X86_VENDOR_INTEL { return; }
    if intel_set_max_freq_ratio() {
        cpus_read_lock();
        freq_invariance_enable();
        cpus_read_unlock();
    }
}

#[cfg(all(target_arch = "x86_64", feature = "SMP"))]
unsafe fn disable_freq_invariance_workfn(_work: *mut work_struct) {
    static_branch_disable(&mut arch_scale_freq_key);
    for_each_possible_cpu!(cpu, { per_cpu!(arch_freq_scale, cpu) = SCHED_CAPACITY_SCALE; });
}

#[cfg(all(target_arch = "x86_64", feature = "SMP"))]
static mut disable_freq_invariance_work: work_struct = DECLARE_WORK!(disable_freq_invariance_workfn);

#[cfg(all(target_arch = "x86_64", feature = "SMP"))]
static mut arch_freq_scale: c_ulong = SCHED_CAPACITY_SCALE;

#[cfg(all(target_arch = "x86_64", feature = "SMP"))]
static mut arch_hybrid_cap_scale_key: static_key_false = DEFINE_STATIC_KEY_FALSE!();

#[repr(C)]
pub struct arch_hybrid_cpu_scale { pub capacity: c_ulong, pub freq_ratio: c_ulong }

#[cfg(all(target_arch = "x86_64", feature = "SMP"))]
static mut arch_cpu_scale: *mut arch_hybrid_cpu_scale = core::ptr::null_mut();

#[cfg(all(target_arch = "x86_64", feature = "SMP"))]
pub unsafe fn arch_enable_hybrid_capacity_scale() -> bool {
    if static_branch_unlikely(&arch_hybrid_cap_scale_key) { WARN_ONCE!(1, "Hybrid CPU capacity scaling already enabled"); return true; }
    arch_cpu_scale = alloc_percpu::<arch_hybrid_cpu_scale>();
    if arch_cpu_scale.is_null() { return false; }
    for_each_possible_cpu!(cpu, { per_cpu_ptr!(arch_cpu_scale, cpu).capacity = SCHED_CAPACITY_SCALE; per_cpu_ptr!(arch_cpu_scale, cpu).freq_ratio = arch_max_freq_ratio; });
    static_branch_enable(&mut arch_hybrid_cap_scale_key);
    pr_info!("Hybrid CPU capacity scaling enabled\n");
    true
}

#[cfg(all(target_arch = "x86_64", feature = "SMP"))]
pub unsafe fn arch_set_cpu_capacity(cpu: i32, cap: c_ulong, max_cap: c_ulong, cap_freq: c_ulong, base_freq: c_ulong) {
    if static_branch_likely(&arch_hybrid_cap_scale_key) {
        WRITE_ONCE!(per_cpu_ptr!(arch_cpu_scale, cpu).capacity, div_u64(cap << SCHED_CAPACITY_SHIFT, max_cap));
        WRITE_ONCE!(per_cpu_ptr!(arch_cpu_scale, cpu).freq_ratio, div_u64(cap_freq << SCHED_CAPACITY_SHIFT, base_freq));
    } else { WARN_ONCE!(1, "Hybrid CPU capacity scaling not enabled"); }
}

#[cfg(all(target_arch = "x86_64", feature = "SMP"))]
pub unsafe fn arch_scale_cpu_capacity(cpu: i32) -> c_ulong {
    if static_branch_unlikely(&arch_hybrid_cap_scale_key) { return READ_ONCE!(per_cpu_ptr!(arch_cpu_scale, cpu).capacity); }
    SCHED_CAPACITY_SCALE
}

pub unsafe fn arch_scale_freq_tick() {
    let s = this_cpu_ptr!(&mut cpu_samples);
    if !cpu_feature_enabled(X86_FEATURE_APERFMPERF) { return; }
    let (mut aperf, mut mperf) = (0u64, 0u64);
    rdmsrq(MSR_IA32_APERF, &mut aperf); rdmsrq(MSR_IA32_MPERF, &mut mperf);
    let acnt = aperf.wrapping_sub((*s).aperf); let mcnt = mperf.wrapping_sub((*s).mperf);
    (*s).aperf = aperf; (*s).mperf = mperf;
    raw_write_seqcount_begin(&mut (*s).seq); (*s).last_update = jiffies; (*s).acnt = acnt; (*s).mcnt = mcnt; raw_write_seqcount_end(&mut (*s).seq);
    scale_freq_tick(acnt, mcnt);
}

pub unsafe fn arch_freq_get_on_cpu(cpu: i32) -> u32 {
    let s = per_cpu_ptr!(&mut cpu_samples, cpu); let mut seq; let mut last; let mut acnt; let mut mcnt;
    if !cpu_feature_enabled(X86_FEATURE_APERFMPERF) { return cpufreq_quick_get(cpu).then_some(cpu_khz).unwrap_or(cpu_khz); }
    loop { seq = raw_read_seqcount_begin(&(*s).seq); last = (*s).last_update; acnt = (*s).acnt; mcnt = (*s).mcnt; if !read_seqcount_retry(&(*s).seq, seq) { break; } }
    if mcnt == 0 || jiffies.wrapping_sub(last) > (HZ / 50) { let freq = cpufreq_quick_get(cpu); return if freq != 0 { freq } else { cpu_khz }; }
    div64_u64(cpu_khz as u64 * acnt, mcnt) as u32
}

unsafe fn bp_init_aperfmperf() -> i32 {
    if !cpu_feature_enabled(X86_FEATURE_APERFMPERF) { return 0; }
    init_counter_refs(core::ptr::null_mut());
    #[cfg(all(target_arch = "x86_64", feature = "SMP"))] { bp_init_freq_invariance(); }
    0
}
early_initcall!(bp_init_aperfmperf);

pub unsafe fn ap_init_aperfmperf() {
    if cpu_feature_enabled(X86_FEATURE_APERFMPERF) { init_counter_refs(core::ptr::null_mut()); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
