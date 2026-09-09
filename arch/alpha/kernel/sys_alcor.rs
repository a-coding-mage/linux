// SPDX-License-Identifier: GPL-2.0
/*
 * linux/arch/alpha/kernel/sys_alcor.c
 *
 * Copyright (C) 1995 David A Rusling
 * Copyright (C) 1996 Jay A Estabrook
 * Copyright (C) 1998, 1999 Richard Henderson
 *
 * Code supporting the ALCOR and XLT (XL-300/366/433).
 */

// Linux and architecture headers from the C source provide the external
// types, constants, functions, and register symbols referenced below.

/* Note mask bit is true for ENABLED irqs.  */
static mut cached_irq_mask: c_ulong = 0;

#[inline]
unsafe fn alcor_update_irq_hw(mask: c_ulong) {
    core::ptr::write_volatile(GRU_INT_MASK as *mut c_ulong, mask);
    mb();
}

#[inline]
unsafe fn alcor_enable_irq(d: *mut irq_data) {
    cached_irq_mask |= 1 as c_ulong << ((*d).irq - 16);
    alcor_update_irq_hw(cached_irq_mask);
}

unsafe fn alcor_disable_irq(d: *mut irq_data) {
    cached_irq_mask &= !(1 as c_ulong << ((*d).irq - 16));
    alcor_update_irq_hw(cached_irq_mask);
}

unsafe fn alcor_mask_and_ack_irq(d: *mut irq_data) {
    alcor_disable_irq(d);

    /* On ALCOR/XLT, need to dismiss interrupt via GRU. */
    core::ptr::write_volatile(GRU_INT_CLEAR as *mut c_ulong, 1 as c_ulong << ((*d).irq - 16));
    mb();
    core::ptr::write_volatile(GRU_INT_CLEAR as *mut c_ulong, 0);
    mb();
}

unsafe fn alcor_isa_mask_and_ack_irq(d: *mut irq_data) {
    i8259a_mask_and_ack_irq(d);

    /* On ALCOR/XLT, need to dismiss interrupt via GRU. */
    core::ptr::write_volatile(GRU_INT_CLEAR as *mut c_ulong, 0x80000000);
    mb();
    core::ptr::write_volatile(GRU_INT_CLEAR as *mut c_ulong, 0);
    mb();
}

static mut alcor_irq_type: irq_chip = irq_chip {
    name: b"ALCOR\0".as_ptr() as *const c_char,
    irq_unmask: Some(alcor_enable_irq),
    irq_mask: Some(alcor_disable_irq),
    irq_mask_ack: Some(alcor_mask_and_ack_irq),
};

unsafe fn alcor_device_interrupt(vector: c_ulong) {
    let mut pld: c_ulong = core::ptr::read_volatile(GRU_INT_REQ as *const c_ulong) & GRU_INT_REQ_BITS;
    let mut i: c_uint;

    /* Read the interrupt summary register of the GRU */
    /*
     * Now for every possible bit set, work through them and call
     * the appropriate interrupt handler.
     */
    while pld != 0 {
        i = ffz(!pld);
        pld &= pld.wrapping_sub(1); /* clear least bit set */
        if i == 31 {
            isa_device_interrupt(vector);
        } else {
            handle_irq(16 + i);
        }
    }
}

unsafe fn alcor_init_irq() {
    let mut i: c_long;

    if alpha_using_srm {
        alpha_mv.device_interrupt = Some(srm_device_interrupt);
    }

    core::ptr::write_volatile(GRU_INT_MASK as *mut c_ulong, 0); mb(); /* all disabled */
    core::ptr::write_volatile(GRU_INT_EDGE as *mut c_ulong, 0); mb(); /* all are level */
    core::ptr::write_volatile(GRU_INT_HILO as *mut c_ulong, 0x80000000u32 as c_ulong); mb(); /* ISA only HI */
    core::ptr::write_volatile(GRU_INT_CLEAR as *mut c_ulong, 0); mb(); /* all clear */

    i = 16;
    while i < 48 {
        /* On Alcor, at least, lines 20..30 are not connected
           and can generate spurious interrupts if we turn them
           on while IRQ probing.  */
        if i >= 16 + 20 && i <= 16 + 30 {
            i += 1;
            continue;
        }
        irq_set_chip_and_handler(i, &raw mut alcor_irq_type, handle_level_irq);
        irq_set_status_flags(i, IRQ_LEVEL);
        i += 1;
    }
    i8259a_irq_type.irq_ack = Some(alcor_isa_mask_and_ack_irq);

    init_i8259a_irqs();
    common_init_isa_dma();

    if request_irq(16 + 31, no_action, 0, b"isa-cascade\0".as_ptr() as *const c_char, core::ptr::null_mut()) != 0 {
        pr_err(b"Failed to register isa-cascade interrupt\n\0".as_ptr() as *const c_char);
    }
}

