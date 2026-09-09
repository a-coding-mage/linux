// SPDX-License-Identifier: GPL-2.0
/*
 * linux/arch/alpha/kernel/sys_sx164.c
 *
 * Copyright (C) 1995 David A Rusling
 * Copyright (C) 1996 Jay A Estabrook
 * Copyright (C) 1998, 1999, 2000 Richard Henderson
 *
 * Code supporting the SX164 (PCA56+PYXIS).
 */

// C header dependencies are supplied by the surrounding Alpha kernel.

unsafe fn sx164_init_irq() {
    outb(0, DMA1_RESET_REG);
    outb(0, DMA2_RESET_REG);
    outb(DMA_MODE_CASCADE, DMA2_MODE_REG);
    outb(0, DMA2_MASK_REG);

    if alpha_using_srm {
        alpha_mv.device_interrupt = Some(srm_device_interrupt);
    }

    init_i8259a_irqs();

    /* Not interested in the bogus interrupts (0,3,4,5,40-47),
       NMI (1), or HALT (2). */
    if alpha_using_srm {
        init_srm_irqs(40, 0x3f0000);
    } else {
        init_pyxis_irqs(0xff00003f0000u64);
    }

    if request_irq(16 + 6, no_action, 0, "timer-cascade", core::ptr::null_mut()) != 0 {
        pr_err!("Failed to register timer-cascade interrupt\n");
    }
}

/*
 * PCI Fixup configuration.
 *
 * Summary @ PYXIS_INT_REQ:
 * Bit      Meaning
 * 0        RSVD
 * 1        NMI
 * 2        Halt/Reset switch
 * 3        MBZ
 * 4        RAZ
 * 5        RAZ
 * 6        Interval timer (RTC)
 * 7        PCI-ISA Bridge
 * 8        Interrupt Line A from slot 3
 * 9        Interrupt Line A from slot 2
 *10        Interrupt Line A from slot 1
 *11        Interrupt Line A from slot 0
 *12        Interrupt Line B from slot 3
 *13        Interrupt Line B from slot 2
 *14        Interrupt Line B from slot 1
 *15        Interrupt line B from slot 0
 *16        Interrupt Line C from slot 3
 *17        Interrupt Line C from slot 2
 *18        Interrupt Line C from slot 1
 *19        Interrupt Line C from slot 0
 *20        Interrupt Line D from slot 3
 *21        Interrupt Line D from slot 2
 *22        Interrupt Line D from slot 1
 *23        Interrupt Line D from slot 0
 *
 * IdSel
 *   5  32 bit PCI option slot 2
 *   6  64 bit PCI option slot 0
 *   7  64 bit PCI option slot 1
 *   8  Cypress I/O
 *   9  32 bit PCI option slot 3
 */
unsafe fn sx164_map_irq(dev: *const pci_dev, slot: u8, pin: u8) -> i32 {
    let irq_tab: [[i8; 5]; 5] = [
        [16 + 9, 16 + 9, 16 + 13, 16 + 17, 16 + 21],
        [16 + 11, 16 + 11, 16 + 15, 16 + 19, 16 + 23],
        [16 + 10, 16 + 10, 16 + 14, 16 + 18, 16 + 22],
        [-1, -1, -1, -1, -1],
        [16 + 8, 16 + 8, 16 + 12, 16 + 16, 16 + 20],
    ];
    let min_idsel: i64 = 5;
    let max_idsel: i64 = 9;
    let irqs_per_slot: i64 = 5;
    common_table_lookup(dev, slot, pin, &irq_tab, min_idsel, max_idsel, irqs_per_slot)
}

unsafe fn sx164_init_pci() {
    cia_init_pci();
    SMC669_Init(0);
}

unsafe fn sx164_init_arch() {
    /*
     * OSF palcode v1.23 forgets to enable PCA56 Motion Video
     * Instructions. Let's enable it.
     * We have to check palcode revision because CSERVE interface
     * is subject to change without notice. For example, it
     * has been changed completely since v1.16 (found in MILO
     * distribution). -ink
     */
    let cpu = ((hwrpb as *mut u8).offset((*hwrpb).processor_offset as isize))
        as *mut percpu_struct;

    if amask(AMASK_MAX) != 0
        && alpha_using_srm
        && ((*cpu).pal_revision & 0xffff) <= 0x117
    {
        // Original Alpha PALRES/hw_mfpr/hw_mtpr sequence:
        // enable PALRES, set the MVE bit in ICSR, then disable PALRES.
        printk("PCA56 MVI set enabled\n");
    }

    pyxis_init_arch();
}

/* The System Vector */
static mut sx164_mv: alpha_machine_vector = alpha_machine_vector {
    vector_name: "SX164",
    DO_EV5_MMU,
    DO_DEFAULT_RTC,
    DO_PYXIS_IO,
    machine_check: Some(cia_machine_check),
    max_isa_dma_address: ALPHA_MAX_ISA_DMA_ADDRESS,
    min_io_address: DEFAULT_IO_BASE,
    min_mem_address: DEFAULT_MEM_BASE,
    pci_dac_offset: PYXIS_DAC_OFFSET,
    nr_irqs: 48,
    device_interrupt: Some(pyxis_device_interrupt),
    init_arch: Some(sx164_init_arch),
    init_irq: Some(sx164_init_irq),
    init_rtc: Some(common_init_rtc),
    init_pci: Some(sx164_init_pci),
    kill_arch: Some(cia_kill_arch),
    pci_map_irq: Some(sx164_map_irq),
    pci_swizzle: Some(common_swizzle),
};

ALIAS_MV!(sx164);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
