// SPDX-License-Identifier: GPL-2.0
/*
 * ACPI 6.6 based NUMA setup for RISCV
 * Lots of code was borrowed from arch/arm64/kernel/acpi_numa.c
 *
 * Copyright 2004 Andi Kleen, SuSE Labs.
 * Copyright (C) 2013-2016, Linaro Ltd.
 *		Author: Hanjun Guo <hanjun.guo@linaro.org>
 * Copyright (C) 2024 Intel Corporation.
 *
 * Reads the ACPI SRAT table to figure out what memory belongs to which CPUs.
 *
 * Called from acpi_numa_init while reading the SRAT and SLIT tables.
 * Assumes all memory regions belonging to a single proximity domain
 * are in one chunk. Holes between them will be included in the node.
 */

// C preprocessor dependencies and build-time annotations are supplied by the
// surrounding kernel translation unit.

#[repr(C)]
pub struct acpi_subtable_headers {
    pub length: u16,
}

#[repr(C)]
pub struct acpi_srat_rintc_affinity {
    pub header: acpi_subtable_headers,
    pub proximity_domain: u32,
    pub flags: u32,
    pub acpi_processor_uid: u32,
}

extern "C" {
    static mut nr_cpu_ids: i32;
    static mut numa_nodes_parsed: core::ffi::c_ulong;

    fn srat_disabled() -> bool;
    fn acpi_get_cpu_uid(cpu: i32, uid: *mut u32) -> i32;
    fn pxm_to_node(pxm: i32) -> i32;
    fn acpi_map_pxm_to_node(pxm: i32) -> i32;
    fn cpuid_to_hartid_map(cpu: i32) -> u64;
    fn early_map_cpu_to_node(cpu: i32, node: i32);
    fn node_set(node: i32, map: *mut core::ffi::c_ulong);
    fn bad_srat();
    fn pr_info(fmt: *const core::ffi::c_char, ...);
    fn pr_err(fmt: *const core::ffi::c_char, ...);
    fn acpi_table_parse_entries(
        sig: u32,
        table_size: usize,
        entry_type: u32,
        handler: unsafe extern "C" fn(*mut acpi_subtable_headers, usize) -> i32,
        max_entries: i32,
    );
}

const NR_CPUS: usize = 0; // Supplied by the kernel configuration.
const NUMA_NO_NODE: i32 = -1;
const EINVAL: i32 = 22;
const ACPI_SRAT_RINTC_ENABLED: u32 = 1;
const ACPI_SIG_SRAT: u32 = 0x5441_5253;
const ACPI_SRAT_TYPE_RINTC_AFFINITY: u32 = 7;

static mut acpi_early_node_map: [i32; NR_CPUS] = [NUMA_NO_NODE; NR_CPUS];

unsafe fn acpi_numa_get_nid(cpu: u32) -> i32 {
    acpi_early_node_map[cpu as usize]
}

unsafe fn get_cpu_for_acpi_id(uid: u32) -> i32 {
    let mut cpu_uid: u32 = 0;
    let mut ret: i32;

    for cpu in 0..nr_cpu_ids {
        ret = acpi_get_cpu_uid(cpu, &mut cpu_uid);
        if ret == 0 && uid == cpu_uid {
            return cpu;
        }
    }

    -EINVAL
}

unsafe extern "C" fn acpi_parse_rintc_pxm(
    header: *mut acpi_subtable_headers,
    _end: usize,
) -> i32 {
    if srat_disabled() {
        return -EINVAL;
    }

    if header.is_null() {
        return -EINVAL;
    }

    let pa = header as *mut acpi_srat_rintc_affinity;
    if (*pa).flags & ACPI_SRAT_RINTC_ENABLED == 0 {
        return 0;
    }

    let pxm = (*pa).proximity_domain as i32;
    let node = pxm_to_node(pxm);

    /*
     * If we can't map the UID to a logical cpu this
     * means that the UID is not part of possible cpus
     * so we do not need a NUMA mapping for it, skip
     * the SRAT entry and keep parsing.
     */
    let cpu = get_cpu_for_acpi_id((*pa).acpi_processor_uid);
    if cpu < 0 {
        return 0;
    }

    acpi_early_node_map[cpu as usize] = node;
    pr_info(
        b"SRAT: PXM %d -> HARTID 0x%lx -> Node %d\n\0".as_ptr() as *const _,
        pxm,
        cpuid_to_hartid_map(cpu),
        node,
    );

    0
}

pub unsafe fn acpi_map_cpus_to_nodes() {
    /*
     * In ACPI, SMP and CPU NUMA information is provided in separate
     * static tables, namely the MADT and the SRAT.
     *
     * Thus, it is simpler to first create the cpu logical map through
     * an MADT walk and then map the logical cpus to their node ids
     * as separate steps.
     */
    acpi_table_parse_entries(
        ACPI_SIG_SRAT,
        core::mem::size_of::<acpi_table_srat>(),
        ACPI_SRAT_TYPE_RINTC_AFFINITY,
        acpi_parse_rintc_pxm,
        0,
    );

    for i in 0..nr_cpu_ids {
        early_map_cpu_to_node(i, acpi_numa_get_nid(i as u32));
    }
}

/* Callback for Proximity Domain -> logical node ID mapping */
pub unsafe fn acpi_numa_rintc_affinity_init(pa: *mut acpi_srat_rintc_affinity) {
    if srat_disabled() {
        return;
    }

    if (*pa).header.length as usize < core::mem::size_of::<acpi_srat_rintc_affinity>() {
        pr_err(
            b"SRAT: Invalid SRAT header length: %d\n\0".as_ptr() as *const _,
            (*pa).header.length,
        );
        bad_srat();
        return;
    }

    if (*pa).flags & ACPI_SRAT_RINTC_ENABLED == 0 {
        return;
    }

    let pxm = (*pa).proximity_domain as i32;
    let node = acpi_map_pxm_to_node(pxm);

    if node == NUMA_NO_NODE {
        pr_err(
            b"SRAT: Too many proximity domains %d\n\0".as_ptr() as *const _,
            pxm,
        );
        bad_srat();
        return;
    }

    node_set(node, &mut numa_nodes_parsed);
}

#[repr(C)]
pub struct acpi_table_srat {
    pub _opaque: [u8; 0],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
