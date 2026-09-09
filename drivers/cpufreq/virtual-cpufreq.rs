// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2024 Google LLC
 */

// Linux kernel dependencies supplied by the surrounding translation unit.

const REG_CUR_PERF_STATE_OFFSET: usize = 0x0;
const REG_SET_PERF_STATE_OFFSET: usize = 0x4;
const REG_PERFTBL_LEN_OFFSET: usize = 0x8;
const REG_PERFTBL_SEL_OFFSET: usize = 0xc;
const REG_PERFTBL_RD_OFFSET: usize = 0x10;
const REG_PERF_DOMAIN_OFFSET: usize = 0x14;
const PER_CPU_OFFSET: usize = 0x1000;
const PERFTBL_MAX_ENTRIES: u32 = 64;

static mut base: *mut core::ffi::c_void = core::ptr::null_mut();
static mut perftbl_num_entries: [u32; 1] = [0; 1]; // DEFINE_PER_CPU(u32, perftbl_num_entries)

unsafe fn virt_scale_freq_tick() {
    let cpu: i32 = smp_processor_id();
    let max_freq: u32 = cpufreq_get_hw_max_freq(cpu) as u32;
    let mut cur_freq: u64;
    let mut scale: usize;

    cur_freq = readl_relaxed((base as *mut u8).add(cpu as usize * PER_CPU_OFFSET
        + REG_CUR_PERF_STATE_OFFSET)) as u64;

    cur_freq <<= SCHED_CAPACITY_SHIFT;
    scale = div_u64(cur_freq, max_freq as u64) as usize;
    scale = core::cmp::min(scale, SCHED_CAPACITY_SCALE as usize);

    this_cpu_write_arch_freq_scale(scale);
}

#[repr(C)]
struct scale_freq_data {
    source: u32,
    set_freq_scale: Option<unsafe fn()>,
}

static mut virt_sfd: scale_freq_data = scale_freq_data {
    source: SCALE_FREQ_SOURCE_VIRT,
    set_freq_scale: Some(virt_scale_freq_tick),
};

unsafe fn virt_cpufreq_set_perf(policy: *mut cpufreq_policy, target_freq: u32) -> u32 {
    writel_relaxed(target_freq,
        (base as *mut u8).add((*policy).cpu as usize * PER_CPU_OFFSET
            + REG_SET_PERF_STATE_OFFSET));
    0
}

unsafe fn virt_cpufreq_fast_switch(policy: *mut cpufreq_policy, target_freq: u32) -> u32 {
    virt_cpufreq_set_perf(policy, target_freq);
    target_freq
}

unsafe fn virt_cpufreq_get_perftbl_entry(cpu: i32, idx: u32) -> u32 {
    writel_relaxed(idx, (base as *mut u8).add(cpu as usize * PER_CPU_OFFSET
        + REG_PERFTBL_SEL_OFFSET));
    readl_relaxed((base as *mut u8).add(cpu as usize * PER_CPU_OFFSET
        + REG_PERFTBL_RD_OFFSET))
}

unsafe fn virt_cpufreq_target(policy: *mut cpufreq_policy, target_freq: u32, _relation: u32) -> i32 {
    let mut freqs = cpufreq_freqs { old: (*policy).cur, new: target_freq };
    let mut ret: i32 = 0;

    cpufreq_freq_transition_begin(policy, &mut freqs);
    ret = virt_cpufreq_set_perf(policy, target_freq) as i32;
    cpufreq_freq_transition_end(policy, &mut freqs, ret != 0);
    ret
}

unsafe fn virt_cpufreq_get_sharing_cpus(policy: *mut cpufreq_policy) -> i32 {
    let cur_perf_domain = readl_relaxed((base as *mut u8).add((*policy).cpu as usize
        * PER_CPU_OFFSET + REG_PERF_DOMAIN_OFFSET));
    for_each_present_cpu!(cpu => {
        let cpu_dev = get_cpu_device(cpu);
        if cpu_dev.is_null() { continue; }
        let perf_domain = readl_relaxed((base as *mut u8).add(cpu as usize * PER_CPU_OFFSET
            + REG_PERF_DOMAIN_OFFSET));
        if perf_domain == cur_perf_domain { cpumask_set_cpu(cpu, (*policy).cpus); }
    });
    0
}

unsafe fn virt_cpufreq_get_freq_info(policy: *mut cpufreq_policy) -> i32 {
    let num_perftbl_entries = per_cpu(perftbl_num_entries, (*policy).cpu);
    if num_perftbl_entries == 1 {
        (*policy).cpuinfo.min_freq = 1;
        (*policy).cpuinfo.max_freq = virt_cpufreq_get_perftbl_entry((*policy).cpu, 0);
        (*policy).cur = (*policy).cpuinfo.max_freq;
        return 0;
    }

    let table = kzalloc_frequency_table((num_perftbl_entries + 1) as usize);
    if table.is_null() { return -ENOMEM; }
    for idx in 0..num_perftbl_entries {
        (*table.add(idx as usize)).frequency = virt_cpufreq_get_perftbl_entry((*policy).cpu, idx);
    }
    (*table.add(num_perftbl_entries as usize)).frequency = CPUFREQ_TABLE_END;
    (*policy).freq_table = table;
    0
}

