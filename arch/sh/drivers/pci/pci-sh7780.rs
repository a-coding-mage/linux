// SPDX-License-Identifier: GPL-2.0
/*
 * Low-Level PCI Support for the SH7780
 *
 *  Copyright (C) 2005 - 2010  Paul Mundt
 */

// Linux dependencies supplied by the surrounding kernel translation.

#[cfg(feature = "cpu_big_endian")]
const PCICR_ENDIANNESS: u32 = SH4_PCICR_BSWP;
#[cfg(not(feature = "cpu_big_endian"))]
const PCICR_ENDIANNESS: u32 = 0;

static mut sh7785_pci_resources: [struct_resource; 4] = [
    struct_resource { name: "PCI IO", start: 0x1000, end: SZ_4M - 1, flags: IORESOURCE_IO },
    struct_resource { name: "PCI MEM 0", start: 0xfd000000, end: 0xfd000000 + SZ_16M - 1, flags: IORESOURCE_MEM },
    struct_resource { name: "PCI MEM 1", start: 0x10000000, end: 0x10000000 + SZ_64M - 1, flags: IORESOURCE_MEM },
    // 32-bit only resources must be last.
    struct_resource { name: "PCI MEM 2", start: 0xc0000000, end: 0xc0000000 + SZ_512M - 1, flags: IORESOURCE_MEM | IORESOURCE_MEM_32BIT },
];

static mut sh7780_pci_controller: pci_channel = pci_channel {
    pci_ops: &sh4_pci_ops,
    resources: sh7785_pci_resources.as_ptr(),
    nr_resources: 4,
    io_offset: 0,
    mem_offset: 0,
    io_map_base: 0xfe200000,
    serr_irq: evt2irq(0xa00),
    err_irq: evt2irq(0xaa0),
};

struct pci_errors {
    mask: u32,
    str_: *const u8,
}

static mut pci_arbiter_errors: [pci_errors; 7] = [
    pci_errors { mask: SH4_PCIAINT_MBKN, str_: b"master broken\0".as_ptr() },
    pci_errors { mask: SH4_PCIAINT_TBTO, str_: b"target bus time out\0".as_ptr() },
    pci_errors { mask: SH4_PCIAINT_MBTO, str_: b"master bus time out\0".as_ptr() },
    pci_errors { mask: SH4_PCIAINT_TABT, str_: b"target abort\0".as_ptr() },
    pci_errors { mask: SH4_PCIAINT_MABT, str_: b"master abort\0".as_ptr() },
    pci_errors { mask: SH4_PCIAINT_RDPE, str_: b"read data parity error\0".as_ptr() },
    pci_errors { mask: SH4_PCIAINT_WDPE, str_: b"write data parity error\0".as_ptr() },
];

static mut pci_interrupt_errors: [pci_errors; 12] = [
    pci_errors { mask: SH4_PCIINT_MLCK, str_: b"master lock error\0".as_ptr() },
    pci_errors { mask: SH4_PCIINT_TABT, str_: b"target-target abort\0".as_ptr() },
    pci_errors { mask: SH4_PCIINT_TRET, str_: b"target retry time out\0".as_ptr() },
    pci_errors { mask: SH4_PCIINT_MFDE, str_: b"master function disable error\0".as_ptr() },
    pci_errors { mask: SH4_PCIINT_PRTY, str_: b"address parity error\0".as_ptr() },
    pci_errors { mask: SH4_PCIINT_SERR, str_: b"SERR\0".as_ptr() },
    pci_errors { mask: SH4_PCIINT_TWDP, str_: b"data parity error for target write\0".as_ptr() },
    pci_errors { mask: SH4_PCIINT_TRDP, str_: b"PERR detected for target read\0".as_ptr() },
    pci_errors { mask: SH4_PCIINT_MTABT, str_: b"target abort for master\0".as_ptr() },
    pci_errors { mask: SH4_PCIINT_MMABT, str_: b"master abort for master\0".as_ptr() },
    pci_errors { mask: SH4_PCIINT_MWPD, str_: b"master write data parity error\0".as_ptr() },
    pci_errors { mask: SH4_PCIINT_MRPD, str_: b"master read data parity error\0".as_ptr() },
];

