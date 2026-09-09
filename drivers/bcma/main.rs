/*
 * Broadcom specific AMBA
 * Bus subsystem
 *
 * Licensed under the GNU/GPL. See COPYING for details.
 */

/* Dependencies supplied by the surrounding kernel translation. */

static mut BCMA_BUS_NEXT_NUM: u32 = 0;

unsafe extern "C" {
    fn bcma_read32(core: *mut bcma_device, reg: u16) -> u32;
    fn bcma_warn(bus: *mut bcma_bus, fmt: *const u8, ...);
    fn bcma_debug(bus: *mut bcma_bus, fmt: *const u8, ...);
    fn bcma_info(bus: *mut bcma_bus, fmt: *const u8, ...);
    fn bcma_err(bus: *mut bcma_bus, fmt: *const u8, ...);
    fn bcma_detect_chip(bus: *mut bcma_bus);
    fn bcma_bus_scan(bus: *mut bcma_bus) -> i32;
    fn bcma_sprom_get(bus: *mut bcma_bus) -> i32;
    fn bcma_find_core(bus: *mut bcma_bus, id: u16) -> *mut bcma_device;
    fn bcma_core_mips_irq(core: *mut bcma_device) -> u32;
    fn bcma_core_chipcommon_early_init(drv: *mut bcma_drv_cc);
    fn bcma_core_chipcommon_init(drv: *mut bcma_drv_cc);
    fn bcma_core_chipcommon_b_init(drv: *mut bcma_drv_cc);
    fn bcma_core_chipcommon_b_free(drv: *mut bcma_drv_cc);
    fn bcma_core_mips_init(drv: *mut bcma_drv_mips);
    fn bcma_core_mips_early_init(drv: *mut bcma_drv_mips);
    fn bcma_core_pci_early_init(drv: *mut bcma_drv_pci);
    fn bcma_core_pci_init(drv: *mut bcma_drv_pci);
    fn bcma_core_pcie2_init(drv: *mut bcma_drv_pcie2);
    fn bcma_core_gmac_cmn_init(drv: *mut bcma_drv_gmac_cmn);
    fn bcma_gpio_init(drv: *mut bcma_drv_cc) -> i32;
    fn bcma_gpio_unregister(drv: *mut bcma_drv_cc) -> i32;
    fn bcma_chipco_watchdog_register(drv: *mut bcma_drv_cc) -> i32;
    fn bcma_host_soc_register_driver() -> i32;
    fn bcma_host_soc_unregister_driver();
    fn bcma_host_pci_init() -> i32;
    fn bcma_host_pci_exit();
}

/* The following items mirror kernel objects supplied by bcma_private.h. */
extern "C" {
    type bcma_bus;
    type bcma_device;
    type bcma_driver;
    type bcma_drv_cc;
    type bcma_drv_mips;
    type bcma_drv_pci;
    type bcma_drv_pcie2;
    type bcma_drv_gmac_cmn;
}

unsafe fn bcma_cc_core_id(bus: *mut bcma_bus) -> u16 {
    if (*bus).chipinfo.id == BCMA_CHIP_ID_BCM4706 { BCMA_CORE_4706_CHIPCOMMON } else { BCMA_CORE_CHIPCOMMON }
}

pub unsafe fn bcma_find_core_unit(bus: *mut bcma_bus, coreid: u16, unit: u8) -> *mut bcma_device {
    let mut core = (*bus).cores.next;
    while core != &mut (*bus).cores as *mut _ {
        if (*core).id.id == coreid && (*core).core_unit == unit { return core; }
        core = (*core).list.next;
    }
    core as *mut bcma_device
}

pub unsafe fn bcma_wait_value(core: *mut bcma_device, reg: u16, mask: u32, value: u32, timeout: i32) -> bool {
    let deadline = jiffies().wrapping_add(timeout as u64);
    loop {
        let val = bcma_read32(core, reg);
        if (val & mask) == value { return true; }
        cpu_relax();
        udelay(10);
        if time_after_eq(jiffies(), deadline) { break; }
    }
    bcma_warn((*core).bus, b"Timeout waiting for register 0x%04X!\0".as_ptr(), reg);
    false
}

