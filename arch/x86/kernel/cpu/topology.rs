// SPDX-License-Identifier: GPL-2.0-only
/*
 * CPU/APIC topology
 *
 * The APIC IDs describe the system topology in multiple domain levels.
 * The CPUID topology parser provides the information which part of the
 * APIC ID is associated to the individual levels:
 *
 * [PACKAGE][DIEGRP][DIE][TILE][MODULE][CORE][THREAD]
 *
 * The root space contains the package (socket) IDs.
 *
 * Not enumerated levels consume 0 bits space, but conceptually they are
 * always represented. If e.g. only CORE and THREAD levels are enumerated
 * then the DIE, MODULE and TILE have the same physical ID as the PACKAGE.
 *
 * If SMT is not supported, then the THREAD domain is still used. It then
 * has the same physical ID as the CORE domain and is the only child of the
 * core domain.
 *
 * This allows a unified view on the system independent of the enumerated
 * domain levels without requiring any conditionals in the code.
 */

// C headers and build-time configuration supplied by the surrounding kernel.

/* Map cpu index to physical APIC ID */
static mut X86_CPU_TO_APICID: [u32; NR_CPUS] = [BAD_APICID; NR_CPUS];
static mut X86_CPU_TO_ACPIID: [u32; NR_CPUS] = [CPU_ACPIID_INVALID; NR_CPUS];

/* Bitmap of physically present CPUs. */
static mut phys_cpu_present_map: Bitmap = Bitmap::new();

/* Used for CPU number allocation and parallel CPU bringup */
static mut cpuid_to_apicid: [u32; NR_CPUS] = [BAD_APICID; NR_CPUS];

/* Bitmaps to mark registered APICs at each topology domain */
static mut apic_maps: [BitmapHolder; TOPO_MAX_DOMAIN] = [BitmapHolder::new(); TOPO_MAX_DOMAIN];

/*
 * Keep track of assigned, disabled and rejected CPUs. Present assigned
 * with 1 as CPU #0 is reserved for the boot CPU.
 */
#[repr(C)]
struct TopoInfo {
    nr_assigned_cpus: u32,
    nr_disabled_cpus: u32,
    nr_rejected_cpus: u32,
    boot_cpu_apic_id: u32,
    real_bsp_apic_id: u32,
}

static mut topo_info: TopoInfo = TopoInfo {
    nr_assigned_cpus: 1,
    nr_disabled_cpus: 0,
    nr_rejected_cpus: 0,
    boot_cpu_apic_id: BAD_APICID,
    real_bsp_apic_id: BAD_APICID,
};

#[inline]
unsafe fn domain_weight(dom: usize) -> u32 {
    bitmap_weight(apic_maps[dom].map.as_ptr(), MAX_LOCAL_APIC)
}

pub unsafe fn arch_match_cpu_phys_id(cpu: i32, phys_id: u64) -> bool {
    phys_id == cpuid_to_apicid[cpu as usize] as u64
}

unsafe fn cpu_mark_primary_thread(cpu: u32, apicid: u32) {
    if apicid & (__max_threads_per_core - 1) == 0 {
        cpumask_set_cpu(cpu, &mut __cpu_primary_thread_mask);
    }
}

/* Convert the APIC ID to a domain level ID by masking out the low bits below the domain level. */
#[inline]
unsafe fn topo_apicid(apicid: u32, dom: x86_topology_domains) -> u32 {
    if dom == TOPO_SMT_DOMAIN { return apicid; }
    apicid & (u32::MAX << x86_topo_system.dom_shifts[dom as usize - 1])
}

unsafe fn topo_lookup_cpuid(apic_id: u32) -> i32 {
    for i in 0..topo_info.nr_assigned_cpus as usize {
        if cpuid_to_apicid[i] == apic_id { return i as i32; }
    }
    -ENODEV
}

unsafe fn topo_get_cpunr(apic_id: u32) -> i32 {
    let cpu = topo_lookup_cpuid(apic_id);
    if cpu >= 0 { cpu } else { let n = topo_info.nr_assigned_cpus; topo_info.nr_assigned_cpus += 1; n as i32 }
}

