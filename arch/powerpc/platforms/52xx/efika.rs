/*
 * Efika 5K2 platform code
 * Some code really inspired from the lite5200b platform.
 *
 * Copyright (C) 2006 bplan GmbH
 *
 * This file is licensed under the terms of the GNU General Public License
 * version 2. This program is licensed "as is" without any warranty of any
 * kind, whether express or implied.
 */

// Dependencies supplied by the surrounding kernel port are intentionally not
// redefined here.

const EFIKA_PLATFORM_NAME: &str = "Efika";

#[cfg(feature = "CONFIG_PCI")]
unsafe fn rtas_read_config(
    bus: *mut pci_bus,
    devfn: u32,
    offset: i32,
    len: i32,
    val: *mut u32,
) -> i32 {
    let hose = pci_bus_to_host(bus);
    let addr: usize = ((offset & 0xff) as usize)
        | (((devfn & 0xff) as usize) << 8)
        | ((((*bus).number.wrapping_sub((*hose).first_busno) & 0xff) as usize) << 16)
        | (((*hose).global_number as usize) << 24);
    let mut ret: i32 = -1;
    let rval = rtas_call(
        rtas_function_token(RTAS_FN_READ_PCI_CONFIG),
        2,
        2,
        &mut ret as *mut i32,
        addr,
        len,
    );
    *val = ret as u32;
    if rval != 0 { PCIBIOS_DEVICE_NOT_FOUND } else { PCIBIOS_SUCCESSFUL }
}

#[cfg(feature = "CONFIG_PCI")]
unsafe fn rtas_write_config(
    bus: *mut pci_bus,
    devfn: u32,
    offset: i32,
    len: i32,
    val: u32,
) -> i32 {
    let hose = pci_bus_to_host(bus);
    let addr: usize = ((offset & 0xff) as usize)
        | (((devfn & 0xff) as usize) << 8)
        | ((((*bus).number.wrapping_sub((*hose).first_busno) & 0xff) as usize) << 16)
        | (((*hose).global_number as usize) << 24);
    let rval = rtas_call(
        rtas_function_token(RTAS_FN_WRITE_PCI_CONFIG),
        3,
        1,
        core::ptr::null_mut(),
        addr,
        len,
        val,
    );
    if rval != 0 { PCIBIOS_DEVICE_NOT_FOUND } else { PCIBIOS_SUCCESSFUL }
}

#[cfg(feature = "CONFIG_PCI")]
static mut RTAS_PCI_OPS: pci_ops = pci_ops {
    read: Some(rtas_read_config),
    write: Some(rtas_write_config),
};

#[cfg(feature = "CONFIG_PCI")]
unsafe fn efika_pcisetup() {
    let mut bus_range: *const i32;
    let mut len: i32 = 0;
    let mut hose: *mut pci_controller;
    let root = of_find_node_by_path(b"/\0".as_ptr() as *const i8);
    if root.is_null() {
        printk(KERN_WARNING, b"Efika: Unable to find the root node\n\0".as_ptr());
        return;
    }

    let mut pcictrl = core::ptr::null_mut();
    for_each_child_of_node(root, child => {
        if of_node_name_eq(child, b"pci\0".as_ptr() as *const i8) { pcictrl = child; }
    });
    of_node_put(root);

    if pcictrl.is_null() {
        printk(KERN_WARNING, b"Efika: Unable to find the PCI bridge node\n\0".as_ptr());
        return;
    }
    bus_range = of_get_property(pcictrl, b"bus-range\0".as_ptr() as *const i8, &mut len);
    if bus_range.is_null() || len < 2 * core::mem::size_of::<i32>() as i32 {
        printk(KERN_WARNING, b"Efika: Can't get bus-range for %pOF\n\0".as_ptr(), pcictrl);
        of_node_put(pcictrl);
        return;
    }
    if *bus_range.add(1) == *bus_range {
        printk(KERN_INFO, b"Efika: PCI bus %d\0".as_ptr(), *bus_range);
    } else {
        printk(KERN_INFO, b"Efika: PCI buses %d..%d\0".as_ptr(), *bus_range, *bus_range.add(1));
    }
    printk(b" controlled by %pOF\n\0".as_ptr(), pcictrl);
    printk(b"\n\0".as_ptr());
    hose = pcibios_alloc_controller(pcictrl);
    if hose.is_null() {
        printk(KERN_WARNING, b"Efika: Can't allocate PCI controller structure for %pOF\n\0".as_ptr(), pcictrl);
        of_node_put(pcictrl);
        return;
    }
    (*hose).first_busno = *bus_range;
    (*hose).last_busno = *bus_range.add(1);
    (*hose).ops = &raw mut RTAS_PCI_OPS;
    pci_process_bridge_OF_ranges(hose, pcictrl, 0);
}

