// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 */

// Kernel and architecture dependencies are supplied by the surrounding tree.

pub static mut irq_stack: usize = 0;
pub static mut irq_stat: irq_cpustat_t = unsafe { core::mem::zeroed() };

pub static mut pch_group: [acpi_vector_group; MAX_IO_PICS] =
    unsafe { core::mem::zeroed() };
pub static mut msi_group: [acpi_vector_group; MAX_IO_PICS] =
    unsafe { core::mem::zeroed() };

/*
 * 'what should we do if we get a hw irq event on an illegal vector'.
 * each architecture has to answer this themselves.
 */
pub unsafe fn ack_bad_irq(irq: u32) {
    pr_warn!("Unexpected IRQ # {}\n", irq);
}

pub static mut irq_err_count: atomic_t = atomic_t { counter: 0 };

pub unsafe extern "C" fn spurious_interrupt() {
    atomic_inc(&raw mut irq_err_count);
}

pub unsafe fn arch_show_interrupts(p: *mut seq_file, prec: i32) -> i32 {
    // CONFIG_SMP: show_ipi_list(p, prec);
    seq_printf!(p, "%*s: %10u\n", prec, "ERR", atomic_read(&raw mut irq_err_count));
    0
}

unsafe fn early_pci_mcfg_parse(header: *mut acpi_table_header) -> i32 {
    let mcfg: *mut acpi_table_mcfg;
    let mut mptr: *mut acpi_mcfg_allocation;
    let mut i: i32;
    let n: i32;

    if (*header).length < core::mem::size_of::<acpi_table_mcfg>() as u32 {
        return -EINVAL;
    }

    n = ((*header).length - core::mem::size_of::<acpi_table_mcfg>() as u32)
        as usize
        .wrapping_div(core::mem::size_of::<acpi_mcfg_allocation>()) as i32;
    mcfg = header.cast();
    mptr = (&mut (*mcfg).allocations as *mut _).cast();

    i = 0;
    while i < n {
        (*msi_group.as_mut_ptr().add(i as usize)).pci_segment = (*mptr).pci_segment;
        (*pch_group.as_mut_ptr().add(i as usize)).node =
            ((*msi_group.as_mut_ptr().add(i as usize)).node = ((*mptr).address >> 44) & 0xf);
        mptr = mptr.add(1);
        i += 1;
    }

    0
}

unsafe fn init_vec_parent_group() {
    let mut i = 0;
    while i < MAX_IO_PICS {
        (*msi_group.as_mut_ptr().add(i)).pci_segment = -1;
        (*msi_group.as_mut_ptr().add(i)).node = -1;
        (*pch_group.as_mut_ptr().add(i)).node = -1;
        i += 1;
    }

    acpi_table_parse(ACPI_SIG_MCFG, Some(early_pci_mcfg_parse));
}

pub unsafe fn arch_probe_nr_irqs() -> i32 {
    let nr_io_pics = bitmap_weight(loongson_sysconf.cores_io_master, NR_CPUS);

    if !cpu_has_avecint {
        irq_set_nr_irqs(64 + NR_VECTORS * nr_io_pics);
    } else {
        irq_set_nr_irqs(64 + NR_VECTORS * (nr_cpu_ids + nr_io_pics));
    }

    NR_IRQS_LEGACY
}

pub unsafe fn arch_dynirq_lower_bound(from: u32) -> u32 {
    MAX(from, NR_IRQS_LEGACY)
}

pub unsafe fn init_IRQ() {
    let order = get_order(IRQ_STACK_SIZE);
    let mut page: *mut page;

    clear_csr_ecfg(ECFG0_IM);
    clear_csr_estat(ESTATF_IP);

    init_vec_parent_group();
    irqchip_init();
    // CONFIG_SMP: mp_ops.init_ipi();

    for i in for_each_possible_cpu() {
        page = alloc_pages_node(cpu_to_node(i), GFP_KERNEL, order);

        per_cpu!(irq_stack, i) = page_address(page) as usize;
        pr_debug!(
            "CPU{} IRQ stack at 0x{:x} - 0x{:x}\n",
            i,
            per_cpu!(irq_stack, i),
            per_cpu!(irq_stack, i) + IRQ_STACK_SIZE
        );
    }

    set_csr_ecfg(ECFGF_SIP0 | ECFGF_IP0 | ECFGF_IP1 | ECFGF_IP2 | ECFGF_IPI | ECFGF_PMC);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
