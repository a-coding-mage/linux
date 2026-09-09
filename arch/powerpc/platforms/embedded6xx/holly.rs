// SPDX-License-Identifier: GPL-2.0-only
/*
 * Board setup routines for the IBM 750GX/CL platform w/ TSI10x bridge
 *
 * Copyright 2007 IBM Corporation
 *
 * Stephen Winiecki <stevewin@us.ibm.com>
 * Josh Boyer <jwboyer@linux.vnet.ibm.com>
 *
 * Based on code from mpc7448_hpc2.c
 */

// Dependencies supplied by the surrounding kernel translation unit.

const HOLLY_PCI_CFG_PHYS: u32 = 0x7c000000;

unsafe fn holly_exclude_device(_hose: *mut pci_controller, bus: u8, devfn: u8) -> i32 {
    if bus == 0 && PCI_SLOT(devfn) == 0 {
        PCIBIOS_DEVICE_NOT_FOUND
    } else {
        PCIBIOS_SUCCESSFUL
    }
}

unsafe fn holly_remap_bridge() {
    let mut lut_val: u32;
    let mut lut_addr: u32;
    let mut i: i32;

    printk(KERN_INFO, "Remapping PCI bridge\\n");

    /* Re-init the PCI bridge and LUT registers to have mappings that don't
     * rely on PIBS
     */
    lut_addr = 0x900;
    i = 0;
    while i < 31 {
        tsi108_write_reg(TSI108_PB_OFFSET + lut_addr, 0x00000201);
        lut_addr += 4;
        tsi108_write_reg(TSI108_PB_OFFSET + lut_addr, 0x0);
        lut_addr += 4;
        i += 1;
    }

    /* Reserve the last LUT entry for PCI I/O space */
    tsi108_write_reg(TSI108_PB_OFFSET + lut_addr, 0x00000241);
    lut_addr += 4;
    tsi108_write_reg(TSI108_PB_OFFSET + lut_addr, 0x0);

    /* Map PCI I/O space */
    tsi108_write_reg(TSI108_PCI_PFAB_IO_UPPER, 0x0);
    tsi108_write_reg(TSI108_PCI_PFAB_IO, 0x1);

    /* Map PCI CFG space */
    tsi108_write_reg(TSI108_PCI_PFAB_BAR0_UPPER, 0x0);
    tsi108_write_reg(TSI108_PCI_PFAB_BAR0, 0x7c000000 | 0x01);

    /* We don't need MEM32 and PRM remapping so disable them */
    tsi108_write_reg(TSI108_PCI_PFAB_MEM32, 0x0);
    tsi108_write_reg(TSI108_PCI_PFAB_PFM3, 0x0);
    tsi108_write_reg(TSI108_PCI_PFAB_PFM4, 0x0);

    /* Set P2O_BAR0 */
    tsi108_write_reg(TSI108_PCI_P2O_BAR0_UPPER, 0x0);
    tsi108_write_reg(TSI108_PCI_P2O_BAR0, 0xc0000000);

    /* Init the PCI LUTs to do no remapping */
    lut_addr = 0x500;
    lut_val = 0x00000002;

    i = 0;
    while i < 32 {
        tsi108_write_reg(TSI108_PCI_OFFSET + lut_addr, lut_val);
        lut_addr += 4;
        tsi108_write_reg(TSI108_PCI_OFFSET + lut_addr, 0x40000000);
        lut_addr += 4;
        lut_val += 0x02000000;
        i += 1;
    }
    tsi108_write_reg(TSI108_PCI_P2O_PAGE_SIZES, 0x00007900);

    /* Set 64-bit PCI bus address for system memory */
    tsi108_write_reg(TSI108_PCI_P2O_BAR2_UPPER, 0x0);
    tsi108_write_reg(TSI108_PCI_P2O_BAR2, 0x0);
}

unsafe fn holly_init_pci() {
    let mut np: *mut device_node;

    if (*ppc_md).progress.is_some() {
        ((*ppc_md).progress.unwrap())("holly_setup_arch():set_bridge", 0);
    }

    /* setup PCI host bridge */
    holly_remap_bridge();

    np = of_find_node_by_type(core::ptr::null_mut(), "pci");
    if !np.is_null() {
        tsi108_setup_pci(np, HOLLY_PCI_CFG_PHYS, 1);
    }

    of_node_put(np);

    (*ppc_md).pci_exclude_device = Some(holly_exclude_device);
    if (*ppc_md).progress.is_some() {
        ((*ppc_md).progress.unwrap())("tsi108: resources set", 0x100);
    }
}

unsafe fn holly_setup_arch() {
    tsi108_csr_vir_base = get_vir_csrbase();
    printk(KERN_INFO, "PPC750GX/CL Platform\\n");
}

