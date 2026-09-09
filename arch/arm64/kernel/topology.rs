/* arch/arm64/kernel/topology.c — source-level Rust translation */

// Kernel dependencies supplied by the surrounding translation unit.

#[cfg(CONFIG_ARM64_AMU_EXTN)]
macro_rules! read_corecnt { () => { read_sysreg_s(SYS_AMEVCNTR0_CORE_EL0) }; }
#[cfg(CONFIG_ARM64_AMU_EXTN)]
macro_rules! read_constcnt { () => { read_sysreg_s(SYS_AMEVCNTR0_CONST_EL0) }; }
#[cfg(not(CONFIG_ARM64_AMU_EXTN))]
macro_rules! read_corecnt { () => { 0u64 }; }
#[cfg(not(CONFIG_ARM64_AMU_EXTN))]
macro_rules! read_constcnt { () => { 0u64 }; }

static mut arch_max_freq_scale: PerCpuReadMostly<usize> =
    PerCpuReadMostly::new(1usize << (2 * SCHED_CAPACITY_SHIFT));
static mut amu_fie_cpus: cpumask_var_t = core::ptr::null_mut();

#[repr(C)]
struct amu_cntr_sample {
    arch_const_cycles_prev: u64,
    arch_core_cycles_prev: u64,
    last_scale_update: usize,
}

static mut cpu_amu_samples: PerCpuSharedAligned<amu_cntr_sample> =
    PerCpuSharedAligned::new();

pub unsafe fn update_freq_counters_refs() {
    let amu_sample = this_cpu_ptr(&raw mut cpu_amu_samples);
    (*amu_sample).arch_core_cycles_prev = read_corecnt!();
    (*amu_sample).arch_const_cycles_prev = read_constcnt!();
}

#[inline]
unsafe fn freq_counters_valid(cpu: i32) -> bool {
    let amu_sample = per_cpu_ptr(&raw mut cpu_amu_samples, cpu);
    if cpu >= nr_cpu_ids || !cpumask_test_cpu(cpu, cpu_present_mask) { return false; }
    if !cpu_has_amu_feat(cpu) { pr_debug!("CPU{}: counters are not supported.\n", cpu); return false; }
    if (*amu_sample).arch_const_cycles_prev == 0 || (*amu_sample).arch_core_cycles_prev == 0 {
        pr_debug!("CPU{}: cycle counters are not enabled.\n", cpu); return false;
    }
    true
}

pub unsafe fn freq_inv_set_max_ratio(cpu: i32, max_rate: u64) {
    let ref_rate = arch_timer_get_rate();
    if max_rate == 0 || ref_rate == 0 { WARN_ONCE!(true, "CPU{}: invalid maximum or reference frequency.\n", cpu); return; }
    let mut ratio = ref_rate << (2 * SCHED_CAPACITY_SHIFT);
    ratio = div64_u64(ratio, max_rate);
    if ratio == 0 { WARN_ONCE!(true, "Reference frequency too low.\n"); return; }
    WRITE_ONCE!(per_cpu(arch_max_freq_scale, cpu), ratio as usize);
}

unsafe fn amu_scale_freq_tick() {
    let amu_sample = this_cpu_ptr(&raw mut cpu_amu_samples);
    let prev_core_cnt = (*amu_sample).arch_core_cycles_prev;
    let prev_const_cnt = (*amu_sample).arch_const_cycles_prev;
    update_freq_counters_refs();
    let const_cnt = (*amu_sample).arch_const_cycles_prev;
    let core_cnt = (*amu_sample).arch_core_cycles_prev;
    if core_cnt <= prev_core_cnt || const_cnt <= prev_const_cnt { return; }
    let mut scale = (core_cnt - prev_core_cnt) * this_cpu_read!(arch_max_freq_scale);
    scale = div64_u64(scale >> SCHED_CAPACITY_SHIFT, const_cnt - prev_const_cnt);
    scale = core::cmp::min(scale as usize, SCHED_CAPACITY_SCALE) as u64;
    this_cpu_write!(arch_freq_scale, scale as usize);
    (*amu_sample).last_scale_update = jiffies;
}

static mut amu_sfd: scale_freq_data = scale_freq_data {
    source: SCALE_FREQ_SOURCE_ARCH,
    set_freq_scale: Some(amu_scale_freq_tick),
};

#[inline(always)]
unsafe fn amu_fie_cpu_supported(cpu: u32) -> bool {
    cpumask_available(amu_fie_cpus) && cpumask_test_cpu(cpu as i32, amu_fie_cpus)
}

pub unsafe fn arch_cpu_idle_enter() {
    let cpu = smp_processor_id();
    if !amu_fie_cpu_supported(cpu) { return; }
    if housekeeping_cpu(cpu, HK_TYPE_TICK) && time_is_before_jiffies(per_cpu!(cpu_amu_samples.last_scale_update, cpu)) {
        amu_scale_freq_tick();
    }
}

