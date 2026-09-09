// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright IBM Corp. 2024
 */

// Hiperdispatch implementation translated from hiperdispatch.c.

const HD_DELAY_FACTOR: u32 = 4;
const HD_DELAY_INTERVAL: u32 = HZ / 4;
const HD_STEAL_THRESHOLD: u32 = 10;
const HD_STEAL_AVG_WEIGHT: u32 = 16;

static mut hd_vl_coremask: cpumask_t = cpumask_t::new();
static mut hd_vmvl_cpumask: cpumask_t = cpumask_t::new();
static mut hd_high_capacity_cores: i32 = 0;
static mut hd_entitled_cores: i32 = 0;
static mut hd_online_cores: i32 = 0;

static mut hd_previous_steal: c_ulong = 0;
static mut hd_high_time: c_ulong = 0;
static mut hd_low_time: c_ulong = 0;
static mut hd_adjustments: atomic64_t = atomic64_t::new();

static mut hd_steal_threshold: u32 = HD_STEAL_THRESHOLD;
static mut hd_delay_factor: u32 = HD_DELAY_FACTOR;
static mut hd_enabled: i32 = 0;

unsafe extern "C" {
    fn cpu_has_topology() -> bool;
    fn cpumask_clear(mask: *mut cpumask_t);
    fn cpumask_or(dst: *mut cpumask_t, src1: *const cpumask_t, src2: *const cpumask_t);
    fn cpumask_set_cpu(cpu: i32, mask: *mut cpumask_t);
    fn smp_cpu_get_polarization(cpu: i32) -> i32;
    fn topology_sibling_cpumask(cpu: i32) -> *const cpumask_t;
    fn smp_set_core_capacity(cpu: i32, capacity: u32);
    fn cancel_delayed_work_sync(work: *mut delayed_work);
    fn mod_delayed_work(wq: *mut workqueue_struct, work: *mut delayed_work, delay: u32) -> bool;
    fn schedule_delayed_work(work: *mut delayed_work, delay: u32) -> bool;
    fn hd_capacity_work_fn(work: *mut work_struct);
    static mut system_dfl_wq: *mut workqueue_struct;
}

static mut hd_capacity_work: delayed_work = delayed_work::new(hd_capacity_work_fn);

unsafe fn hd_set_hiperdispatch_mode(mut enable: i32) -> i32 {
    if !cpu_has_topology() { enable = 0; }
    if hd_enabled == enable { return 0; }
    hd_enabled = enable;
    1
}

pub unsafe fn hd_reset_state() {
    cpumask_clear(&raw mut hd_vl_coremask);
    cpumask_clear(&raw mut hd_vmvl_cpumask);
    hd_entitled_cores = 0;
    hd_online_cores = 0;
}

pub unsafe fn hd_add_core(cpu: i32) {
    let siblings: *const cpumask_t;
    let polarization: i32;
    hd_online_cores += 1;
    polarization = smp_cpu_get_polarization(cpu);
    siblings = topology_sibling_cpumask(cpu);
    match polarization {
        POLARIZATION_VH => hd_entitled_cores += 1,
        POLARIZATION_VM => {
            hd_entitled_cores += 1;
            cpumask_or(&raw mut hd_vmvl_cpumask, &raw const hd_vmvl_cpumask, siblings);
        }
        POLARIZATION_VL => {
            cpumask_set_cpu(cpu, &raw mut hd_vl_coremask);
            cpumask_or(&raw mut hd_vmvl_cpumask, &raw const hd_vmvl_cpumask, siblings);
        }
        _ => {}
    }
}

static mut hd_counter_mutex: mutex = mutex::new();

unsafe fn hd_update_times() {
    static mut prev: ktime_t = ktime_t::zero();
    let now: ktime_t;
    if hd_entitled_cores == 0 || hd_enabled == 0 { prev = ktime_set(0, 0); return; }
    now = ktime_get();
    if ktime_after(prev, 0) {
        if hd_high_capacity_cores == hd_online_cores { hd_high_time += ktime_ms_delta(now, prev); }
        else { hd_low_time += ktime_ms_delta(now, prev); }
    }
    prev = now;
}

