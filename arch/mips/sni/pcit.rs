/*
 * PCI Tower specific code
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2006 Thomas Bogendoerfer (tsbogend@alpha.franken.de)
 */

// Linux and architecture dependencies supplied by other translation units.

macro_rules! PORT {
    ($base:expr, $irq:expr) => {
        plat_serial8250_port {
            iobase: $base,
            irq: $irq,
            uartclk: 1_843_200,
            iotype: UPIO_PORT,
            flags: UPF_BOOT_AUTOCONF,
            ..unsafe { core::mem::zeroed() }
        }
    };
}

static mut pcit_data: [plat_serial8250_port; 3] = [
    PORT!(0x3f8, 0),
    PORT!(0x2f8, 3),
    unsafe { core::mem::zeroed() },
];

static mut pcit_serial8250_device: platform_device = platform_device {
    name: "serial8250",
    id: PLAT8250_DEV_PLATFORM,
    dev: device {
        platform_data: unsafe { &mut pcit_data as *mut _ as *mut core::ffi::c_void },
        ..unsafe { core::mem::zeroed() }
    },
    ..unsafe { core::mem::zeroed() }
};

static mut pcit_cplus_data: [plat_serial8250_port; 5] = [
    PORT!(0x3f8, 0),
    PORT!(0x2f8, 3),
    PORT!(0x3e8, 4),
    PORT!(0x2e8, 3),
    unsafe { core::mem::zeroed() },
];

static mut pcit_cplus_serial8250_device: platform_device = platform_device {
    name: "serial8250",
    id: PLAT8250_DEV_PLATFORM,
    dev: device {
        platform_data: unsafe { &mut pcit_cplus_data as *mut _ as *mut core::ffi::c_void },
        ..unsafe { core::mem::zeroed() }
    },
    ..unsafe { core::mem::zeroed() }
};

static mut pcit_cmos_rsrc: [resource; 2] = [
    resource { start: 0x70, end: 0x71, flags: IORESOURCE_IO, ..unsafe { core::mem::zeroed() } },
    resource { start: 8, end: 8, flags: IORESOURCE_IRQ, ..unsafe { core::mem::zeroed() } },
];

static mut pcit_cmos_device: platform_device = platform_device {
    name: "rtc_cmos",
    num_resources: 2,
    resource: unsafe { pcit_cmos_rsrc.as_mut_ptr() },
    ..unsafe { core::mem::zeroed() }
};

static mut pcit_pcspeaker_pdev: platform_device = platform_device {
    name: "pcspkr",
    id: -1,
    ..unsafe { core::mem::zeroed() }
};

static mut sni_io_resource: resource = resource {
    start: 0x00000000,
    end: 0x03bfffff,
    name: "PCIT IO",
    flags: IORESOURCE_IO,
    ..unsafe { core::mem::zeroed() }
};

static mut pcit_io_resources: [resource; 7] = [
    resource { start: 0x00, end: 0x1f, name: "dma1", flags: IORESOURCE_BUSY, ..unsafe { core::mem::zeroed() } },
    resource { start: 0x40, end: 0x5f, name: "timer", flags: IORESOURCE_BUSY, ..unsafe { core::mem::zeroed() } },
    resource { start: 0x60, end: 0x6f, name: "keyboard", flags: IORESOURCE_BUSY, ..unsafe { core::mem::zeroed() } },
    resource { start: 0x80, end: 0x8f, name: "dma page reg", flags: IORESOURCE_BUSY, ..unsafe { core::mem::zeroed() } },
    resource { start: 0xc0, end: 0xdf, name: "dma2", flags: IORESOURCE_BUSY, ..unsafe { core::mem::zeroed() } },
    resource { start: 0xcf8, end: 0xcfb, name: "PCI config addr", flags: IORESOURCE_BUSY, ..unsafe { core::mem::zeroed() } },
    resource { start: 0xcfc, end: 0xcff, name: "PCI config data", flags: IORESOURCE_BUSY, ..unsafe { core::mem::zeroed() } },
];

unsafe fn sni_pcit_resource_init() {
    for i in 0..pcit_io_resources.len() {
        request_resource(&mut sni_io_resource, pcit_io_resources.as_mut_ptr().add(i));
    }
}

extern "C" {
    static mut sni_pcit_ops: pci_ops;
}

// CONFIG_PCI conditional retained from the C source.
#[cfg(CONFIG_PCI)]
static mut sni_mem_resource: resource = resource {
    start: 0x18000000,
    end: 0x1fbfffff,
    name: "PCIT PCI MEM",
    flags: IORESOURCE_MEM,
    ..unsafe { core::mem::zeroed() }
};

#[cfg(CONFIG_PCI)]
static mut sni_pcit_controller: pci_controller = pci_controller {
    pci_ops: unsafe { &mut sni_pcit_ops },
    mem_resource: unsafe { &mut sni_mem_resource },
    mem_offset: 0,
    io_resource: unsafe { &mut sni_io_resource },
    io_offset: 0,
    io_map_base: SNI_PORT_BASE,
    ..unsafe { core::mem::zeroed() }
};

unsafe fn enable_pcit_irq(d: *mut irq_data) {
    let mask: u32 = 1u32 << ((*d).irq - SNI_PCIT_INT_START + 24);
    let reg = SNI_PCIT_INT_REG as *mut u32;
    core::ptr::write_volatile(reg, core::ptr::read_volatile(reg) | mask);
}