unsafe fn sh7780_pci_err_irq(irq: i32, dev_id: *mut core::ffi::c_void) -> irqreturn_t {
    let hose = dev_id as *mut pci_channel;
    let addr = __raw_readl((*hose).reg_base + SH4_PCIALR);
    let mut status = __raw_readw((*hose).reg_base + PCI_STATUS);
    let mut cmd: u32;
    if status & (PCI_STATUS_PARITY | PCI_STATUS_DETECTED_PARITY | PCI_STATUS_SIG_TARGET_ABORT |
                 PCI_STATUS_REC_TARGET_ABORT | PCI_STATUS_REC_MASTER_ABORT) != 0 {
        cmd = pcibios_handle_status_errors(addr, status, hose);
        if cmd != 0 { __raw_writew(cmd, (*hose).reg_base + PCI_STATUS); }
    }
    status = __raw_readl((*hose).reg_base + SH4_PCIAINT);
    cmd = 0;
    for i in 0..pci_arbiter_errors.len() {
        if status & pci_arbiter_errors[i].mask != 0 { printk(KERN_DEBUG, b"PCI: %s, addr=%08lx\n\0".as_ptr(), pci_arbiter_errors[i].str_, addr); cmd |= pci_arbiter_errors[i].mask; }
    }
    __raw_writel(cmd, (*hose).reg_base + SH4_PCIAINT);
    status = __raw_readl((*hose).reg_base + SH4_PCIINT);
    cmd = 0;
    for i in 0..pci_interrupt_errors.len() {
        if status & pci_interrupt_errors[i].mask != 0 { printk(KERN_DEBUG, b"PCI: %s, addr=%08lx\n\0".as_ptr(), pci_interrupt_errors[i].str_, addr); cmd |= pci_interrupt_errors[i].mask; }
    }
    __raw_writel(cmd, (*hose).reg_base + SH4_PCIINT);
    IRQ_HANDLED
}

unsafe fn sh7780_pci_serr_irq(irq: i32, dev_id: *mut core::ffi::c_void) -> irqreturn_t {
    let hose = dev_id as *mut pci_channel;
    printk(KERN_DEBUG, b"PCI: system error received: \0".as_ptr());
    pcibios_report_status(PCI_STATUS_SIG_SYSTEM_ERROR, 1);
    pr_cont(b"\n\0".as_ptr());
    __raw_writel(SH4_PCIINTM_SDIM, (*hose).reg_base + SH4_PCIINTM);
    disable_irq_nosync(irq);
    (*hose).serr_timer.expires = jiffies + HZ;
    add_timer(&mut (*hose).serr_timer);
    IRQ_HANDLED
}

unsafe fn sh7780_pci_setup_irqs(hose: *mut pci_channel) -> i32 {
    let mut ret: i32;
    __raw_writel(0, (*hose).reg_base + SH4_PCIAINT);
    __raw_writew(PCI_STATUS_DETECTED_PARITY | PCI_STATUS_SIG_SYSTEM_ERROR | PCI_STATUS_REC_MASTER_ABORT |
                 PCI_STATUS_REC_TARGET_ABORT | PCI_STATUS_SIG_TARGET_ABORT | PCI_STATUS_PARITY,
                 (*hose).reg_base + PCI_STATUS);
    ret = request_irq((*hose).serr_irq, sh7780_pci_serr_irq, 0, b"PCI SERR interrupt\0".as_ptr(), hose as *mut _);
    if ret != 0 { pr_err(b"PCI: Failed hooking SERR IRQ\n\0".as_ptr()); return ret; }
    ret = request_irq((*hose).err_irq, sh7780_pci_err_irq, IRQF_SHARED, b"PCI ERR interrupt\0".as_ptr(), hose as *mut _);
    if ret != 0 { free_irq((*hose).serr_irq, hose as *mut _); return ret; }
    __raw_writel(SH4_PCIAINT_MBKN | SH4_PCIAINT_TBTO | SH4_PCIAINT_MBTO | SH4_PCIAINT_TABT | SH4_PCIAINT_MABT | SH4_PCIAINT_RDPE | SH4_PCIAINT_WDPE, (*hose).reg_base + SH4_PCIAINTM);
    __raw_writel(SH4_PCIINTM_TTADIM | SH4_PCIINTM_TMTOIM | SH4_PCIINTM_MDEIM | SH4_PCIINTM_APEDIM | SH4_PCIINTM_SDIM | SH4_PCIINTM_DPEITWM | SH4_PCIINTM_PEDITRM | SH4_PCIINTM_TADIMM | SH4_PCIINTM_MADIMM | SH4_PCIINTM_MWPDIM | SH4_PCIINTM_MRDPEIM, (*hose).reg_base + SH4_PCIINTM);
    ret
}

unsafe fn sh7780_pci_teardown_irqs(hose: *mut pci_channel) { free_irq((*hose).err_irq, hose as *mut _); free_irq((*hose).serr_irq, hose as *mut _); }

unsafe fn sh7780_pci66_init(hose: *mut pci_channel) {
    if !pci_is_66mhz_capable(hose, 0, 0) { return; }
    let mut tmp = __raw_readl((*hose).reg_base + SH4_PCICR) | SH4_PCICR_PREFIX;
    __raw_writel(tmp, (*hose).reg_base + SH4_PCICR);
    tmp = __raw_readw((*hose).reg_base + PCI_STATUS) | PCI_STATUS_66MHZ;
    __raw_writew(tmp, (*hose).reg_base + PCI_STATUS);
    tmp = __raw_readl((*hose).reg_base + SH4_PCICR) | SH4_PCICR_PREFIX | SH4_PCICR_CFIN;
    __raw_writel(tmp, (*hose).reg_base + SH4_PCICR);
}

