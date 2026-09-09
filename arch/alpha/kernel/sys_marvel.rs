// SPDX-License-Identifier: GPL-2.0
/*
 * linux/arch/alpha/kernel/sys_marvel.c
 *
 * Marvel / IO7 support
 */

/* Linux and Alpha dependencies are supplied by the surrounding translation unit. */

unsafe fn io7_device_interrupt(vector: libc::c_ulong) {
    let pid: libc::c_uint = vector >> 16;
    let mut irq: libc::c_uint = ((vector & 0xffff).wrapping_sub(0x800)) >> 4;

    /* Vector is 0x800 + (interrupt); interrupt contains PE and irq fields. */
    irq = irq.wrapping_add(16);
    irq &= MARVEL_IRQ_VEC_IRQ_MASK;
    irq |= pid << MARVEL_IRQ_VEC_PE_SHIFT;
    handle_irq(irq);
}

unsafe fn io7_get_irq_ctl(irq: libc::c_uint, pio7: *mut *mut io7) -> *mut libc::c_ulong {
    let pid = irq >> MARVEL_IRQ_VEC_PE_SHIFT;
    let io7 = marvel_find_io7(pid);
    if io7.is_null() {
        printk(KERN_ERR, "%s for nonexistent io7 -- vec %x, pid %d\n", __func__, irq, pid);
        return core::ptr::null_mut();
    }

    let mut irq = irq & MARVEL_IRQ_VEC_IRQ_MASK;
    irq -= 16;
    if irq >= 0x180 {
        printk(KERN_ERR, "%s for invalid irq -- pid %d adjusted irq %x\n", __func__, pid, irq);
        return core::ptr::null_mut();
    }

    let mut ctl = &mut (*(*io7).csrs).PO7_LSI_CTL[(irq & 0xff) as usize].csr as *mut _;
    if irq >= 0x80 {
        ctl = &mut (*(*io7).csrs).PO7_MSI_CTL[((irq - 0x80) >> 5 & 0x0f) as usize].csr as *mut _;
    }
    if !pio7.is_null() { *pio7 = io7; }
    ctl
}

unsafe fn io7_enable_irq(d: *mut irq_data) {
    let irq = (*d).irq;
    let mut io7 = core::ptr::null_mut();
    let ctl = io7_get_irq_ctl(irq, &mut io7);
    if ctl.is_null() || io7.is_null() {
        printk(KERN_ERR, "%s: get_ctl failed for irq %x\n", __func__, irq);
        return;
    }
    raw_spin_lock(&mut (*io7).irq_lock);
    core::ptr::write_volatile(ctl, core::ptr::read_volatile(ctl) | (1usize << 24) as libc::c_ulong);
    mb();
    core::ptr::read_volatile(ctl);
    raw_spin_unlock(&mut (*io7).irq_lock);
}

unsafe fn io7_disable_irq(d: *mut irq_data) {
    let irq = (*d).irq;
    let mut io7 = core::ptr::null_mut();
    let ctl = io7_get_irq_ctl(irq, &mut io7);
    if ctl.is_null() || io7.is_null() {
        printk(KERN_ERR, "%s: get_ctl failed for irq %x\n", __func__, irq);
        return;
    }
    raw_spin_lock(&mut (*io7).irq_lock);
    core::ptr::write_volatile(ctl, core::ptr::read_volatile(ctl) & !((1usize << 24) as libc::c_ulong));
    mb();
    core::ptr::read_volatile(ctl);
    raw_spin_unlock(&mut (*io7).irq_lock);
}

unsafe fn marvel_irq_noop(_d: *mut irq_data) {}

static mut marvel_legacy_irq_type: irq_chip = irq_chip {
    name: "LEGACY", irq_mask: Some(marvel_irq_noop), irq_unmask: Some(marvel_irq_noop),
};
static mut io7_lsi_irq_type: irq_chip = irq_chip {
    name: "LSI", irq_unmask: Some(io7_enable_irq), irq_mask: Some(io7_disable_irq), irq_mask_ack: Some(io7_disable_irq),
};
static mut io7_msi_irq_type: irq_chip = irq_chip {
    name: "MSI", irq_unmask: Some(io7_enable_irq), irq_mask: Some(io7_disable_irq), irq_ack: Some(marvel_irq_noop),
};

unsafe fn io7_redirect_irq(_io7: *mut io7, csr: *mut libc::c_ulong, where_: libc::c_uint) {
    let mut val = core::ptr::read_volatile(csr);
    val &= !(0x1ffusize << 24) as libc::c_ulong;
    val |= (where_ as libc::c_ulong) << 24;
    core::ptr::write_volatile(csr, val);
    mb(); core::ptr::read_volatile(csr);
}

