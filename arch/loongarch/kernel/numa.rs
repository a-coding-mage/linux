// SPDX-License-Identifier: GPL-2.0
/*
 * Author:  Xiang Gao <gaoxiang@loongson.cn>
 *          Huacai Chen <chenhuacai@loongson.cn>
 *
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 */

// Declarations supplied by the Linux and LoongArch headers are external
// dependencies of this translation.

pub static mut numa_off: ::core::ffi::c_int = 0;
pub static mut cpus_on_node: [cpumask_t; MAX_NUMNODES] = [cpumask_t::default(); MAX_NUMNODES];
pub static mut phys_cpus_on_node: [cpumask_t; MAX_NUMNODES] = [cpumask_t::default(); MAX_NUMNODES];

/* apicid, cpu, node mappings */
#[no_mangle]
pub static mut __cpuid_to_node: [i16; CONFIG_NR_CPUS] = [NUMA_NO_NODE as i16; CONFIG_NR_CPUS];

#[cfg(feature = "CONFIG_HAVE_SETUP_PER_CPU_AREA")]
#[no_mangle]
pub static mut __per_cpu_offset: [usize; NR_CPUS] = [0; NR_CPUS];

#[cfg(feature = "CONFIG_HAVE_SETUP_PER_CPU_AREA")]
unsafe fn pcpu_cpu_to_node(cpu: ::core::ffi::c_int) -> ::core::ffi::c_int {
    early_cpu_to_node(cpu)
}

#[cfg(feature = "CONFIG_HAVE_SETUP_PER_CPU_AREA")]
unsafe fn pcpu_cpu_distance(from: u32, to: u32) -> ::core::ffi::c_int {
    if early_cpu_to_node(from as ::core::ffi::c_int) == early_cpu_to_node(to as ::core::ffi::c_int) {
        LOCAL_DISTANCE
    } else {
        REMOTE_DISTANCE
    }
}

#[cfg(feature = "CONFIG_HAVE_SETUP_PER_CPU_AREA")]
pub unsafe fn pcpu_populate_pte(addr: usize) {
    populate_kernel_pte(addr);
}

#[cfg(feature = "CONFIG_HAVE_SETUP_PER_CPU_AREA")]
pub unsafe fn setup_per_cpu_areas() {
    let mut delta: usize;
    let mut cpu: u32;
    let mut rc: ::core::ffi::c_int = -EINVAL;

    if pcpu_chosen_fc == PCPU_FC_AUTO {
        if nr_node_ids >= 8 {
            pcpu_chosen_fc = PCPU_FC_PAGE;
        } else {
            pcpu_chosen_fc = PCPU_FC_EMBED;
        }
    }

    /* Always reserve area for module percpu variables. */
    if pcpu_chosen_fc != PCPU_FC_PAGE {
        rc = pcpu_embed_first_chunk(
            PERCPU_MODULE_RESERVE, PERCPU_DYNAMIC_RESERVE, PMD_SIZE,
            pcpu_cpu_distance, pcpu_cpu_to_node,
        );
        if rc < 0 {
            pr_warn("%s allocator failed (%d), falling back to page size\n", pcpu_fc_names[pcpu_chosen_fc], rc);
        }
    }
    if rc < 0 {
        rc = pcpu_page_first_chunk(PERCPU_MODULE_RESERVE, pcpu_cpu_to_node);
    }
    if rc < 0 {
        panic!("cannot initialize percpu area (err=%d)", rc);
    }

    delta = pcpu_base_addr as usize - __per_cpu_start as usize;
    for_each_possible_cpu!(cpu) {
        __per_cpu_offset[cpu as usize] = delta + pcpu_unit_offsets[cpu as usize];
    }
}

/*
 * Get nodeid by logical cpu number.
 * __cpuid_to_node maps phyical cpu id to node, so we
 * should use cpu_logical_map(cpu) to index it.
 *
 * This routine is only used in early phase during
 * booting, after setup_per_cpu_areas calling and numa_node
 * initialization, cpu_to_node will be used instead.
 */
pub unsafe fn early_cpu_to_node(cpu: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let physid = cpu_logical_map(cpu);
    if physid < 0 { NUMA_NO_NODE } else { __cpuid_to_node[physid as usize] as ::core::ffi::c_int }
}

pub unsafe fn early_numa_add_cpu(cpuid: ::core::ffi::c_int, node: i16) {
    let cpu = __cpu_number_map[cpuid as usize];
    if cpu < 0 { return; }
    cpumask_set_cpu(cpu, &mut cpus_on_node[node as usize]);
    cpumask_set_cpu(cpuid, &mut phys_cpus_on_node[node as usize]);
}

pub unsafe fn numa_add_cpu(cpu: u32) {
    let nid = cpu_to_node(cpu);
    cpumask_set_cpu(cpu, &mut cpus_on_node[nid as usize]);
}

