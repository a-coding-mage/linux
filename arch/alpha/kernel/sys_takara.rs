// SPDX-License-Identifier: GPL-2.0
/*
 * linux/arch/alpha/kernel/sys_takara.c
 *
 * Code supporting the TAKARA.
 */

// Dependencies supplied by the surrounding kernel translation.

static mut CACHED_IRQ_MASK: [u64; 2] = [u64::MAX, u64::MAX];

unsafe fn takara_update_irq_hw(irq: u64, mut mask: u64) {
    let regaddr: i32;

    mask = if irq >= 64 {
        mask.wrapping_shl(16)
    } else {
        mask.wrapping_shr(((irq.wrapping_sub(16)) & 0x30) as u32)
    };
    regaddr = 0x510 + (((irq.wrapping_sub(16)) >> 2) & 0x0c) as i32;
    outl((mask & 0xffff0000u64) as u32, regaddr);
}

unsafe fn takara_enable_irq(d: *mut irq_data) {
    let irq = (*d).irq as u64;
    let mask = {
        let index = (irq >= 64) as usize;
        CACHED_IRQ_MASK[index] &= !(1u64 << (irq & 63));
        CACHED_IRQ_MASK[index]
    };
    takara_update_irq_hw(irq, mask);
}

unsafe fn takara_disable_irq(d: *mut irq_data) {
    let irq = (*d).irq as u64;
    let mask = {
        let index = (irq >= 64) as usize;
        CACHED_IRQ_MASK[index] |= 1u64 << (irq & 63);
        CACHED_IRQ_MASK[index]
    };
    takara_update_irq_hw(irq, mask);
}

static mut TAKARA_IRQ_TYPE: irq_chip = irq_chip {
    name: "TAKARA" as *const str,
    irq_unmask: Some(takara_enable_irq),
    irq_mask: Some(takara_disable_irq),
    irq_mask_ack: Some(takara_disable_irq),
};

unsafe fn takara_device_interrupt(vector: u64) {
    let intstatus = (inw(0x500) & 15) as u32;
    if intstatus != 0 {
        if intstatus & 8 != 0 { handle_irq(16 + 3); }
        if intstatus & 4 != 0 { handle_irq(16 + 2); }
        if intstatus & 2 != 0 { handle_irq(16 + 1); }
        if intstatus & 1 != 0 { handle_irq(16 + 0); }
    } else {
        isa_device_interrupt(vector);
    }
}

unsafe fn takara_srm_device_interrupt(vector: u64) {
    let irq = ((vector.wrapping_sub(0x800)) >> 4) as i32;
    handle_irq(irq);
}

unsafe fn takara_init_irq() {
    init_i8259a_irqs();

    if alpha_using_srm {
        alpha_mv.device_interrupt = Some(takara_srm_device_interrupt);
    } else {
        let mut ctlreg = inl(0x500);
        ctlreg &= !0x8000;
        outl(ctlreg, 0x500);
        ctlreg = 0x05107c00;
        outl(ctlreg, 0x500);
    }

    let mut i = 16;
    while i < 128 {
        takara_update_irq_hw(i, u64::MAX);
        i += 16;
    }

    i = 16;
    while i < 128 {
        irq_set_chip_and_handler(i as u32, &mut TAKARA_IRQ_TYPE, handle_level_irq);
        irq_set_status_flags(i as u32, IRQ_LEVEL);
        i += 1;
    }

    common_init_isa_dma();
}

