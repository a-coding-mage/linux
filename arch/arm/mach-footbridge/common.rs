// SPDX-License-Identifier: GPL-2.0-only
/*
 *  linux/arch/arm/mach-footbridge/common.c
 *
 *  Copyright (C) 1998-2000 Russell King, Dave Gilbert.
 */
// Dependencies supplied by the surrounding kernel translation unit are intentionally external.

unsafe fn dc21285_get_irq() -> i32 {
    let irqstatus = CSR_IRQ_STATUS as *const core::ffi::c_void;
    let mask: u32 = readl(irqstatus as *const u32);

    if mask & IRQ_MASK_SDRAMPARITY != 0 { return IRQ_SDRAMPARITY; }
    if mask & IRQ_MASK_UART_RX != 0 { return IRQ_CONRX; }
    if mask & IRQ_MASK_DMA1 != 0 { return IRQ_DMA1; }
    if mask & IRQ_MASK_DMA2 != 0 { return IRQ_DMA2; }
    if mask & IRQ_MASK_IN0 != 0 { return IRQ_IN0; }
    if mask & IRQ_MASK_IN1 != 0 { return IRQ_IN1; }
    if mask & IRQ_MASK_IN2 != 0 { return IRQ_IN2; }
    if mask & IRQ_MASK_IN3 != 0 { return IRQ_IN3; }
    if mask & IRQ_MASK_PCI != 0 { return IRQ_PCI; }
    if mask & IRQ_MASK_DOORBELLHOST != 0 { return IRQ_DOORBELLHOST; }
    if mask & IRQ_MASK_I2OINPOST != 0 { return IRQ_I2OINPOST; }
    if mask & IRQ_MASK_TIMER1 != 0 { return IRQ_TIMER1; }
    if mask & IRQ_MASK_TIMER2 != 0 { return IRQ_TIMER2; }
    if mask & IRQ_MASK_TIMER3 != 0 { return IRQ_TIMER3; }
    if mask & IRQ_MASK_UART_TX != 0 { return IRQ_CONTX; }
    if mask & IRQ_MASK_PCI_ABORT != 0 { return IRQ_PCI_ABORT; }
    if mask & IRQ_MASK_PCI_SERR != 0 { return IRQ_PCI_SERR; }
    if mask & IRQ_MASK_DISCARD_TIMER != 0 { return IRQ_DISCARD_TIMER; }
    if mask & IRQ_MASK_PCI_DPERR != 0 { return IRQ_PCI_DPERR; }
    if mask & IRQ_MASK_PCI_PERR != 0 { return IRQ_PCI_PERR; }
    0
}

unsafe fn dc21285_handle_irq(_regs: *mut pt_regs) {
    loop {
        let irq = dc21285_get_irq();
        if irq == 0 { break; }
        generic_handle_irq(irq);
    }
}

pub static mut mem_fclk_21285: u32 = 50000000;

unsafe fn early_fclk(arg: *mut i8) -> i32 {
    mem_fclk_21285 = simple_strtoul(arg, core::ptr::null_mut(), 0);
    0
}

unsafe fn parse_tag_memclk(tag: *const tag) -> i32 {
    mem_fclk_21285 = (*tag).u.memclk.fmemclk;
    0
}

/* Footbridge IRQ translation table: converts IRQ numbers into FootBridge masks. */
static FB_IRQ_MASK: [i32; 20] = [
    IRQ_MASK_UART_RX, IRQ_MASK_UART_TX, IRQ_MASK_TIMER1, IRQ_MASK_TIMER2,
    IRQ_MASK_TIMER3, IRQ_MASK_IN0, IRQ_MASK_IN1, IRQ_MASK_IN2, IRQ_MASK_IN3,
    IRQ_MASK_DOORBELLHOST, IRQ_MASK_DMA1, IRQ_MASK_DMA2, IRQ_MASK_PCI,
    IRQ_MASK_SDRAMPARITY, IRQ_MASK_I2OINPOST, IRQ_MASK_PCI_ABORT,
    IRQ_MASK_PCI_SERR, IRQ_MASK_DISCARD_TIMER, IRQ_MASK_PCI_DPERR, IRQ_MASK_PCI_PERR,
];

