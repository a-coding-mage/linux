// SPDX-License-Identifier: GPL-2.0
/* Arch specific cpu topology information */

/* External kernel types, constants, macros, and functions are supplied by other translation units. */

static mut SFT_DATA: [*mut scale_freq_data; NR_CPUS] = [core::ptr::null_mut(); NR_CPUS];
static mut SCALE_FREQ_COUNTERS_MASK: cpumask = unsafe { core::mem::zeroed() };
static mut SCALE_FREQ_INVARIANT: bool = false;
static mut CAPACITY_FREQ_REF: [c_ulong; NR_CPUS] = [0; NR_CPUS];

unsafe fn supports_scale_freq_counters(cpus: *const cpumask) -> bool {
    let mut i: c_int = 0;
    for_each_cpu!(i, cpus) {
        if cpumask_test_cpu(i, &SCALE_FREQ_COUNTERS_MASK) { return true; }
    }
    false
}

pub unsafe fn topology_scale_freq_invariant() -> bool {
    cpufreq_supports_freq_invariance() || supports_scale_freq_counters(cpu_online_mask)
}

unsafe fn update_scale_freq_invariant(status: bool) {
    if SCALE_FREQ_INVARIANT == status { return; }
    /* Task scheduler behavior depends on frequency invariance support. */
    if topology_scale_freq_invariant() == status {
        SCALE_FREQ_INVARIANT = status;
        rebuild_sched_domains_energy();
    }
}

pub unsafe fn topology_set_scale_freq_source(data: *mut scale_freq_data, cpus: *const cpumask) {
    let mut cpu: c_int = 0;
    if cpumask_empty(&SCALE_FREQ_COUNTERS_MASK) { SCALE_FREQ_INVARIANT = topology_scale_freq_invariant(); }
    rcu_read_lock();
    for_each_cpu!(cpu, cpus) {
        let sfd = rcu_dereference!(SFT_DATA[cpu as usize]);
        if sfd.is_null() || (*sfd).source != SCALE_FREQ_SOURCE_ARCH {
            rcu_assign_pointer!(SFT_DATA[cpu as usize], data);
            cpumask_set_cpu(cpu, &mut SCALE_FREQ_COUNTERS_MASK);
        }
    }
    rcu_read_unlock();
    update_scale_freq_invariant(true);
}

pub unsafe fn topology_clear_scale_freq_source(source: enum_scale_freq_source, cpus: *const cpumask) {
    let mut cpu: c_int = 0;
    rcu_read_lock();
    for_each_cpu!(cpu, cpus) {
        let sfd = rcu_dereference!(SFT_DATA[cpu as usize]);
        if !sfd.is_null() && (*sfd).source == source {
            rcu_assign_pointer!(SFT_DATA[cpu as usize], core::ptr::null_mut());
            cpumask_clear_cpu(cpu, &mut SCALE_FREQ_COUNTERS_MASK);
        }
    }
    rcu_read_unlock();
    synchronize_rcu();
    update_scale_freq_invariant(false);
}

pub unsafe fn topology_scale_freq_tick() {
    let sfd = rcu_dereference_sched!(SFT_DATA[this_cpu_id()]);
    if !sfd.is_null() { ((*sfd).set_freq_scale)(); }
}

static mut ARCH_FREQ_SCALE: [c_ulong; NR_CPUS] = [SCHED_CAPACITY_SCALE; NR_CPUS];

pub unsafe fn topology_set_freq_scale(cpus: *const cpumask, cur_freq: c_ulong, max_freq: c_ulong) {
    if warn_on_once!(cur_freq == 0 || max_freq == 0) { return; }
    if supports_scale_freq_counters(cpus) { return; }
    let scale = (cur_freq << SCHED_CAPACITY_SHIFT) / max_freq;
    let mut i: c_int = 0;
    for_each_cpu!(i, cpus) { ARCH_FREQ_SCALE[i as usize] = scale; }
}

static mut HW_PRESSURE: [c_ulong; NR_CPUS] = [0; NR_CPUS];

pub unsafe fn topology_update_hw_pressure(cpus: *const cpumask, capped_freq: c_ulong) {
    let cpu = cpumask_first(cpus);
    let max_capacity = arch_scale_cpu_capacity(cpu);
    let max_freq: u32 = arch_scale_freq_ref(cpu);
    let capacity = if max_freq as c_ulong <= capped_freq { max_capacity } else { mult_frac(max_capacity, capped_freq, max_freq as c_ulong) };
    let pressure = max_capacity - capacity;
    trace_hw_pressure_update(cpu, pressure);
    let mut c: c_int = 0;
    for_each_cpu!(c, cpus) { write_once!(&mut HW_PRESSURE[c as usize], pressure); }
}