unsafe fn takara_map_irq_srm(dev: *const pci_dev, slot: u8, pin: u8) -> i32 {
    let irq_tab: [[i8; 5]; 15] = [
        [19, 19, 19, 19, 19], [18, 18, 18, 18, 18], [17, 17, 17, 17, 17],
        [-1, -1, -1, -1, -1], [-1, -1, -1, -1, -1], [-1, -1, -1, -1, -1],
        [12, 12, 13, 14, 15], [8, 8, 9, 19, 11], [4, 4, 5, 6, 7],
        [0, 0, 1, 2, 3], [-1, -1, -1, -1, -1], [64, 64, 65, 66, 67],
        [48, 48, 49, 50, 51], [32, 32, 33, 34, 35], [16, 16, 17, 18, 19],
    ];
    let min_idsel = 6i32;
    let max_idsel = 20i32;
    let irqs_per_slot = 5i32;
    let mut irq = common_table_lookup(dev, slot, pin, &irq_tab, min_idsel, max_idsel, irqs_per_slot);
    if irq >= 0 && irq < 16 {
        let busslot = pci_slot((*(*dev).bus).self_.unwrap().devfn);
        irq += irq_tab[(busslot as i32 - min_idsel) as usize][0] as i32;
    }
    irq
}

unsafe fn takara_map_irq(dev: *const pci_dev, slot: u8, pin: u8) -> i32 {
    let irq_tab: [[i8; 5]; 15] = [
        [19, 19, 19, 19, 19], [18, 18, 18, 18, 18], [17, 17, 17, 17, 17],
        [-1, -1, -1, -1, -1], [-1, -1, -1, -1, -1], [-1, -1, -1, -1, -1],
        [-1, -1, -1, -1, -1], [-1, -1, -1, -1, -1], [-1, -1, -1, -1, -1],
        [-1, -1, -1, -1, -1], [-1, -1, -1, -1, -1], [-1, -1, -1, -1, -1],
        [19, 19, 19, 19, 19], [18, 18, 18, 18, 18], [17, 17, 17, 17, 17],
    ];
    common_table_lookup(dev, slot, pin, &irq_tab, 6, 20, 5)
}

unsafe fn takara_swizzle(dev: *mut pci_dev, pinp: *mut u8) -> u8 {
    let slot = pci_slot((*dev).devfn);
    let mut pin = *pinp as i32;
    let ctlreg = inl(0x500);
    let bus = (*dev).bus;
    if (*bus).self_.is_none() { return slot; }
    let busslot = pci_slot((*(*bus).self_.as_ref().unwrap()).devfn);
    if (*bus).number != 0 && busslot > 16 && ((1u32 << (36 - busslot)) & ctlreg) != 0 {
        if pin == 1 { pin += 20 - busslot as i32; }
        else { printk(KERN_WARNING, "takara_swizzle: can only handle cards with INTA IRQ pin.\n"); }
    } else {
        printk(KERN_WARNING, "takara_swizzle: cannot handle card-bridge behind builtin bridge yet.\n");
    }
    *pinp = pin as u8;
    slot
}

unsafe fn takara_init_pci() {
    if alpha_using_srm { alpha_mv.pci_map_irq = Some(takara_map_irq_srm); }
    cia_init_pci();
    if pc873xx_probe() == -1 {
        printk(KERN_ERR, "Probing for PC873xx Super IO chip failed.\n");
    } else {
        printk(KERN_INFO, "Found %s Super IO chip at 0x%x\n", pc873xx_get_model(), pc873xx_get_base());
        pc873xx_enable_ide();
    }
}

static mut TAKARA_MV: alpha_machine_vector = alpha_machine_vector {
    vector_name: "Takara",
    machine_check: Some(cia_machine_check),
    max_isa_dma_address: ALPHA_MAX_ISA_DMA_ADDRESS,
    min_io_address: DEFAULT_IO_BASE,
    min_mem_address: CIA_DEFAULT_MEM_BASE,
    nr_irqs: 128,
    device_interrupt: Some(takara_device_interrupt),
    init_arch: Some(cia_init_arch),
    init_irq: Some(takara_init_irq),
    init_rtc: Some(common_init_rtc),
    init_pci: Some(takara_init_pci),
    kill_arch: Some(cia_kill_arch),
    pci_map_irq: Some(takara_map_irq),
    pci_swizzle: Some(takara_swizzle),
};

ALIAS_MV!(takara);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
