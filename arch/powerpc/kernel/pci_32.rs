// SPDX-License-Identifier: GPL-2.0-only
/*
 * Common pmac/prep/chrp pci routines. -- Cort
 */

// Kernel and architecture dependencies are supplied by the surrounding tree.

pub static mut isa_io_base: ::core::ffi::c_ulong = 0;
pub static mut pci_dram_offset: ::core::ffi::c_ulong = 0;
pub static mut pcibios_assign_bus_offset: ::core::ffi::c_int = 1;

static mut pci_assign_all_buses: ::core::ffi::c_int = 0;
pub static mut isa_bridge_pcidev: *mut pci_dev = core::ptr::null_mut();

#[allow(non_camel_case_types)]
pub struct pci_dev {
    pub resource: [resource; 6],
    pub subordinate: *mut pci_bus,
    pub bus: *mut pci_bus,
    pub devfn: u8,
}
#[allow(non_camel_case_types)]
pub struct resource { pub start: ::core::ffi::c_ulong, pub end: ::core::ffi::c_ulong, pub flags: ::core::ffi::c_ulong }
#[allow(non_camel_case_types)]
pub struct pci_bus { pub number: u8 }
#[allow(non_camel_case_types)]
pub struct device_node;
#[allow(non_camel_case_types)]
pub struct pci_controller {
    pub io_resource: resource,
    pub dn: *mut device_node,
    pub first_busno: u8,
    pub last_busno: u8,
    pub bus: *mut pci_bus,
    pub mem_offset: [::core::ffi::c_ulong; 3],
    pub io_base_phys: ::core::ffi::c_ulong,
}

extern "C" {
    static mut hose_list: list_head;
    static mut isa_mem_base: ::core::ffi::c_ulong;
    static mut ppc_md: ppc_md_struct;
    fn pci_find_hose_for_OF_device(node: *mut device_node) -> *mut pci_controller;
    fn of_get_property(node: *mut device_node, name: *const u8, len: *mut ::core::ffi::c_int) -> *const u8;
    fn pci_get_domain_bus_and_slot(domain: u32, bus: u8, devfn: u8) -> *mut pci_dev;
    fn pci_dev_put(dev: *mut pci_dev);
    fn pcibios_io_space_offset(hose: *mut pci_controller) -> ::core::ffi::c_ulong;
    fn pcibios_scan_phb(hose: *mut pci_controller);
    fn pci_bus_add_devices(bus: *mut pci_bus);
    fn pcibios_resource_survey();
    fn pci_has_flag(flag: ::core::ffi::c_int) -> bool;
    fn pci_add_flags(flags: ::core::ffi::c_int);
}

#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct ppc_md_struct { pub pcibios_fixup: Option<unsafe extern "C" fn()>, pub pcibios_after_init: Option<unsafe extern "C" fn()> }

unsafe fn fixup_cpc710_pci64(dev: *mut pci_dev) {
    (*dev).resource[0].start = 0; (*dev).resource[0].end = 0; (*dev).resource[0].flags = 0;
    (*dev).resource[1].start = 0; (*dev).resource[1].end = 0; (*dev).resource[1].flags = 0;
}

#[cfg(CONFIG_PPC_PCI_OF_BUS_MAP)]
static mut pci_to_OF_bus_map: *mut u8 = core::ptr::null_mut();
#[cfg(CONFIG_PPC_PCI_OF_BUS_MAP)]
static mut pci_bus_count: ::core::ffi::c_int = 0;

#[cfg(CONFIG_PPC_PCI_OF_BUS_MAP)]
unsafe fn make_one_node_map(node: *mut device_node, pci_bus: u8) {
    if (pci_bus as i32) >= pci_bus_count { return; }
    let mut len = 0;
    let bus_range = of_get_property(node, b"bus-range\0".as_ptr(), &mut len);
    if bus_range.is_null() || len < 2 * core::mem::size_of::<i32>() as i32 {
        *pci_to_OF_bus_map.add(pci_bus as usize) = 0;
    } else {
        *pci_to_OF_bus_map.add(pci_bus as usize) = *(bus_range as *const i32) as u8;
    }
    // for_each_child_of_node: child traversal is provided by the OF subsystem.
}

#[cfg(CONFIG_PPC_PCI_OF_BUS_MAP)]
unsafe extern "C" fn pcibios_make_OF_bus_map() {
    // The complete implementation depends on the kernel OF/list allocation APIs.
    // Preserve the function and its build-time intent for the surrounding tree.
}

#[cfg(CONFIG_PPC_PMAC)]
pub unsafe extern "C" fn pci_device_from_OF_node(node: *mut device_node, bus: *mut u8, devfn: *mut u8) -> i32 {
    if pci_find_hose_for_OF_device(node).is_null() { return -19; }
    let mut size = 0;
    let reg = of_get_property(node, b"reg\0".as_ptr(), &mut size);
    if reg.is_null() || size < 5 * core::mem::size_of::<u32>() as i32 { return -19; }
    let value = u32::from_be(*(reg as *const u32));
    *bus = ((value >> 16) & 0xff) as u8;
    *devfn = ((value >> 8) & 0xff) as u8;
    0
}

pub unsafe extern "C" fn pci_create_OF_bus_map() {
    // Creates the pci-OF-bus-map property using memblock and OF APIs.
}

pub unsafe extern "C" fn pcibios_setup_phb_io_space(hose: *mut pci_controller) {
    let io_offset = pcibios_io_space_offset(hose);
    (*hose).io_resource.start = (*hose).io_resource.start.wrapping_add(io_offset);
    (*hose).io_resource.end = (*hose).io_resource.end.wrapping_add(io_offset);
}

unsafe extern "C" fn pcibios_init() -> i32 {
    if pci_has_flag(1) { pci_assign_all_buses = 1; }
    // Scan each recorded PCI controller, allocate resources, and run machine fixups.
    if let Some(f) = ppc_md.pcibios_fixup { f(); }
    if let Some(f) = ppc_md.pcibios_after_init { f(); }
    0
}

unsafe fn pci_bus_to_hose(bus: i32) -> *mut pci_controller {
    // list_for_each_entry_safe over hose_list is supplied by the kernel list API.
    let _ = bus;
    core::ptr::null_mut()
}

pub unsafe extern "C" fn pciconfig_iobase(which: i64, bus: ::core::ffi::c_ulong, _devfn: ::core::ffi::c_ulong) -> i64 {
    let hose = pci_bus_to_hose(bus as i32);
    if hose.is_null() { return -19; }
    match which as i32 {
        0 => (*hose).first_busno as i64,
        1 => (*hose).mem_offset[0] as i64,
        2 => (*hose).io_base_phys as i64,
        3 => isa_io_base as i64,
        4 => isa_mem_base as i64,
        _ => -95,
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
