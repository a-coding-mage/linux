// SPDX-License-Identifier: GPL-2.0
/*
 * linux/arch/alpha/kernel/sys_nautilus.c
 *
 * Code supporting NAUTILUS systems.
 *
 * NAUTILUS has the following I/O features:
 * a) AMD 751 aka IRONGATE northbridge: 4 PCI slots, 1 AGP slot
 * b) ALI M1543C southbridge: 2 ISA slots, 2 IDE connectors, FDD,
 *    2 serial ports, parallel port, and 2 USB ports.
 */

// C kernel includes and local headers are supplied by the surrounding build.

unsafe fn nautilus_init_irq() {
    if alpha_using_srm {
        alpha_mv.device_interrupt = Some(srm_device_interrupt);
    }
    init_i8259a_irqs();
    common_init_isa_dma();
}

unsafe fn nautilus_map_irq(dev: *const pci_dev, slot: u8, pin: u8) -> i32 {
    /* Preserve the IRQ set up by the console. */
    let mut irq: u8 = 0;
    /* UP1500: AGP INTA is actually routed to IRQ 5, not IRQ 10. */
    if slot == 1 && pin == 2 && !(*(*dev).bus).self_.is_null()
        && (*(*(*dev).bus).self_).device == 0x700f
    {
        return 5;
    }
    pci_read_config_byte(dev, PCI_INTERRUPT_LINE, &mut irq);
    irq as i32
}

unsafe fn nautilus_kill_arch(mode: i32) {
    let bus = (*pci_isa_hose).bus;
    let mut pmuport: u32 = 0;
    let mut off: i32;

    match mode {
        LINUX_REBOOT_CMD_RESTART => {
            if !alpha_using_srm {
                let mut t8: u8 = 0;
                pci_bus_read_config_byte(bus, 0x38, 0x43, &mut t8);
                pci_bus_write_config_byte(bus, 0x38, 0x43, t8 | 0x80);
                outb(1, 0x92);
                outb(0, 0x92);
                // NOTREACHED
            }
        }
        LINUX_REBOOT_CMD_POWER_OFF => {
            // Assume M1543C; SLP_TYPE = 0, SLP_EN = 1.
            off = 0x2000;
            pci_bus_read_config_dword(bus, 0x88, 0x10, &mut pmuport);
            if pmuport == 0 {
                // M1535D/D+: SLP_TYPE = 5, SLP_EN = 1.
                off = 0x3400;
                pci_bus_read_config_dword(bus, 0x88, 0xe0, &mut pmuport);
            }
            pmuport &= 0xfffe;
            outw(0xffff, pmuport); // Clear pending events.
            outw(off as u16, pmuport + 4);
            // NOTREACHED
        }
        _ => {}
    }
}

unsafe fn naut_sys_machine_check(_vector: c_ulong, _la_ptr: c_ulong, regs: *mut pt_regs) {
    printk!("PC %lx RA %lx\n", (*regs).pc, (*regs).r26);
    irongate_pci_clr_err();
}

unsafe fn nautilus_machine_check(vector: c_ulong, la_ptr: c_ulong) {
    let mchk_class: *const c_char;

    if vector == SCB_Q_SYSMCHK && ((*IRONGATE0).dramms & 0x300) == 0x300 {
        let mut nmi_ctl = inb(0x61) as c_ulong;
        nmi_ctl |= 0x0c;
        outb(nmi_ctl as u8, 0x61);
        nmi_ctl &= !0x0c;
        outb(nmi_ctl as u8, 0x61);

        (*IRONGATE0).stat_cmd = (*IRONGATE0).stat_cmd & !0x100;
        mb();
        let _ = (*IRONGATE0).stat_cmd;
        (*IRONGATE0).dramms = (*IRONGATE0).dramms;
        mb();
        let _ = (*IRONGATE0).dramms;
        draina();
        wrmces(0x7);
        mb();
        return;
    }

    if vector == SCB_Q_SYSERR {
        mchk_class = c"Correctable".as_ptr();
    } else if vector == SCB_Q_SYSMCHK {
        mchk_class = c"Fatal".as_ptr();
    } else {
        ev6_machine_check(vector, la_ptr);
        return;
    }

    printk!(KERN_CRIT "NAUTILUS Machine check 0x%lx [%s System Machine Check (NMI)]\n",
            vector, mchk_class);
    naut_sys_machine_check(vector, la_ptr, get_irq_regs());
    draina();
    wrmces(0x7);
    mb();
}

