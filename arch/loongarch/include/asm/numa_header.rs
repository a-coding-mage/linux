/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Author: Jianmin Lv <lvjianmin@loongson.cn>
 *         Huacai Chen <chenhuacai@loongson.cn>
 *
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 */

// C header guard: _ASM_LOONGARCH_NUMA_H
// Dependency: linux/nodemask.h

pub const NODE_ADDRSPACE_SHIFT: u32 = 44;

#[inline]
pub const fn pa_to_nid(addr: u64) -> u64 {
    (addr & 0xf00000000000) >> NODE_ADDRSPACE_SHIFT
}

#[inline]
pub const fn nid_to_addrbase(nid: u64) -> u64 {
    nid << NODE_ADDRSPACE_SHIFT
}

// The following declarations and definitions are present when CONFIG_NUMA is enabled.
#[cfg(CONFIG_NUMA)]
extern "C" {
    pub static mut numa_off: ::core::ffi::c_int;
    pub static mut __cpuid_to_node: [i16; CONFIG_NR_CPUS];
    // __initdata
    pub static mut numa_nodes_parsed: nodemask_t;

    pub fn early_numa_add_cpu(cpuid: ::core::ffi::c_int, node: i16);
    pub fn numa_add_cpu(cpu: ::core::ffi::c_uint);
    pub fn numa_remove_cpu(cpu: ::core::ffi::c_uint);
    pub fn early_cpu_to_node(cpu: ::core::ffi::c_int) -> ::core::ffi::c_int;
}

#[cfg(CONFIG_NUMA)]
#[inline]
pub unsafe fn numa_clear_node(_cpu: ::core::ffi::c_int) {
}

#[cfg(CONFIG_NUMA)]
#[inline]
pub unsafe fn set_cpuid_to_node(cpuid: ::core::ffi::c_int, node: i16) {
    __cpuid_to_node[cpuid as usize] = node;
}

// The following stubs are present when CONFIG_NUMA is disabled.
#[cfg(not(CONFIG_NUMA))]
#[inline]
pub unsafe fn early_numa_add_cpu(_cpuid: ::core::ffi::c_int, _node: i16) {
}

#[cfg(not(CONFIG_NUMA))]
#[inline]
pub unsafe fn numa_add_cpu(_cpu: ::core::ffi::c_uint) {
}

#[cfg(not(CONFIG_NUMA))]
#[inline]
pub unsafe fn numa_remove_cpu(_cpu: ::core::ffi::c_uint) {
}

#[cfg(not(CONFIG_NUMA))]
#[inline]
pub unsafe fn set_cpuid_to_node(_cpuid: ::core::ffi::c_int, _node: i16) {
}

#[cfg(not(CONFIG_NUMA))]
#[inline]
pub fn early_cpu_to_node(_cpu: ::core::ffi::c_int) -> ::core::ffi::c_int {
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