unsafe fn topo_set_cpuids(cpu: u32, apic_id: u32, acpi_id: u32) {
    X86_CPU_TO_APICID[cpu as usize] = apic_id;
    X86_CPU_TO_ACPIID[cpu as usize] = acpi_id;
    set_cpu_present(cpu, true);
}

unsafe fn check_for_real_bsp(apic_id: u32) -> bool {
    let mut is_bsp = false;
    let has_apic_base = boot_cpu_data.x86 >= 6;
    let mut msr = 0u64;
    if topo_info.real_bsp_apic_id != BAD_APICID { return false; }
    if has_apic_base { rdmsrq(MSR_IA32_APICBASE, &mut msr); is_bsp = (msr & MSR_IA32_APICBASE_BSP) != 0; }
    if apic_id == topo_info.boot_cpu_apic_id {
        if is_bsp || !has_apic_base { topo_info.real_bsp_apic_id = apic_id; return false; }
        pr_warn!("Enumerated BSP APIC %x is not marked in APICBASE MSR\n", apic_id);
        pr_warn!("Assuming crash kernel. Limiting to one CPU to prevent machine INIT\n");
        set_nr_cpu_ids(1); return false;
    }
    pr_warn!("Boot CPU APIC ID not the first enumerated APIC ID: %x != %x\n", topo_info.boot_cpu_apic_id, apic_id);
    if is_bsp { topo_info.real_bsp_apic_id = topo_info.boot_cpu_apic_id; return false; }
    pr_warn!("Crash kernel detected. Disabling real BSP to prevent machine INIT\n");
    topo_info.real_bsp_apic_id = apic_id; true
}

unsafe fn topo_unit_count(lvlid: u32, at_level: x86_topology_domains, map: *const u64) -> u32 {
    bitmap_weight_from(map, lvlid, lvlid + (1u32 << x86_topo_system.dom_shifts[at_level as usize]))
}

unsafe fn topo_register_apic(apic_id: u32, acpi_id: u32, present: bool) {
    let cpu;
    if present {
        set_bit(apic_id, &mut phys_cpu_present_map);
        cpu = if apic_id == topo_info.boot_cpu_apic_id { 0 } else { topo_get_cpunr(apic_id) as usize };
        cpuid_to_apicid[cpu] = apic_id;
        topo_set_cpuids(cpu as u32, apic_id, acpi_id);
    } else { topo_info.nr_disabled_cpus += 1; }
    for dom in TOPO_SMT_DOMAIN as usize..TOPO_MAX_DOMAIN {
        set_bit(topo_apicid(apic_id, dom as x86_topology_domains), &mut apic_maps[dom].map);
    }
}

pub unsafe fn topology_register_apic(apic_id: u32, acpi_id: u32, present: bool) {
    if apic_id >= MAX_LOCAL_APIC { pr_err_once!("APIC ID %x exceeds kernel limit of: %x\n", apic_id, MAX_LOCAL_APIC - 1); topo_info.nr_rejected_cpus += 1; return; }
    if check_for_real_bsp(apic_id) { topo_info.nr_rejected_cpus += 1; return; }
    if apic_id != topo_info.boot_cpu_apic_id && topo_info.nr_assigned_cpus >= nr_cpu_ids { pr_warn_once!("CPU limit of %d reached. Ignoring further CPUs\n", nr_cpu_ids); topo_info.nr_rejected_cpus += 1; return; }
    topo_register_apic(apic_id, acpi_id, present);
}

pub unsafe fn topology_register_boot_apic(apic_id: u32) {
    WARN_ON_ONCE!(topo_info.boot_cpu_apic_id != BAD_APICID);
    topo_info.boot_cpu_apic_id = apic_id;
    topo_register_apic(apic_id, CPU_ACPIID_INVALID, true);
}

