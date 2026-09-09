// SPDX-License-Identifier: GPL-2.0
/*
 * linux/arch/alpha/kernel/sys_mikasa.c
 *
 * Code supporting the MIKASA (AlphaServer 1000).
 */

// Linux and Alpha architecture headers supply the declarations referenced below.

/* Note mask bit is true for ENABLED irqs. */
static mut CACHED_IRQ_MASK: i32 = 0;

#[inline]
unsafe fn mikasa_update_irq_hw(mask: i32) {
    outw(mask as u16, 0x536);
}

#[inline]
unsafe fn mikasa_enable_irq(d: *mut irq_data) {
    CACHED_IRQ_MASK |= 1i32 << ((*d).irq - 16);
    mikasa_update_irq_hw(CACHED_IRQ_MASK);
}

unsafe fn mikasa_disable_irq(d: *mut irq_data) {
    CACHED_IRQ_MASK &= !(1i32 << ((*d).irq - 16));
    mikasa_update_irq_hw(CACHED_IRQ_MASK);
}

static mut MIKASA_IRQ_TYPE: irq_chip = irq_chip {
    name: b"MIKASA\0".as_ptr() as *const i8,
    irq_unmask: Some(mikasa_enable_irq),
    irq_mask: Some(mikasa_disable_irq),
    irq_mask_ack: Some(mikasa_disable_irq),
};

unsafe fn mikasa_device_interrupt(vector: u64) {
    let mut pld: u64;
    let mut i: u32;

    /* Read the interrupt summary registers */
    pld = (((!inw(0x534) as u64 & 0x0000ffffu64) << 16)
        | ((inb(0xa0) as u64) << 8)
        | inb(0x20) as u64);

    /*
     * Now for every possible bit set, work through them and call
     * the appropriate interrupt handler.
     */
    while pld != 0 {
        i = (!pld).trailing_zeros();
        pld &= pld.wrapping_sub(1); /* clear least bit set */
        if i < 16 {
            isa_device_interrupt(vector);
        } else {
            handle_irq(i);
        }
    }
}

unsafe fn mikasa_init_irq() {
    let mut i: i64;

    if alpha_using_srm {
        alpha_mv.device_interrupt = Some(srm_device_interrupt);
    }

    mikasa_update_irq_hw(0);

    i = 16;
    while i < 32 {
        irq_set_chip_and_handler(i as u32, &raw mut MIKASA_IRQ_TYPE, handle_level_irq);
        irq_set_status_flags(i as u32, IRQ_LEVEL);
        i += 1;
    }

    init_i8259a_irqs();
    common_init_isa_dma();
}

unsafe fn mikasa_map_irq(dev: *const pci_dev, slot: u8, pin: u8) -> i32 {
    let irq_tab: [[i8; 5]; 8] = [
        [16 + 12, 16 + 12, 16 + 12, 16 + 12, 16 + 12],
        [-1, -1, -1, -1, -1],
        [-1, -1, -1, -1, -1],
        [-1, -1, -1, -1, -1],
        [-1, -1, -1, -1, -1],
        [16 + 0, 16 + 0, 16 + 1, 16 + 2, 16 + 3],
        [16 + 4, 16 + 4, 16 + 5, 16 + 6, 16 + 7],
        [16 + 8, 16 + 8, 16 + 9, 16 + 10, 16 + 11],
    ];
    let min_idsel: i64 = 6;
    let max_idsel: i64 = 13;
    let irqs_per_slot: i64 = 5;

    // COMMON_TABLE_LOOKUP: supplied by the Alpha PCI support code.
    common_table_lookup(dev, slot, pin, &irq_tab, min_idsel, max_idsel, irqs_per_slot)
}

static mut MIKASA_PRIMO_MV: alpha_machine_vector = alpha_machine_vector {
    vector_name: b"Mikasa-Primo\0".as_ptr() as *const i8,
    // DO_EV5_MMU, DO_DEFAULT_RTC, and DO_CIA_IO
    machine_check: Some(cia_machine_check),
    max_isa_dma_address: ALPHA_MAX_ISA_DMA_ADDRESS,
    min_io_address: DEFAULT_IO_BASE,
    min_mem_address: CIA_DEFAULT_MEM_BASE,
    nr_irqs: 32,
    device_interrupt: Some(mikasa_device_interrupt),
    init_arch: Some(cia_init_arch),
    init_irq: Some(mikasa_init_irq),
    init_rtc: Some(common_init_rtc),
    init_pci: Some(cia_init_pci),
    kill_arch: Some(cia_kill_arch),
    pci_map_irq: Some(mikasa_map_irq),
    pci_swizzle: Some(common_swizzle),
};

ALIAS_MV!(mikasa_primo);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
