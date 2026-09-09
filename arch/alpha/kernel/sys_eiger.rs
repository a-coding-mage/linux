// SPDX-License-Identifier: GPL-2.0
/*
 * linux/arch/alpha/kernel/sys_eiger.c
 *
 * Code supporting the EIGER (EV6+TSUNAMI).
 */

// C includes and build-time machine-vector macros are supplied by the kernel
// environment; their symbols are referenced below.

/* Note that this interrupt code is identical to TAKARA. */

/* Note mask bit is true for DISABLED irqs. */
static mut CACHED_IRQ_MASK: [c_ulong; 2] = [!0, !0];

#[inline]
unsafe fn eiger_update_irq_hw(irq: c_ulong, mut mask: c_ulong) {
    let regaddr: c_int;

    mask = if irq >= 64 {
        mask << 16
    } else {
        mask >> ((irq.wrapping_sub(16)) & 0x30)
    };
    regaddr = 0x510 + (((irq.wrapping_sub(16)) >> 2) & 0x0c) as c_int;
    outl(mask & 0xffff0000, regaddr);
}

#[inline]
unsafe fn eiger_enable_irq(d: *mut irq_data) {
    let irq: c_uint = (*d).irq;
    let index = (irq >= 64) as usize;
    let mask = {
        CACHED_IRQ_MASK[index] &= !(1 as c_ulong << (irq & 63));
        CACHED_IRQ_MASK[index]
    };
    eiger_update_irq_hw(irq as c_ulong, mask);
}

unsafe fn eiger_disable_irq(d: *mut irq_data) {
    let irq: c_uint = (*d).irq;
    let index = (irq >= 64) as usize;
    let mask = {
        CACHED_IRQ_MASK[index] |= 1 as c_ulong << (irq & 63);
        CACHED_IRQ_MASK[index]
    };
    eiger_update_irq_hw(irq as c_ulong, mask);
}

static mut EIGER_IRQ_TYPE: irq_chip = irq_chip {
    name: b"EIGER\0".as_ptr() as *const c_char,
    irq_unmask: Some(eiger_enable_irq),
    irq_mask: Some(eiger_disable_irq),
    irq_mask_ack: Some(eiger_disable_irq),
};

unsafe fn eiger_device_interrupt(vector: c_ulong) {
    let intstatus: c_uint = (inw(0x500) & 15) as c_uint;

    if intstatus != 0 {
        if intstatus & 8 != 0 { handle_irq(16 + 3); }
        if intstatus & 4 != 0 { handle_irq(16 + 2); }
        if intstatus & 2 != 0 { handle_irq(16 + 1); }
        if intstatus & 1 != 0 { handle_irq(16 + 0); }
    } else {
        isa_device_interrupt(vector);
    }
}

unsafe fn eiger_srm_device_interrupt(vector: c_ulong) {
    let irq: c_int = ((vector.wrapping_sub(0x800)) >> 4) as c_int;
    handle_irq(irq);
}

unsafe fn eiger_init_irq() {
    outb(0, DMA1_RESET_REG);
    outb(0, DMA2_RESET_REG);
    outb(DMA_MODE_CASCADE, DMA2_MODE_REG);
    outb(0, DMA2_MASK_REG);

    if alpha_using_srm {
        alpha_mv.device_interrupt = Some(eiger_srm_device_interrupt);
    }

    let mut i: c_long = 16;
    while i < 128 {
        eiger_update_irq_hw(i as c_ulong, !0);
        i += 16;
    }

    init_i8259a_irqs();

    i = 16;
    while i < 128 {
        irq_set_chip_and_handler(i as c_uint, &raw mut EIGER_IRQ_TYPE, handle_level_irq);
        irq_set_status_flags(i as c_uint, IRQ_LEVEL);
        i += 1;
    }
}

unsafe fn eiger_map_irq(dev: *const pci_dev, _slot: u8, _pin: u8) -> c_int {
    let mut irq_orig: u8 = 0;
    pci_read_config_byte(dev, PCI_INTERRUPT_LINE, &mut irq_orig);
    irq_orig.wrapping_sub(0x80) as c_int
}

unsafe fn eiger_swizzle(mut dev: *mut pci_dev, pinp: *mut u8) -> u8 {
    let hose: *mut pci_controller = (*dev).sysdata as *mut pci_controller;
    let mut slot: c_int;
    let mut pin: c_int = *pinp as c_int;
    let mut bridge_count: c_int = 0;
    let backplane = inw(0x502) & 0x0f;

    match backplane {
        0x00 => bridge_count = 0,
        0x01 => bridge_count = 1,
        0x03 => bridge_count = 2,
        0x07 => bridge_count = 3,
        0x0f => bridge_count = 4,
        _ => {}
    }

    slot = PCI_SLOT((*dev).devfn) as c_int;
    while !(*(*dev).bus).self_.is_null() {
        if (*hose).index == 0
            && (PCI_SLOT((*(*dev).bus).self_.as_ref().unwrap().devfn) as c_int > 20 - bridge_count) {
            slot = PCI_SLOT((*dev).devfn) as c_int;
            break;
        }
        pin = pci_swizzle_interrupt_pin(dev, pin as u8) as c_int;
        dev = (*(*dev).bus).self_;
    }
    *pinp = pin as u8;
    slot as u8
}

/* The System Vectors. */
static mut EIGER_MV: alpha_machine_vector = alpha_machine_vector {
    vector_name: b"Eiger\0".as_ptr() as *const c_char,
    // DO_EV6_MMU, DO_DEFAULT_RTC, and DO_TSUNAMI_IO expand to machine-vector fields.
    machine_check: Some(tsunami_machine_check),
    max_isa_dma_address: ALPHA_MAX_ISA_DMA_ADDRESS,
    min_io_address: DEFAULT_IO_BASE,
    min_mem_address: DEFAULT_MEM_BASE,
    pci_dac_offset: TSUNAMI_DAC_OFFSET,
    nr_irqs: 128,
    device_interrupt: Some(eiger_device_interrupt),
    init_arch: Some(tsunami_init_arch),
    init_irq: Some(eiger_init_irq),
    init_rtc: Some(common_init_rtc),
    init_pci: Some(common_init_pci),
    kill_arch: Some(tsunami_kill_arch),
    pci_map_irq: Some(eiger_map_irq),
    pci_swizzle: Some(eiger_swizzle),
};

// ALIAS_MV(eiger)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
