// SPDX-License-Identifier: GPL-2.0-only
/*
 * SMP initialisation and IPI support
 * Based on arch/arm64/kernel/smp.c
 *
 * Copyright (C) 2012 ARM Ltd.
 * Copyright (C) 2015 Regents of the University of California
 * Copyright (C) 2017 SiFive
 */

// Dependencies supplied by the surrounding kernel translation unit are intentionally external.

#[cfg(not(CONFIG_HOTPLUG_PARALLEL))]
static mut CPU_RUNNING: Completion = Completion::new();

pub unsafe fn smp_prepare_cpus(max_cpus: u32) {
    let mut cpuid: i32;
    let curr_cpuid: u32;

    init_cpu_topology();
    curr_cpuid = smp_processor_id();
    store_cpu_topology(curr_cpuid);
    numa_store_cpu_info(curr_cpuid);
    numa_add_cpu(curr_cpuid);

    if max_cpus == 0 { return; }

    for_each_possible_cpu!(cpuid) {
        if cpuid as u32 == curr_cpuid { continue; }
        set_cpu_present(cpuid as u32, true);
        numa_store_cpu_info(cpuid as u32);
    }
}

#[cfg(CONFIG_ACPI)]
static mut CPU_COUNT: u32 = 1;

#[cfg(CONFIG_ACPI)]
unsafe fn acpi_parse_rintc(header: *mut AcpiSubtableHeaders, end: usize) -> i32 {
    let hart: usize;
    static mut FOUND_BOOT_CPU: bool = false;
    let processor = header as *mut AcpiMadtRintc;

    if ((*processor).flags & ACPI_MADT_ENABLED) == 0 { return 0; }
    if bad_madt_entry(processor, end) { return -EINVAL; }
    acpi_table_print_madt_entry(&(*header).common);

    hart = (*processor).hart_id;
    if hart == INVALID_HARTID {
        pr_warn!("Invalid hartid\n");
        return 0;
    }
    if hart == cpuid_to_hartid_map(0) {
        BUG_ON!(FOUND_BOOT_CPU);
        FOUND_BOOT_CPU = true;
        return 0;
    }
    if CPU_COUNT >= NR_CPUS {
        pr_warn!("NR_CPUS is too small for the number of ACPI tables.\n");
        return 0;
    }
    *cpuid_to_hartid_map_ptr(CPU_COUNT) = hart;
    CPU_COUNT += 1;
    0
}

#[cfg(CONFIG_ACPI)]
unsafe fn acpi_parse_and_init_cpus() {
    acpi_table_parse_madt(ACPI_MADT_TYPE_RINTC, Some(acpi_parse_rintc), 0);
}

#[cfg(not(CONFIG_ACPI))]
unsafe fn acpi_parse_and_init_cpus() {}

unsafe fn of_parse_and_init_cpus() {
    let mut dn: *mut DeviceNode;
    let mut hart: usize = 0;
    let mut found_boot_cpu = false;
    let mut cpuid = 1i32;
    let mut rc: i32;

    for_each_of_cpu_node!(dn) {
        rc = riscv_early_of_processor_hartid(dn, &mut hart);
        if rc < 0 { continue; }
        if hart == cpuid_to_hartid_map(0) {
            BUG_ON!(found_boot_cpu);
            found_boot_cpu = true;
            early_map_cpu_to_node(0, of_node_to_nid(dn));
            continue;
        }
        if cpuid as u32 >= NR_CPUS {
            pr_warn!("Invalid cpuid [{}] for hartid [{}]\n", cpuid, hart);
            continue;
        }
        *cpuid_to_hartid_map_ptr(cpuid as u32) = hart;
        early_map_cpu_to_node(cpuid as u32, of_node_to_nid(dn));
        cpuid += 1;
    }
    BUG_ON!(!found_boot_cpu);
    if cpuid > nr_cpu_ids {
        pr_warn!("Total number of cpus [{}] is greater than nr_cpus option value [{}]\n", cpuid, nr_cpu_ids);
    }
}

pub unsafe fn setup_smp() {
    let mut cpuid: i32;
    cpu_set_ops();
    if acpi_disabled { of_parse_and_init_cpus(); } else { acpi_parse_and_init_cpus(); }
    for cpuid in 1..nr_cpu_ids {
        if cpuid_to_hartid_map(cpuid as u32) != INVALID_HARTID { set_cpu_possible(cpuid as u32, true); }
    }
}

unsafe fn start_secondary_cpu(cpu: i32, tidle: *mut TaskStruct) -> i32 {
    if !(*cpu_ops).cpu_start.is_none() { return (*cpu_ops).cpu_start.unwrap()(cpu, tidle); }
    -EOPNOTSUPP
}

#[cfg(CONFIG_HOTPLUG_PARALLEL)]
pub unsafe fn arch_cpuhp_kick_ap_alive(cpu: u32, tidle: *mut TaskStruct) -> i32 { start_secondary_cpu(cpu as i32, tidle) }

#[cfg(not(CONFIG_HOTPLUG_PARALLEL))]
pub unsafe fn __cpu_up(cpu: u32, tidle: *mut TaskStruct) -> i32 {
    (*tidle).thread_info.cpu = cpu;
    let mut ret = start_secondary_cpu(cpu as i32, tidle);
    if ret == 0 {
        wait_for_completion_timeout(&mut CPU_RUNNING, secs_to_jiffies(1));
        if !cpu_online(cpu) { pr_crit!("CPU{}: failed to come online\n", cpu); ret = -EIO; }
    } else { pr_crit!("CPU{}: failed to start\n", cpu); }
    ret
}

pub unsafe fn smp_cpus_done(_max_cpus: u32) {}

/* C entry point for a secondary processor. */
pub unsafe fn smp_callin() {
    let mm = &mut init_mm as *mut MmStruct;
    let curr_cpuid = smp_processor_id();
    if has_vector() && riscv_v_setup_vsize() { return; }
    mmgrab(mm);
    (*current).active_mm = mm;
    #[cfg(CONFIG_HOTPLUG_PARALLEL)] cpuhp_ap_sync_alive();
    store_cpu_topology(curr_cpuid);
    notify_cpu_starting(curr_cpuid);
    riscv_ipi_enable();
    numa_add_cpu(curr_cpuid);
    pr_debug!("CPU{}: Booted secondary hartid {}\n", curr_cpuid, cpuid_to_hartid_map(curr_cpuid));
    set_cpu_online(curr_cpuid, true);
    local_flush_icache_all();
    local_flush_tlb_all();
    #[cfg(not(CONFIG_HOTPLUG_PARALLEL))] complete(&mut CPU_RUNNING);
    local_irq_enable();
    cpu_startup_entry(CPUHP_AP_ONLINE_IDLE);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
