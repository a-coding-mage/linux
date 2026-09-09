// SPDX-License-Identifier: GPL-2.0
// Dependencies supplied by the Linux kernel and sibling translation units.

pub static mut x86_topo_system: x86_topology_system = unsafe { core::mem::zeroed() };
pub static mut __amd_nodes_per_pkg: u32 = 0;
pub static mut __cpu_primary_thread_mask: cpumask = unsafe { core::mem::zeroed() };

#[repr(C)]
pub struct cpuid_ebx1 {
    pub raw: u32,
}

impl cpuid_ebx1 {
    fn nproc(&self) -> u32 { (self.raw >> 16) & 0xff }
    fn apicid(&self) -> u32 { (self.raw >> 24) & 0xff }
}

#[repr(C)]
pub struct cpuid_eax4 {
    pub raw: u32,
}

impl cpuid_eax4 {
    fn cache_type(&self) -> u32 { self.raw & 0x1f }
    fn ncores(&self) -> u32 { (self.raw >> 26) & 0x3f }
}

pub unsafe fn topology_set_dom(tscan: *mut topo_scan, mut dom: x86_topology_domains,
                               shift: u32, ncpus: u32) {
    topology_update_dom(tscan, dom, shift, ncpus);
    /* Propagate to the upper levels */
    dom += 1;
    while dom < TOPO_MAX_DOMAIN {
        (*tscan).dom_shifts[dom as usize] = (*tscan).dom_shifts[(dom - 1) as usize];
        (*tscan).dom_ncpus[dom as usize] = (*tscan).dom_ncpus[(dom - 1) as usize];
        dom += 1;
    }
}

pub unsafe fn get_topology_cpu_type(c: *mut cpuinfo_x86) -> x86_topology_cpu_type {
    if (*c).x86_vendor == X86_VENDOR_INTEL {
        match (*c).topo.intel_type {
            INTEL_CPU_TYPE_ATOM => return TOPO_CPU_TYPE_EFFICIENCY,
            INTEL_CPU_TYPE_CORE => return TOPO_CPU_TYPE_PERFORMANCE,
            _ => {}
        }
    }
    if (*c).x86_vendor == X86_VENDOR_AMD {
        match (*c).topo.amd_type {
            AMD_CPU_TYPE_PERFORMANCE => return TOPO_CPU_TYPE_PERFORMANCE,
            AMD_CPU_TYPE_EFFICIENCY => return TOPO_CPU_TYPE_EFFICIENCY,
            AMD_CPU_TYPE_LOW_POWER => return TOPO_CPU_TYPE_LOW_POWER,
            _ => {}
        }
    }
    TOPO_CPU_TYPE_UNKNOWN
}

pub unsafe fn get_topology_cpu_type_name(c: *mut cpuinfo_x86) -> *const u8 {
    match get_topology_cpu_type(c) {
        TOPO_CPU_TYPE_PERFORMANCE => b"performance\0".as_ptr(),
        TOPO_CPU_TYPE_EFFICIENCY => b"efficiency\0".as_ptr(),
        TOPO_CPU_TYPE_LOW_POWER => b"low_power\0".as_ptr(),
        _ => b"unknown\0".as_ptr(),
    }
}

unsafe fn parse_num_cores_legacy(c: *mut cpuinfo_x86) -> u32 {
    if (*c).cpuid_level < 4 { return 1; }
    let mut eax = cpuid_eax4 { raw: 0 };
    cpuid_subleaf_reg(4, 0, CPUID_EAX, &mut eax.raw);
    if eax.cache_type() == 0 { return 1; }
    eax.ncores() + 1
}

unsafe fn parse_legacy(tscan: *mut topo_scan) {
    let c = (*tscan).c;
    let cores = parse_num_cores_legacy(c);
    let mut core_shift = get_count_order(cores);
    let mut smt_shift = 0;
    if cpu_has(c, X86_FEATURE_HT) {
        if !WARN_ON_ONCE((*tscan).ebx1_nproc_shift < core_shift) {
            smt_shift = (*tscan).ebx1_nproc_shift - core_shift;
        }
        core_shift += smt_shift;
        topology_set_dom(tscan, TOPO_SMT_DOMAIN, smt_shift, 1u32 << smt_shift);
        topology_set_dom(tscan, TOPO_CORE_DOMAIN, core_shift, cores << smt_shift);
    } else {
        topology_set_dom(tscan, TOPO_SMT_DOMAIN, smt_shift, 1u32 << smt_shift);
        topology_set_dom(tscan, TOPO_CORE_DOMAIN, core_shift, cores);
    }
}

unsafe fn fake_topology(tscan: *mut topo_scan) -> bool {
    /* Preset the CORE level shift for CPUID less systems and XEN_PV. */
    topology_set_dom(tscan, TOPO_SMT_DOMAIN, 0, 1);
    topology_set_dom(tscan, TOPO_CORE_DOMAIN, 0, 1);
    (*tscan).c.cpuid_level < 1
}