pub unsafe fn numa_remove_cpu(cpu: u32) {
    let nid = cpu_to_node(cpu);
    cpumask_clear_cpu(cpu, &mut cpus_on_node[nid as usize]);
}

unsafe fn node_mem_init(node: u32) {
    let mut start_pfn: usize = 0;
    let mut end_pfn: usize = 0;
    let node_addrspace_offset = nid_to_addrbase(node);
    pr_info!("Node{}'s addrspace_offset is 0x{:x}\n", node, node_addrspace_offset);
    get_pfn_range_for_nid(node, &mut start_pfn, &mut end_pfn);
    pr_info!("Node{}: start_pfn=0x{:x}, end_pfn=0x{:x}\n", node, start_pfn, end_pfn);
    alloc_node_data(node);
}

#[cfg(feature = "CONFIG_ACPI_NUMA")]
static mut num_physpages: usize = 0;

#[cfg(feature = "CONFIG_ACPI_NUMA")]
unsafe fn info_node_memblock() {
    for_each_efi_memory_desc!(md) {
        let mem_type = (*md).type_;
        let mem_start = (*md).phys_addr;
        let mem_size = (*md).num_pages << EFI_PAGE_SHIFT;
        let mem_end = mem_start + mem_size;
        match mem_type {
            EFI_LOADER_CODE | EFI_LOADER_DATA | EFI_BOOT_SERVICES_CODE |
            EFI_BOOT_SERVICES_DATA | EFI_PERSISTENT_MEMORY | EFI_CONVENTIONAL_MEMORY => {
                num_physpages += mem_size >> PAGE_SHIFT;
                pr_info!("Node{}: mem_type:{}, mem_start:0x{:x}, mem_size:0x{:x} Bytes\n", pa_to_nid(mem_start), mem_type, mem_start, mem_size);
                pr_info!("       start_pfn:0x{:x}, end_pfn:0x{:x}, num_physpages:0x{:x}\n", mem_start >> PAGE_SHIFT, mem_end >> PAGE_SHIFT, num_physpages);
            }
            EFI_PAL_CODE | EFI_UNUSABLE_MEMORY | EFI_ACPI_RECLAIM_MEMORY => {
                num_physpages += mem_size >> PAGE_SHIFT;
                pr_info!("Node{}: mem_type:{}, mem_start:0x{:x}, mem_size:0x{:x} Bytes\n", pa_to_nid(mem_start), mem_type, mem_start, mem_size);
                pr_info!("       start_pfn:0x{:x}, end_pfn:0x{:x}, num_physpages:0x{:x}\n", mem_start >> PAGE_SHIFT, mem_end >> PAGE_SHIFT, num_physpages);
                pr_info!("Resvd: mem_type:{}, mem_start:0x{:x}, mem_size:0x{:x} Bytes\n", mem_type, mem_start, mem_size);
            }
            EFI_RESERVED_TYPE | EFI_RUNTIME_SERVICES_CODE | EFI_RUNTIME_SERVICES_DATA |
            EFI_MEMORY_MAPPED_IO | EFI_MEMORY_MAPPED_IO_PORT_SPACE => {
                pr_info!("Resvd: mem_type:{}, mem_start:0x{:x}, mem_size:0x{:x} Bytes\n", mem_type, mem_start, mem_size);
            }
            _ => {}
        }
    }
}

#[cfg(feature = "CONFIG_ACPI_NUMA")]
unsafe fn fake_numa_init() -> ::core::ffi::c_int {
    let start = memblock_start_of_DRAM();
    let end = memblock_end_of_DRAM() - 1;
    pr_info!("Faking a node at [mem %pap-%pap]\n", &start, &end);
    numa_add_memblk(0, start, end + 1)
}

#[cfg(feature = "CONFIG_ACPI_NUMA")]
pub unsafe fn init_numa_memory() -> ::core::ffi::c_int {
    for i in 0..NR_CPUS { set_cpuid_to_node(i, NUMA_NO_NODE); }
    let ret = if !acpi_disabled { numa_memblks_init(acpi_numa_init, false) } else { numa_memblks_init(fake_numa_init, false) };
    if ret < 0 { return ret; }
    info_node_memblock();
    if !memblock_validate_numa_coverage(SZ_1M) { return -EINVAL; }
    for_each_node_mask!(node, node_possible_map) {
        node_mem_init(node);
        node_set_online(node);
    }
    max_pfn = PFN_DOWN(memblock_end_of_DRAM());
    max_low_pfn = core::cmp::min(PFN_DOWN(HIGHMEM_START), max_pfn);
    setup_nr_node_ids();
    loongson_sysconf.nr_nodes = nr_node_ids;
    loongson_sysconf.cores_per_node = cpumask_weight(&phys_cpus_on_node[0]);
    0
}

pub unsafe fn pcibus_to_node(bus: *mut pci_bus) -> ::core::ffi::c_int {
    dev_to_node(&(*bus).dev)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
