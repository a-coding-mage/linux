// SPDX-License-Identifier: GPL-2.0-only
/*
 *  linux/arch/arm/kernel/dec21285.c: PCI functions for DC21285
 *
 *  Copyright (C) 1998-2001 Russell King
 *  Copyright (C) 1998-2000 Phil Blundell
 */

// Kernel dependencies are supplied by the surrounding tree.

const MAX_SLOTS: usize = 21;
const PCICMD_ABORT: u32 = (PCI_STATUS_REC_MASTER_ABORT | PCI_STATUS_REC_TARGET_ABORT) << 16;
const PCICMD_ERROR_BITS: u32 = (PCI_STATUS_DETECTED_PARITY |
    PCI_STATUS_REC_MASTER_ABORT | PCI_STATUS_REC_TARGET_ABORT | PCI_STATUS_PARITY) << 16;

extern "C" {
    fn setup_arm_irq(irq: i32, action: *mut irqaction) -> i32;
}

unsafe fn dc21285_base_address(bus: *mut pci_bus, mut devfn: u32) -> usize {
    let mut addr: usize = 0;
    if (*bus).number == 0 {
        if PCI_SLOT(devfn) == 0 {
            /* For devfn 0, point at the 21285 */
            addr = ARMCSR_BASE;
        } else {
            devfn -= 1 << 3;
            if devfn < PCI_DEVFN(MAX_SLOTS, 0) {
                addr = PCICFG0_BASE | 0xc00000 | ((devfn as usize) << 8);
            }
        }
    } else {
        addr = PCICFG1_BASE | (((*bus).number as usize) << 16) | ((devfn as usize) << 8);
    }
    addr
}

unsafe fn dc21285_read_config(bus: *mut pci_bus, devfn: u32, where_: i32,
                               size: i32, value: *mut u32) -> i32 {
    let addr = dc21285_base_address(bus, devfn);
    let mut v: u32 = 0xffff_ffff;
    if addr != 0 {
        let p = (addr + where_ as usize) as *const u8;
        v = match size {
            1 => core::ptr::read_volatile(p) as u32,
            2 => core::ptr::read_volatile(p as *const u16) as u32,
            4 => core::ptr::read_volatile(p as *const u32),
            _ => v,
        };
    }
    *value = v;
    v = core::ptr::read_volatile(CSR_PCICMD);
    if v & PCICMD_ABORT != 0 {
        core::ptr::write_volatile(CSR_PCICMD, v & (0xffff | PCICMD_ABORT));
        return -1;
    }
    PCIBIOS_SUCCESSFUL
}

unsafe fn dc21285_write_config(bus: *mut pci_bus, devfn: u32, where_: i32,
                                size: i32, value: u32) -> i32 {
    let addr = dc21285_base_address(bus, devfn);
    if addr != 0 {
        let p = (addr + where_ as usize) as *mut u8;
        match size {
            1 => core::ptr::write_volatile(p, value as u8),
            2 => core::ptr::write_volatile(p as *mut u16, value as u16),
            4 => core::ptr::write_volatile(p as *mut u32, value),
            _ => (),
        }
    }
    let v = core::ptr::read_volatile(CSR_PCICMD);
    if v & PCICMD_ABORT != 0 {
        core::ptr::write_volatile(CSR_PCICMD, v & (0xffff | PCICMD_ABORT));
        return -1;
    }
    PCIBIOS_SUCCESSFUL
}

static mut dc21285_ops: pci_ops = pci_ops {
    read: Some(dc21285_read_config),
    write: Some(dc21285_write_config),
};

static mut serr_timer: timer_list = timer_list::default();
static mut perr_timer: timer_list = timer_list::default();

unsafe fn dc21285_enable_error(timer: *mut timer_list) {
    timer_delete(timer);
    if timer == &raw mut serr_timer { enable_irq(IRQ_PCI_SERR); }
    else if timer == &raw mut perr_timer { enable_irq(IRQ_PCI_PERR); }
}