/* PCI Fixup configuration. */
unsafe fn alcor_map_irq(dev: *const pci_dev, slot: u8, pin: u8) -> c_int {
    static mut irq_tab: [[c_char; 5]; 7] = [
        [16+13, 16+13, 16+13, 16+13, 16+13],
        [16+8, 16+8, 16+9, 16+10, 16+11],
        [16+16, 16+16, 16+17, 16+18, 16+19],
        [16+12, 16+12, 16+13, 16+14, 16+15],
        [-1, -1, -1, -1, -1],
        [16+0, 16+0, 16+1, 16+2, 16+3],
        [16+4, 16+4, 16+5, 16+6, 16+7],
    ];
    let min_idsel = 6;
    let max_idsel = 12;
    let irqs_per_slot = 5;
    COMMON_TABLE_LOOKUP
}

unsafe fn alcor_kill_arch(mode: c_int) {
    cia_kill_arch(mode);

    // The C build condition is preserved here; define ALPHA_RESTORE_SRM_SETUP
    // in the surrounding build when that configuration is selected.
    #[cfg(not(ALPHA_RESTORE_SRM_SETUP))]
    {
        match mode {
            LINUX_REBOOT_CMD_RESTART => {
                /* Who said DEC engineer's have no sense of humor? ;-)  */
                if alpha_using_srm {
                    core::ptr::write_volatile(GRU_RESET as *mut c_ulong, 0x0000dead);
                    mb();
                }
            }
            LINUX_REBOOT_CMD_HALT => {}
            LINUX_REBOOT_CMD_POWER_OFF => {}
            _ => {}
        }
        halt();
    }
}

unsafe fn alcor_init_pci() {
    let dev = pci_get_device(PCI_VENDOR_ID_DEC, PCI_DEVICE_ID_DEC_TULIP, core::ptr::null_mut());
    if !dev.is_null() && (*dev).devfn == PCI_DEVFN(6, 0) {
        alpha_mv.sys.cia.gru_int_req_bits = XLT_GRU_INT_REQ_BITS;
        printk(KERN_INFO, b"%s: Detected AS500 or XLT motherboard.\n\0".as_ptr() as *const c_char, b"alcor_init_pci\0".as_ptr() as *const c_char);
    }
    pci_dev_put(dev);
}

/* The System Vectors */

static mut alcor_mv: alpha_machine_vector = alpha_machine_vector {
    vector_name: b"Alcor\0".as_ptr() as *const c_char,
    // DO_EV5_MMU, DO_DEFAULT_RTC, and DO_CIA_IO expand to their C fields.
    machine_check: Some(cia_machine_check),
    max_isa_dma_address: ALPHA_ALCOR_MAX_ISA_DMA_ADDRESS,
    min_io_address: EISA_DEFAULT_IO_BASE,
    min_mem_address: CIA_DEFAULT_MEM_BASE,
    nr_irqs: 48,
    device_interrupt: Some(alcor_device_interrupt),
    init_arch: Some(cia_init_arch),
    init_irq: Some(alcor_init_irq),
    init_rtc: Some(common_init_rtc),
    init_pci: Some(alcor_init_pci),
    kill_arch: Some(alcor_kill_arch),
    pci_map_irq: Some(alcor_map_irq),
    pci_swizzle: Some(common_swizzle),
    sys: alpha_machine_sys { cia: cia_machine_sys { gru_int_req_bits: ALCOR_GRU_INT_REQ_BITS } },
};

ALIAS_MV!(alcor);

static mut xlt_mv: alpha_machine_vector = alpha_machine_vector {
    vector_name: b"XLT\0".as_ptr() as *const c_char,
    machine_check: Some(cia_machine_check),
    max_isa_dma_address: ALPHA_MAX_ISA_DMA_ADDRESS,
    min_io_address: EISA_DEFAULT_IO_BASE,
    min_mem_address: CIA_DEFAULT_MEM_BASE,
    nr_irqs: 48,
    device_interrupt: Some(alcor_device_interrupt),
    init_arch: Some(cia_init_arch),
    init_irq: Some(alcor_init_irq),
    init_rtc: Some(common_init_rtc),
    init_pci: Some(alcor_init_pci),
    kill_arch: Some(alcor_kill_arch),
    pci_map_irq: Some(alcor_map_irq),
    pci_swizzle: Some(common_swizzle),
    sys: alpha_machine_sys { cia: cia_machine_sys { gru_int_req_bits: XLT_GRU_INT_REQ_BITS } },
};

/* No alpha_mv alias for XLT, since we compile it in unconditionally
   with ALCOR; setup_arch knows how to cope.  */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
