// SPDX-License-Identifier: GPL-2.0
/*
 * linux/arch/alpha/kernel/sys_ruffian.c
 *
 * Copyright (C) 1995 David A Rusling
 * Copyright (C) 1996 Jay A Estabrook
 * Copyright (C) 1998, 1999, 2000 Richard Henderson
 *
 * Code supporting the RUFFIAN.
 */

// Linux and architecture headers provide the types, constants, macros, and
// external functions referenced below.

unsafe fn ruffian_init_irq() {
    /* Invert 6&7 for i82371 */
    *(PYXIS_INT_HILO as *mut u32) = 0x000000c0u32;
    mb();
    *(PYXIS_INT_CNFG as *mut u32) = 0x00002064u32;
    mb(); /* all clear */

    outb(0x11, 0xA0);
    outb(0x08, 0xA1);
    outb(0x02, 0xA1);
    outb(0x01, 0xA1);
    outb(0xFF, 0xA1);

    outb(0x11, 0x20);
    outb(0x00, 0x21);
    outb(0x04, 0x21);
    outb(0x01, 0x21);
    outb(0xFF, 0x21);

    /* Finish writing the 82C59A PIC Operation Control Words */
    outb(0x20, 0xA0);
    outb(0x20, 0x20);

    init_i8259a_irqs();

    /* Not interested in the bogus interrupts (0,3,6),
       NMI (1), HALT (2), flash (5), or 21142 (8). */
    init_pyxis_irqs(0x16f0000);

    common_init_isa_dma();
}

const RUFFIAN_LATCH: u64 = DIV_ROUND_CLOSEST(PIT_TICK_RATE, HZ);

unsafe fn ruffian_init_rtc() {
    /* Ruffian does not have the RTC connected to the CPU timer
       interrupt. Instead, it uses the PIT connected to IRQ 0. */

    /* Setup interval timer. */
    outb(0x34, 0x43); // binary, mode 2, LSB/MSB, ch 0
    outb(RUFFIAN_LATCH & 0xff, 0x40); // LSB
    outb(RUFFIAN_LATCH >> 8, 0x40); // MSB

    outb(0xb6, 0x43); // pit counter 2: speaker
    outb(0x31, 0x42);
    outb(0x13, 0x42);

    if (request_irq(0, rtc_timer_interrupt, 0, "timer", core::ptr::null_mut()) != 0) {
        pr_err!("Failed to request irq 0 (timer)\n");
    }
}

unsafe fn ruffian_kill_arch(mode: i32) {
    cia_kill_arch(mode);
    // This only causes re-entry to ARCSBIOS; perhaps this works for other
    // PYXIS as well? The original code is disabled with #if 0.
}

unsafe fn ruffian_map_irq(dev: *const pci_dev, slot: u8, pin: u8) -> i32 {
    let irq_tab: [[i8; 5]; 11] = [
        [-1, -1, -1, -1, -1], // IdSel 13, 21052
        [-1, -1, -1, -1, -1], // IdSel 14, SIO
        [44, 44, 44, 44, 44], // IdSel 15, 21143
        [-1, -1, -1, -1, -1], // IdSel 16, none
        [43, 43, 42, 41, 40], // IdSel 17, 64-bit slot
        // The next 6 are actually on PCI bus 1, across the bridge.
        [19, 19, 18, 17, 16], // IdSel 8, slot 0
        [31, 31, 30, 29, 28], // IdSel 9, slot 1
        [27, 27, 26, 25, 24], // IdSel 10, slot 2
        [39, 39, 38, 37, 36], // IdSel 11, slot 3
        [35, 35, 34, 33, 32], // IdSel 12, slot 4
        [20, 20, 20, 20, 20], // IdSel 13, 53c875
    ];
    let min_idsel: i64 = 13;
    let max_idsel: i64 = 23;
    let irqs_per_slot: i64 = 5;
    COMMON_TABLE_LOOKUP!(dev, irq_tab, slot, pin, min_idsel, max_idsel, irqs_per_slot)
}

unsafe fn ruffian_swizzle(dev: *mut pci_dev, pinp: *mut u8) -> u8 {
    let mut slot: u8;
    let mut pin: i32 = *pinp as i32;

    if (*(*dev).bus).number == 0 {
        slot = PCI_SLOT!((*dev).devfn);
    } else if PCI_SLOT!((*(*dev).bus).self_.as_ref().unwrap().devfn) == 13 {
        slot = PCI_SLOT!((*dev).devfn).wrapping_add(10);
    } else {
        loop {
            if PCI_SLOT!((*(*dev).bus).self_.as_ref().unwrap().devfn) == 13 {
                slot = PCI_SLOT!((*dev).devfn).wrapping_add(10);
                break;
            }
            pin = pci_swizzle_interrupt_pin(dev, pin as u8) as i32;
            /* Move up the chain of bridges. */
            dev = (*dev).bus.self_;
            /* Slot of the next bridge. */
            slot = PCI_SLOT!((*dev).devfn);
            if (*(*dev).bus).self_.is_null() {
                break;
            }
        }
    }
    *pinp = pin as u8;
    slot
}

#[cfg(BUILDING_FOR_MILO)]
unsafe fn ruffian_get_bank_size(offset: u64) -> u64 {
    let bank_addr = (PYXIS_MCR as u64).wrapping_add(offset);
    let mut bank = *(bank_addr as *const u64);
    let mut ret = 0u64;
    if bank & 0x01 != 0 {
        let size: [u64; 9] = [
            0x40000000, 0x20000000, 0x10000000, 0x08000000, 0x04000000,
            0x02000000, 0x01000000, 0x00800000, 0x80000000,
        ];
        bank = (bank & 0x1e) >> 1;
        if bank < size.len() as u64 {
            ret = size[bank as usize];
        }
    }
    ret
}

/* The System Vector */
static mut ruffian_mv: alpha_machine_vector = alpha_machine_vector {
    vector_name: "Ruffian",
    // DO_EV5_MMU, DO_DEFAULT_RTC, and DO_PYXIS_IO expand to fields here.
    machine_check: cia_machine_check,
    max_isa_dma_address: ALPHA_RUFFIAN_MAX_ISA_DMA_ADDRESS,
    min_io_address: DEFAULT_IO_BASE,
    min_mem_address: DEFAULT_MEM_BASE,
    pci_dac_offset: PYXIS_DAC_OFFSET,
    nr_irqs: 48,
    device_interrupt: pyxis_device_interrupt,
    init_arch: pyxis_init_arch,
    init_irq: ruffian_init_irq,
    init_rtc: ruffian_init_rtc,
    init_pci: cia_init_pci,
    kill_arch: ruffian_kill_arch,
    pci_map_irq: ruffian_map_irq,
    pci_swizzle: ruffian_swizzle,
};

ALIAS_MV!(ruffian);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
