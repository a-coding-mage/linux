// SPDX-License-Identifier: GPL-2.0
/*
 * linux/arch/alpha/kernel/sys_rx164.c
 *
 * Copyright (C) 1995 David A Rusling
 * Copyright (C) 1996 Jay A Estabrook
 * Copyright (C) 1998, 1999 Richard Henderson
 *
 * Code supporting the RX164 (PCA56+POLARIS).
 */

// Dependencies are supplied by the surrounding kernel translation unit.

/* Note mask bit is true for ENABLED irqs. */
static mut CACHED_IRQ_MASK: ::core::ffi::c_ulong = 0;

#[inline]
unsafe fn rx164_update_irq_hw(mask: ::core::ffi::c_ulong) {
    let irq_mask = (POLARIS_DENSE_CONFIG_BASE + 0x74) as *mut ::core::ffi::c_uint;
    core::ptr::write_volatile(irq_mask, mask as ::core::ffi::c_uint);
    mb();
    let _ = core::ptr::read_volatile(irq_mask);
}

#[inline]
unsafe fn rx164_enable_irq(d: *mut irq_data) {
    CACHED_IRQ_MASK |= 1 as ::core::ffi::c_ulong
        << ((*d).irq.wrapping_sub(16) as ::core::ffi::c_ulong);
    rx164_update_irq_hw(CACHED_IRQ_MASK);
}

unsafe fn rx164_disable_irq(d: *mut irq_data) {
    CACHED_IRQ_MASK &= !(1 as ::core::ffi::c_ulong
        << ((*d).irq.wrapping_sub(16) as ::core::ffi::c_ulong));
    rx164_update_irq_hw(CACHED_IRQ_MASK);
}

static mut rx164_irq_type: irq_chip = irq_chip {
    name: "RX164" as *const u8,
    irq_unmask: Some(rx164_enable_irq),
    irq_mask: Some(rx164_disable_irq),
    irq_mask_ack: Some(rx164_disable_irq),
    ..unsafe { core::mem::zeroed() }
};

unsafe fn rx164_device_interrupt(vector: ::core::ffi::c_ulong) {
    let dirr = (POLARIS_DENSE_CONFIG_BASE + 0x84) as *const ::core::ffi::c_uint;
    let mut pld = core::ptr::read_volatile(dirr) as ::core::ffi::c_ulong;

    /*
     * Now for every possible bit set, work through them and call
     * the appropriate interrupt handler.
     */
    while pld != 0 {
        let i = ffz(!pld);
        pld &= pld.wrapping_sub(1); /* clear least bit set */
        if i == 20 {
            isa_no_iack_sc_device_interrupt(vector);
        } else {
            handle_irq(16 + i);
        }
    }
}

unsafe fn rx164_init_irq() {
    rx164_update_irq_hw(0);
    let mut i = 16;
    while i < 40 {
        irq_set_chip_and_handler(i, &mut rx164_irq_type, handle_level_irq);
        irq_set_status_flags(i, IRQ_LEVEL);
        i += 1;
    }

    init_i8259a_irqs();
    common_init_isa_dma();

    if request_irq(16 + 20, no_action, 0, "isa-cascade", core::ptr::null_mut()) != 0 {
        pr_err!("Failed to register isa-cascade interrupt\n");
    }
}

unsafe fn rx164_map_irq(dev: *const pci_dev, slot: u8, pin: u8) -> ::core::ffi::c_int {
    let _ = dev;
    /*
     * JRP - Need to figure out how to distinguish pass1 from pass2,
     * and use the correct table.
     */
    const IRQ_TAB: [[i8; 5]; 6] = [
        [16 + 0, 16 + 0, 16 + 6, 16 + 11, 16 + 16],
        [16 + 1, 16 + 1, 16 + 7, 16 + 12, 16 + 17],
        [-1, -1, -1, -1, -1],
        [16 + 2, 16 + 2, 16 + 8, 16 + 13, 16 + 18],
        [16 + 3, 16 + 3, 16 + 9, 16 + 14, 16 + 19],
        [16 + 4, 16 + 4, 16 + 10, 16 + 15, 16 + 5],
    ];
    let _ = IRQ_TAB;
    let min_idsel = 5;
    let max_idsel = 10;
    let irqs_per_slot = 5;
    COMMON_TABLE_LOOKUP!(dev, slot, pin, min_idsel, max_idsel, irqs_per_slot)
}

/* The System Vector */
static mut rx164_mv: alpha_machine_vector = alpha_machine_vector {
    vector_name: "RX164",
    DO_EV5_MMU,
    DO_DEFAULT_RTC,
    DO_POLARIS_IO,
    machine_check: polaris_machine_check,
    max_isa_dma_address: ALPHA_MAX_ISA_DMA_ADDRESS,
    min_io_address: DEFAULT_IO_BASE,
    min_mem_address: DEFAULT_MEM_BASE,
    nr_irqs: 40,
    device_interrupt: Some(rx164_device_interrupt),
    init_arch: Some(polaris_init_arch),
    init_irq: Some(rx164_init_irq),
    init_rtc: Some(common_init_rtc),
    init_pci: Some(common_init_pci),
    kill_arch: None,
    pci_map_irq: Some(rx164_map_irq),
    pci_swizzle: Some(common_swizzle),
    ..unsafe { core::mem::zeroed() }
};

ALIAS_MV!(rx164);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