pub unsafe extern "C" fn disable_pcit_irq(d: *mut irq_data) {
    let mask: u32 = 1u32 << ((*d).irq - SNI_PCIT_INT_START + 24);
    let reg = SNI_PCIT_INT_REG as *mut u32;
    core::ptr::write_volatile(reg, core::ptr::read_volatile(reg) & !mask);
}

static mut pcit_irq_type: irq_chip = irq_chip {
    name: "PCIT",
    irq_mask: Some(disable_pcit_irq),
    irq_unmask: Some(enable_pcit_irq),
    ..unsafe { core::mem::zeroed() }
};

unsafe fn pcit_hwint1() {
    let pending = core::ptr::read_volatile(SNI_PCIT_INT_REG as *const u32);
    clear_c0_status(IE_IRQ1);
    let irq = ffs((pending >> 16) & 0x7f);
    if irq > 0 { do_IRQ(irq + SNI_PCIT_INT_START - 1); }
    set_c0_status(IE_IRQ1);
}

unsafe fn pcit_hwint0() {
    let pending = core::ptr::read_volatile(SNI_PCIT_INT_REG as *const u32);
    clear_c0_status(IE_IRQ0);
    let irq = ffs((pending >> 16) & 0x3f);
    if irq > 0 { do_IRQ(irq + SNI_PCIT_INT_START - 1); }
    set_c0_status(IE_IRQ0);
}

unsafe fn sni_pcit_hwint() {
    let pending = read_c0_cause() & read_c0_status();
    if pending & C_IRQ1 != 0 { pcit_hwint1(); }
    else if pending & C_IRQ2 != 0 { do_IRQ(MIPS_CPU_IRQ_BASE + 4); }
    else if pending & C_IRQ3 != 0 { do_IRQ(MIPS_CPU_IRQ_BASE + 5); }
    else if pending & C_IRQ5 != 0 { do_IRQ(MIPS_CPU_IRQ_BASE + 7); }
}

unsafe fn sni_pcit_hwint_cplus() {
    let pending = read_c0_cause() & read_c0_status();
    if pending & C_IRQ0 != 0 { pcit_hwint0(); }
    else if pending & C_IRQ1 != 0 { do_IRQ(MIPS_CPU_IRQ_BASE + 3); }
    else if pending & C_IRQ2 != 0 { do_IRQ(MIPS_CPU_IRQ_BASE + 4); }
    else if pending & C_IRQ3 != 0 { do_IRQ(MIPS_CPU_IRQ_BASE + 5); }
    else if pending & C_IRQ5 != 0 { do_IRQ(MIPS_CPU_IRQ_BASE + 7); }
}

pub unsafe extern "C" fn sni_pcit_irq_init() {
    mips_cpu_irq_init();
    for i in SNI_PCIT_INT_START..=SNI_PCIT_INT_END {
        irq_set_chip_and_handler(i, &mut pcit_irq_type, handle_level_irq);
    }
    core::ptr::write_volatile(SNI_PCIT_INT_REG as *mut u32, 0);
    sni_hwint = Some(sni_pcit_hwint);
    change_c0_status(ST0_IM, IE_IRQ1);
    if request_irq(SNI_PCIT_INT_START + 6, sni_isa_irq_handler, 0, "ISA", core::ptr::null_mut()) != 0 {
        pr_err!("Failed to register ISA interrupt\n");
    }
}

pub unsafe extern "C" fn sni_pcit_cplus_irq_init() {
    mips_cpu_irq_init();
    for i in SNI_PCIT_INT_START..=SNI_PCIT_INT_END {
        irq_set_chip_and_handler(i, &mut pcit_irq_type, handle_level_irq);
    }
    core::ptr::write_volatile(SNI_PCIT_INT_REG as *mut u32, 0x40000000);
    sni_hwint = Some(sni_pcit_hwint_cplus);
    change_c0_status(ST0_IM, IE_IRQ0);
    if request_irq(MIPS_CPU_IRQ_BASE + 3, sni_isa_irq_handler, 0, "ISA", core::ptr::null_mut()) != 0 {
        pr_err!("Failed to register ISA interrupt\n");
    }
}

pub unsafe extern "C" fn sni_pcit_init() {
    ioport_resource.end = sni_io_resource.end;
    // CONFIG_PCI conditional retained from the C source.
    #[cfg(CONFIG_PCI)] {
        PCIBIOS_MIN_IO = 0x9000;
        register_pci_controller(&mut sni_pcit_controller);
    }
    sni_pcit_resource_init();
}

unsafe fn snirm_pcit_setup_devinit() -> i32 {
    match sni_brd_type {
        SNI_BRD_PCI_TOWER => {
            platform_device_register(&mut pcit_serial8250_device);
            platform_device_register(&mut pcit_cmos_device);
            platform_device_register(&mut pcit_pcspeaker_pdev);
        }
        SNI_BRD_PCI_TOWER_CPLUS => {
            platform_device_register(&mut pcit_cplus_serial8250_device);
            platform_device_register(&mut pcit_cmos_device);
            platform_device_register(&mut pcit_pcspeaker_pdev);
        }
        _ => {}
    }
    0
}

device_initcall!(snirm_pcit_setup_devinit);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