pub unsafe fn topology_get_logical_id(apicid: u32, at_level: x86_topology_domains) -> i32 {
    let lvlid = topo_apicid(apicid, at_level);
    if lvlid >= MAX_LOCAL_APIC { return -ERANGE; }
    if !test_bit(lvlid, apic_maps[at_level as usize].map.as_ptr()) { return -ENODEV; }
    bitmap_weight(apic_maps[at_level as usize].map.as_ptr(), lvlid) as i32
}

pub unsafe fn topology_unit_count(apicid: u32, which_units: x86_topology_domains, at_level: x86_topology_domains) -> u32 {
    let lvlid = topo_apicid(apicid, at_level);
    if lvlid >= MAX_LOCAL_APIC || !test_bit(lvlid, apic_maps[at_level as usize].map.as_ptr()) || which_units > at_level { return 0; }
    if which_units == at_level { return 1; }
    topo_unit_count(lvlid, at_level, apic_maps[which_units as usize].map.as_ptr())
}

#[cfg(CONFIG_SMP)]
pub unsafe fn topology_get_primary_thread(cpu: u32) -> i32 {
    topo_lookup_cpuid(topo_apicid(cpuid_to_apicid[cpu as usize], TOPO_CORE_DOMAIN))
}

#[cfg(CONFIG_ACPI_HOTPLUG_CPU)]
pub unsafe fn topology_hotplug_apic(apic_id: u32, acpi_id: u32) -> i32 {
    if apic_id >= MAX_LOCAL_APIC { return -EINVAL; }
    if !test_bit(apic_id, apic_maps[TOPO_SMT_DOMAIN as usize].map.as_ptr()) { return -ENODEV; }
    let cpu = topo_lookup_cpuid(apic_id);
    if cpu < 0 { return -ENOSPC; }
    set_bit(apic_id, &mut phys_cpu_present_map);
    topo_set_cpuids(cpu as u32, apic_id, acpi_id);
    cpu_mark_primary_thread(cpu as u32, apic_id);
    cpu
}

#[cfg(CONFIG_ACPI_HOTPLUG_CPU)]
pub unsafe fn topology_hotunplug_apic(cpu: u32) {
    let apic_id = cpuid_to_apicid[cpu as usize];
    if apic_id == BAD_APICID { return; }
    X86_CPU_TO_APICID[cpu as usize] = BAD_APICID;
    clear_bit(apic_id, &mut phys_cpu_present_map);
    set_cpu_present(cpu, false);
}

#[cfg(CONFIG_X86_LOCAL_APIC)]
static mut max_possible_cpus: u32 = NR_CPUS;

#[cfg(CONFIG_X86_LOCAL_APIC)]
pub unsafe fn topology_apply_cmdline_limits_early() {
    let mut possible = nr_cpu_ids;
    if !setup_max_cpus || apic_is_disabled { possible = 1; }
    possible = core::cmp::min(max_possible_cpus, possible);
    if possible < nr_cpu_ids {
        pr_info!("Limiting to %u possible CPUs\n", possible);
        set_nr_cpu_ids(possible);
    }
}

#[cfg(CONFIG_X86_LOCAL_APIC)]
unsafe fn restrict_to_up() -> bool {
    if !smp_found_config { return true; }
    if xen_pv_domain() { return false; }
    apic_is_disabled
}