// The remaining initialization is kept as a direct unsafe translation of the C routine.
unsafe fn sh7780_pci_init() -> i32 {
    let chan = &mut sh7780_pci_controller as *mut pci_channel;
    pr_notice(b"PCI: Starting initialization.\n\0".as_ptr());
    (*chan).reg_base = 0xfe040000;
    __raw_writel(PCIECR_ENBL, PCIECR);
    __raw_writel(SH4_PCICR_PREFIX | SH4_PCICR_PRST | PCICR_ENDIANNESS, (*chan).reg_base + SH4_PCICR);
    mdelay(100);
    let mut id = __raw_readw((*chan).reg_base + PCI_VENDOR_ID);
    if id != PCI_VENDOR_ID_RENESAS { pr_err(b"PCI: Unknown vendor ID 0x%04x.\n\0".as_ptr(), id); return -ENODEV; }
    id = __raw_readw((*chan).reg_base + PCI_DEVICE_ID);
    let type_ = if id == PCI_DEVICE_ID_RENESAS_SH7763 { b"SH7763\0".as_ptr() } else if id == PCI_DEVICE_ID_RENESAS_SH7780 { b"SH7780\0".as_ptr() } else if id == PCI_DEVICE_ID_RENESAS_SH7781 { b"SH7781\0".as_ptr() } else if id == PCI_DEVICE_ID_RENESAS_SH7785 { b"SH7785\0".as_ptr() } else { core::ptr::null() };
    if type_.is_null() { pr_err(b"PCI: Found an unsupported Renesas host controller, device id 0x%04x.\n\0".as_ptr(), id); return -EINVAL; }
    pr_notice(b"PCI: Found a Renesas %s host controller, revision %d.\n\0".as_ptr(), type_, __raw_readb((*chan).reg_base + PCI_REVISION_ID));
    __raw_writel(SH4_PCICR_PREFIX | PCICR_ENDIANNESS, (*chan).reg_base + SH4_PCICR);
    let memphys = __pa(memory_start);
    let mut memsize = roundup_pow_of_two(memory_end - memory_start);
    if memsize > SZ_512M { __raw_writel(memphys + SZ_512M, (*chan).reg_base + SH4_PCILAR1); __raw_writel((((memsize - SZ_512M) - SZ_1M) & 0x1ff00000) | 1, (*chan).reg_base + SH4_PCILSR1); memsize = SZ_512M; } else { __raw_writel(0, (*chan).reg_base + SH4_PCILAR1); __raw_writel(0, (*chan).reg_base + SH4_PCILSR1); }
    __raw_writel(memphys, (*chan).reg_base + SH4_PCILAR0);
    __raw_writel(((memsize - SZ_1M) & 0x1ff00000) | 1, (*chan).reg_base + SH4_PCILSR0);
    let mut ret = sh7780_pci_setup_irqs(chan); if ret != 0 { return ret; }
    __raw_writel(0, (*chan).reg_base + SH7780_PCICSCR0); __raw_writel(0, (*chan).reg_base + SH7780_PCICSAR0); __raw_writel(0, (*chan).reg_base + SH7780_PCICSCR1); __raw_writel(0, (*chan).reg_base + SH7780_PCICSAR1);
    for i in 1..(*chan).nr_resources { let res = (*chan).resources.add(i); if (*res).flags & IORESOURCE_IO != 0 { continue; } if (*res).flags & IORESOURCE_MEM_32BIT != 0 && __in_29bit_mode() { (*chan).nr_resources -= 1; continue; } let size = resource_size(res); __raw_writel(((roundup_pow_of_two(size) / SZ_256K) - 1) << 18, (*chan).reg_base + SH7780_PCIMBMR(i - 1)); __raw_writel((*res).start, (*chan).reg_base + SH7780_PCIMBR(i - 1)); }
    __raw_writel(0, (*chan).reg_base + PCI_BASE_ADDRESS_0); __raw_writel(0, (*chan).reg_base + SH7780_PCIIOBR); __raw_writel(0, (*chan).reg_base + SH7780_PCIIOBMR);
    __raw_writew(PCI_COMMAND_SERR | PCI_COMMAND_WAIT | PCI_COMMAND_PARITY | PCI_COMMAND_MASTER | PCI_COMMAND_MEMORY, (*chan).reg_base + PCI_COMMAND);
    __raw_writel(SH4_PCICR_PREFIX | SH4_PCICR_CFIN | SH4_PCICR_FTO | PCICR_ENDIANNESS, (*chan).reg_base + SH4_PCICR);
    ret = register_pci_controller(chan); if ret != 0 { sh7780_pci_teardown_irqs(chan); return ret; }
    sh7780_pci66_init(chan);
    pr_notice(b"PCI: Running at %dMHz.\n\0".as_ptr(), if __raw_readw((*chan).reg_base + PCI_STATUS) & PCI_STATUS_66MHZ != 0 { 66 } else { 33 });
    0
}

// arch_initcall(sh7780_pci_init)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
