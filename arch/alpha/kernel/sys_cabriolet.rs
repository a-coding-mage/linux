// SPDX-License-Identifier: GPL-2.0
/*
 * linux/arch/alpha/kernel/sys_cabriolet.c
 *
 * Code supporting the PC164 and LX164.
 */

// Linux and architecture headers from the C translation unit are supplied by
// the surrounding kernel environment.

/* Note mask bit is true for DISABLED irqs. */
static mut cached_irq_mask: ::core::primitive::c_ulong = !0;

#[inline]
unsafe fn cabriolet_update_irq_hw(irq: ::core::primitive::c_uint, mask: ::core::primitive::c_ulong) {
    let ofs: ::core::primitive::c_int = ((irq - 16) / 8) as ::core::primitive::c_int;
    outb(mask >> (16 + ofs * 8), 0x804 + ofs as ::core::primitive::c_ulong);
}

#[inline]
unsafe fn cabriolet_enable_irq(d: *mut irq_data) {
    cached_irq_mask &= !(1 as ::core::primitive::c_ulong << (*d).irq);
    cabriolet_update_irq_hw((*d).irq, cached_irq_mask);
}

unsafe fn cabriolet_disable_irq(d: *mut irq_data) {
    cached_irq_mask |= 1 as ::core::primitive::c_ulong << (*d).irq;
    cabriolet_update_irq_hw((*d).irq, cached_irq_mask);
}

static mut cabriolet_irq_type: irq_chip = irq_chip {
    name: b"CABRIOLET\0".as_ptr() as *const ::core::ffi::c_char,
    irq_unmask: Some(cabriolet_enable_irq),
    irq_mask: Some(cabriolet_disable_irq),
    irq_mask_ack: Some(cabriolet_disable_irq),
};

unsafe fn cabriolet_device_interrupt(v: ::core::primitive::c_ulong) {
    let mut pld: ::core::primitive::c_ulong;
    let mut i: ::core::primitive::c_uint;

    /* Read the interrupt summary registers */
    pld = inb(0x804) as ::core::primitive::c_ulong
        | ((inb(0x805) as ::core::primitive::c_ulong) << 8)
        | ((inb(0x806) as ::core::primitive::c_ulong) << 16);

    /* Now for every possible bit set, work through them and call the
     * appropriate interrupt handler. */
    while pld != 0 {
        i = ffz(!pld);
        pld &= pld.wrapping_sub(1); /* clear least bit set */
        if i == 4 {
            isa_device_interrupt(v);
        } else {
            handle_irq(16 + i);
        }
    }
}

unsafe fn common_init_irq(srm_dev_int: unsafe fn(::core::primitive::c_ulong)) {
    init_i8259a_irqs();

    if alpha_using_srm {
        alpha_mv.device_interrupt = Some(srm_dev_int);
        init_srm_irqs(35, 0);
    } else {
        let mut i: ::core::primitive::c_long;

        outb(0xff, 0x804);
        outb(0xff, 0x805);
        outb(0xff, 0x806);

        i = 16;
        while i < 35 {
            irq_set_chip_and_handler(i as _, &mut cabriolet_irq_type, handle_level_irq);
            irq_set_status_flags(i as _, IRQ_LEVEL);
            i += 1;
        }
    }

    common_init_isa_dma();
    if request_irq(16 + 4, no_action, 0, b"isa-cascade\0".as_ptr() as _, core::ptr::null_mut()) != 0 {
        pr_err!("Failed to register isa-cascade interrupt\n");
    }
}

#[cfg(not(CONFIG_ALPHA_PC164))]
unsafe fn cabriolet_init_irq() {
    common_init_irq(srm_device_interrupt);
}

#[cfg(any(CONFIG_ALPHA_GENERIC, CONFIG_ALPHA_PC164))]
unsafe fn pc164_srm_device_interrupt(v: ::core::primitive::c_ulong) {
    __min_ipl = getipl();
    srm_device_interrupt(v);
    __min_ipl = 0;
}

#[cfg(any(CONFIG_ALPHA_GENERIC, CONFIG_ALPHA_PC164))]
unsafe fn pc164_device_interrupt(v: ::core::primitive::c_ulong) {
    __min_ipl = getipl();
    cabriolet_device_interrupt(v);
    __min_ipl = 0;
}

#[cfg(any(CONFIG_ALPHA_GENERIC, CONFIG_ALPHA_PC164))]
unsafe fn pc164_init_irq() {
    common_init_irq(pc164_srm_device_interrupt);
}

#[inline]
unsafe fn eb66p_map_irq(dev: *const pci_dev, slot: u8, pin: u8) -> ::core::primitive::c_int {
    static irq_tab: [[i8; 5]; 5] = [
        [16 + 0, 16 + 0, 16 + 5, 16 + 9, 16 + 13],
        [16 + 1, 16 + 1, 16 + 6, 16 + 10, 16 + 14],
        [-1, -1, -1, -1, -1],
        [16 + 2, 16 + 2, 16 + 7, 16 + 11, 16 + 15],
        [16 + 3, 16 + 3, 16 + 8, 16 + 12, 16 + 6],
    ];
    let min_idsel: ::core::primitive::c_long = 6;
    let max_idsel: ::core::primitive::c_long = 10;
    let irqs_per_slot: ::core::primitive::c_long = 5;
    COMMON_TABLE_LOOKUP
}