#[cfg(CONFIG_X86_LOCAL_APIC)]
pub unsafe fn topology_init_possible_cpus() {
    let mut assigned = topo_info.nr_assigned_cpus;
    let mut disabled = topo_info.nr_disabled_cpus;
    let mut allowed = 1u32;
    let total = assigned + disabled;
    let mut apicid;
    if topo_info.boot_cpu_apic_id == BAD_APICID { topology_register_boot_apic(0); }
    if !restrict_to_up() {
        if WARN_ON_ONCE!(assigned > nr_cpu_ids) { disabled += assigned - nr_cpu_ids; assigned = nr_cpu_ids; }
        allowed = core::cmp::min(total, nr_cpu_ids);
    }
    if total > allowed { pr_warn!("%u possible CPUs exceed the limit of %u\n", total, allowed); }
    assigned = core::cmp::min(allowed, assigned);
    disabled = allowed - assigned;
    topo_info.nr_assigned_cpus = assigned;
    topo_info.nr_disabled_cpus = disabled;
    total_cpus = allowed;
    set_nr_cpu_ids(allowed);
    let cnta = domain_weight(TOPO_PKG_DOMAIN as usize);
    __max_logical_packages = cnta;
    pr_info!("Max. logical packages: %3u\n", __max_logical_packages);
    let mut cntb = num_phys_nodes();
    __num_nodes_per_package = DIV_ROUND_UP(cntb, cnta);
    pr_info!("Max. logical nodes:    %3u\n", cntb);
    pr_info!("Num. nodes per package:%3u\n", __num_nodes_per_package);
    cntb = domain_weight(TOPO_DIE_DOMAIN as usize);
    __max_dies_per_package = 1u32 << (get_count_order(cntb) - get_count_order(cnta));
    pr_info!("Max. logical dies:     %3u\n", cntb);
    pr_info!("Max. dies per package: %3u\n", __max_dies_per_package);
    let cnta = domain_weight(TOPO_CORE_DOMAIN as usize);
    cntb = domain_weight(TOPO_SMT_DOMAIN as usize);
    __max_threads_per_core = DIV_ROUND_UP(cntb, cnta);
    pr_info!("Max. threads per core: %3u\n", __max_threads_per_core);
    let firstid = find_first_bit(apic_maps[TOPO_SMT_DOMAIN as usize].map.as_ptr(), MAX_LOCAL_APIC);
    __num_cores_per_package = topology_unit_count(firstid, TOPO_CORE_DOMAIN, TOPO_PKG_DOMAIN);
    __num_threads_per_package = topology_unit_count(firstid, TOPO_SMT_DOMAIN, TOPO_PKG_DOMAIN);
    pr_info!("Num. cores per package:   %3u\n", __num_cores_per_package);
    pr_info!("Num. threads per package: %3u\n", __num_threads_per_package);
    pr_info!("Allowing %u present CPUs plus %u hotplug CPUs\n", assigned, disabled);
    if topo_info.nr_rejected_cpus != 0 { pr_info!("Rejected CPUs %u\n", topo_info.nr_rejected_cpus); }
    init_cpu_present(cpumask_of(0));
    init_cpu_possible(cpumask_of(0));
    apicid = 0;
    while disabled != 0 {
        apicid = find_next_andnot_bit(apic_maps[TOPO_SMT_DOMAIN as usize].map.as_ptr(), phys_cpu_present_map.as_ptr(), MAX_LOCAL_APIC, apicid);
        if apicid >= MAX_LOCAL_APIC { break; }
        cpuid_to_apicid[topo_info.nr_assigned_cpus as usize] = apicid;
        topo_info.nr_assigned_cpus += 1;
        disabled -= 1;
        apicid += 1;
    }
    for cpu in 0..allowed {
        apicid = cpuid_to_apicid[cpu as usize];
        set_cpu_possible(cpu, true);
        if apicid == BAD_APICID { continue; }
        cpu_mark_primary_thread(cpu, apicid);
        set_cpu_present(cpu, test_bit(apicid, phys_cpu_present_map.as_ptr()));
    }
}

#[cfg(CONFIG_X86_LOCAL_APIC)]
pub unsafe fn topology_reset_possible_cpus_up() {
    init_cpu_present(cpumask_of(0));
    init_cpu_possible(cpumask_of(0));
    bitmap_zero(&mut phys_cpu_present_map, MAX_LOCAL_APIC);
    if topo_info.boot_cpu_apic_id != BAD_APICID { set_bit(topo_info.boot_cpu_apic_id, &mut phys_cpu_present_map); }
}

#[cfg(CONFIG_X86_LOCAL_APIC)]
unsafe fn setup_possible_cpus(str_: *mut u8) -> i32 {
    get_option(&mut str_, &mut max_possible_cpus);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
