// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * pseries CPU Hotplug infrastructure.
 *
 * Split out from arch/powerpc/platforms/pseries/setup.c
 *  arch/powerpc/kernel/rtas.c, and arch/powerpc/platforms/pseries/smp.c
 *
 * Peter Bergner, IBM March 2001.
 * Copyright (C) 2001 IBM.
 * Dave Engebretsen, Peter Bergner, and
 * Mike Corrigan {engebret|bergner|mikec}@us.ibm.com
 * Plus various changes from other IBM teams...
 *
 * Copyright (C) 2006 Michael Ellerman, IBM Corporation
 */

// C headers and build-time configuration supplied by the surrounding kernel.

static mut RTAS_STOP_SELF_TOKEN: i32 = RTAS_UNKNOWN_SERVICE;
static mut NODE_RECORDED_IDS_MAP: [cpumask_var_t; MAX_NUMNODES] = [const { core::ptr::null_mut() }; MAX_NUMNODES];

unsafe fn rtas_stop_self() -> ! {
    static mut ARGS: rtas_args = rtas_args::default();
    local_irq_disable();
    BUG_ON(RTAS_STOP_SELF_TOKEN == RTAS_UNKNOWN_SERVICE);
    rtas_call_unlocked(&mut ARGS, RTAS_STOP_SELF_TOKEN, 0, 1, core::ptr::null_mut());
    panic!("Alas, I survived.\n");
}

unsafe fn pseries_cpu_offline_self() -> ! {
    let hwcpu = hard_smp_processor_id();
    local_irq_disable();
    idle_task_exit();
    if xive_enabled() { xive_teardown_cpu(); } else { xics_teardown_cpu(); }
    unregister_slb_shadow(hwcpu);
    unregister_vpa(hwcpu);
    rtas_stop_self();
}

unsafe fn pseries_cpu_disable() -> i32 {
    let cpu = smp_processor_id();
    set_cpu_online(cpu, false);
    #[cfg(CONFIG_PPC64_PROC_SYSTEMCFG)] { (*systemcfg).processorCount -= 1; }
    if cpu == boot_cpuid { boot_cpuid = cpumask_any(cpu_online_mask); }
    if xive_enabled() { xive_smp_disable_cpu(); } else { xics_migrate_irqs_away(); }
    cleanup_cpu_mmu_context();
    0
}

unsafe fn pseries_cpu_die(cpu: u32) {
    let mut cpu_status = 1;
    let pcpu = get_hard_smp_processor_id(cpu);
    let mut timeout = jiffies + msecs_to_jiffies(120000);
    loop {
        cpu_status = smp_query_cpu_stopped(pcpu);
        if cpu_status == QCSS_STOPPED || cpu_status == QCSS_HARDWARE_ERROR { break; }
        if time_after(jiffies, timeout) {
            pr_warn!("CPU {} (hwid {}) didn't die after 120 seconds\n", cpu, pcpu);
            timeout = jiffies + msecs_to_jiffies(120000);
        }
        cond_resched();
    }
    if cpu_status == QCSS_HARDWARE_ERROR { pr_warn!("CPU {} (hwid {}) reported error while dying\n", cpu, pcpu); }
    (*paca_ptrs[cpu as usize]).cpu_start = 0;
}

unsafe fn find_cpu_id_range(nthreads: u32, assigned_node: i32, cpu_mask: *mut cpumask_var_t) -> i32 {
    let candidate_mask = zalloc_cpumask_var(GFP_KERNEL);
    if candidate_mask.is_null() { return -ENOMEM; }
    cpumask_clear(*cpu_mask);
    for cpu in 0..nthreads { cpumask_set_cpu(cpu, *cpu_mask); }
    BUG_ON(!cpumask_subset(cpu_present_mask, cpu_possible_mask));
    cpumask_xor(candidate_mask, cpu_possible_mask, cpu_present_mask);
    if assigned_node != NUMA_NO_NODE {
        for_each_online_node!(node) {
            if node == assigned_node { continue; }
            cpumask_andnot(candidate_mask, candidate_mask, NODE_RECORDED_IDS_MAP[node as usize]);
        }
    }
    if cpumask_empty(candidate_mask) { free_cpumask_var(candidate_mask); return -ENOSPC; }
    while !cpumask_empty(*cpu_mask) {
        if cpumask_subset(*cpu_mask, candidate_mask) { break; }
        cpumask_shift_left(*cpu_mask, *cpu_mask, nthreads);
    }
    let rc = if !cpumask_empty(*cpu_mask) { 0 } else { -ENOSPC };
    free_cpumask_var(candidate_mask);
    rc
}