/* Warn on PCI errors. */
unsafe fn dc21285_abort_irq(_irq: i32, _dev_id: *mut core::ffi::c_void) -> irqreturn_t {
    let mut cmd = core::ptr::read_volatile(CSR_PCICMD);
    let status = cmd >> 16;
    cmd &= 0xffff;
    if status & PCI_STATUS_REC_MASTER_ABORT != 0 {
        printk(KERN_DEBUG, "PCI: master abort, pc=0x%08lx\n", instruction_pointer(get_irq_regs()));
        cmd |= PCI_STATUS_REC_MASTER_ABORT << 16;
    }
    if status & PCI_STATUS_REC_TARGET_ABORT != 0 {
        printk(KERN_DEBUG, "PCI: target abort: ");
        pcibios_report_status(PCI_STATUS_REC_MASTER_ABORT | PCI_STATUS_SIG_TARGET_ABORT |
                              PCI_STATUS_REC_TARGET_ABORT, 1);
        printk("\n");
        cmd |= PCI_STATUS_REC_TARGET_ABORT << 16;
    }
    core::ptr::write_volatile(CSR_PCICMD, cmd);
    IRQ_HANDLED
}

unsafe fn dc21285_serr_irq(irq: i32, dev_id: *mut core::ffi::c_void) -> irqreturn_t {
    let timer = dev_id as *mut timer_list;
    printk(KERN_DEBUG, "PCI: system error received: ");
    pcibios_report_status(PCI_STATUS_SIG_SYSTEM_ERROR, 1);
    printk("\n");
    let cntl = core::ptr::read_volatile(CSR_SA110_CNTL) & 0xffffdf07;
    core::ptr::write_volatile(CSR_SA110_CNTL, cntl | SA110_CNTL_RXSERR);
    disable_irq(irq); (*timer).expires = jiffies + HZ; add_timer(timer); IRQ_HANDLED
}

unsafe fn dc21285_discard_irq(_irq: i32, _dev_id: *mut core::ffi::c_void) -> irqreturn_t {
    printk(KERN_DEBUG, "PCI: discard timer expired\n");
    let p = core::ptr::read_volatile(CSR_SA110_CNTL); core::ptr::write_volatile(CSR_SA110_CNTL, p & 0xffffde07); IRQ_HANDLED
}

unsafe fn dc21285_dparity_irq(_irq: i32, _dev_id: *mut core::ffi::c_void) -> irqreturn_t {
    printk(KERN_DEBUG, "PCI: data parity error detected: "); pcibios_report_status(PCI_STATUS_PARITY | PCI_STATUS_DETECTED_PARITY, 1); printk("\n");
    let cmd = core::ptr::read_volatile(CSR_PCICMD) & 0xffff; core::ptr::write_volatile(CSR_PCICMD, cmd | (1 << 24)); IRQ_HANDLED
}

unsafe fn dc21285_parity_irq(irq: i32, dev_id: *mut core::ffi::c_void) -> irqreturn_t {
    let timer = dev_id as *mut timer_list;
    printk(KERN_DEBUG, "PCI: parity error detected: "); pcibios_report_status(PCI_STATUS_PARITY | PCI_STATUS_DETECTED_PARITY, 1); printk("\n");
    let cmd = core::ptr::read_volatile(CSR_PCICMD) & 0xffff; core::ptr::write_volatile(CSR_PCICMD, cmd | (1 << 31));
    disable_irq(irq); (*timer).expires = jiffies + HZ; add_timer(timer); IRQ_HANDLED
}

unsafe fn dc21285_pci_bus_notifier(_nb: *mut notifier_block, action: usize, data: *mut core::ffi::c_void) -> i32 {
    if action != BUS_NOTIFY_ADD_DEVICE { return NOTIFY_DONE; }
    dma_direct_set_offset(data, PHYS_OFFSET, BUS_OFFSET, SZ_256M); NOTIFY_OK
}

static mut dc21285_pci_bus_nb: notifier_block = notifier_block { notifier_call: Some(dc21285_pci_bus_notifier) };