static mut irongate_mem: resource = resource {
    name: c"Irongate PCI MEM".as_ptr(), flags: IORESOURCE_MEM, ..resource::zeroed()
};
static mut busn_resource: resource = resource {
    name: c"PCI busn".as_ptr(), start: 0, end: 255, flags: IORESOURCE_BUS,
    ..resource::zeroed()
};

unsafe fn nautilus_init_pci() {
    let hose = hose_head;
    let mut bridge: *mut pci_host_bridge = pci_alloc_host_bridge(0);
    let mut bus: *mut pci_bus;
    let (mut bus_align, bus_size, mut pci_mem): (c_ulong, c_ulong, c_ulong);
    let mut memtop = max_low_pfn << PAGE_SHIFT;

    if bridge.is_null() { return; }
    pci_add_resource(&mut (*bridge).windows, &mut ioport_resource);
    pci_add_resource(&mut (*bridge).windows, &mut irongate_mem);
    pci_add_resource(&mut (*bridge).windows, &mut busn_resource);
    (*bridge).dev.parent = core::ptr::null_mut();
    (*bridge).sysdata = hose as *mut c_void;
    (*bridge).busnr = 0;
    (*bridge).ops = alpha_mv.pci_ops;
    (*bridge).swizzle_irq = alpha_mv.pci_swizzle;
    (*bridge).map_irq = alpha_mv.pci_map_irq;
    (*bridge).size_windows = 1;
    if pci_scan_root_bus_bridge(bridge) != 0 {
        pci_free_host_bridge(bridge);
        return;
    }
    bus = (*hose).bus = (*bridge).bus;
    pcibios_claim_one_bus(bus);
    pci_bus_size_bridges(bus);
    bus_align = irongate_mem.start;
    bus_size = irongate_mem.end + 1 - bus_align;
    if bus_align < 0x1000000 { bus_align = 0x1000000; }
    pci_mem = (0x100000000u64 - bus_size as u64) as c_ulong & bus_align.wrapping_neg();
    irongate_mem.start = pci_mem;
    irongate_mem.end = 0xffffffff;
    if request_resource(&mut iomem_resource, &mut irongate_mem) < 0 {
        printk!(KERN_ERR "Failed to request MEM on hose 0\n");
    }
    printk!(KERN_INFO "Irongate pci_mem %pR\n", &irongate_mem);
    if pci_mem < memtop { memtop = pci_mem; }
    if memtop > alpha_mv.min_mem_address {
        free_reserved_area(__va(alpha_mv.min_mem_address), __va(memtop), -1, core::ptr::null());
        printk!(KERN_INFO "nautilus_init_pci: %ldk freed\n",
                (memtop - alpha_mv.min_mem_address) >> 10);
    }
    if ((*IRONGATE0).dev_vendor >> 16) > 0x7006 { (*IRONGATE0).pci_mem = pci_mem; }
    pci_bus_assign_resources(bus);
    pci_bus_add_devices(bus);
}

/* The System Vectors. */
#[no_mangle]
pub static mut nautilus_mv: alpha_machine_vector = alpha_machine_vector {
    vector_name: c"Nautilus".as_ptr(),
    // DO_EV6_MMU, DO_DEFAULT_RTC, and DO_IRONGATE_IO expand to their
    // architecture-specific machine-vector fields in the kernel build.
    machine_check: Some(nautilus_machine_check),
    max_isa_dma_address: ALPHA_MAX_ISA_DMA_ADDRESS,
    min_io_address: DEFAULT_IO_BASE,
    min_mem_address: IRONGATE_DEFAULT_MEM_BASE,
    nr_irqs: 16,
    device_interrupt: Some(isa_device_interrupt),
    init_arch: Some(irongate_init_arch),
    init_irq: Some(nautilus_init_irq),
    init_rtc: Some(common_init_rtc),
    init_pci: Some(nautilus_init_pci),
    kill_arch: Some(nautilus_kill_arch),
    pci_map_irq: Some(nautilus_map_irq),
    pci_swizzle: Some(common_swizzle),
    ..alpha_machine_vector::zeroed()
};

ALIAS_MV!(nautilus);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
