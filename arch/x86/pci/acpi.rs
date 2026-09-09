// SPDX-License-Identifier: GPL-2.0
// Translated from x86/pci/acpi.c. Kernel types and symbols are supplied by dependencies.

#[repr(C)]
pub struct PciRootInfo {
    pub common: AcpiPciRootInfo,
    pub sd: PciSysdata,
    #[cfg(CONFIG_PCI_MMCONFIG)] pub mcfg_added: bool,
    #[cfg(CONFIG_PCI_MMCONFIG)] pub start_bus: u8,
    #[cfg(CONFIG_PCI_MMCONFIG)] pub end_bus: u8,
}

pub static mut pci_use_e820: bool = true;
static mut pci_use_crs: bool = true;
static mut pci_ignore_seg: bool = false;

unsafe extern "C" {
    static mut iomem_resource: Resource;
    static mut pci_probe: u32;
    static mut pci_domains_supported: bool;
    static mut acpi_noirq: bool;
    static mut pci_routeirq: bool;
    static mut pcibios_enable_irq: Option<unsafe extern "C" fn(*mut PciDev)>;
    static mut pcibios_disable_irq: Option<unsafe extern "C" fn(*mut PciDev)>;
    static mut x86_init: X86Init;
    static mut raw_pci_ext_ops: *mut PciOps;
    static pci_mmcfg: PciOps;
}

unsafe extern "C" fn set_use_crs(_: *const DmiSystemId) -> i32 { pci_use_crs = true; 0 }
unsafe extern "C" fn set_nouse_crs(_: *const DmiSystemId) -> i32 { pci_use_crs = false; 0 }
unsafe extern "C" fn set_ignore_seg(id: *const DmiSystemId) -> i32 {
    pr_info("%s detected: ignoring ACPI _SEG\n", (*id).ident); pci_ignore_seg = true; 0
}
unsafe extern "C" fn set_no_e820(id: *const DmiSystemId) -> i32 {
    pr_info("%s detected: not clipping E820 regions from _CRS\n", (*id).ident);
    pci_use_e820 = false; 0
}

// DMI quirk table; DMI_MATCH entries and callbacks are represented by the kernel ABI.
static pci_crs_quirks: [DmiSystemId; 1] = [DmiSystemId::empty()];

#[cfg(CONFIG_PCI_MMCONFIG)]
unsafe fn check_segment(seg: u16, dev: *mut Device, estr: *const u8) -> i32 {
    if seg != 0 { dev_err(dev, "%s can't access configuration space under this host bridge\n", estr); return -EIO; }
    dev_warn(dev, "%s can't access extended configuration space under this bridge\n", estr); 0
}
#[cfg(CONFIG_PCI_MMCONFIG)]
unsafe fn setup_mcfg_map(ci: *mut AcpiPciRootInfo) -> i32 {
    let info = container_of(ci, PciRootInfo, common); let root = (*ci).root;
    (*info).start_bus = (*root).secondary.start as u8; (*info).end_bus = (*root).secondary.end as u8; (*info).mcfg_added = false;
    let seg = (*info).sd.domain;
    if !raw_pci_ext_ops.is_null() && raw_pci_ext_ops != &pci_mmcfg as *const _ as *mut _ { return 0; }
    if pci_probe & PCI_PROBE_MMCONF == 0 { return check_segment(seg as u16, &mut (*(*ci).bridge).dev, "MMCONFIG is disabled,\0".as_ptr()); }
    let result = pci_mmconfig_insert(&mut (*(*ci).bridge).dev, seg, (*info).start_bus, (*info).end_bus, (*root).mcfg_addr);
    if result == 0 { if raw_pci_ext_ops.is_null() { raw_pci_ext_ops = &pci_mmcfg as *const _ as *mut _; } (*info).mcfg_added = true; }
    else if result != -EEXIST { return check_segment(seg as u16, &mut (*(*ci).bridge).dev, "fail to add MMCONFIG information,\0".as_ptr()); } 0
}
#[cfg(CONFIG_PCI_MMCONFIG)]
unsafe fn teardown_mcfg_map(ci: *mut AcpiPciRootInfo) { let info = container_of(ci, PciRootInfo, common); if (*info).mcfg_added { pci_mmconfig_delete((*info).sd.domain, (*info).start_bus, (*info).end_bus); (*info).mcfg_added = false; } }
#[cfg(not(CONFIG_PCI_MMCONFIG))]
unsafe fn setup_mcfg_map(_: *mut AcpiPciRootInfo) -> i32 { 0 }
#[cfg(not(CONFIG_PCI_MMCONFIG))]
unsafe fn teardown_mcfg_map(_: *mut AcpiPciRootInfo) {}

