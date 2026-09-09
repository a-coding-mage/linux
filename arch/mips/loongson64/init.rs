// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2009 Lemote Inc.
 * Author: Wu Zhangjin, wuzhangjin@gmail.com
 */

// Linux and architecture headers from the original implementation provide the
// external types, constants, globals, and functions referenced below.

const NODE_ID_OFFSET_ADDR: *mut core::ffi::c_void = TO_UNCAC(0x1001041c);

pub static mut node_id_offset: u32 = 0;

unsafe fn mips_nmi_setup() {
    let base = (CAC_BASE + 0x380) as *mut core::ffi::c_void;
    memcpy(base, except_vec_nmi, 0x80);
    flush_icache_range(base as usize, base as usize + 0x80);
}

pub unsafe fn ls7a_early_config() {
    node_id_offset = ((readl(NODE_ID_OFFSET_ADDR) >> 8) & 0x1f) + 36;
}

pub unsafe fn rs780e_early_config() {
    node_id_offset = 37;
}

pub unsafe fn virtual_early_config() {
    node_id_offset = 44;
}

pub unsafe fn szmem(node: u32) {
    let mut i: u32;
    let mut mem_type: u32;
    let mut node_id: phys_addr_t;
    let mut mem_start: phys_addr_t;
    let mut mem_size: phys_addr_t;

    /* Otherwise come from DTB */
    if loongson_sysconf.fw_interface != LOONGSON_LEFI {
        return;
    }

    /* Parse memory information and activate */
    i = 0;
    while i < loongson_memmap.nr_map {
        node_id = loongson_memmap.map[i as usize].node_id;
        if node_id != node as phys_addr_t {
            i += 1;
            continue;
        }

        mem_type = loongson_memmap.map[i as usize].mem_type;
        mem_size = loongson_memmap.map[i as usize].mem_size;

        /* Memory size comes in MB if MEM_SIZE_IS_IN_BYTES not set */
        if (mem_size & MEM_SIZE_IS_IN_BYTES) != 0 {
            mem_size &= !MEM_SIZE_IS_IN_BYTES;
        } else {
            mem_size = mem_size << 20;
        }

        mem_start = (node_id << 44) | loongson_memmap.map[i as usize].mem_start;

        match mem_type {
            SYSTEM_RAM_LOW | SYSTEM_RAM_HIGH | UMA_VIDEO_RAM => {
                pr_info("Node %d, mem_type:%d\t[%pa], %pa bytes usable\n",
                    node_id as u32, mem_type, &mem_start, &mem_size);
                memblock_add_node(mem_start, mem_size, node, MEMBLOCK_NONE);
            }
            SYSTEM_RAM_RESERVED | VIDEO_ROM | ADAPTER_ROM | ACPI_TABLE | SMBIOS_TABLE => {
                pr_info("Node %d, mem_type:%d\t[%pa], %pa bytes reserved\n",
                    node_id as u32, mem_type, &mem_start, &mem_size);
                memblock_reserve(mem_start, mem_size);
            }
            /* We should not reserve VUMA_VIDEO_RAM as it overlaps with MMIO */
            VUMA_VIDEO_RAM | _ => {
                pr_info("Node %d, mem_type:%d\t[%pa], %pa bytes unhandled\n",
                    node_id as u32, mem_type, &mem_start, &mem_size);
            }
        }
        i += 1;
    }

    /* Reserve vgabios if it comes from firmware */
    if loongson_sysconf.vgabios_addr != 0 {
        memblock_reserve(virt_to_phys(loongson_sysconf.vgabios_addr as *mut core::ffi::c_void), SZ_256K);
    }
    /* set nid for reserved memory */
    memblock_set_node((node as u64) << 44, ((node as u64) + 1) << 44,
        &mut memblock.reserved, node);
}

// C code is conditional on CONFIG_NUMA; this declaration is used when NUMA is disabled.
#[cfg(not(CONFIG_NUMA))]
unsafe fn prom_init_memory() {
    szmem(0);
}