#[inline]
unsafe fn cabriolet_map_irq(dev: *const pci_dev, slot: u8, pin: u8) -> ::core::primitive::c_int {
    static irq_tab: [[i8; 5]; 5] = [
        [16 + 2, 16 + 2, 16 + 7, 16 + 11, 16 + 15],
        [16 + 0, 16 + 0, 16 + 5, 16 + 9, 16 + 13],
        [16 + 1, 16 + 1, 16 + 6, 16 + 10, 16 + 14],
        [-1, -1, -1, -1, -1],
        [16 + 3, 16 + 3, 16 + 8, 16 + 12, 16 + 16],
    ];
    let min_idsel: ::core::primitive::c_long = 5;
    let max_idsel: ::core::primitive::c_long = 9;
    let irqs_per_slot: ::core::primitive::c_long = 5;
    COMMON_TABLE_LOOKUP
}

#[inline]
unsafe fn cabriolet_enable_ide() {
    if pc873xx_probe() == -1 {
        printk!(KERN_ERR "Probing for PC873xx Super IO chip failed.\n");
    } else {
        printk!(KERN_INFO "Found %s Super IO chip at 0x%x\n", pc873xx_get_model(), pc873xx_get_base());
        pc873xx_enable_ide();
    }
}

#[inline]
unsafe fn cia_cab_init_pci() {
    cia_init_pci();
    cabriolet_enable_ide();
}

#[inline]
unsafe fn alphapc164_map_irq(dev: *const pci_dev, slot: u8, pin: u8) -> ::core::primitive::c_int {
    static irq_tab: [[i8; 5]; 7] = [
        [16 + 2, 16 + 2, 16 + 9, 16 + 13, 16 + 17],
        [16 + 0, 16 + 0, 16 + 7, 16 + 11, 16 + 15],
        [16 + 1, 16 + 1, 16 + 8, 16 + 12, 16 + 16],
        [-1, -1, -1, -1, -1],
        [16 + 3, 16 + 3, 16 + 10, 16 + 14, 16 + 18],
        [16 + 6, 16 + 6, 16 + 6, 16 + 6, 16 + 6],
        [16 + 5, 16 + 5, 16 + 5, 16 + 5, 16 + 5],
    ];
    let min_idsel: ::core::primitive::c_long = 5;
    let max_idsel: ::core::primitive::c_long = 11;
    let irqs_per_slot: ::core::primitive::c_long = 5;
    COMMON_TABLE_LOOKUP
}

#[inline]
unsafe fn alphapc164_init_pci() {
    cia_init_pci();
    SMC93x_Init();
}

// The C source defines lx164_mv and pc164_mv machine vectors, including
// configuration-dependent initializers and ALIAS_MV registrations. Their
// exact struct and macro definitions are supplied by the surrounding kernel.
#[cfg(any(CONFIG_ALPHA_GENERIC, CONFIG_ALPHA_LX164))]
static mut lx164_mv: alpha_machine_vector = alpha_machine_vector {
    vector_name: b"LX164\0".as_ptr() as _,
    DO_EV5_MMU,
    DO_DEFAULT_RTC,
    DO_PYXIS_IO,
    machine_check: Some(cia_machine_check),
    max_isa_dma_address: ALPHA_MAX_ISA_DMA_ADDRESS,
    min_io_address: DEFAULT_IO_BASE,
    min_mem_address: DEFAULT_MEM_BASE,
    pci_dac_offset: PYXIS_DAC_OFFSET,
    nr_irqs: 35,
    device_interrupt: Some(cabriolet_device_interrupt),
    init_arch: Some(pyxis_init_arch),
    init_irq: Some(cabriolet_init_irq),
    init_rtc: Some(common_init_rtc),
    init_pci: Some(alphapc164_init_pci),
    kill_arch: Some(cia_kill_arch),
    pci_map_irq: Some(alphapc164_map_irq),
    pci_swizzle: Some(common_swizzle),
};

#[cfg(any(CONFIG_ALPHA_GENERIC, CONFIG_ALPHA_PC164))]
static mut pc164_mv: alpha_machine_vector = alpha_machine_vector {
    vector_name: b"PC164\0".as_ptr() as _,
    DO_EV5_MMU,
    DO_DEFAULT_RTC,
    DO_CIA_IO,
    machine_check: Some(cia_machine_check),
    max_isa_dma_address: ALPHA_MAX_ISA_DMA_ADDRESS,
    min_io_address: DEFAULT_IO_BASE,
    min_mem_address: CIA_DEFAULT_MEM_BASE,
    nr_irqs: 35,
    device_interrupt: Some(pc164_device_interrupt),
    init_arch: Some(cia_init_arch),
    init_irq: Some(pc164_init_irq),
    init_rtc: Some(common_init_rtc),
    init_pci: Some(alphapc164_init_pci),
    kill_arch: Some(cia_kill_arch),
    pci_map_irq: Some(alphapc164_map_irq),
    pci_swizzle: Some(common_swizzle),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