unsafe fn hd_update_capacities() {
    let mut capacity: u32;
    let mut upscaling_cores = hd_high_capacity_cores - hd_entitled_cores;
    capacity = if upscaling_cores > 0 { CPU_CAPACITY_HIGH } else { CPU_CAPACITY_LOW };
    hd_high_capacity_cores = hd_entitled_cores;
    for_each_cpu!(cpu, &raw const hd_vl_coremask, {
        smp_set_core_capacity(cpu, capacity);
        if capacity != CPU_CAPACITY_HIGH { continue; }
        hd_high_capacity_cores += 1;
        upscaling_cores -= 1;
        if upscaling_cores == 0 { capacity = CPU_CAPACITY_LOW; }
    });
}

pub unsafe fn hd_disable_hiperdispatch() {
    cancel_delayed_work_sync(&raw mut hd_capacity_work);
    hd_high_capacity_cores = hd_online_cores;
    hd_previous_steal = 0;
}

pub unsafe fn hd_enable_hiperdispatch() -> i32 {
    mutex_lock(&raw mut hd_counter_mutex); hd_update_times(); mutex_unlock(&raw mut hd_counter_mutex);
    if hd_enabled == 0 || hd_entitled_cores == 0 || hd_online_cores <= hd_entitled_cores { return 0; }
    mod_delayed_work(system_dfl_wq, &raw mut hd_capacity_work, HD_DELAY_INTERVAL * hd_delay_factor);
    hd_update_capacities();
    1
}

unsafe fn hd_steal_avg(new_value: c_ulong) -> c_ulong {
    static mut steal: c_ulong = 0;
    steal = (steal * (HD_STEAL_AVG_WEIGHT as c_ulong - 1) + new_value) / HD_STEAL_AVG_WEIGHT as c_ulong;
    steal
}

unsafe fn hd_calculate_steal_percentage() -> c_ulong {
    static mut prev: ktime_t = ktime_t::zero();
    let mut cpus = 0; let mut steal: c_ulong = 0;
    for_each_cpu!(cpu, &raw const hd_vmvl_cpumask, { steal += kcpustat_cpu(cpu).cpustat[CPUTIME_STEAL]; cpus += 1; });
    if cpus == 0 { return 0; }
    let now = ktime_get();
    let time_delta = ktime_to_ns(ktime_sub(now, prev));
    let mut percentage = 0;
    if steal > hd_previous_steal && hd_previous_steal != 0 {
        let steal_delta = (steal - hd_previous_steal) * 100 / time_delta;
        percentage = steal_delta / cpus;
    }
    hd_previous_steal = steal; prev = now; percentage
}

unsafe fn hd_capacity_work_fn(_work: *mut work_struct) {
    mutex_lock(&raw mut smp_cpu_state_mutex);
    if hd_online_cores <= hd_entitled_cores { topology_schedule_update(); mutex_unlock(&raw mut smp_cpu_state_mutex); return; }
    let steal_percentage = hd_steal_avg(hd_calculate_steal_percentage());
    let new_cores = if steal_percentage < hd_steal_threshold as c_ulong { hd_online_cores } else { hd_entitled_cores };
    if hd_high_capacity_cores != new_cores {
        trace_s390_hd_rebuild_domains(hd_high_capacity_cores, new_cores);
        hd_high_capacity_cores = new_cores; atomic64_inc(&raw mut hd_adjustments); topology_schedule_update();
    }
    trace_s390_hd_work_fn(steal_percentage, hd_entitled_cores, hd_high_capacity_cores);
    mutex_unlock(&raw mut smp_cpu_state_mutex);
    schedule_delayed_work(&raw mut hd_capacity_work, HD_DELAY_INTERVAL);
}

unsafe fn hiperdispatch_ctl_handler(_ctl: *const ctl_table, _write: i32, _buffer: *mut c_void, _lenp: *mut usize, _ppos: *mut loff_t) -> i32 {
    // The C implementation delegates parsing and range checking to proc_douintvec_minmax,
    // then serializes mode changes with smp_cpu_state_mutex.
    todo!("proc_douintvec_minmax and ctl_table integration are external dependencies")
}