unsafe fn io7_redirect_one_lsi(io7: *mut io7, which: libc::c_uint, where_: libc::c_uint) {
    let csr = &mut (*(*io7).csrs).PO7_LSI_CTL[which as usize].csr as *mut _;
    let mut val = core::ptr::read_volatile(csr);
    val &= !(0x1ffusize << 14) as libc::c_ulong;
    val |= (where_ as libc::c_ulong) << 14;
    core::ptr::write_volatile(csr, val); mb(); core::ptr::read_volatile(csr);
}

unsafe fn io7_redirect_one_msi(io7: *mut io7, which: libc::c_uint, where_: libc::c_uint) {
    let csr = &mut (*(*io7).csrs).PO7_MSI_CTL[which as usize].csr as *mut _;
    let mut val = core::ptr::read_volatile(csr);
    val &= !(0x1ffusize << 14) as libc::c_ulong;
    val |= (where_ as libc::c_ulong) << 14;
    core::ptr::write_volatile(csr, val); mb(); core::ptr::read_volatile(csr);
}

unsafe fn init_one_io7_lsi(io7: *mut io7, which: libc::c_uint, where_: libc::c_uint) {
    let csr = &mut (*(*io7).csrs).PO7_LSI_CTL[which as usize].csr as *mut _;
    core::ptr::write_volatile(csr, (where_ as libc::c_ulong) << 14); mb(); core::ptr::read_volatile(csr);
}
unsafe fn init_one_io7_msi(io7: *mut io7, which: libc::c_uint, where_: libc::c_uint) {
    let csr = &mut (*(*io7).csrs).PO7_MSI_CTL[which as usize].csr as *mut _;
    core::ptr::write_volatile(csr, (where_ as libc::c_ulong) << 14); mb(); core::ptr::read_volatile(csr);
}

unsafe fn init_io7_irqs(io7: *mut io7, lsi_ops: *mut irq_chip, msi_ops: *mut irq_chip) {
    let base = ((*io7).pe << MARVEL_IRQ_VEC_PE_SHIFT) as libc::c_long + 16;
    printk(0, "Initializing interrupts for IO7 at PE %u - base %lx\n", (*io7).pe, base);
    printk(0, "  Interrupts reported to CPU at PE %u\n", boot_cpuid);
    for i in 0..128 { irq_set_chip_and_handler(base + i, lsi_ops, handle_level_irq); irq_set_status_flags(base + i, IRQ_LEVEL); }
    for i in 128..640 { irq_set_chip_and_handler(base + i, msi_ops, handle_level_irq); irq_set_status_flags(base + i, IRQ_LEVEL); }
    raw_spin_lock(&mut (*io7).irq_lock);
    io7_redirect_irq(io7, &mut (*(*io7).csrs).HLT_CTL.csr, boot_cpuid);
    io7_redirect_irq(io7, &mut (*(*io7).csrs).HPI_CTL.csr, boot_cpuid);
    io7_redirect_irq(io7, &mut (*(*io7).csrs).CRD_CTL.csr, boot_cpuid);
    io7_redirect_irq(io7, &mut (*(*io7).csrs).STV_CTL.csr, boot_cpuid);
    io7_redirect_irq(io7, &mut (*(*io7).csrs).HEI_CTL.csr, boot_cpuid);
    for i in 0..0x60 { init_one_io7_lsi(io7, i, boot_cpuid); }
    init_one_io7_lsi(io7, 0x74, boot_cpuid); init_one_io7_lsi(io7, 0x75, boot_cpuid);
    for i in 0..16 { init_one_io7_msi(io7, i, boot_cpuid); }
    raw_spin_unlock(&mut (*io7).irq_lock);
}

unsafe fn marvel_init_irq() {
    for i in 0..16 { irq_set_chip_and_handler(i, &mut marvel_legacy_irq_type, handle_level_irq); }
    let mut io7 = core::ptr::null_mut();
    loop { io7 = marvel_next_io7(io7); if io7.is_null() { break; } init_io7_irqs(io7, &mut io7_lsi_irq_type, &mut io7_msi_irq_type); }
}