/*
 * Interrupt setup and service.  Interrupts on the holly come
 * from the four external INT pins, PCI interrupts are routed via
 * PCI interrupt control registers, it generates internal IRQ23
 *
 * Interrupt routing on the Holly Board:
 * TSI108:PB_INT[0] -> CPU0:INT#
 * TSI108:PB_INT[1] -> CPU0:MCP#
 * TSI108:PB_INT[2] -> N/C
 * TSI108:PB_INT[3] -> N/C
 */
unsafe fn holly_init_IRQ() {
    let mpic: *mut mpic;

    mpic = mpic_alloc(core::ptr::null_mut(), 0,
        MPIC_BIG_ENDIAN | MPIC_SPV_EOI | MPIC_NO_PTHROU_DIS | MPIC_REGSET_TSI108,
        24, 0, "Tsi108_PIC");

    BUG_ON(mpic.is_null());
    mpic_assign_isu(mpic, 0, (*mpic).paddr + 0x100);
    mpic_init(mpic);

    // CONFIG_PCI guarded block from the C source.
    #[cfg(CONFIG_PCI)]
    {
        let tsi_pci = of_find_node_by_type(core::ptr::null_mut(), "pci");
        if tsi_pci.is_null() {
            printk(KERN_ERR, "%s: No tsi108 pci node found !\\n", __func__);
            return;
        }
        let cascade_node = of_find_node_by_type(core::ptr::null_mut(), "pic-router");
        if cascade_node.is_null() {
            printk(KERN_ERR, "%s: No tsi108 pci cascade node found !\\n", __func__);
            return;
        }
        let cascade_pci_irq = irq_of_parse_and_map(tsi_pci, 0);
        pr_debug!("%s: tsi108 cascade_pci_irq = 0x%x\\n", __func__, cascade_pci_irq as u32);
        tsi108_pci_int_init(cascade_node);
        irq_set_handler_data(cascade_pci_irq, mpic as *mut core::ffi::c_void);
        irq_set_chained_handler(cascade_pci_irq, Some(tsi108_irq_cascade));
        of_node_put(tsi_pci);
        of_node_put(cascade_node);
    }

    /* Configure MPIC outputs to CPU0 */
    tsi108_write_reg(TSI108_MPIC_OFFSET + 0x30c, 0);
}

unsafe fn holly_show_cpuinfo(m: *mut seq_file) {
    seq_printf(m, "vendor\\t\\t: IBM\\n");
    seq_printf(m, "machine\\t\\t: PPC750 GX/CL\\n");
}

unsafe fn holly_restart(_cmd: *mut i8) -> ! {
    let mut ocn_bar1: *mut u32 = core::ptr::null_mut();
    let mut bar: u64;
    let mut bridge: *mut device_node = core::ptr::null_mut();
    let mut res: resource;
    let mut addr: u64 = 0xc0000000;

    local_irq_disable();
    bridge = of_find_node_by_type(core::ptr::null_mut(), "tsi-bridge");
    if !bridge.is_null() {
        of_address_to_resource(bridge, 0, &mut res);
        addr = res.start;
        of_node_put(bridge);
    }
    addr += TSI108_PB_OFFSET as u64 + 0x414;
    ocn_bar1 = ioremap(addr, 0x4) as *mut u32;
    /* Turn on the BOOT bit so the addresses are correctly
     * routed to the HLP interface */
    bar = ioread32be(ocn_bar1) as u64;
    bar |= 2;
    iowrite32be(bar as u32, ocn_bar1);
    iosync();
    /* Set SRR0 to the reset vector and turn on MSR_IP */
    mtspr(SPRN_SRR0, 0xfff00100);
    mtspr(SPRN_SRR1, MSR_IP);
    /* Do an rfi to jump back to firmware.  Somewhat evil,
     * but it works
     */
    core::arch::asm!("rfi", options(nostack, preserves_flags));
    /* Spin until reset happens.  Shouldn't really get here */
    loop {}
}

unsafe fn ppc750_machine_check_exception(regs: *mut pt_regs) -> i32 {
    let entry = search_exception_tables((*regs).nip);
    if !entry.is_null() {
        tsi108_clear_pci_cfg_error();
        regs_set_recoverable(regs);
        regs_set_return_ip(regs, extable_fixup(entry));
        return 1;
    }
    0
}

// Equivalent machine descriptor for define_machine(holly).
static mut HOLLY_MACHINE: machine_desc = machine_desc {
    name: "PPC750 GX/CL TSI",
    compatible: "ibm,holly",
    setup_arch: Some(holly_setup_arch),
    discover_phbs: Some(holly_init_pci),
    init_IRQ: Some(holly_init_IRQ),
    show_cpuinfo: Some(holly_show_cpuinfo),
    get_irq: Some(mpic_get_irq),
    restart: Some(holly_restart),
    machine_check_exception: Some(ppc750_machine_check_exception),
    progress: Some(udbg_progress),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