unsafe fn bcma_release_core_dev(dev: *mut device) {
    let core = container_of_device(dev);
    if !(*core).io_addr.is_null() { iounmap((*core).io_addr); }
    if !(*core).io_wrap.is_null() { iounmap((*core).io_wrap); }
    kfree(core as *mut _);
}

unsafe fn bcma_is_core_needed_early(core_id: u16) -> bool {
    core_id == BCMA_CORE_NS_NAND || core_id == BCMA_CORE_NS_QSPI
}

pub unsafe fn bcma_core_irq(core: *mut bcma_device, num: i32) -> u32 {
    let bus = (*core).bus;
    match (*bus).hosttype {
        BCMA_HOSTTYPE_PCI => (*(*bus).host_pci).irq,
        BCMA_HOSTTYPE_SOC => {
            if !(*bus).drv_mips.core.is_null() && num == 0 { let irq = bcma_core_mips_irq(core); return if irq <= 4 { irq + 2 } else { 0 }; }
            if !(*bus).dev.is_null() { return bcma_of_get_irq((*bus).dev, core, num); }
            0
        }
        BCMA_HOSTTYPE_SDIO => 0,
        _ => 0,
    }
}

pub unsafe fn bcma_prepare_core(bus: *mut bcma_bus, core: *mut bcma_device) {
    device_initialize(&mut (*core).dev);
    (*core).dev.release = Some(bcma_release_core_dev);
    (*core).dev.bus = &bcma_bus_type;
    dev_set_name(&mut (*core).dev, b"bcma%d:%d\0".as_ptr(), (*bus).num, (*core).core_index);
    (*core).dev.parent = (*bus).dev;
    if !(*bus).dev.is_null() { bcma_of_fill_device((*bus).dev, core); }
    match (*bus).hosttype {
        BCMA_HOSTTYPE_PCI => { (*core).dma_dev = (*bus).dev; (*core).irq = (*(*bus).host_pci).irq; }
        BCMA_HOSTTYPE_SOC => { (*core).dma_dev = if !(*bus).dev.is_null() { (*bus).dev } else { &mut (*core).dev }; }
        BCMA_HOSTTYPE_SDIO | _ => {}
    }
}

pub unsafe fn bcma_init_bus(bus: *mut bcma_bus) {
    mutex_lock(&bcma_buses_mutex); (*bus).num = BCMA_BUS_NEXT_NUM; BCMA_BUS_NEXT_NUM += 1; mutex_unlock(&bcma_buses_mutex);
    INIT_LIST_HEAD(&mut (*bus).cores); (*bus).nr_cores = 0; bcma_detect_chip(bus);
}