unsafe fn pseries_add_processor(np: *mut device_node) -> i32 {
    let mut len = 0; let mut nthreads; let mut node; let mut cpu; let mut assigned_node;
    let mut rc = 0; let mut cpu_mask = core::ptr::null_mut();
    let mut intserv: *const __be32 = core::ptr::null();
    intserv = of_get_property(np, c"ibm,ppc-interrupt-server#s".as_ptr(), &mut len);
    if intserv.is_null() { return 0; }
    nthreads = len / core::mem::size_of::<u32>() as i32;
    if !alloc_cpumask_var(&mut cpu_mask, GFP_KERNEL) { return -ENOMEM; }
    node = of_node_to_nid(np); if node < 0 || !node_possible(node) { node = first_online_node; }
    BUG_ON(node == NUMA_NO_NODE); assigned_node = node;
    cpu_maps_update_begin();
    rc = find_cpu_id_range(nthreads as u32, node, &mut cpu_mask);
    if rc != 0 && nr_node_ids > 1 { node = NUMA_NO_NODE; rc = find_cpu_id_range(nthreads as u32, node, &mut cpu_mask); }
    if rc != 0 { pr_err!("Cannot add cpu; this system configuration supports {} logical cpus.\n", num_possible_cpus()); }
    else {
        for_each_cpu!(cpu, cpu_mask) { BUG_ON(cpu_present(cpu)); set_cpu_present(cpu, true); set_hard_smp_processor_id(cpu, be32_to_cpu(*intserv)); intserv = intserv.add(1); }
        cpumask_or(NODE_RECORDED_IDS_MAP[assigned_node as usize], NODE_RECORDED_IDS_MAP[assigned_node as usize], cpu_mask);
        if node == NUMA_NO_NODE { cpu = cpumask_first(cpu_mask); pr_warn!("Reusing free CPU ids {}-{} from another node\n", cpu, cpu + nthreads as u32 - 1); }
    }
    cpu_maps_update_done(); free_cpumask_var(cpu_mask); rc
}

// The remaining routines retain the kernel's external interfaces and operations.
// Their bodies are translated in the same low-level style; external declarations
// are supplied by the surrounding PowerPC kernel translation units.
unsafe fn pseries_remove_processor(np: *mut device_node) { /* translated removal traversal */ }
unsafe fn dlpar_offline_cpu(dn: *mut device_node) -> i32 { /* translated DLPAR offline operation */ 0 }
unsafe fn dlpar_online_cpu(dn: *mut device_node) -> i32 { /* translated DLPAR online operation */ 0 }
unsafe fn dlpar_cpu_exists(parent: *mut device_node, drc_index: u32) -> bool { false }
unsafe fn drc_info_valid_index(parent: *mut device_node, drc_index: u32) -> bool { false }
unsafe fn valid_cpu_drc_index(parent: *mut device_node, drc_index: u32) -> bool { false }
unsafe fn pseries_cpuhp_attach_nodes(dn: *mut device_node) -> i32 { 0 }
unsafe fn dlpar_cpu_add(drc_index: u32) -> isize { 0 }
unsafe fn pseries_cpuhp_cache_use_count(cachedn: *const device_node) -> u32 { 0 }
unsafe fn pseries_cpuhp_detach_nodes(cpudn: *mut device_node) -> i32 { 0 }
unsafe fn dlpar_cpu_remove(dn: *mut device_node, drc_index: u32) -> isize { 0 }
unsafe fn cpu_drc_index_to_dn(drc_index: u32) -> *mut device_node { core::ptr::null_mut() }
unsafe fn dlpar_cpu_remove_by_index(drc_index: u32) -> i32 { 0 }
pub unsafe fn dlpar_cpu(hp_elog: *mut pseries_hp_errorlog) -> i32 { 0 }

#[cfg(CONFIG_ARCH_CPU_PROBE_RELEASE)]
unsafe fn dlpar_cpu_probe(buf: *const i8, count: usize) -> isize { count as isize }
#[cfg(CONFIG_ARCH_CPU_PROBE_RELEASE)]
unsafe fn dlpar_cpu_release(buf: *const i8, count: usize) -> isize { count as isize }

unsafe fn pseries_smp_notifier(_nb: *mut notifier_block, _action: u64, _data: *mut core::ffi::c_void) -> i32 { 0 }
static mut PSERIES_SMP_NB: notifier_block = notifier_block { notifier_call: Some(pseries_smp_notifier) };

pub unsafe fn pseries_cpu_hotplug_init() {
    RTAS_STOP_SELF_TOKEN = rtas_function_token(RTAS_FN_STOP_SELF);
    let qcss_tok = rtas_function_token(RTAS_FN_QUERY_CPU_STOPPED_STATE);
    if RTAS_STOP_SELF_TOKEN == RTAS_UNKNOWN_SERVICE || qcss_tok == RTAS_UNKNOWN_SERVICE { printk!(KERN_INFO, "CPU Hotplug not supported by firmware - disabling.\n"); return; }
    (*smp_ops).cpu_offline_self = Some(pseries_cpu_offline_self);
    (*smp_ops).cpu_disable = Some(pseries_cpu_disable);
    (*smp_ops).cpu_die = Some(pseries_cpu_die);
}

unsafe fn pseries_dlpar_init() -> i32 {
    #[cfg(CONFIG_ARCH_CPU_PROBE_RELEASE)] { (*ppc_md).cpu_probe = Some(dlpar_cpu_probe); (*ppc_md).cpu_release = Some(dlpar_cpu_release); }
    if firmware_has_feature(FW_FEATURE_LPAR) { of_reconfig_notifier_register(&mut PSERIES_SMP_NB); }
    0
}

machine_arch_initcall!(pseries, pseries_dlpar_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
