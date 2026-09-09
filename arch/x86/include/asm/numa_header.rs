/* SPDX-License-Identifier: GPL-2.0 */

// C dependencies: linux/nodemask.h, linux/errno.h, asm/topology.h,
// and asm/apicdef.h are supplied by other translated files.

#[cfg(CONFIG_NUMA)]
extern "C" {
    pub static mut numa_off: ::core::ffi::c_int;

    /*
     * __apicid_to_node[] stores the raw mapping between physical apicid and
     * node and is used to initialize cpu_to_node mapping.
     *
     * The mapping may be overridden by apic->numa_cpu_node() on 32bit and thus
     * should be accessed by the accessors - set_apicid_to_node() and
     * numa_cpu_node().
     */
    pub static mut __apicid_to_node: [i16; MAX_LOCAL_APIC];
    pub static mut numa_nodes_parsed: nodemask_t;
    pub static mut numa_phys_nodes_parsed: nodemask_t;

    pub fn numa_cpu_node(cpu: ::core::ffi::c_int) -> ::core::ffi::c_int;

    pub fn numa_set_node(cpu: ::core::ffi::c_int, node: ::core::ffi::c_int);
    pub fn numa_clear_node(cpu: ::core::ffi::c_int);
    pub fn init_cpu_to_node();
    pub fn numa_add_cpu(cpu: ::core::ffi::c_uint);
    pub fn numa_remove_cpu(cpu: ::core::ffi::c_uint);
    pub fn init_gi_nodes();
    pub fn num_phys_nodes() -> ::core::ffi::c_int;
}

#[cfg(CONFIG_NUMA)]
#[inline]
pub unsafe fn set_apicid_to_node(apicid: ::core::ffi::c_int, node: i16) {
    __apicid_to_node[apicid as usize] = node;
}

#[cfg(not(CONFIG_NUMA))]
#[inline]
pub fn set_apicid_to_node(_apicid: ::core::ffi::c_int, _node: i16) {}

#[cfg(not(CONFIG_NUMA))]
#[inline]
pub fn numa_cpu_node(_cpu: ::core::ffi::c_int) -> ::core::ffi::c_int {
    NUMA_NO_NODE
}

#[cfg(not(CONFIG_NUMA))]
#[inline]
pub fn numa_set_node(_cpu: ::core::ffi::c_int, _node: ::core::ffi::c_int) {}

#[cfg(not(CONFIG_NUMA))]
#[inline]
pub fn numa_clear_node(_cpu: ::core::ffi::c_int) {}

#[cfg(not(CONFIG_NUMA))]
#[inline]
pub fn init_cpu_to_node() {}

#[cfg(not(CONFIG_NUMA))]
#[inline]
pub fn numa_add_cpu(_cpu: ::core::ffi::c_uint) {}

#[cfg(not(CONFIG_NUMA))]
#[inline]
pub fn numa_remove_cpu(_cpu: ::core::ffi::c_uint) {}

#[cfg(not(CONFIG_NUMA))]
#[inline]
pub fn init_gi_nodes() {}

#[cfg(not(CONFIG_NUMA))]
#[inline]
pub fn num_phys_nodes() -> ::core::ffi::c_int {
    1
}

#[cfg(CONFIG_DEBUG_PER_CPU_MAPS)]
extern "C" {
    pub fn debug_cpumask_set_cpu(
        cpu: ::core::ffi::c_uint,
        node: ::core::ffi::c_int,
        enable: bool,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