pub unsafe fn prom_init() {
    fw_init_cmdline();

    if fw_arg2 == 0 || fdt_magic(fw_arg2) == FDT_MAGIC {
        loongson_sysconf.fw_interface = LOONGSON_DTB;
        prom_dtb_init_env();
    } else {
        loongson_sysconf.fw_interface = LOONGSON_LEFI;
        prom_lefi_init_env();
    }

    /* init base address of io space */
    set_io_port_base(PCI_IOBASE as usize);

    if let Some(early_config) = loongson_sysconf.early_config {
        early_config();
    }

    // CONFIG_NUMA selects prom_init_numa_memory; the non-NUMA path is local above.
    #[cfg(CONFIG_NUMA)]
    prom_init_numa_memory();
    #[cfg(not(CONFIG_NUMA))]
    prom_init_memory();

    /* Hardcode to CPU UART 0 */
    if (read_c0_prid() & PRID_IMP_MASK) == PRID_IMP_LOONGSON_64R {
        setup_8250_early_printk_port(TO_UNCAC(LOONGSON_REG_BASE), 0, 1024);
    } else {
        setup_8250_early_printk_port(TO_UNCAC(LOONGSON_REG_BASE + 0x1e0), 0, 1024);
    }

    register_smp_ops(&loongson3_smp_ops);
    board_nmi_handler_setup = Some(mips_nmi_setup);
}

unsafe fn add_legacy_isa_io(fwnode: *mut fwnode_handle, hw_start: resource_size_t,
                            mut size: resource_size_t) -> i32 {
    let mut ret: i32 = 0;
    let range = kzalloc_obj::<logic_pio_hwaddr>(GFP_ATOMIC);
    if range.is_null() {
        return -ENOMEM;
    }

    (*range).fwnode = fwnode;
    size = round_up(size, PAGE_SIZE);
    (*range).size = size;
    (*range).hw_start = hw_start;
    (*range).flags = LOGIC_PIO_CPU_MMIO;

    ret = logic_pio_register_range(range);
    if ret != 0 {
        kfree(range);
        return ret;
    }

    /* Legacy ISA must placed at the start of PCI_IOBASE */
    if (*range).io_start != 0 {
        logic_pio_unregister_range(range);
        kfree(range);
        return -EINVAL;
    }

    let vaddr = PCI_IOBASE as usize + (*range).io_start as usize;
    vmap_page_range(vaddr, vaddr + size as usize, hw_start, pgprot_device(PAGE_KERNEL));
    0
}

unsafe fn reserve_pio_range() {
    let mut np: *mut device_node;
    for_each_node_by_name!(np, "isa", {
        let mut range: of_range;
        let mut parser: of_range_parser;

        pr_info("ISA Bridge: %pOF\n", np);
        if of_range_parser_init(&mut parser, np) != 0 {
            pr_info("Failed to parse resources.\n");
            of_node_put(np);
            break;
        }

        for_each_of_range!(&mut parser, &mut range, {
            match range.flags & IORESOURCE_TYPE_BITS {
                IORESOURCE_IO => {
                    pr_info(" IO 0x%016llx..0x%016llx  ->  0x%016llx\n",
                        range.cpu_addr, range.cpu_addr + range.size - 1, range.bus_addr);
                    if add_legacy_isa_io(&mut (*np).fwnode, range.cpu_addr, range.size) != 0 {
                        pr_warn!("Failed to reserve legacy IO in Logic PIO\n");
                    }
                }
                IORESOURCE_MEM => {
                    pr_info(" MEM 0x%016llx..0x%016llx  ->  0x%016llx\n",
                        range.cpu_addr, range.cpu_addr + range.size - 1, range.bus_addr);
                }
                _ => {}
            }
        });
    });
}

pub unsafe fn arch_init_irq() {
    reserve_pio_range();
    irqchip_init();
}

pub fn arch_dynirq_lower_bound(from: u32) -> u32 {
    core::cmp::max(from, NR_IRQS_LEGACY)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