static mut acpi_pci_root_ops: AcpiPciRootOps = AcpiPciRootOps {
    pci_ops: core::ptr::null(), init_info: Some(pci_acpi_root_init_info), release_info: Some(pci_acpi_root_release_info), prepare_resources: Some(pci_acpi_root_prepare_resources),
};

pub unsafe extern "C" fn pci_acpi_crs_quirks() {
    let year = dmi_get_bios_year();
    if year >= 0 && year < 2008 && iomem_resource.end <= 0xffff_ffff { pci_use_crs = false; }
    if year >= 2023 { pci_use_e820 = false; }
    dmi_check_system(pci_crs_quirks.as_ptr());
    if pci_probe & PCI_ROOT_NO_CRS != 0 { pci_use_crs = false; }
    else if pci_probe & PCI_USE__CRS != 0 { pci_use_crs = true; }
    pr_info("%s host bridge windows from ACPI; if necessary, use \"pci=%s\" and report a bug\n",
        if pci_use_crs { "Using" } else { "Ignoring" }, if pci_use_crs { "nocrs" } else { "use_crs" });
    if pci_probe & PCI_NO_E820 != 0 { pci_use_e820 = false; }
    else if pci_probe & PCI_USE_E820 != 0 { pci_use_e820 = true; }
    pr_info("%s E820 reservations for host bridge windows\n", if pci_use_e820 { "Using" } else { "Ignoring" });
    if pci_probe & (PCI_NO_E820 | PCI_USE_E820) != 0 { pr_info("Please notify linux-pci@vger.kernel.org so future kernels can do this automatically\n"); }
}

unsafe fn pcie_switch_directly_under(bridge: *mut PciDev, pdev: *mut PciDev) -> bool {
    let mut parent = pci_upstream_bridge(pdev); if parent.is_null() { return false; }
    match pci_pcie_type(pdev) {
        PCI_EXP_TYPE_UPSTREAM => parent == bridge,
        PCI_EXP_TYPE_DOWNSTREAM => { if pci_pcie_type(parent) != PCI_EXP_TYPE_UPSTREAM { return false; } parent = pci_upstream_bridge(parent); parent == bridge }
        PCI_EXP_TYPE_ENDPOINT => { if pci_pcie_type(parent) != PCI_EXP_TYPE_DOWNSTREAM { return false; } parent = pci_upstream_bridge(parent); if parent.is_null() || pci_pcie_type(parent) != PCI_EXP_TYPE_UPSTREAM { return false; } parent = pci_upstream_bridge(parent); parent == bridge }
        _ => false,
    }
}

unsafe fn pcie_has_usb4_host_interface(pdev: *mut PciDev) -> bool {
    let f = fwnode_find_reference(dev_fwnode(&mut (*pdev).dev), "usb4-host-interface\0".as_ptr() as _, 0);
    if !is_err(f) { fwnode_handle_put(f); return true; }
    if (*pdev).vendor == PCI_VENDOR_ID_INTEL { match (*pdev).device { 0x8a1d|0x8a1f|0x8a21|0x8a23|0x9a23|0x9a25|0x9a27|0x9a29|0x9a2b|0x9a2d|0x9a2f|0x9a31 => return true, _ => {} } }
    false
}

pub unsafe extern "C" fn arch_pci_dev_is_removable(pdev: *mut PciDev) -> bool {
    let parent = pci_upstream_bridge(pdev); if parent.is_null() { return false; }
    let root = pcie_find_root_port(pdev); if root.is_null() || !(*root).external_facing { return false; }
    if pcie_has_usb4_host_interface(parent) { return true; }
    !pcie_switch_directly_under(root, pdev)
}