unsafe fn parse_topology(tscan: *mut topo_scan, early: bool) {
    let c = (*tscan).c;
    (*c).topo.cu_id = 0xff;
    (*c).topo.llc_id = BAD_APICID;
    (*c).topo.l2c_id = BAD_APICID;
    (*c).topo.cpu_type = TOPO_CPU_TYPE_UNKNOWN;
    if fake_topology(tscan) { return; }
    let mut ebx = cpuid_ebx1 { raw: 0 };
    cpuid_leaf_reg(1, CPUID_EBX, &mut ebx.raw);
    (*c).topo.initial_apicid = ebx.apicid();
    (*c).topo.apicid = if early { (*c).topo.initial_apicid } else { read_apic_id() };
    if !IS_ENABLED(CONFIG_SMP) { return; }
    (*tscan).ebx1_nproc_shift = get_count_order(ebx.nproc());
    match (*c).x86_vendor {
        X86_VENDOR_AMD | X86_VENDOR_HYGON => cpu_parse_topology_amd(tscan),
        X86_VENDOR_CENTAUR | X86_VENDOR_ZHAOXIN => parse_legacy(tscan),
        X86_VENDOR_INTEL => {
            if !IS_ENABLED(CONFIG_CPU_SUP_INTEL) || !cpu_parse_topology_ext(tscan) { parse_legacy(tscan); }
            if (*c).cpuid_level >= 0x1a {
                (*c).topo.hw_cpu_type = cpuid_eax(0x1a);
                (*c).topo.cpu_type = get_topology_cpu_type(c);
            }
        }
        _ => {}
    }
}

unsafe fn topo_set_ids(tscan: *mut topo_scan, early: bool) {
    let c = (*tscan).c;
    let apicid = (*c).topo.apicid;
    (*c).topo.pkg_id = topo_shift_apicid(apicid, TOPO_PKG_DOMAIN);
    (*c).topo.die_id = topo_shift_apicid(apicid, TOPO_DIE_DOMAIN);
    if !early {
        (*c).topo.logical_pkg_id = topology_get_logical_id(apicid, TOPO_PKG_DOMAIN);
        (*c).topo.logical_die_id = topology_get_logical_id(apicid, TOPO_DIE_DOMAIN);
        (*c).topo.logical_core_id = topology_get_logical_id(apicid, TOPO_CORE_DOMAIN);
    }
    (*c).topo.core_id = (apicid & topo_domain_mask(TOPO_PKG_DOMAIN)) >> x86_topo_system.dom_shifts[TOPO_SMT_DOMAIN as usize];
    (*c).topo.amd_node_id = (*tscan).amd_node_id;
    if (*c).x86_vendor == X86_VENDOR_AMD { cpu_topology_fixup_amd(tscan); }
}

pub unsafe fn cpu_parse_topology(c: *mut cpuinfo_x86) {
    let cpu = smp_processor_id();
    let mut tscan: topo_scan = core::mem::zeroed();
    tscan.c = c;
    parse_topology(&mut tscan, false);
    if IS_ENABLED(CONFIG_X86_LOCAL_APIC) {
        if (*c).topo.initial_apicid != (*c).topo.apicid { pr_err(FW_BUG, cpu, (*c).topo.initial_apicid, (*c).topo.apicid); }
        if (*c).topo.apicid != cpuid_to_apicid[cpu as usize] { pr_err(FW_BUG, cpu, cpuid_to_apicid[cpu as usize], (*c).topo.apicid); }
    }
    let mut dom = TOPO_SMT_DOMAIN;
    while dom < TOPO_MAX_DOMAIN {
        if tscan.dom_shifts[dom as usize] != x86_topo_system.dom_shifts[dom as usize] { pr_err(FW_BUG, cpu, dom, tscan.dom_shifts[dom as usize], x86_topo_system.dom_shifts[dom as usize]); }
        dom += 1;
    }
    topo_set_ids(&mut tscan, false);
}

pub unsafe fn cpu_init_topology(c: *mut cpuinfo_x86) {
    let mut tscan: topo_scan = core::mem::zeroed();
    tscan.c = c;
    parse_topology(&mut tscan, true);
    for dom in 0..TOPO_MAX_DOMAIN as usize { x86_topo_system.dom_shifts[dom] = tscan.dom_shifts[dom]; }
    let mut dom = TOPO_SMT_DOMAIN;
    x86_topo_system.dom_size[dom as usize] = 1u32 << x86_topo_system.dom_shifts[dom as usize];
    dom += 1;
    while dom < TOPO_MAX_DOMAIN {
        let sft = x86_topo_system.dom_shifts[dom as usize] - x86_topo_system.dom_shifts[(dom - 1) as usize];
        x86_topo_system.dom_size[dom as usize] = 1u32 << sft;
        dom += 1;
    }
    topo_set_ids(&mut tscan, true);
    __amd_nodes_per_pkg = tscan.amd_nodes_per_pkg;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
