// SPDX-License-Identifier: GPL-2.0
// Translated from topology_amd.c. C headers and symbols are supplied by the
// surrounding kernel translation.

#[repr(C)]
struct Parse80000008Ecx {
    cpu_nthreads: u32,
    apicid_coreid_len: u32,
    perf_tsc_len: u32,
}

#[repr(C)]
struct Parse8000001eLeaf {
    ext_apic_id: u32,
    core_id: u32,
    core_nthreads: u32,
    node_id: u32,
    nnodes_per_socket: u32,
}

unsafe fn parse_8000_0008(tscan: *mut topo_scan) -> bool {
    if (*(*tscan).c).extended_cpuid_level < 0x80000008 {
        return false;
    }

    let mut ecx = Parse80000008Ecx { cpu_nthreads: 0, apicid_coreid_len: 0, perf_tsc_len: 0 };
    cpuid_leaf_reg(0x80000008, CPUID_ECX, &mut ecx);

    let mut sft = ecx.apicid_coreid_len;
    if sft == 0 {
        sft = get_count_order(ecx.cpu_nthreads + 1);
    }

    topology_update_dom(tscan, TOPO_SMT_DOMAIN, 0, 1);
    topology_set_dom(tscan, TOPO_CORE_DOMAIN, sft, ecx.cpu_nthreads + 1);
    true
}

unsafe fn store_node(tscan: *mut topo_scan, nr_nodes: u16, node_id: u16) {
    (*tscan).amd_nodes_per_pkg = nr_nodes;
    (*tscan).amd_node_id = node_id;
}

unsafe fn parse_8000_001e(tscan: *mut topo_scan) -> bool {
    if !boot_cpu_has(X86_FEATURE_TOPOEXT) {
        return false;
    }

    let mut leaf = Parse8000001eLeaf { ext_apic_id: 0, core_id: 0, core_nthreads: 0, node_id: 0, nnodes_per_socket: 0 };
    cpuid_read(0x8000001e, &mut leaf);

    if !cpu_feature_enabled(X86_FEATURE_XTOPOLOGY) {
        (*(*tscan).c).topo.initial_apicid = leaf.ext_apic_id;
        if (*(*tscan).c).x86 >= 0x17 {
            let nthreads = leaf.core_nthreads + 1;
            topology_update_dom(tscan, TOPO_SMT_DOMAIN, get_count_order(nthreads), nthreads);
        }
    }

    store_node(tscan, (leaf.nnodes_per_socket + 1) as u16, leaf.node_id as u16);
    if (*(*tscan).c).x86_vendor == X86_VENDOR_AMD {
        if (*(*tscan).c).x86 == 0x15 {
            (*(*tscan).c).topo.cu_id = leaf.core_id;
        }
        cacheinfo_amd_init_llc_id((*tscan).c, leaf.node_id);
    } else {
        if !boot_cpu_has(X86_FEATURE_HYPERVISOR) && (*(*tscan).c).x86_model <= 0x3 {
            topology_set_dom(tscan, TOPO_CORE_DOMAIN, 6, (*tscan).dom_ncpus[TOPO_CORE_DOMAIN]);
        }
        cacheinfo_hygon_init_llc_id((*tscan).c);
    }
    true
}

unsafe fn parse_fam10h_node_id(tscan: *mut topo_scan) {
    if !boot_cpu_has(X86_FEATURE_NODEID_MSR) { return; }
    let mut msr: u64 = 0;
    rdmsrq(MSR_FAM10H_NODE_ID, msr);
    store_node(tscan, (((msr >> 3) & 7) + 1) as u16, (msr & 7) as u16);
    (*(*tscan).c).topo.llc_id = (msr & 7) as u32;
}

unsafe fn legacy_set_llc(tscan: *mut topo_scan) {
    let apicid = (*(*tscan).c).topo.initial_apicid;
    if (*(*tscan).c).topo.llc_id == BAD_APICID {
        (*(*tscan).c).topo.llc_id = apicid >> (*tscan).dom_shifts[TOPO_CORE_DOMAIN];
    }
}

unsafe fn topoext_fixup(tscan: *mut topo_scan) {
    let c = (*tscan).c;
    let mut msrval: u64 = 0;
    if cpu_has(c, X86_FEATURE_TOPOEXT) || (*c).x86_vendor != X86_VENDOR_AMD || (*c).x86 != 0x15 || (*c).x86_model < 0x10 || (*c).x86_model > 0x6f { return; }
    if msr_set_bit(MSR_AMD64_CPUID_EXT_FEAT, MSR_AMD64_CPUID_EXT_FEAT_TOPOEXT_BIT) <= 0 { return; }
    rdmsrq(MSR_AMD64_CPUID_EXT_FEAT, msrval);
    if msrval & MSR_AMD64_CPUID_EXT_FEAT_TOPOEXT != 0 {
        set_cpu_cap(c, X86_FEATURE_TOPOEXT);
        pr_info_once!(FW_INFO "CPU: Re-enabling disabled Topology Extensions Support.\n");
    }
}

unsafe fn parse_topology_amd(tscan: *mut topo_scan) {
    if cpu_feature_enabled(X86_FEATURE_AMD_HTR_CORES) {
        (*(*tscan).c).topo.hw_cpu_type = cpuid_ebx(0x80000026);
        (*(*tscan).c).topo.cpu_type = get_topology_cpu_type((*tscan).c);
    }
    if !cpu_parse_topology_ext(tscan) && !parse_8000_0008(tscan) { return; }
    if parse_8000_001e(tscan) { return; }
    parse_fam10h_node_id(tscan);
}

pub unsafe fn cpu_parse_topology_amd(tscan: *mut topo_scan) {
    (*tscan).amd_nodes_per_pkg = 1;
    topoext_fixup(tscan);
    parse_topology_amd(tscan);
    legacy_set_llc(tscan);
    if (*tscan).amd_nodes_per_pkg > 1 { set_cpu_cap((*tscan).c, X86_FEATURE_AMD_DCM); }
}

pub unsafe fn cpu_topology_fixup_amd(tscan: *mut topo_scan) {
    let c = (*tscan).c;
    if (*c).x86 < 0x17 && (*tscan).amd_nodes_per_pkg > 1 {
        (*c).topo.core_id %= (*tscan).dom_ncpus[TOPO_CORE_DOMAIN] / (*tscan).amd_nodes_per_pkg;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