unsafe fn resource_is_pcicfg_ioport(res: *mut Resource) -> bool { ((*res).flags & IORESOURCE_IO) != 0 && (*res).start == 0xcf8 && (*res).end == 0xcff }

unsafe fn pci_acpi_root_get_node(root: *mut AcpiPciRoot) -> i32 {
    let busnum = (*root).secondary.start; let device = (*root).device; let mut node = acpi_get_node((*device).handle);
    if node == NUMA_NO_NODE { node = x86_pci_root_bus_node(busnum); if node != 0 && node != NUMA_NO_NODE { dev_info(&mut (*device).dev, "no _PXM; falling back to node %d from hardware (may be inconsistent with ACPI node numbers)\n", node); } }
    if node != NUMA_NO_NODE && !node_online(node) { node = NUMA_NO_NODE; } node
}

unsafe fn pci_acpi_root_init_info(ci: *mut AcpiPciRootInfo) -> i32 { setup_mcfg_map(ci) }
unsafe fn pci_acpi_root_release_info(ci: *mut AcpiPciRootInfo) { teardown_mcfg_map(ci); kfree(container_of(ci, PciRootInfo, common)); }

unsafe fn pci_acpi_root_prepare_resources(ci: *mut AcpiPciRootInfo) -> i32 {
    let busnum = (*(*ci).root).secondary.start; let status = acpi_pci_probe_root_resources(ci);
    if pci_use_crs { resource_list_for_each_entry_safe(&mut (*ci).resources, |e| { if resource_is_pcicfg_ioport((*e).res) { resource_list_destroy_entry(e); } }); return status; }
    resource_list_for_each_entry_safe(&mut (*ci).resources, |e| { dev_printk(KERN_DEBUG, &mut (*(*ci).bridge).dev, "host bridge window %pR (ignored)\n", (*e).res); resource_list_destroy_entry(e); });
    x86_pci_root_bus_resources(busnum, &mut (*ci).resources); 0
}

pub unsafe extern "C" fn pci_acpi_scan_root(root: *mut AcpiPciRoot) -> *mut PciBus {
    let mut domain = (*root).segment; let busnum = (*root).secondary.start; let node = pci_acpi_root_get_node(root);
    if pci_ignore_seg { (*root).segment = 0; domain = 0; }
    if domain != 0 && !pci_domains_supported { pr_warn("pci_bus %04x:%02x: ignored (multiple domains not supported)\n", domain, busnum); return core::ptr::null_mut(); }
    let bus = pci_find_bus(domain, busnum); if !bus.is_null() { let sd = PciSysdata { domain, node, companion: (*root).device }; core::ptr::copy_nonoverlapping(&sd, (*bus).sysdata as *mut PciSysdata, 1); return bus; }
    let info = kzalloc_obj::<PciRootInfo>(); if info.is_null() { dev_err(&mut (*(*root).device).dev, "pci_bus %04x:%02x: ignored (out of memory)\n", domain, busnum); return core::ptr::null_mut(); }
    (*info).sd = PciSysdata { domain, node, companion: (*root).device }; acpi_pci_root_create(root, &acpi_pci_root_ops, &mut (*info).common, &mut (*info).sd)
}

pub unsafe extern "C" fn pcibios_root_bridge_prepare(bridge: *mut PciHostBridge) -> i32 { if (*bridge).dev.parent.is_null() { let sd = (*(*bridge).bus).sysdata as *mut PciSysdata; ACPI_COMPANION_SET(&mut (*bridge).dev, (*sd).companion); } 0 }

pub unsafe extern "C" fn pci_acpi_init() -> i32 {
    if acpi_noirq { return -ENODEV; } pr_info("Using ACPI for IRQ routing\n"); acpi_irq_penalty_init(); pcibios_enable_irq = Some(acpi_pci_irq_enable); pcibios_disable_irq = Some(acpi_pci_irq_disable); x86_init.pci.init_irq = x86_init_noop;
    if pci_routeirq { pr_info("Routing PCI interrupts for all devices because \"pci=routeirq\" specified\n"); for_each_pci_dev(|dev| { acpi_pci_irq_enable(dev); }); } 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