static mut UPDATE_TOPOLOGY: c_int = 0;
pub unsafe fn topology_update_cpu_topology() -> c_int { UPDATE_TOPOLOGY }
unsafe fn update_topology_flags_workfn(_work: *mut work_struct) {
    UPDATE_TOPOLOGY = 1; rebuild_sched_domains(); pr_debug!("sched_domain hierarchy rebuilt, flags updated\n"); UPDATE_TOPOLOGY = 0;
}

static mut RAW_CAPACITY: *mut u32 = core::ptr::null_mut();
unsafe fn free_raw_capacity() -> c_int { kfree(RAW_CAPACITY as *mut c_void); RAW_CAPACITY = core::ptr::null_mut(); 0 }

pub unsafe fn topology_normalize_cpu_scale() {
    if RAW_CAPACITY.is_null() { return; }
    let mut capacity_scale: u64 = 1; let mut cpu: c_int = 0;
    for_each_possible_cpu!(cpu) { let capacity = (*RAW_CAPACITY.add(cpu as usize) as u64) * (if CAPACITY_FREQ_REF[cpu as usize] != 0 { CAPACITY_FREQ_REF[cpu as usize] } else { 1 }); capacity_scale = max(capacity, capacity_scale); }
    pr_debug!("cpu_capacity: capacity_scale=%llu\n", capacity_scale);
    for_each_possible_cpu!(cpu) { let mut capacity = (*RAW_CAPACITY.add(cpu as usize) as u64) * (if CAPACITY_FREQ_REF[cpu as usize] != 0 { CAPACITY_FREQ_REF[cpu as usize] } else { 1 }); capacity = div64_u64(capacity << SCHED_CAPACITY_SHIFT, capacity_scale); topology_set_cpu_scale(cpu, capacity); pr_debug!("cpu_capacity: CPU%d cpu_capacity=%lu\n", cpu, topology_get_cpu_scale(cpu)); }
}

pub unsafe fn topology_parse_cpu_capacity(cpu_node: *mut device_node, cpu: c_int) -> bool {
    static mut CAP_PARSING_FAILED: bool = false; let mut cpu_capacity: u32 = 0;
    if CAP_PARSING_FAILED { return false; }
    let ret = of_property_read_u32(cpu_node, b"capacity-dmips-mhz\0".as_ptr() as *const c_char, &mut cpu_capacity);
    if ret == 0 {
        if RAW_CAPACITY.is_null() { RAW_CAPACITY = kcalloc(num_possible_cpus(), core::mem::size_of::<u32>(), GFP_KERNEL) as *mut u32; if RAW_CAPACITY.is_null() { CAP_PARSING_FAILED = true; return false; } }
        *RAW_CAPACITY.add(cpu as usize) = cpu_capacity;
        let cpu_clk = of_clk_get(cpu_node, 0);
        if !IS_ERR_OR_NULL(cpu_clk) { CAPACITY_FREQ_REF[cpu as usize] = clk_get_rate(cpu_clk) / HZ_PER_KHZ; clk_put(cpu_clk); }
    } else { if !RAW_CAPACITY.is_null() { pr_err!("cpu_capacity: missing %pOF raw capacity\n", cpu_node); pr_err!("cpu_capacity: partial information: fallback to 1024 for all CPUs\n"); } CAP_PARSING_FAILED = true; free_raw_capacity(); }
    ret == 0
}

pub unsafe fn freq_inv_set_max_ratio(_cpu: c_int, _max_rate: u64) {}

/* ACPI/CPU-frequency conditional implementation is preserved below. */
#[cfg(any(CONFIG_ARM64, CONFIG_RISCV))]
static mut MAX_SMT_THREAD_NUM: c_uint = 1;

pub static mut CPU_TOPOLOGY: [cpu_topology; NR_CPUS] = [cpu_topology::default(); NR_CPUS];

pub unsafe fn cpu_coregroup_mask(cpu: c_int) -> *const cpumask {
    let mut core_mask = cpumask_of_node(cpu_to_node(cpu));
    if cpumask_subset(&CPU_TOPOLOGY[cpu as usize].core_sibling, core_mask) { core_mask = &CPU_TOPOLOGY[cpu as usize].core_sibling; }
    if last_level_cache_is_valid(cpu) && cpumask_subset(&CPU_TOPOLOGY[cpu as usize].llc_sibling, core_mask) { core_mask = &CPU_TOPOLOGY[cpu as usize].llc_sibling; }
    if IS_ENABLED!(CONFIG_SCHED_CLUSTER) && cpumask_subset(core_mask, &CPU_TOPOLOGY[cpu as usize].cluster_sibling) { core_mask = &CPU_TOPOLOGY[cpu as usize].cluster_sibling; }
    core_mask
}