unsafe fn dc21285_setup(_nr: i32, sys: *mut pci_sys_data) -> i32 {
    let res = kzalloc_objs::<resource>(2); if res.is_null() { printk("out of memory for root bus resources"); return 0; }
    (*res).flags = IORESOURCE_MEM; (*res).name = "Footbridge non-prefetch";
    (*res.add(1)).flags = IORESOURCE_MEM | IORESOURCE_PREFETCH; (*res.add(1)).name = "Footbridge prefetch";
    allocate_resource(&mut iomem_resource, res.add(1), 0x20000000, 0xa0000000, 0xffffffff, 0x20000000, None, None);
    allocate_resource(&mut iomem_resource, res, 0x40000000, 0x80000000, 0xffffffff, 0x40000000, None, None);
    (*sys).mem_offset = DC21285_PCI_MEM;
    pci_add_resource_offset(&mut (*sys).resources, res, (*sys).mem_offset); pci_add_resource_offset(&mut (*sys).resources, res.add(1), (*sys).mem_offset);
    bus_register_notifier(&pci_bus_type, &mut dc21285_pci_bus_nb); 1
}

unsafe fn dc21285_preinit() {
    pcibios_min_mem = 0x81000000;
    let mem_size = high_memory as usize - PAGE_OFFSET; let mut mem_mask = 0x00100000;
    while mem_mask < 0x10000000 { if mem_mask >= mem_size { break; } mem_mask <<= 1; }
    core::ptr::write_volatile(CSR_SDRAMBASEMASK, (mem_mask - 1) & 0x0ffc0000); core::ptr::write_volatile(CSR_SDRAMBASEOFFSET, 0); core::ptr::write_volatile(CSR_ROMBASEMASK, 0x80000000); core::ptr::write_volatile(CSR_CSRBASEMASK, 0); core::ptr::write_volatile(CSR_CSRBASEOFFSET, 0); core::ptr::write_volatile(CSR_PCIADDR_EXTN, 0);
    printk(KERN_INFO, "PCI: DC21285 footbridge, revision %02lX, in central function mode\n", core::ptr::read_volatile(CSR_CLASSREV) & 0xff);
    let c = core::ptr::read_volatile(CSR_SA110_CNTL); core::ptr::write_volatile(CSR_SA110_CNTL, (c & 0xffffde07) | SA110_CNTL_RXSERR); let c = core::ptr::read_volatile(CSR_PCICMD); core::ptr::write_volatile(CSR_PCICMD, (c & 0xffff) | PCICMD_ERROR_BITS);
    timer_setup(&mut serr_timer, dc21285_enable_error, 0); timer_setup(&mut perr_timer, dc21285_enable_error, 0);
    request_irq(IRQ_PCI_SERR, dc21285_serr_irq, 0, "PCI system error", &mut serr_timer); request_irq(IRQ_PCI_PERR, dc21285_parity_irq, 0, "PCI parity error", &mut perr_timer); request_irq(IRQ_PCI_ABORT, dc21285_abort_irq, 0, "PCI abort", core::ptr::null_mut()); request_irq(IRQ_DISCARD_TIMER, dc21285_discard_irq, 0, "Discard timer", core::ptr::null_mut()); request_irq(IRQ_PCI_DPERR, dc21285_dparity_irq, 0, "PCI data parity", core::ptr::null_mut());
    core::ptr::write_volatile(CSR_PCICSRBASE, 0xf4000000); core::ptr::write_volatile(CSR_PCICSRIOBASE, 0); core::ptr::write_volatile(CSR_PCISDRAMBASE, BUS_OFFSET); core::ptr::write_volatile(CSR_PCIROMBASE, 0); core::ptr::write_volatile(CSR_PCICMD, PCI_COMMAND_MEMORY | PCI_COMMAND_MASTER | PCI_COMMAND_INVALIDATE | PCICMD_ERROR_BITS);
}

unsafe fn dc21285_postinit() { register_isa_ports(DC21285_PCI_MEM, DC21285_PCI_IO, 0); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
