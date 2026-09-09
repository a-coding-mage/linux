/* SPDX-License-Identifier: GPL-2.0 */

// Declarations supplied by the surrounding x86 topology implementation.
#[repr(C)]
pub struct cpuinfo_x86 {
    _private: [u8; 0],
}

pub type x86_topology_domains = u32;
pub type x86_topology_cpu_type = u32;

pub const TOPO_SMT_DOMAIN: x86_topology_domains = 0;
pub const TOPO_MAX_DOMAIN: usize = 8;

#[repr(C)]
pub struct X86TopoSystem {
    pub dom_shifts: [u32; TOPO_MAX_DOMAIN],
    pub dom_size: [u32; TOPO_MAX_DOMAIN],
}

extern "C" {
    pub static x86_topo_system: X86TopoSystem;
}

#[repr(C)]
pub struct topo_scan {
    pub c: *mut cpuinfo_x86,
    pub dom_shifts: [u32; TOPO_MAX_DOMAIN],
    pub dom_ncpus: [u32; TOPO_MAX_DOMAIN],

    /* Legacy CPUID[1]:EBX[23:16] number of logical processors */
    pub ebx1_nproc_shift: u32,

    /* AMD specific node ID which cannot be mapped into APIC space. */
    pub amd_nodes_per_pkg: u16,
    pub amd_node_id: u16,
}

extern "C" {
    pub fn cpu_init_topology(c: *mut cpuinfo_x86);
    pub fn cpu_parse_topology(c: *mut cpuinfo_x86);
    pub fn topology_set_dom(
        tscan: *mut topo_scan,
        dom: x86_topology_domains,
        shift: u32,
        ncpus: u32,
    );
    pub fn cpu_parse_topology_ext(tscan: *mut topo_scan) -> bool;
    pub fn cpu_parse_topology_amd(tscan: *mut topo_scan);
    pub fn cpu_topology_fixup_amd(tscan: *mut topo_scan);
    pub fn get_topology_cpu_type(c: *mut cpuinfo_x86) -> x86_topology_cpu_type;
}

pub unsafe fn topo_shift_apicid(apicid: u32, dom: x86_topology_domains) -> u32 {
    if dom == TOPO_SMT_DOMAIN {
        return apicid;
    }
    apicid >> x86_topo_system.dom_shifts[(dom - 1) as usize]
}

pub unsafe fn topo_relative_domain_id(apicid: u32, dom: x86_topology_domains) -> u32 {
    let mut apicid = apicid;
    if dom != TOPO_SMT_DOMAIN {
        apicid >>= x86_topo_system.dom_shifts[(dom - 1) as usize];
    }
    apicid & (x86_topo_system.dom_size[dom as usize] - 1)
}

pub unsafe fn topo_domain_mask(dom: x86_topology_domains) -> u32 {
    (1u32 << x86_topo_system.dom_shifts[dom as usize]) - 1
}

/*
 * Update a domain level after the fact without propagating. Used to fixup
 * broken CPUID enumerations.
 */
pub unsafe fn topology_update_dom(
    tscan: *mut topo_scan,
    dom: x86_topology_domains,
    shift: u32,
    ncpus: u32,
) {
    (*tscan).dom_shifts[dom as usize] = shift;
    (*tscan).dom_ncpus[dom as usize] = ncpus;
}

// CONFIG_X86_LOCAL_APIC selects the external implementation; otherwise the
// header provides the constant fallback below.
#[cfg(CONFIG_X86_LOCAL_APIC)]
extern "C" {
    pub fn topology_unit_count(
        apicid: u32,
        which_units: x86_topology_domains,
        at_level: x86_topology_domains,
    ) -> u32;
}

#[cfg(not(CONFIG_X86_LOCAL_APIC))]
pub unsafe fn topology_unit_count(
    _apicid: u32,
    _which_units: x86_topology_domains,
    _at_level: x86_topology_domains,
) -> u32 {
    1
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