unsafe fn marvel_map_irq(cdev: *const pci_dev, _slot: u8, _pin: u8) -> libc::c_int {
    let dev = cdev as *mut pci_dev;
    let hose = (*dev).sysdata as *mut pci_controller;
    let io7 = (*((*hose).sysdata as *mut io7_port)).io7;
    let mut intline = 0u8; pci_read_config_byte(dev, PCI_INTERRUPT_LINE, &mut intline);
    let mut irq = intline as libc::c_int;
    let msi_loc = (*dev).msi_cap; let mut msg_ctl = 0u16; let mut msg_dat = 0u16;
    if msi_loc != 0 { pci_read_config_word(dev, msi_loc + PCI_MSI_FLAGS, &mut msg_ctl); }
    if msg_ctl & PCI_MSI_FLAGS_ENABLE != 0 {
        let off = if msg_ctl & PCI_MSI_FLAGS_64BIT != 0 { PCI_MSI_DATA_64 } else { PCI_MSI_DATA_32 };
        pci_read_config_word(dev, msi_loc + off, &mut msg_dat);
        irq = (msg_dat & 0x1ff) as libc::c_int; irq += 0x80;
        printk(0, "PCI:%d:%d:%d (hose %d) is using MSI\n", (*(*dev).bus).number, PCI_SLOT((*dev).devfn), PCI_FUNC((*dev).devfn), (*hose).index);
        printk(0, "  %d message(s) from 0x%04x\n", 1 << ((msg_ctl & PCI_MSI_FLAGS_QSIZE) >> 4), msg_dat);
        printk(0, "  reporting on %d IRQ(s) from %d (0x%x)\n", 1 << ((msg_ctl & PCI_MSI_FLAGS_QSIZE) >> 4), (irq + 16) | ((*io7).pe << MARVEL_IRQ_VEC_PE_SHIFT) as libc::c_int, (irq + 16) | ((*io7).pe << MARVEL_IRQ_VEC_PE_SHIFT) as libc::c_int);
    }
    (irq + 16) | ((*io7).pe << MARVEL_IRQ_VEC_PE_SHIFT) as libc::c_int
}

unsafe fn marvel_init_pci() { marvel_register_error_handlers(); pci_set_flags(PCI_PROBE_ONLY); common_init_pci(); locate_and_init_vga(core::ptr::null_mut()); let mut io7 = core::ptr::null_mut(); loop { io7 = marvel_next_io7(io7); if io7.is_null() { break; } io7_clear_errors(io7); } }
unsafe fn marvel_init_rtc() { init_rtc_irq(core::ptr::null_mut()); }

unsafe fn marvel_smp_callin() {
    let cpuid = hard_smp_processor_id(); let io7 = marvel_find_io7(cpuid); if io7.is_null() { return; }
    printk(0, "Redirecting IO7 interrupts to local CPU at PE %u\n", cpuid);
    io7_redirect_irq(io7, &mut (*(*io7).csrs).HLT_CTL.csr, cpuid); io7_redirect_irq(io7, &mut (*(*io7).csrs).HPI_CTL.csr, cpuid); io7_redirect_irq(io7, &mut (*(*io7).csrs).CRD_CTL.csr, cpuid); io7_redirect_irq(io7, &mut (*(*io7).csrs).STV_CTL.csr, cpuid); io7_redirect_irq(io7, &mut (*(*io7).csrs).HEI_CTL.csr, cpuid);
    for i in 0..0x60 { io7_redirect_one_lsi(io7, i, cpuid); } io7_redirect_one_lsi(io7, 0x74, cpuid); io7_redirect_one_lsi(io7, 0x75, cpuid); for i in 0..16 { io7_redirect_one_msi(io7, i, cpuid); }
}

/* System Vectors */
static mut marvel_ev7_mv: alpha_machine_vector = alpha_machine_vector {
    vector_name: "MARVEL/EV7", rtc_port: 0x70, rtc_boot_cpu_only: 1,
    machine_check: Some(marvel_machine_check), max_isa_dma_address: ALPHA_MAX_ISA_DMA_ADDRESS,
    min_io_address: DEFAULT_IO_BASE, min_mem_address: DEFAULT_MEM_BASE, pci_dac_offset: IO7_DAC_OFFSET,
    nr_irqs: MARVEL_NR_IRQS, device_interrupt: Some(io7_device_interrupt), agp_info: marvel_agp_info,
    smp_callin: Some(marvel_smp_callin), init_arch: Some(marvel_init_arch), init_irq: Some(marvel_init_irq),
    init_rtc: Some(marvel_init_rtc), init_pci: Some(marvel_init_pci), kill_arch: Some(marvel_kill_arch),
    pci_map_irq: Some(marvel_map_irq), pci_swizzle: Some(common_swizzle),
};

ALIAS_MV!(marvel_ev7);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
