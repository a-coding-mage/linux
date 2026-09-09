// SPDX-License-Identifier: GPL-2.0
/*
 * ACPI 5.1 based NUMA setup for ARM64
 * Lots of code was borrowed from arch/x86/mm/srat.c
 *
 * Copyright 2004 Andi Kleen, SuSE Labs.
 * Copyright (C) 2013-2016, Linaro Ltd.
 *		Author: Hanjun Guo <hanjun.guo@linaro.org>
 *
 * Reads the ACPI SRAT table to figure out what memory belongs to which CPUs.
 *
 * Called from acpi_numa_init while reading the SRAT and SLIT tables.
 * Assumes all memory regions belonging to a single proximity domain
 * are in one chunk. Holes between them will be included in the node.
 */

// #define pr_fmt(fmt) "ACPI: NUMA: " fmt
// C header dependencies are supplied by the surrounding kernel translation.

static mut acpi_early_node_map: [i32; NR_CPUS] = [NUMA_NO_NODE; NR_CPUS];

pub unsafe fn acpi_numa_get_nid(cpu: u32) -> i32
{
	acpi_early_node_map[cpu as usize]
}

unsafe fn acpi_parse_gicc_pxm(
	header: *mut acpi_subtable_headers,
	_end: u64,
) -> i32
{
	let pa: *mut acpi_srat_gicc_affinity;
	let cpu: i32;
	let pxm: i32;
	let node: i32;

	if srat_disabled() {
		return -EINVAL;
	}

	pa = header as *mut acpi_srat_gicc_affinity;
	if pa.is_null() {
		return -EINVAL;
	}

	if ((*pa).flags & ACPI_SRAT_GICC_ENABLED) == 0 {
		return 0;
	}

	pxm = (*pa).proximity_domain;
	node = pxm_to_node(pxm);

	/*
	 * If we can't map the UID to a logical cpu this
	 * means that the UID is not part of possible cpus
	 * so we do not need a NUMA mapping for it, skip
	 * the SRAT entry and keep parsing.
	 */
	cpu = get_cpu_for_acpi_id((*pa).acpi_processor_uid);
	if cpu < 0 {
		return 0;
	}

	acpi_early_node_map[cpu as usize] = node;
	pr_info!("SRAT: PXM {} -> MPIDR 0x{:x} -> Node {}\n", pxm,
		cpu_logical_map(cpu), node);

	0
}

pub unsafe fn acpi_map_cpus_to_nodes()
{
	acpi_table_parse_entries(
		ACPI_SIG_SRAT,
		core::mem::size_of::<acpi_table_srat>(),
		ACPI_SRAT_TYPE_GICC_AFFINITY,
		acpi_parse_gicc_pxm,
		0,
	);
}

/* Callback for Proximity Domain -> ACPI processor UID mapping */
pub unsafe fn acpi_numa_gicc_affinity_init(pa: *mut acpi_srat_gicc_affinity)
{
	let pxm: i32;
	let node: i32;

	if srat_disabled() {
		return;
	}

	if (*pa).header.length < core::mem::size_of::<acpi_srat_gicc_affinity>() {
		pr_err!("SRAT: Invalid SRAT header length: {}\n", (*pa).header.length);
		bad_srat();
		return;
	}

	if ((*pa).flags & ACPI_SRAT_GICC_ENABLED) == 0 {
		return;
	}

	pxm = (*pa).proximity_domain;
	node = acpi_map_pxm_to_node(pxm);

	if node == NUMA_NO_NODE {
		pr_err!("SRAT: Too many proximity domains {}\n", pxm);
		bad_srat();
		return;
	}

	node_set(node, &mut numa_nodes_parsed);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