static hiperdispatch_ctl_table: [ctl_table; 2] = [
    ctl_table { procname: "hiperdispatch", mode: 0o644, proc_handler: Some(hiperdispatch_ctl_handler), ..ctl_table::ZERO },
    ctl_table::ZERO,
];

unsafe fn hd_steal_threshold_show(_dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> isize {
    sysfs_emit(buf, "%u\n", hd_steal_threshold)
}

unsafe fn hd_steal_threshold_store(_dev: *mut device, _attr: *mut device_attribute, buf: *const c_char, count: usize) -> isize {
    let mut val: u32 = 0;
    let rc = kstrtouint(buf, 0, &raw mut val);
    if rc != 0 { return rc as isize; }
    if val > 100 { return -ERANGE as isize; }
    hd_steal_threshold = val;
    count as isize
}

unsafe fn hd_delay_factor_show(_dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> isize {
    sysfs_emit(buf, "%u\n", hd_delay_factor)
}

unsafe fn hd_delay_factor_store(_dev: *mut device, _attr: *mut device_attribute, buf: *const c_char, count: usize) -> isize {
    let mut val: u32 = 0;
    let rc = kstrtouint(buf, 0, &raw mut val);
    if rc != 0 { return rc as isize; }
    if val == 0 { return -ERANGE as isize; }
    hd_delay_factor = val;
    count as isize
}

static mut hd_attrs: [*mut attribute; 3] = [
    &raw mut dev_attr_hd_steal_threshold.attr,
    &raw mut dev_attr_hd_delay_factor.attr,
    core::ptr::null_mut(),
];

static hd_attr_group: attribute_group = attribute_group { name: "hiperdispatch", attrs: &raw mut hd_attrs };

unsafe fn hd_greedy_time_get(_unused: *mut c_void, val: *mut u64) -> i32 {
    mutex_lock(&raw mut hd_counter_mutex); hd_update_times(); *val = hd_high_time; mutex_unlock(&raw mut hd_counter_mutex); 0
}

unsafe fn hd_conservative_time_get(_unused: *mut c_void, val: *mut u64) -> i32 {
    mutex_lock(&raw mut hd_counter_mutex); hd_update_times(); *val = hd_low_time; mutex_unlock(&raw mut hd_counter_mutex); 0
}

unsafe fn hd_adjustment_count_get(_unused: *mut c_void, val: *mut u64) -> i32 {
    *val = atomic64_read(&raw const hd_adjustments); 0
}

unsafe fn hd_create_debugfs_counters() {
    let dir = debugfs_create_dir("hiperdispatch", arch_debugfs_dir);
    debugfs_create_file("conservative_time_ms", 0o400, dir, core::ptr::null_mut(), &hd_conservative_time_fops);
    debugfs_create_file("greedy_time_ms", 0o400, dir, core::ptr::null_mut(), &hd_greedy_time_fops);
    debugfs_create_file("adjustment_count", 0o400, dir, core::ptr::null_mut(), &hd_adjustments_fops);
}

unsafe fn hd_create_attributes() {
    let dev = bus_get_dev_root(&raw mut cpu_subsys);
    if dev.is_null() { return; }
    if sysfs_create_group(&(*dev).kobj, &raw const hd_attr_group) != 0 { pr_warn!("Unable to create hiperdispatch attribute group\n"); }
    put_device(dev);
}

unsafe fn hd_init() -> i32 {
    if IS_ENABLED!(CONFIG_HIPERDISPATCH_ON) { hd_set_hiperdispatch_mode(1); topology_schedule_update(); }
    if register_sysctl("s390", &raw const hiperdispatch_ctl_table).is_null() { pr_warn!("Failed to register s390.hiperdispatch sysctl attribute\n"); }
    hd_create_debugfs_counters(); hd_create_attributes(); 0
}

late_initcall!(hd_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
