// SPDX-License-Identifier: GPL-2.0-only
/* Low-Level PCI Support for PC */

// Linux and architecture headers supplying the declarations used below.

pub static mut pci_probe: u32 = PCI_PROBE_BIOS | PCI_PROBE_CONF1 | PCI_PROBE_CONF2 | PCI_PROBE_MMCONF;
static mut pci_bf_sort: i32 = 0;
pub static mut pci_routeirq: i32 = 0;
pub static mut noioapicquirk: i32 = 0;
#[cfg(CONFIG_X86_REROUTE_FOR_BROKEN_BOOT_IRQS)]
pub static mut noioapicreroute: i32 = 0;
#[cfg(not(CONFIG_X86_REROUTE_FOR_BROKEN_BOOT_IRQS))]
pub static mut noioapicreroute: i32 = 1;
pub static mut pcibios_last_bus: i32 = -1;
pub static mut pirq_table_addr: usize = 0;
pub static mut raw_pci_ops: *const pci_raw_ops = core::ptr::null();
pub static mut raw_pci_ext_ops: *const pci_raw_ops = core::ptr::null();

pub unsafe fn raw_pci_read(domain: u32, bus: u32, devfn: u32, reg: i32, len: i32, val: *mut u32) -> i32 {
    if domain == 0 && reg < 256 && !raw_pci_ops.is_null() {
        return ((*raw_pci_ops).read)(domain, bus, devfn, reg, len, val);
    }
    if !raw_pci_ext_ops.is_null() {
        return ((*raw_pci_ext_ops).read)(domain, bus, devfn, reg, len, val);
    }
    -EINVAL
}

pub unsafe fn raw_pci_write(domain: u32, bus: u32, devfn: u32, reg: i32, len: i32, val: u32) -> i32 {
    if domain == 0 && reg < 256 && !raw_pci_ops.is_null() {
        return ((*raw_pci_ops).write)(domain, bus, devfn, reg, len, val);
    }
    if !raw_pci_ext_ops.is_null() {
        return ((*raw_pci_ext_ops).write)(domain, bus, devfn, reg, len, val);
    }
    -EINVAL
}

unsafe fn pci_read(bus: *mut pci_bus, devfn: u32, where_: i32, size: i32, value: *mut u32) -> i32 {
    raw_pci_read(pci_domain_nr(bus), (*bus).number, devfn, where_, size, value)
}

unsafe fn pci_write(bus: *mut pci_bus, devfn: u32, where_: i32, size: i32, value: u32) -> i32 {
    raw_pci_write(pci_domain_nr(bus), (*bus).number, devfn, where_, size, value)
}

#[no_mangle]
pub static mut pci_root_ops: pci_ops = pci_ops { read: pci_read, write: pci_write };

// This interrupt-safe spinlock protects PCI configuration-space accesses except MMCONFIG.
pub static mut pci_config_lock: raw_spinlock_t = RAW_SPINLOCK_INITIALIZER;

unsafe fn can_skip_ioresource_align(d: *const dmi_system_id) -> i32 {
    pci_probe |= PCI_CAN_SKIP_ISA_ALIGN;
    printk(KERN_INFO, c"PCI: %s detected, can skip ISA alignment\n", (*d).ident);
    0
}

// DMI table entries are represented using the kernel's externally supplied DMI structures.
static can_skip_pciprobe_dmi_table: [dmi_system_id; 4] = [
    dmi_entry!(can_skip_ioresource_align, "IBM System x3800", "IBM", "x3800"),
    dmi_entry!(can_skip_ioresource_align, "IBM System x3850", "IBM", "x3850"),
    dmi_entry!(can_skip_ioresource_align, "IBM System x3950", "IBM", "x3950"),
    dmi_empty!(),
];

pub unsafe fn dmi_check_skip_isa_align() {
    dmi_check_system(can_skip_pciprobe_dmi_table.as_ptr());
}

unsafe fn pcibios_fixup_device_resources(dev: *mut pci_dev) {
    let rom_r = &mut (*dev).resource[PCI_ROM_RESOURCE as usize];
    if pci_probe & PCI_NOASSIGN_BARS != 0 {
        for bar in 0..PCI_STD_NUM_BARS as usize {
            let bar_r = &mut (*dev).resource[bar];
            if bar_r.start == 0 && bar_r.end != 0 { bar_r.flags = 0; bar_r.end = 0; }
        }
    }
    if pci_probe & PCI_NOASSIGN_ROMS != 0 {
        if !rom_r.parent.is_null() || rom_r.start != 0 { return; }
        rom_r.start = 0; rom_r.end = 0; rom_r.flags = 0;
    }
}

pub unsafe fn pcibios_fixup_bus(b: *mut pci_bus) {
    pci_read_bridge_bases(b);
    list_for_each_entry!((*b).devices, dev, bus_list, { pcibios_fixup_device_resources(dev); });
}

pub unsafe fn pcibios_add_bus(bus: *mut pci_bus) { acpi_pci_add_bus(bus); }
pub unsafe fn pcibios_remove_bus(bus: *mut pci_bus) { acpi_pci_remove_bus(bus); }

unsafe fn set_bf_sort(d: *const dmi_system_id) -> i32 {
    if pci_bf_sort == pci_bf_sort_default {
        pci_bf_sort = pci_dmi_bf;
        printk(KERN_INFO, c"PCI: %s detected, enabling pci=bfsort.\n", (*d).ident);
    }
    0
}