const AMU_SAMPLE_EXP_MS: usize = 20;

pub unsafe fn arch_freq_get_on_cpu(mut cpu: i32) -> i32 {
    let start_cpu = cpu;
    if !amu_fie_cpu_supported(cpu as u32) || !arch_scale_freq_ref(cpu) { return -EOPNOTSUPP; }
    loop {
        let amu_sample = per_cpu_ptr(&raw mut cpu_amu_samples, cpu);
        let last_update = (*amu_sample).last_scale_update;
        if !housekeeping_cpu(cpu as u32, HK_TYPE_TICK) || time_is_before_jiffies(last_update + msecs_to_jiffies(AMU_SAMPLE_EXP_MS)) {
            let policy = cpufreq_cpu_get(cpu);
            if policy.is_null() { return -EINVAL; }
            if !cpumask_intersects((*policy).related_cpus, housekeeping_cpumask(HK_TYPE_TICK)) { cpufreq_cpu_put(policy); return -EOPNOTSUPP; }
            let mut ref_cpu = nr_cpu_ids;
            for_each_cpu_wrap!(ref_cpu, (*policy).cpus, cpu + 1, {
                if ref_cpu == start_cpu { ref_cpu = nr_cpu_ids; break; }
                if !idle_cpu(ref_cpu) { break; }
            });
            cpufreq_cpu_put(policy);
            if ref_cpu >= nr_cpu_ids { return -EAGAIN; }
            cpu = ref_cpu;
        } else { break; }
    }
    let mut freq = arch_scale_freq_capacity(cpu) * arch_scale_freq_ref(cpu);
    freq >>= SCHED_CAPACITY_SHIFT;
    freq as i32
}

unsafe fn amu_fie_setup(cpus: *const cpumask) {
    let mut cpu = 0;
    if cpumask_available(amu_fie_cpus) && unlikely(cpumask_subset(cpus, amu_fie_cpus)) { return; }
    for_each_cpu!(cpu, cpus) { if !freq_counters_valid(cpu) { return; } }
    if !cpumask_available(amu_fie_cpus) && !zalloc_cpumask_var(&raw mut amu_fie_cpus, GFP_KERNEL) { WARN_ONCE!(true, "Failed to allocate FIE cpumask for CPUs[%*pbl]\n", cpumask_pr_args(cpus)); return; }
    cpumask_or(amu_fie_cpus, amu_fie_cpus, cpus);
    topology_set_scale_freq_source(&raw mut amu_sfd, cpus);
    pr_debug!("CPUs[%*pbl]: counters will be used for FIE.", cpumask_pr_args(cpus));
}

unsafe fn init_amu_fie_callback(_nb: *mut notifier_block, val: usize, data: *mut core::ffi::c_void) -> i32 {
    if val == CPUFREQ_CREATE_POLICY { amu_fie_setup((* (data as *mut cpufreq_policy)).cpus); }
    0
}

static mut init_amu_fie_notifier: notifier_block = notifier_block { notifier_call: Some(init_amu_fie_callback) };

unsafe fn cpuhp_topology_online(cpu: u32) -> i32 {
    let policy = cpufreq_cpu_policy(cpu);
    if policy.is_null() || !cpumask_available(amu_fie_cpus) || cpumask_test_cpu(cpu as i32, amu_fie_cpus) { return 0; }
    if !cpumask_subset((*policy).cpus, amu_fie_cpus) { return 0; }
    if !freq_counters_valid(cpu as i32) { topology_clear_scale_freq_source(SCALE_FREQ_SOURCE_ARCH, (*policy).related_cpus); cpumask_andnot(amu_fie_cpus, amu_fie_cpus, (*policy).related_cpus); return 0; }
    cpumask_set_cpu(cpu as i32, amu_fie_cpus);
    topology_set_scale_freq_source(&raw mut amu_sfd, cpumask_of(cpu));
    pr_debug!("CPU[{}]: counter will be used for FIE.", cpu);
    0
}

unsafe fn init_amu_fie() -> i32 {
    let mut ret = cpufreq_register_notifier(&raw mut init_amu_fie_notifier, CPUFREQ_POLICY_NOTIFIER);
    if ret != 0 { return ret; }
    ret = cpuhp_setup_state_nocalls(CPUHP_AP_ONLINE_DYN, "arm64/topology:online", cpuhp_topology_online, None);
    if ret < 0 { cpufreq_unregister_notifier(&raw mut init_amu_fie_notifier, CPUFREQ_POLICY_NOTIFIER); return ret; }
    0
}

core_initcall!(init_amu_fie);

#[cfg(CONFIG_ACPI_CPPC_LIB)]
#[repr(C)] struct amu_ffh_ctrs { corecnt: u64, constcnt: u64 }
#[cfg(CONFIG_ACPI_CPPC_LIB)]
const CPC_FFH_CTR_CORE: u64 = 0x0;
#[cfg(CONFIG_ACPI_CPPC_LIB)]
const CPC_FFH_CTR_CONST: u64 = 0x1;