pub unsafe fn cpu_clustergroup_mask(cpu: c_int) -> *const cpumask {
    if cpumask_subset(cpu_coregroup_mask(cpu), &CPU_TOPOLOGY[cpu as usize].cluster_sibling) { return topology_sibling_cpumask(cpu); }
    &CPU_TOPOLOGY[cpu as usize].cluster_sibling
}

pub unsafe fn update_siblings_masks(cpuid: c_uint) {
    let cpuid_topo = &mut CPU_TOPOLOGY[cpuid as usize]; let ret = detect_cache_attributes(cpuid as c_int);
    if ret != 0 && ret != -ENOENT { pr_info!("Early cacheinfo allocation failed, ret = %d\n", ret); }
    let mut cpu: c_int = 0;
    for_each_online_cpu!(cpu) {
        let cpu_topo = &mut CPU_TOPOLOGY[cpu as usize];
        if last_level_cache_is_shared(cpu, cpuid as c_int) { cpumask_set_cpu(cpu, &mut cpuid_topo.llc_sibling); cpumask_set_cpu(cpuid as c_int, &mut cpu_topo.llc_sibling); }
        if cpuid_topo.package_id != cpu_topo.package_id { continue; }
        cpumask_set_cpu(cpuid as c_int, &mut cpu_topo.core_sibling); cpumask_set_cpu(cpu, &mut cpuid_topo.core_sibling);
        if cpuid_topo.cluster_id != cpu_topo.cluster_id { continue; }
        if cpuid_topo.cluster_id >= 0 { cpumask_set_cpu(cpu, &mut cpuid_topo.cluster_sibling); cpumask_set_cpu(cpuid as c_int, &mut cpu_topo.cluster_sibling); }
        if cpuid_topo.core_id != cpu_topo.core_id { continue; }
        cpumask_set_cpu(cpuid as c_int, &mut cpu_topo.thread_sibling); cpumask_set_cpu(cpu, &mut cpuid_topo.thread_sibling);
    }
}

unsafe fn clear_cpu_topology(cpu: c_int) {
    let t = &mut CPU_TOPOLOGY[cpu as usize];
    cpumask_clear(&mut t.llc_sibling); cpumask_set_cpu(cpu, &mut t.llc_sibling); cpumask_clear(&mut t.cluster_sibling); cpumask_set_cpu(cpu, &mut t.cluster_sibling); cpumask_clear(&mut t.core_sibling); cpumask_set_cpu(cpu, &mut t.core_sibling); cpumask_clear(&mut t.thread_sibling); cpumask_set_cpu(cpu, &mut t.thread_sibling);
}

pub unsafe fn reset_cpu_topology() { let mut cpu: c_uint = 0; for_each_possible_cpu!(cpu) { let t = &mut CPU_TOPOLOGY[cpu as usize]; t.thread_id = -1; t.core_id = -1; t.cluster_id = -1; t.package_id = -1; clear_cpu_topology(cpu as c_int); } }

pub unsafe fn remove_cpu_topology(cpu: c_uint) {
    let mut sibling: c_int = 0;
    for_each_cpu!(sibling, topology_core_cpumask(cpu as c_int)) { cpumask_clear_cpu(cpu as c_int, topology_core_cpumask(sibling)); }
    for_each_cpu!(sibling, topology_sibling_cpumask(cpu as c_int)) { cpumask_clear_cpu(cpu as c_int, topology_sibling_cpumask(sibling)); }
    for_each_cpu!(sibling, topology_cluster_cpumask(cpu as c_int)) { cpumask_clear_cpu(cpu as c_int, topology_cluster_cpumask(sibling)); }
    for_each_cpu!(sibling, topology_llc_cpumask(cpu as c_int)) { cpumask_clear_cpu(cpu as c_int, topology_llc_cpumask(sibling)); }
    clear_cpu_topology(cpu as c_int);
}

#[cfg(any(CONFIG_ARM64, CONFIG_RISCV))]
unsafe fn get_cpu_for_node(node: *mut device_node) -> c_int {
    let cpu_node = of_parse_phandle(node, b"cpu\0".as_ptr() as *const c_char, 0);
    if cpu_node.is_null() { return -1; }
    let cpu = of_cpu_node_to_id(cpu_node);
    if cpu >= 0 { topology_parse_cpu_capacity(cpu_node, cpu); }
    else { pr_info!("CPU node for %pOF exist but the possible cpu range is :%*pbl\n", cpu_node, cpumask_pr_args(cpu_possible_mask)); }
    cpu
}

