// SPDX-License-Identifier: GPL-2.0
/*
 * ACPI 3.0 based NUMA setup
 * Copyright 2004 Andi Kleen, SuSE Labs.
 *
 * Reads the ACPI SRAT table to figure out what memory belongs to which CPUs.
 *
 * Called from acpi_numa_init while reading the SRAT and SLIT tables.
 * Assumes all memory regions belonging to a single proximity domain
 * are in one chunk. Holes between them will be included in the node.
 */

// Dependencies supplied by the surrounding kernel translation.

/* Callback for Proximity Domain -> x2APIC mapping */
pub unsafe extern "C" fn acpi_numa_x2apic_affinity_init(
    pa: *mut acpi_srat_x2apic_cpu_affinity,
) {
    let mut pxm: i32;
    let mut node: i32;
    let apic_id: i32;

    if srat_disabled() {
        return;
    }
    if (*pa).header.length < core::mem::size_of::<acpi_srat_x2apic_cpu_affinity>() as _ {
        bad_srat();
        return;
    }
    if ((*pa).flags & ACPI_SRAT_CPU_ENABLED) == 0 {
        return;
    }
    pxm = (*pa).proximity_domain as i32;
    apic_id = (*pa).apic_id as i32;
    if !apic_id_valid(apic_id) {
        pr_info!("SRAT: PXM %u -> X2APIC 0x%04x ignored\n", pxm, apic_id);
        return;
    }
    node = acpi_map_pxm_to_node(pxm);
    if node < 0 {
        printk!(KERN_ERR, "SRAT: Too many proximity domains %x\n", pxm);
        bad_srat();
        return;
    }

    if apic_id >= MAX_LOCAL_APIC {
        printk!(KERN_INFO, "SRAT: PXM %u -> APIC 0x%04x -> Node %u skipped apicid that is too big\n", pxm, apic_id, node);
        return;
    }
    set_apicid_to_node(apic_id, node);
    node_set(node, numa_nodes_parsed);
    node_set(node, numa_phys_nodes_parsed);
    pr_debug!("SRAT: PXM %u -> APIC 0x%04x -> Node %u\n", pxm, apic_id, node);
}

/* Callback for Proximity Domain -> LAPIC mapping */
pub unsafe extern "C" fn acpi_numa_processor_affinity_init(
    pa: *mut acpi_srat_cpu_affinity,
) {
    let mut pxm: i32;
    let mut node: i32;
    let apic_id: i32;

    if srat_disabled() {
        return;
    }
    if (*pa).header.length != core::mem::size_of::<acpi_srat_cpu_affinity>() as _ {
        bad_srat();
        return;
    }
    if ((*pa).flags & ACPI_SRAT_CPU_ENABLED) == 0 {
        return;
    }
    pxm = (*pa).proximity_domain_lo as i32;
    if acpi_srat_revision >= 2 {
        pxm |= ((*pa).proximity_domain_hi.as_ptr() as *const u32).read() << 8) as i32;
    }
    node = acpi_map_pxm_to_node(pxm);
    if node < 0 {
        printk!(KERN_ERR, "SRAT: Too many proximity domains %x\n", pxm);
        bad_srat();
        return;
    }

    if get_uv_system_type() >= UV_X2APIC {
        apic_id = (((*pa).apic_id as i32) << 8) | (*pa).local_sapic_eid as i32;
    } else {
        apic_id = (*pa).apic_id as i32;
    }

    if apic_id >= MAX_LOCAL_APIC {
        printk!(KERN_INFO, "SRAT: PXM %u -> APIC 0x%02x -> Node %u skipped apicid that is too big\n", pxm, apic_id, node);
        return;
    }

    set_apicid_to_node(apic_id, node);
    node_set(node, numa_nodes_parsed);
    node_set(node, numa_phys_nodes_parsed);
    pr_debug!("SRAT: PXM %u -> APIC 0x%02x -> Node %u\n", pxm, apic_id, node);
}

pub unsafe extern "C" fn x86_acpi_numa_init() -> i32 {
    let ret: i32;

    ret = acpi_numa_init();
    if ret < 0 {
        return ret;
    }
    if srat_disabled() {
        return -EINVAL;
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
