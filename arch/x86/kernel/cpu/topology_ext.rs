// SPDX-License-Identifier: GPL-2.0

#[repr(u32)]
enum TopoTypes {
    INVALID_TYPE = 0,
    SMT_TYPE = 1,
    CORE_TYPE = 2,
    MAX_TYPE_0B = 3,
    MODULE_TYPE = 3,
    AMD_CCD_TYPE = 3,
    TILE_TYPE = 4,
    AMD_SOCKET_TYPE = 4,
    MAX_TYPE_80000026 = 5,
    DIE_TYPE = 5,
    DIEGRP_TYPE = 6,
    MAX_TYPE_1F = 7,
}

/*
 * Use a lookup table for the case that there are future types > 6 which
 * describe an intermediate domain level which does not exist today.
 */
static TOPO_DOMAIN_MAP_0B_1F: [u32; TopoTypes::MAX_TYPE_1F as usize] = [
    /* SMT_TYPE */ TOPO_SMT_DOMAIN,
    /* CORE_TYPE */ TOPO_CORE_DOMAIN,
    /* MODULE_TYPE */ TOPO_MODULE_DOMAIN,
    /* TILE_TYPE */ TOPO_TILE_DOMAIN,
    /* DIE_TYPE */ TOPO_DIE_DOMAIN,
    /* DIEGRP_TYPE */ TOPO_DIEGRP_DOMAIN,
];

static TOPO_DOMAIN_MAP_80000026: [u32; TopoTypes::MAX_TYPE_80000026 as usize] = [
    /* SMT_TYPE */ TOPO_SMT_DOMAIN,
    /* CORE_TYPE */ TOPO_CORE_DOMAIN,
    /* AMD_CCD_TYPE */ TOPO_TILE_DOMAIN,
    /* AMD_SOCKET_TYPE */ TOPO_DIE_DOMAIN,
];

#[repr(C)]
struct TopologySubleaf {
    eax: u32,
    ebx: u32,
    ecx: u32,
    edx: u32,
}

impl TopologySubleaf {
    #[inline]
    unsafe fn x2apic_shift(&self) -> u32 { self.eax & 0x1f }
    #[inline]
    unsafe fn num_processors(&self) -> u32 { self.ebx & 0xffff }
    #[inline]
    unsafe fn level(&self) -> u32 { self.ecx & 0xff }
    #[inline]
    unsafe fn type_(&self) -> u32 { (self.ecx >> 8) & 0xff }
    #[inline]
    unsafe fn x2apic_id(&self) -> u32 { self.edx }
}

#[inline]
unsafe fn topo_subleaf(tscan: *mut topo_scan, leaf: u32, subleaf: u32, last_dom: *mut u32) -> bool {
    let (maxtype, map): (u32, *const u32) = match leaf {
        0x0b => (TopoTypes::MAX_TYPE_0B as u32, TOPO_DOMAIN_MAP_0B_1F.as_ptr()),
        0x1f => (TopoTypes::MAX_TYPE_1F as u32, TOPO_DOMAIN_MAP_0B_1F.as_ptr()),
        0x80000026 => (TopoTypes::MAX_TYPE_80000026 as u32, TOPO_DOMAIN_MAP_80000026.as_ptr()),
        _ => return false,
    };

    let mut sl = TopologySubleaf { eax: 0, ebx: 0, ecx: 0, edx: 0 };
    cpuid_read_subleaf(leaf, subleaf, &mut sl);

    if sl.num_processors() == 0 || sl.type_() == TopoTypes::INVALID_TYPE as u32 { return false; }

    let dom;
    if sl.type_() >= maxtype {
        pr_err_once!("Topology: leaf 0x%x:%d Unknown domain type %u\n", leaf, subleaf, sl.type_());
        /*
         * It really would have been too obvious to make the domain
         * type space sparse and leave a few reserved types between
         * the points which might change instead of following the
         * usual "this can be fixed in software" principle.
         */
        dom = (*last_dom).wrapping_add(1);
    } else {
        dom = *map.add(sl.type_() as usize);
        *last_dom = dom;
    }

    if dom == 0 {
        (*(*tscan).c).topo.initial_apicid = sl.x2apic_id();
    } else if (*(*tscan).c).topo.initial_apicid != sl.x2apic_id() {
        pr_warn_once!("FW_BUG CPUID leaf 0x%x subleaf %d APIC ID mismatch %x != %x\n",
            leaf, subleaf, (*(*tscan).c).topo.initial_apicid, sl.x2apic_id());
    }

    topology_set_dom(tscan, dom, sl.x2apic_shift(), sl.num_processors());
    true
}

unsafe fn parse_topology_leaf(tscan: *mut topo_scan, leaf: u32) -> bool {
    let mut last_dom = 0;
    let mut subleaf = 0;

    /* Read all available subleafs and populate the levels */
    while topo_subleaf(tscan, leaf, subleaf, &mut last_dom) { subleaf = subleaf.wrapping_add(1); }

    /* If subleaf 0 failed to parse, give up */
    if subleaf == 0 { return false; }

    /*
     * There are machines in the wild which have shift 0 in the subleaf
     * 0, but advertise 2 logical processors at that level. They are
     * truly SMT.
     */
    if (*tscan).dom_shifts[TOPO_SMT_DOMAIN as usize] == 0 && (*tscan).dom_ncpus[TOPO_SMT_DOMAIN as usize] > 1 {
        let sft = get_count_order((*tscan).dom_ncpus[TOPO_SMT_DOMAIN as usize]);
        pr_warn_once!("FW_BUG CPUID leaf 0x%x subleaf 0 has shift level 0 but %u CPUs. Fixing it up.\n",
            leaf, (*tscan).dom_ncpus[TOPO_SMT_DOMAIN as usize]);
        topology_update_dom(tscan, TOPO_SMT_DOMAIN, sft, (*tscan).dom_ncpus[TOPO_SMT_DOMAIN as usize]);
    }

    set_cpu_cap((*tscan).c, X86_FEATURE_XTOPOLOGY);
    true
}

pub unsafe fn cpu_parse_topology_ext(tscan: *mut topo_scan) -> bool {
    /* Intel: Try leaf 0x1F first. */
    if (*(*tscan).c).cpuid_level >= 0x1f && parse_topology_leaf(tscan, 0x1f) { return true; }

    /* AMD: Try leaf 0x80000026 first. */
    if (*(*tscan).c).extended_cpuid_level >= 0x80000026 && parse_topology_leaf(tscan, 0x80000026) { return true; }

    /* Intel/AMD: Fall back to leaf 0xB if available */
    (*(*tscan).c).cpuid_level >= 0x0b && parse_topology_leaf(tscan, 0x0b)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