unsafe fn read_dmi_type_b1(dm: *const dmi_header, private_data: *mut core::ffi::c_void) {
    let data = (dm as *const u8).add(4);
    if (*dm).type_ != 0xB1 { return; }
    if (((*(data as *const u32) >> 9) & 0x03) == 0x01) { set_bf_sort(private_data as *const dmi_system_id); }
}

unsafe fn find_sort_method(d: *const dmi_system_id) -> i32 {
    dmi_walk(read_dmi_type_b1, d as *mut core::ffi::c_void);
    0
}

#[cfg(target_arch = "x86")]
unsafe fn assign_all_busses(d: *const dmi_system_id) -> i32 {
    pci_probe |= PCI_ASSIGN_ALL_BUSSES;
    printk(KERN_INFO, c"%s detected: enabling PCI bus# renumbering (pci=assign-busses)\n", (*d).ident);
    0
}

unsafe fn set_scan_all(d: *const dmi_system_id) -> i32 {
    printk(KERN_INFO, c"PCI: %s detected, enabling pci=pcie_scan_all\n", (*d).ident);
    pci_add_flags(PCI_SCAN_ALL_PCIE_DEVS);
    0
}

static pciprobe_dmi_table: [dmi_system_id; 25] = [
    #[cfg(target_arch = "x86")]
    dmi_entry!(assign_all_busses, "Samsung X20 Laptop", "Samsung Electronics", "SX20S"),
    dmi_entry!(set_bf_sort, "Dell PowerEdge 1950", "Dell", "PowerEdge 1950"),
    dmi_entry!(set_bf_sort, "Dell PowerEdge 1955", "Dell", "PowerEdge 1955"),
    dmi_entry!(set_bf_sort, "Dell PowerEdge 2900", "Dell", "PowerEdge 2900"),
    dmi_entry!(set_bf_sort, "Dell PowerEdge 2950", "Dell", "PowerEdge 2950"),
    dmi_entry!(set_bf_sort, "Dell PowerEdge R900", "Dell", "PowerEdge R900"),
    dmi_entry!(find_sort_method, "Dell System", "Dell Inc", core::ptr::null()),
    dmi_entry!(set_bf_sort, "HP ProLiant BL20p G3", "HP", "ProLiant BL20p G3"),
    dmi_entry!(set_bf_sort, "HP ProLiant BL20p G4", "HP", "ProLiant BL20p G4"),
    dmi_entry!(set_bf_sort, "HP ProLiant BL30p G1", "HP", "ProLiant BL30p G1"),
    dmi_entry!(set_bf_sort, "HP ProLiant BL25p G1", "HP", "ProLiant BL25p G1"),
    dmi_entry!(set_bf_sort, "HP ProLiant BL35p G1", "HP", "ProLiant BL35p G1"),
    dmi_entry!(set_bf_sort, "HP ProLiant BL45p G1", "HP", "ProLiant BL45p G1"),
    dmi_entry!(set_bf_sort, "HP ProLiant BL45p G2", "HP", "ProLiant BL45p G2"),
    dmi_entry!(set_bf_sort, "HP ProLiant BL460c G1", "HP", "ProLiant BL460c G1"),
    dmi_entry!(set_bf_sort, "HP ProLiant BL465c G1", "HP", "ProLiant BL465c G1"),
    dmi_entry!(set_bf_sort, "HP ProLiant BL480c G1", "HP", "ProLiant BL480c G1"),
    dmi_entry!(set_bf_sort, "HP ProLiant BL685c G1", "HP", "ProLiant BL685c G1"),
    dmi_entry!(set_bf_sort, "HP ProLiant DL360", "HP", "ProLiant DL360"),
    dmi_entry!(set_bf_sort, "HP ProLiant DL380", "HP", "ProLiant DL380"),
    #[cfg(target_arch = "x86")]
    dmi_entry!(assign_all_busses, "Compaq EVO N800c", "Compaq", "EVO N800c"),
    dmi_entry!(set_bf_sort, "HP ProLiant DL385 G2", "HP", "ProLiant DL385 G2"),
    dmi_entry!(set_bf_sort, "HP ProLiant DL585 G2", "HP", "ProLiant DL585 G2"),
    dmi_entry!(set_scan_all, "Stratus/NEC ftServer", "Stratus", "ftServer"),
    dmi_entry!(set_scan_all, "Stratus/NEC ftServer", "NEC", "Express5800/R32"),
    dmi_entry!(set_scan_all, "Stratus/NEC ftServer", "NEC", "Express5800/R31"),
    dmi_empty!(),
];

pub unsafe fn dmi_check_pciprobe() { dmi_check_system(pciprobe_dmi_table.as_ptr()); }

pub unsafe fn pcibios_scan_root(busnum: i32) {
    let mut resources = list_head::default();
    let sd = kzalloc_obj::<pci_sysdata>();
    if sd.is_null() {
        printk(KERN_ERR, c"PCI: OOM, skipping PCI bus %02x\n", busnum);
        return;
    }
    (*sd).node = x86_pci_root_bus_node(busnum);
    x86_pci_root_bus_resources(busnum, &mut resources);
    printk(KERN_DEBUG, c"PCI: Probing PCI hardware (bus %02x)\n", busnum);
    let bus = pci_scan_root_bus(core::ptr::null_mut(), busnum, &mut pci_root_ops, sd, &mut resources);
    if bus.is_null() {
        pci_free_resource_list(&mut resources);
        kfree(sd as *mut core::ffi::c_void);
        return;
    }
    pci_bus_add_devices(bus);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