unsafe fn virt_cpufreq_cpu_init(policy: *mut cpufreq_policy) -> i32 {
    let cpu_dev = get_cpu_device((*policy).cpu);
    if cpu_dev.is_null() { return -ENODEV; }
    let ret = virt_cpufreq_get_freq_info(policy);
    if ret != 0 { dev_warn(cpu_dev, "failed to get cpufreq info\n"); return ret; }
    let ret = virt_cpufreq_get_sharing_cpus(policy);
    if ret != 0 { dev_warn(cpu_dev, "failed to get sharing cpumask\n"); return ret; }

    /* Ensure the vCPU thread triggering the MMIO abort is updated. */
    (*policy).dvfs_possible_from_any_cpu = false;
    (*policy).fast_switch_possible = true;
    /* Additional FIE source for accurate frequency-scale updates. */
    topology_set_scale_freq_source(&mut virt_sfd, (*policy).cpus);
    0
}

unsafe fn virt_cpufreq_cpu_exit(policy: *mut cpufreq_policy) {
    topology_clear_scale_freq_source(SCALE_FREQ_SOURCE_VIRT, (*policy).related_cpus);
    kfree((*policy).freq_table);
}

unsafe fn virt_cpufreq_online(_policy: *mut cpufreq_policy) -> i32 { 0 }

unsafe fn virt_cpufreq_offline(_policy: *mut cpufreq_policy) -> i32 { 0 }

unsafe fn virt_cpufreq_verify_policy(policy: *mut cpufreq_policy_data) -> i32 {
    if !(*policy).freq_table.is_null() { return cpufreq_frequency_table_verify(policy); }
    cpufreq_verify_within_cpu_limits(policy);
    0
}

#[repr(C)]
struct cpufreq_driver {
    name: *const core::ffi::c_char,
    init: Option<unsafe fn(*mut cpufreq_policy) -> i32>,
    exit: Option<unsafe fn(*mut cpufreq_policy)>,
    online: Option<unsafe fn(*mut cpufreq_policy) -> i32>,
    offline: Option<unsafe fn(*mut cpufreq_policy) -> i32>,
    verify: Option<unsafe fn(*mut cpufreq_policy_data) -> i32>,
    target: Option<unsafe fn(*mut cpufreq_policy, u32, u32) -> i32>,
    fast_switch: Option<unsafe fn(*mut cpufreq_policy, u32) -> u32>,
}

static mut cpufreq_virt_driver: cpufreq_driver = cpufreq_driver {
    name: b"virt-cpufreq\0".as_ptr() as *const _,
    init: Some(virt_cpufreq_cpu_init), exit: Some(virt_cpufreq_cpu_exit),
    online: Some(virt_cpufreq_online), offline: Some(virt_cpufreq_offline),
    verify: Some(virt_cpufreq_verify_policy), target: Some(virt_cpufreq_target),
    fast_switch: Some(virt_cpufreq_fast_switch),
};

unsafe fn virt_cpufreq_driver_probe(pdev: *mut platform_device) -> i32 {
    base = devm_platform_ioremap_resource(pdev, 0);
    if is_err(base) { return ptr_err(base); }
    for_each_possible_cpu!(cpu => {
        let n = readl_relaxed((base as *mut u8).add(cpu as usize * PER_CPU_OFFSET
            + REG_PERFTBL_LEN_OFFSET));
        if n == 0 || n > PERFTBL_MAX_ENTRIES { return -ENODEV; }
        per_cpu_write(perftbl_num_entries, cpu, n);
    });
    let ret = cpufreq_register_driver(&mut cpufreq_virt_driver);
    if ret != 0 { dev_err((*pdev).dev, "Virtual CPUFreq driver failed to register: %d\n", ret); return ret; }
    dev_dbg((*pdev).dev, "Virtual CPUFreq driver initialized\n");
    0
}

unsafe fn virt_cpufreq_driver_remove(_pdev: *mut platform_device) {
    cpufreq_unregister_driver(&mut cpufreq_virt_driver);
}

// Device-tree match table and platform-driver registration are supplied by the kernel bindings.
unsafe fn virt_cpufreq_init() -> i32 { platform_driver_register(virt_cpufreq_driver()) }
unsafe fn virt_cpufreq_exit() { platform_driver_unregister(virt_cpufreq_driver()); }

// MODULE_DESCRIPTION("Virtual cpufreq driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
