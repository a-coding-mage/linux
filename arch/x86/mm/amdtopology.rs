// SPDX-License-Identifier: GPL-2.0
/*
 * AMD NUMA support.
 * Discover the memory map and associated nodes.
 *
 * This version reads it directly from the AMD northbridge.
 *
 * Copyright 2002,2003 Andi Kleen, SuSE Labs.
 */

// Linux kernel dependencies supplied by other translation units.

static mut NODEIDS: [u8; 8] = [0; 8];

unsafe fn find_northbridge() -> i32 {
    let mut num: i32;

    for num in 0..32 {
        let mut header: u32;

        header = read_pci_config(0, num, 0, 0x00);
        if header != (PCI_VENDOR_ID_AMD | (0x1100 << 16))
            && header != (PCI_VENDOR_ID_AMD | (0x1200 << 16))
            && header != (PCI_VENDOR_ID_AMD | (0x1300 << 16))
        {
            continue;
        }

        header = read_pci_config(0, num, 1, 0x00);
        if header != (PCI_VENDOR_ID_AMD | (0x1101 << 16))
            && header != (PCI_VENDOR_ID_AMD | (0x1201 << 16))
            && header != (PCI_VENDOR_ID_AMD | (0x1301 << 16))
        {
            continue;
        }
        return num;
    }

    -ENOENT
}

pub unsafe fn amd_numa_init() -> i32 {
    let numnodes: u32;
    let mut cores: u32;
    let mut apicid: u32;
    let mut prevbase: u64;
    let start: u64 = PFN_PHYS(0);
    let end: u64 = PFN_PHYS(max_pfn);
    let mut nodeid: u32;
    let mut reg: u32;
    let mut i: i32;
    let mut j: u32;
    let nb: i32;

    if !early_pci_allowed() {
        return -EINVAL;
    }

    nb = find_northbridge();
    if nb < 0 {
        return nb;
    }

    pr_info!("Scanning NUMA topology in Northbridge {}\n", nb);

    reg = read_pci_config(0, nb, 0, 0x60);
    numnodes = ((reg >> 4) & 0xF) + 1;
    if numnodes <= 1 {
        return -ENOENT;
    }

    pr_info!("Number of physical nodes {}\n", numnodes);

    prevbase = 0;
    for i in 0..8 {
        let mut base: u64;
        let mut limit: u64;

        base = read_pci_config(0, nb, 1, 0x40 + i * 8) as u64;
        limit = read_pci_config(0, nb, 1, 0x44 + i * 8) as u64;

        NODEIDS[i as usize] = (limit & 7) as u8;
        nodeid = limit as u32 & 7;
        if (base & 3) == 0 {
            if (i as u32) < numnodes {
                pr_info!("Skipping disabled node {}\n", i);
            }
            continue;
        }
        if nodeid >= numnodes {
            pr_info!("Ignoring excess node {} ({:x}:{:x})\n", nodeid, base, limit);
            continue;
        }

        if limit == 0 {
            pr_info!("Skipping node entry {} (base {:x})\n", i, base);
            continue;
        }
        if ((base >> 8) & 3) != 0 || ((limit >> 8) & 3) != 0 {
            pr_err!("Node {} using interleaving mode {:x}/{:x}\n", nodeid, (base >> 8) & 3, (limit >> 8) & 3);
            return -EINVAL;
        }
        if node_isset(nodeid, numa_nodes_parsed) {
            pr_info!("Node {} already present, skipping\n", nodeid);
            continue;
        }

        limit >>= 16;
        limit += 1;
        limit <<= 24;

        if limit > end { limit = end; }
        if limit <= base { continue; }

        base >>= 16;
        base <<= 24;

        if base < start { base = start; }
        if limit > end { limit = end; }
        if limit == base {
            pr_err!("Empty node {}\n", nodeid);
            continue;
        }
        if limit < base {
            pr_err!("Node {} bogus settings {:x}-{:x}.\n", nodeid, base, limit);
            continue;
        }

        /* Could sort here, but pun for now. Should not happen anyroads. */
        if prevbase > base {
            pr_err!("Node map not sorted {:x},{:x}\n", prevbase, base);
            return -EINVAL;
        }

        pr_info!("Node {} MemBase {:016x} Limit {:016x}\n", nodeid, base, limit);

        prevbase = base;
        numa_add_memblk(nodeid, base, limit);
    }

    if nodes_empty(numa_nodes_parsed) {
        return -ENOENT;
    }

    /*
     * We seem to have valid NUMA configuration. Map apicids to nodes
     * using the size of the core domain in the APIC space.
     */
    cores = topology_get_domain_size(TOPO_CORE_DOMAIN);

    apicid = boot_cpu_physical_apicid;
    if apicid > 0 {
        pr_info!("BSP APIC ID: {:02x}\n", apicid);
    }

    for_each_node_mask!(i, numa_nodes_parsed, {
        for j in 0..cores {
            set_apicid_to_node(apicid, i);
            apicid += 1;
        }
    });
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