#[cfg(not(feature = "CONFIG_PCI"))]
unsafe fn efika_pcisetup() {}

unsafe fn efika_show_cpuinfo(m: *mut seq_file) {
    let root = of_find_node_by_path(b"/\0".as_ptr() as *const i8);
    if root.is_null() { return; }
    let revision = of_get_property(root, b"revision\0".as_ptr() as *const i8, core::ptr::null_mut());
    let description = of_get_property(root, b"CODEGEN,description\0".as_ptr() as *const i8, core::ptr::null_mut());
    let vendor = of_get_property(root, b"CODEGEN,vendor\0".as_ptr() as *const i8, core::ptr::null_mut());
    if !description.is_null() { seq_printf(m, b"machine\t\t: %s\n\0".as_ptr(), description); }
    else { seq_printf(m, b"machine\t\t: Efika\n\0".as_ptr()); }
    if !revision.is_null() { seq_printf(m, b"revision\t: %s\n\0".as_ptr(), revision); }
    if !vendor.is_null() { seq_printf(m, b"vendor\t\t: %s\n\0".as_ptr(), vendor); }
    of_node_put(root);
}

#[cfg(feature = "CONFIG_PM")]
unsafe fn efika_suspend_prepare(_mbar: *mut core::ffi::c_void) {
    let pin: u8 = 4; // GPIO_WKUP_4 (GPIO_PSC6_0 - IRDA_RX)
    let level: u8 = 1; // wakeup on high level
    // IOW. to wake it up, short pins 1 and 3 on IRDA connector
    mpc52xx_set_wakeup_gpio(pin, level);
}

unsafe fn efika_setup_arch() {
    rtas_initialize();
    mpc52xx_map_common_devices();
    #[cfg(feature = "CONFIG_PM")]
    {
        mpc52xx_suspend.board_suspend_prepare = Some(efika_suspend_prepare);
        mpc52xx_pm_init();
    }
    if !ppc_md.progress.is_none() {
        ppc_md.progress.unwrap()(b"Linux/PPC " UTS_RELEASE b" running on Efika ;-)\n\0".as_ptr(), 0x0);
    }
}

unsafe fn efika_probe() -> i32 {
    let root = of_find_node_by_path(b"/\0".as_ptr() as *const i8);
    let model = of_get_property(root, b"model\0".as_ptr() as *const i8, core::ptr::null_mut());
    of_node_put(root);
    if model.is_null() || strcmp(model, b"EFIKA5K2\0".as_ptr() as *const i8) != 0 { return 0; }
    DMA_MODE_READ = 0x44;
    DMA_MODE_WRITE = 0x48;
    pm_power_off = Some(rtas_power_off);
    1
}

// Equivalent machine descriptor registration; external kernel macros/types provide the descriptor.
define_machine!(efika {
    name: EFIKA_PLATFORM_NAME,
    probe: efika_probe,
    setup_arch: efika_setup_arch,
    discover_phbs: efika_pcisetup,
    init: mpc52xx_declare_of_platform_devices,
    show_cpuinfo: efika_show_cpuinfo,
    init_IRQ: mpc52xx_init_irq,
    get_irq: mpc52xx_get_irq,
    restart: rtas_restart,
    halt: rtas_halt,
    set_rtc_time: rtas_set_rtc_time,
    get_rtc_time: rtas_get_rtc_time,
    progress: rtas_progress,
    get_boot_time: rtas_get_boot_time,
    #[cfg(feature = "CONFIG_PCI")]
    phys_mem_access_prot: pci_phys_mem_access_prot,
});

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