unsafe fn fb_mask_irq(d: *mut irq_data) {
    *(CSR_IRQ_DISABLE as *mut u32) = FB_IRQ_MASK[_DC21285_INR((*d).irq) as usize] as u32;
}

unsafe fn fb_unmask_irq(d: *mut irq_data) {
    *(CSR_IRQ_ENABLE as *mut u32) = FB_IRQ_MASK[_DC21285_INR((*d).irq) as usize] as u32;
}

static mut fb_chip: irq_chip = irq_chip {
    irq_ack: Some(fb_mask_irq),
    irq_mask: Some(fb_mask_irq),
    irq_unmask: Some(fb_unmask_irq),
};

unsafe fn __fb_init_irq() {
    *(CSR_IRQ_DISABLE as *mut u32) = u32::MAX;
    *(CSR_FIQ_DISABLE as *mut u32) = u32::MAX;
    let mut irq = _DC21285_IRQ(0);
    while irq < _DC21285_IRQ(20) {
        irq_set_chip_and_handler(irq, &raw mut fb_chip, handle_level_irq);
        irq_clear_status_flags(irq, IRQ_NOREQUEST | IRQ_NOPROBE);
        irq += 1;
    }
}

unsafe fn footbridge_init_irq() {
    set_handle_irq(dc21285_handle_irq);
    __fb_init_irq();
    if machine_is_ebsa285() { isa_init_irq(IRQ_PCI); }
    if machine_is_netwinder() { isa_init_irq(IRQ_IN3); }
}

static mut ebsa285_host_io_desc: [map_desc; 5] = [
    map_desc { virtual_: ARMCSR_BASE, pfn: __phys_to_pfn(DC21285_ARMCSR_BASE), length: ARMCSR_SIZE, type_: MT_DEVICE },
    map_desc { virtual_: PCIMEM_BASE, pfn: __phys_to_pfn(DC21285_PCI_MEM), length: PCIMEM_SIZE, type_: MT_DEVICE },
    map_desc { virtual_: PCICFG0_BASE, pfn: __phys_to_pfn(DC21285_PCI_TYPE_0_CONFIG), length: PCICFG0_SIZE, type_: MT_DEVICE },
    map_desc { virtual_: PCICFG1_BASE, pfn: __phys_to_pfn(DC21285_PCI_TYPE_1_CONFIG), length: PCICFG1_SIZE, type_: MT_DEVICE },
    map_desc { virtual_: PCIIACK_BASE, pfn: __phys_to_pfn(DC21285_PCI_IACK), length: PCIIACK_SIZE, type_: MT_DEVICE },
];

unsafe fn footbridge_map_io() {
    iotable_init(ebsa285_host_io_desc.as_mut_ptr(), ebsa285_host_io_desc.len());
    pci_map_io_early(__phys_to_pfn(DC21285_PCI_IO));
    vga_base = PCIMEM_BASE;
}

unsafe fn footbridge_restart(mode: reboot_mode, _cmd: *const i8) {
    if mode == REBOOT_SOFT {
        soft_restart(0x41000000);
    } else {
        *(CSR_SA110_CNTL as *mut u32) &= !(1 << 13);
        *(CSR_TIMER4_CNTL as *mut u32) = TIMER_CNTL_ENABLE | TIMER_CNTL_AUTORELOAD | TIMER_CNTL_DIV16;
        *(CSR_TIMER4_LOAD as *mut u32) = 0x2;
        *(CSR_TIMER4_CLR as *mut u32) = 0;
        *(CSR_SA110_CNTL as *mut u32) |= 1 << 13;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