/* Remaining bus registration and driver callbacks are direct kernel glue. */
pub unsafe fn bcma_bus_register(bus: *mut bcma_bus) -> i32 {
    let err = bcma_bus_scan(bus);
    if err != 0 { bcma_err(bus, b"Failed to scan: %d\n\0".as_ptr(), err); return err; }
    let core = bcma_find_core(bus, bcma_cc_core_id(bus));
    if !core.is_null() { (*bus).drv_cc.core = core; bcma_core_chipcommon_early_init(&mut (*bus).drv_cc); }
    let core = bcma_find_core(bus, BCMA_CORE_PCIE);
    if !core.is_null() { (*bus).drv_pci[0].core = core; bcma_core_pci_early_init(&mut (*bus).drv_pci[0]); }
    /* Early cores, SPROM, and the remaining core initialization follow the
       kernel ordering and are supplied by the surrounding translation. */
    bcma_sprom_get(bus);
    let core = bcma_find_core(bus, bcma_cc_core_id(bus));
    if !core.is_null() { (*bus).drv_cc.core = core; bcma_core_chipcommon_init(&mut (*bus).drv_cc); }
    let core = bcma_find_core(bus, BCMA_CORE_NS_CHIPCOMMON_B);
    if !core.is_null() { (*bus).drv_cc_b.core = core; bcma_core_chipcommon_b_init(&mut (*bus).drv_cc_b); }
    let core = bcma_find_core(bus, BCMA_CORE_MIPS_74K);
    if !core.is_null() { (*bus).drv_mips.core = core; bcma_core_mips_init(&mut (*bus).drv_mips); }
    let core = bcma_find_core_unit(bus, BCMA_CORE_PCIE, 0);
    if !core.is_null() { (*bus).drv_pci[0].core = core; bcma_core_pci_init(&mut (*bus).drv_pci[0]); }
    let core = bcma_find_core_unit(bus, BCMA_CORE_PCIE, 1);
    if !core.is_null() { (*bus).drv_pci[1].core = core; bcma_core_pci_init(&mut (*bus).drv_pci[1]); }
    let core = bcma_find_core_unit(bus, BCMA_CORE_PCIE2, 0);
    if !core.is_null() { (*bus).drv_pcie2.core = core; bcma_core_pcie2_init(&mut (*bus).drv_pcie2); }
    let core = bcma_find_core(bus, BCMA_CORE_4706_MAC_GBIT_COMMON);
    if !core.is_null() { (*bus).drv_gmac_cmn.core = core; bcma_core_gmac_cmn_init(&mut (*bus).drv_gmac_cmn); }
    bcma_register_devices(bus); bcma_info(bus, b"Bus registered\n\0".as_ptr()); 0
}

pub unsafe fn bcma_bus_unregister(bus: *mut bcma_bus) {
    let err = bcma_gpio_unregister(&mut (*bus).drv_cc);
    if err == -16 { bcma_err(bus, b"Some GPIOs are still in use.\n\0".as_ptr()); }
    bcma_core_chipcommon_b_free(&mut (*bus).drv_cc_b); bcma_unregister_cores(bus);
}

pub unsafe fn bcma_bus_early_register(bus: *mut bcma_bus) -> i32 {
    if bcma_bus_scan(bus) != 0 { bcma_err(bus, b"Failed to scan bus: %d\n\0".as_ptr(), -1); return -1; }
    let core = bcma_find_core(bus, bcma_cc_core_id(bus));
    if !core.is_null() { (*bus).drv_cc.core = core; bcma_core_chipcommon_early_init(&mut (*bus).drv_cc); }
    let core = bcma_find_core(bus, BCMA_CORE_MIPS_74K);
    if !core.is_null() { (*bus).drv_mips.core = core; bcma_core_mips_early_init(&mut (*bus).drv_mips); }
    bcma_info(bus, b"Early bus registered\n\0".as_ptr()); 0
}

pub unsafe fn __bcma_driver_register(drv: *mut bcma_driver, owner: *mut module) -> i32 { (*drv).drv.name = (*drv).name; (*drv).drv.bus = &bcma_bus_type; (*drv).drv.owner = owner; driver_register(&mut (*drv).drv) }
pub unsafe fn bcma_driver_unregister(drv: *mut bcma_driver) { driver_unregister(&mut (*drv).drv); }

static mut BCMA_BUS_REGISTERED: u32 = 0;
unsafe fn bcma_init_bus_register() -> i32 { if BCMA_BUS_REGISTERED != 0 { return 0; } let err = bus_register(&bcma_bus_type); if err == 0 { BCMA_BUS_REGISTERED = 1; } err }
unsafe fn bcma_modinit() -> i32 { let mut err = bcma_init_bus_register(); if err != 0 { return err; } err = bcma_host_soc_register_driver(); if err != 0 { pr_err(b"SoC host initialization failed\n\0".as_ptr()); err = 0; } err }
unsafe fn bcma_modexit() { bcma_host_soc_unregister_driver(); bus_unregister(&bcma_bus_type); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
