// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * AmigaOne platform setup
 *
 * Copyright 2008 Gerhard Pircher (gerhard_pircher@gmx.net)
 *
 *   Based on original amigaone_setup.c source code
 * Copyright 2003 by Hans-Joerg Frieden and Thomas Frieden
 */

// Linux and PowerPC kernel dependencies supplied by other translation units.

extern "C" {
    fn __flush_disable_L1();
}

unsafe fn amigaone_show_cpuinfo(m: *mut seq_file) {
    seq_printf(m, c"vendor\t\t: Eyetech Ltd.\n".as_ptr());
}

unsafe fn amigaone_add_bridge(dev: *mut device_node) -> c_int {
    let cfg_addr: *const u32;
    let cfg_data: *const u32;
    let mut len: c_int = 0;
    let bus_range: *const c_int;
    let hose: *mut pci_controller;

    printk(KERN_INFO, c"Adding PCI host bridge %pOF\n".as_ptr(), dev);

    cfg_addr = of_get_address(dev, 0, core::ptr::null_mut(), core::ptr::null_mut());
    cfg_data = of_get_address(dev, 1, core::ptr::null_mut(), core::ptr::null_mut());
    if cfg_addr.is_null() || cfg_data.is_null() {
        return -ENODEV;
    }

    bus_range = of_get_property(dev, c"bus-range\0".as_ptr(), &mut len);
    if bus_range.is_null() || len < 2 * core::mem::size_of::<c_int>() as c_int {
        printk(KERN_WARNING, c"Can't get bus-range for %pOF, assume bus 0\n".as_ptr(), dev);
    }

    hose = pcibios_alloc_controller(dev);
    if hose.is_null() {
        return -ENOMEM;
    }

    (*hose).first_busno = if !bus_range.is_null() { *bus_range } else { 0 };
    (*hose).last_busno = if !bus_range.is_null() { *bus_range.add(1) } else { 0xff };

    setup_indirect_pci(hose, *cfg_addr, *cfg_data, 0);

    /* Interpret the "ranges" property */
    /* This also maps the I/O region and sets isa_io/mem_base */
    pci_process_bridge_OF_ranges(hose, dev, 1);

    0
}

unsafe fn amigaone_setup_arch() {
    if !ppc_md.progress.is_none() {
        ppc_md.progress.unwrap()(c"Linux/PPC " UTS_RELEASE c"\n".as_ptr(), 0);
    }
}

unsafe fn amigaone_discover_phbs() {
    let mut np: *mut device_node;
    let mut phb: c_int = -ENODEV;

    /* Lookup PCI host bridges. */
    for_each_compatible_node!(np, c"pci\0", c"mai-logic,articia-s\0") {
        phb = amigaone_add_bridge(np);
    }

    BUG_ON(phb != 0);
}

unsafe fn amigaone_init_IRQ() {
    let pic: *mut device_node;
    let mut np: *mut device_node = core::ptr::null_mut();
    let mut prop: *const c_ulong = core::ptr::null();
    let mut int_ack: c_ulong = 0;

    /* Search for ISA interrupt controller. */
    pic = of_find_compatible_node(core::ptr::null_mut(), c"interrupt-controller\0".as_ptr(), c"pnpPNP,000\0".as_ptr());
    BUG_ON(pic.is_null());

    /* Look for interrupt acknowledge address in the PCI root node. */
    np = of_find_compatible_node(core::ptr::null_mut(), c"pci\0".as_ptr(), c"mai-logic,articia-s\0".as_ptr());
    if !np.is_null() {
        prop = of_get_property(np, c"8259-interrupt-acknowledge\0".as_ptr(), core::ptr::null_mut());
        if !prop.is_null() {
            int_ack = *prop;
        }
        of_node_put(np);
    }

    if int_ack == 0 {
        printk(KERN_WARNING, c"Cannot find PCI interrupt acknowledge address, polling\n".as_ptr());
    }

    i8259_init(pic, int_ack);
    ppc_md.get_irq = Some(i8259_irq);
    irq_set_default_domain(i8259_get_host());
}

unsafe fn request_isa_regions() -> c_int {
    request_region(0x00, 0x20, c"dma1\0".as_ptr());
    request_region(0x40, 0x20, c"timer\0".as_ptr());
    request_region(0x80, 0x10, c"dma page reg\0".as_ptr());
    request_region(0xc0, 0x20, c"dma2\0".as_ptr());

    0
}

machine_device_initcall!(amigaone, request_isa_regions);

unsafe fn amigaone_restart(_cmd: *mut c_char) -> ! {
    local_irq_disable();

    /* Flush and disable caches. */
    __flush_disable_L1();

    /* Set SRR0 to the reset vector and turn on MSR_IP. */
    mtspr(SPRN_SRR0, 0xfff00100);
    mtspr(SPRN_SRR1, MSR_IP);

    /* Do an rfi to jump back to firmware. */
    core::arch::asm!("rfi", options(nostack, preserves_flags));

    /* Not reached. */
    loop {}
}

unsafe fn amigaone_probe() -> c_int {
    /*
     * Coherent memory access cause complete system lockup! Thus
     * disable this CPU feature, even if the CPU needs it.
     */
    (*cur_cpu_spec).cpu_features &= !CPU_FTR_NEED_COHERENT;

    DMA_MODE_READ = 0x44;
    DMA_MODE_WRITE = 0x48;

    1
}

define_machine!(amigaone {
    .name = c"AmigaOne\0".as_ptr(),
    .compatible = c"eyetech,amigaone\0".as_ptr(),
    .probe = Some(amigaone_probe),
    .setup_arch = Some(amigaone_setup_arch),
    .discover_phbs = Some(amigaone_discover_phbs),
    .show_cpuinfo = Some(amigaone_show_cpuinfo),
    .init_IRQ = Some(amigaone_init_IRQ),
    .restart = Some(amigaone_restart),
    .progress = Some(udbg_progress),
});

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