#[cfg(any(CONFIG_ARM64, CONFIG_RISCV))]
unsafe fn parse_core(core: *mut device_node, package_id: c_int, cluster_id: c_int, core_id: c_int) -> c_int {
    let mut leaf = true; let mut i = 0; let mut cpu;
    loop { let name = format!("thread{}", i); let t = of_get_child_by_name(core, name.as_ptr() as *const c_char); if t.is_null() { break; } leaf = false; cpu = get_cpu_for_node(t); if cpu >= 0 { let x=&mut CPU_TOPOLOGY[cpu as usize]; x.package_id=package_id; x.cluster_id=cluster_id; x.core_id=core_id; x.thread_id=i; } else if cpu != -ENODEV { pr_err!("%pOF: Can't get CPU for thread\n", t); return -EINVAL; } i+=1; }
    MAX_SMT_THREAD_NUM = max!(MAX_SMT_THREAD_NUM, i as c_uint);
    cpu = get_cpu_for_node(core);
    if cpu >= 0 { if !leaf { pr_err!("%pOF: Core has both threads and CPU\n", core); return -EINVAL; } let x=&mut CPU_TOPOLOGY[cpu as usize]; x.package_id=package_id; x.cluster_id=cluster_id; x.core_id=core_id; }
    else if leaf && cpu != -ENODEV { pr_err!("%pOF: Can't get CPU for leaf core\n", core); return -EINVAL; }
    0
}

#[cfg(any(CONFIG_ARM64, CONFIG_RISCV))]
unsafe fn parse_cluster(cluster: *mut device_node, package_id: c_int, cluster_id: c_int, depth: c_int) -> c_int {
    let mut leaf=true; let mut has_cores=false; let mut core_id=0; let mut i=0;
    loop { let n=format!("cluster{}",i); let c=of_get_child_by_name(cluster,n.as_ptr() as *const c_char); if c.is_null(){break;} leaf=false; let ret=parse_cluster(c,package_id,i,depth+1); if depth>0 {pr_warn!("Topology for clusters of clusters not yet supported\n");} if ret!=0{return ret;} i+=1; }
    i=0; loop { let n=format!("core{}",i); let c=of_get_child_by_name(cluster,n.as_ptr() as *const c_char); if c.is_null(){break;} has_cores=true; if depth==0 {pr_err!("%pOF: cpu-map children should be clusters\n",c);return -EINVAL;} if !leaf {pr_err!("%pOF: Non-leaf cluster with core %s\n",cluster,n.as_ptr());return -EINVAL;} let ret=parse_core(c,package_id,cluster_id,core_id); if ret!=0{return ret;} core_id+=1;i+=1; }
    if leaf&&!has_cores {pr_warn!("%pOF: empty cluster\n",cluster);} 0
}

#[cfg(any(CONFIG_ARM64, CONFIG_RISCV))]
unsafe fn parse_socket(socket:*mut device_node)->c_int { let mut package=0; let mut ret=0; let mut has=false; loop{let n=format!("socket{}",package);let c=of_get_child_by_name(socket,n.as_ptr() as *const c_char);if c.is_null(){break;}has=true;ret=parse_cluster(c,package,-1,0);if ret!=0{return ret;}package+=1;}if !has{ret=parse_cluster(socket,0,-1,0);}if ret!=0{MAX_SMT_THREAD_NUM=1;}cpu_smt_set_num_threads(MAX_SMT_THREAD_NUM,MAX_SMT_THREAD_NUM);ret }

#[cfg(any(CONFIG_ARM64, CONFIG_RISCV))]
unsafe fn parse_dt_topology()->c_int { let cn=of_find_node_by_path(b"/cpus\0".as_ptr() as *const c_char);if cn.is_null(){pr_err!("No CPU information found in DT\n");return 0;}let map=of_get_child_by_name(cn,b"cpu-map\0".as_ptr() as *const c_char);if map.is_null(){return 0;}let ret=parse_socket(map);if ret!=0{return ret;}topology_normalize_cpu_scale();let mut cpu=0;for_each_possible_cpu!(cpu){if CPU_TOPOLOGY[cpu as usize].package_id<0{return -EINVAL;}}0 }

#[cfg(any(CONFIG_ARM64, CONFIG_RISCV))]
pub unsafe fn init_cpu_topology(){reset_cpu_topology();let mut ret=parse_acpi_topology();if ret==0&&of_have_populated_dt(){ret=parse_dt_topology();}if ret!=0{reset_cpu_topology();}let mut cpu=0;for_each_possible_cpu!(cpu){ret=fetch_cache_info(cpu);if ret==0{continue;}if ret!=-ENOENT{pr_err!("Early cacheinfo failed, ret = %d\n",ret);}return;}}

pub unsafe fn store_cpu_topology(cpuid:c_uint){let t=&mut CPU_TOPOLOGY[cpuid as usize];if t.package_id==-1{t.thread_id=-1;t.core_id=cpuid as c_int;t.package_id=cpu_to_node(cpuid as c_int);pr_debug!("CPU%u: package %d core %d thread %d\n",cpuid,t.package_id,t.core_id,t.thread_id);}update_siblings_masks(cpuid);}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