#[cfg(CONFIG_ACPI_CPPC_LIB)]
unsafe fn cpu_read_corecnt(val: *mut core::ffi::c_void) { *(val as *mut u64) = read_corecnt!(); }
#[cfg(CONFIG_ACPI_CPPC_LIB)]
unsafe fn cpu_read_constcnt(val: *mut core::ffi::c_void) { *(val as *mut u64) = if this_cpu_has_cap(ARM64_WORKAROUND_2457168) { 0 } else { read_constcnt!() }; }

#[cfg(CONFIG_ACPI_CPPC_LIB)]
unsafe fn counters_read_on_cpu(cpu: i32, func: unsafe fn(*mut core::ffi::c_void), val: *mut core::ffi::c_void) -> i32 {
    if !cpu_has_amu_feat(cpu) { return -EOPNOTSUPP; }
    if irqs_disabled() { if WARN_ON_ONCE!(cpu != smp_processor_id()) { return -EPERM; } func(val); } else { smp_call_function_single(cpu, func, val, 1); }
    0
}

#[cfg(CONFIG_ACPI_CPPC_LIB)]
pub unsafe fn cpc_ffh_supported() -> bool { let cpu = get_cpu_with_amu_feat(); cpu < nr_cpu_ids && cpumask_test_cpu(cpu, cpu_present_mask) }

#[cfg(CONFIG_ACPI_CPPC_LIB)]
unsafe fn cpc_ffh_extract_bits(reg: *const cpc_reg, mut val: u64) -> u64 { val &= GENMASK_ULL!((*reg).bit_offset + (*reg).bit_width - 1, (*reg).bit_offset); val >>= (*reg).bit_offset; val }

#[cfg(CONFIG_ACPI_CPPC_LIB)]
unsafe fn amu_read_core_const_ctrs(val: *mut core::ffi::c_void) {
    let ctrs = val as *mut amu_ffh_ctrs;
    cpu_read_constcnt(&mut (*ctrs).constcnt as *mut _ as *mut core::ffi::c_void);
    cpu_read_corecnt(&mut (*ctrs).corecnt as *mut _ as *mut core::ffi::c_void);
}

#[cfg(CONFIG_ACPI_CPPC_LIB)]
unsafe fn cpc_ffh_ctr_value(reg: *const cpc_reg, ctrs: *const amu_ffh_ctrs, val: *mut u64) {
    match (*reg).address as u64 {
        CPC_FFH_CTR_CORE => *val = (*ctrs).corecnt,
        CPC_FFH_CTR_CONST => *val = (*ctrs).constcnt,
        _ => {}
    }
    *val = cpc_ffh_extract_bits(reg, *val);
}

#[cfg(CONFIG_ACPI_CPPC_LIB)]
unsafe fn is_amu_ctr_reg(reg: *const cpc_reg) -> bool {
    (*reg).address as u64 == CPC_FFH_CTR_CORE || (*reg).address as u64 == CPC_FFH_CTR_CONST
}

#[cfg(CONFIG_ACPI_CPPC_LIB)]
pub unsafe fn cpc_read_ffh_fb_ctrs(cpu: i32, reg1: *mut cpc_reg, val1: *mut u64, reg2: *mut cpc_reg, val2: *mut u64) -> i32 {
    if !is_amu_ctr_reg(reg1) || !is_amu_ctr_reg(reg2) { return -EINVAL; }
    let mut ctrs = amu_ffh_ctrs { corecnt: 0, constcnt: 0 };
    let ret = counters_read_on_cpu(cpu, amu_read_core_const_ctrs, &mut ctrs as *mut _ as *mut core::ffi::c_void);
    if ret != 0 { return if ret == -EOPNOTSUPP { -ENODEV } else { ret }; }
    cpc_ffh_ctr_value(reg1, &ctrs, val1);
    cpc_ffh_ctr_value(reg2, &ctrs, val2);
    0
}

#[cfg(CONFIG_ACPI_CPPC_LIB)]
pub unsafe fn cpc_read_ffh(cpu: i32, reg: *mut cpc_reg, val: *mut u64) -> i32 {
    let ret = match (*reg).address as u64 {
        CPC_FFH_CTR_CORE => counters_read_on_cpu(cpu, cpu_read_corecnt, val as *mut _),
        CPC_FFH_CTR_CONST => counters_read_on_cpu(cpu, cpu_read_constcnt, val as *mut _),
        _ => -EOPNOTSUPP,
    };
    if ret == 0 { *val = cpc_ffh_extract_bits(reg, *val); }
    ret
}

#[cfg(CONFIG_ACPI_CPPC_LIB)]
pub unsafe fn cpc_write_ffh(_cpunum: i32, _reg: *mut cpc_reg, _val: u64) -> i32 { -EOPNOTSUPP }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
