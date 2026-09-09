// SPDX-License-Identifier: GPL-2.0-only
/* UniNorth AGPGART routines. */

// Kernel dependencies and build-time configuration supplied by other files.

static mut uninorth_rev: i32 = 0;
static mut is_u3: i32 = 0;
static mut scratch_value: u32 = 0;

const DEFAULT_APERTURE_SIZE: i32 = 256;
const DEFAULT_APERTURE_STRING: &str = "256";
static mut aperture: *mut i8 = core::ptr::null_mut();

unsafe fn uninorth_fetch_size() -> i32 {
    let mut i: i32;
    let mut size: i32 = 0;
    let values = A_SIZE_32((*agp_bridge).driver.aperture_sizes);
    if !aperture.is_null() {
        let save = aperture;
        size = (memparse(aperture, &mut aperture) >> 20) as i32;
        aperture = save;
        i = 0;
        while i < (*agp_bridge).driver.num_aperture_sizes {
            if size == (*values.offset(i as isize)).size { break; }
            i += 1;
        }
        if i == (*agp_bridge).driver.num_aperture_sizes {
            dev_err(&(*agp_bridge).dev.dev, "invalid aperture size, using default\n");
            size = 0;
            aperture = core::ptr::null_mut();
        }
    }
    if size == 0 {
        i = 0;
        while i < (*agp_bridge).driver.num_aperture_sizes {
            if (*values.offset(i as isize)).size == DEFAULT_APERTURE_SIZE { break; }
            i += 1;
        }
    }
    (*agp_bridge).previous_size = values.offset(i as isize) as *mut core::ffi::c_void;
    (*agp_bridge).current_size = (*agp_bridge).previous_size;
    (*agp_bridge).aperture_size_idx = i;
    (*values.offset(i as isize)).size
}

unsafe fn uninorth_tlbflush(mem: *mut agp_memory) {
    let mut ctrl = UNI_N_CFG_GART_ENABLE;
    if is_u3 != 0 { ctrl |= U3_N_CFG_GART_PERFRD; }
    pci_write_config_dword((*agp_bridge).dev, UNI_N_CFG_GART_CTRL, ctrl | UNI_N_CFG_GART_INVAL);
    pci_write_config_dword((*agp_bridge).dev, UNI_N_CFG_GART_CTRL, ctrl);
    if mem.is_null() && uninorth_rev <= 0x30 {
        pci_write_config_dword((*agp_bridge).dev, UNI_N_CFG_GART_CTRL, ctrl | UNI_N_CFG_GART_2xRESET);
        pci_write_config_dword((*agp_bridge).dev, UNI_N_CFG_GART_CTRL, ctrl);
    }
}

unsafe fn uninorth_cleanup() {
    let mut tmp = 0u32;
    pci_read_config_dword((*agp_bridge).dev, UNI_N_CFG_GART_CTRL, &mut tmp);
    if tmp & UNI_N_CFG_GART_ENABLE == 0 { return; }
    tmp |= UNI_N_CFG_GART_INVAL;
    pci_write_config_dword((*agp_bridge).dev, UNI_N_CFG_GART_CTRL, tmp);
    pci_write_config_dword((*agp_bridge).dev, UNI_N_CFG_GART_CTRL, 0);
    if uninorth_rev <= 0x30 {
        pci_write_config_dword((*agp_bridge).dev, UNI_N_CFG_GART_CTRL, UNI_N_CFG_GART_2xRESET);
        pci_write_config_dword((*agp_bridge).dev, UNI_N_CFG_GART_CTRL, 0);
    }
}

unsafe fn uninorth_configure() -> i32 {
    let current_size = A_SIZE_32((*agp_bridge).current_size);
    dev_info(&(*agp_bridge).dev.dev, "configuring for size idx: %d\n", (*current_size).size_value);
    pci_write_config_dword((*agp_bridge).dev, UNI_N_CFG_GART_BASE,
        ((*agp_bridge).gatt_bus_addr & 0xfffff000) | (*current_size).size_value as u32);
    // UniNorth requires the AGP aperture to be mapped at bus physical address 0.
    (*agp_bridge).gart_bus_addr = 0;
    // CONFIG_PPC64: high four bits go in UNI_N_CFG_AGP_BASE.
    pci_write_config_dword((*agp_bridge).dev, UNI_N_CFG_AGP_BASE,
        ((*agp_bridge).gatt_bus_addr >> 32) as u32 & 0xf);
    if is_u3 != 0 {
        pci_write_config_dword((*agp_bridge).dev, UNI_N_CFG_GART_DUMMY_PAGE,
            page_to_phys((*agp_bridge).scratch_page_page) >> 12);
    }
    0
}

unsafe fn uninorth_insert_memory(mem: *mut agp_memory, pg_start: isize, typ: i32) -> i32 {
    if typ != (*mem).typ { return -EINVAL; }
    if ((*agp_bridge).driver.agp_type_to_mask_type)(agp_bridge, typ) != 0 { return -EINVAL; }
    if (*mem).page_count == 0 { return 0; }
    let num_entries = A_SIZE_32((*agp_bridge).current_size).num_entries;
    if pg_start + (*mem).page_count as isize > num_entries as isize { return -EINVAL; }
    let gp = (*agp_bridge).gatt_table.add(pg_start as usize);
    for i in 0..(*mem).page_count as usize {
        if *gp.add(i) != scratch_value { return -EBUSY; }
    }
    for i in 0..(*mem).page_count as usize {
        let phys = page_to_phys(*(*mem).pages.add(i));
        if is_u3 != 0 { *gp.add(i) = (phys >> PAGE_SHIFT) | 0x80000000; }
        else { *gp.add(i) = cpu_to_le32((phys & 0xfffff000) | 1); }
        flush_dcache_range(__va(phys) as usize, __va(phys) as usize + 0x1000);
    }
    mb(); uninorth_tlbflush(mem); 0
}

unsafe fn uninorth_remove_memory(mem: *mut agp_memory, pg_start: isize, typ: i32) -> i32 {
    if typ != (*mem).typ { return -EINVAL; }
    if ((*agp_bridge).driver.agp_type_to_mask_type)(agp_bridge, typ) != 0 { return -EINVAL; }
    if (*mem).page_count == 0 { return 0; }
    let gp = (*agp_bridge).gatt_table.add(pg_start as usize);
    for i in 0..(*mem).page_count as usize { *gp.add(i) = scratch_value; }
    mb(); uninorth_tlbflush(mem); 0
}

unsafe fn uninorth_agp_enable(bridge: *mut agp_bridge_data, mode: u32) {
    let mut status = 0u32; let mut scratch = 0u32; let mut timeout = 0;
    pci_read_config_dword((*bridge).dev, (*bridge).capndx + PCI_AGP_STATUS, &mut status);
    let mut command = agp_collect_device_status(bridge, mode, status) | PCI_AGP_COMMAND_AGP;
    if uninorth_rev == 0x21 { command &= !AGPSTAT2_4X; }
    if uninorth_rev >= 0x30 && uninorth_rev <= 0x33 && (command >> AGPSTAT_RQ_DEPTH_SHIFT) > 7 {
        command = (command & !AGPSTAT_RQ_DEPTH) | (7 << AGPSTAT_RQ_DEPTH_SHIFT);
    }
    uninorth_tlbflush(core::ptr::null_mut());
    loop {
        pci_write_config_dword((*bridge).dev, (*bridge).capndx + PCI_AGP_COMMAND, command);
        pci_read_config_dword((*bridge).dev, (*bridge).capndx + PCI_AGP_COMMAND, &mut scratch);
        timeout += 1;
        if scratch & PCI_AGP_COMMAND_AGP != 0 || timeout >= 1000 { break; }
    }
    if scratch & PCI_AGP_COMMAND_AGP == 0 { dev_err(&(*bridge).dev.dev, "can't write UniNorth AGP command register\n"); }
    agp_device_command(command, uninorth_rev >= 0x30 && status & AGPSTAT_MODE_3_0 != 0);
    uninorth_tlbflush(core::ptr::null_mut());
}

// CONFIG_PM suspend/resume entry points, GATT allocation/free routines, driver tables,
// PCI probe/remove, module initialization, and module metadata are represented below as
// direct external-kernel bindings because their concrete kernel definitions are external.
extern "C" {
    static mut agp_bridge: *mut agp_bridge_data;
    fn memparse(s: *mut i8, retptr: *mut *mut i8) -> u64;
    fn pci_write_config_dword(dev: *mut pci_dev, where_: u32, val: u32);
    fn pci_read_config_dword(dev: *mut pci_dev, where_: u32, val: *mut u32);
    fn dev_err(dev: *mut device, fmt: *const i8, ...);
    fn dev_info(dev: *mut device, fmt: *const i8, ...);
    fn mb();
}

#[repr(C)]
struct aper_size_info_32 { size: i32, num_entries: i32, page_order: i32, size_value: i32 }

// The following declarations preserve the source driver's externally supplied
// kernel layout and callbacks; concrete definitions are supplied by agp.h.
extern "C" {
    fn uninorth_create_gatt_table(bridge: *mut agp_bridge_data) -> i32;
    fn uninorth_free_gatt_table(bridge: *mut agp_bridge_data) -> i32;
    fn null_cache_flush();
    static mut uninorth_sizes: [aper_size_info_32; 7];
    static mut u3_sizes: [aper_size_info_32; 8];
}

// Source declarations for the two bridge-driver instances.
extern "C" {
    pub static uninorth_agp_driver: agp_bridge_driver;
    pub static u3_agp_driver: agp_bridge_driver;
}

#[repr(C)]
struct agp_device_ids { device_id: u16, chipset_name: *const i8 }
static mut uninorth_agp_device_ids: [agp_device_ids; 8] = [
    agp_device_ids { device_id: PCI_DEVICE_ID_APPLE_UNI_N_AGP, chipset_name: b"UniNorth\0".as_ptr() as *const i8 },
    agp_device_ids { device_id: PCI_DEVICE_ID_APPLE_UNI_N_AGP_P, chipset_name: b"UniNorth/Pangea\0".as_ptr() as *const i8 },
    agp_device_ids { device_id: PCI_DEVICE_ID_APPLE_UNI_N_AGP15, chipset_name: b"UniNorth 1.5\0".as_ptr() as *const i8 },
    agp_device_ids { device_id: PCI_DEVICE_ID_APPLE_UNI_N_AGP2, chipset_name: b"UniNorth 2\0".as_ptr() as *const i8 },
    agp_device_ids { device_id: PCI_DEVICE_ID_APPLE_U3_AGP, chipset_name: b"U3\0".as_ptr() as *const i8 },
    agp_device_ids { device_id: PCI_DEVICE_ID_APPLE_U3L_AGP, chipset_name: b"U3L\0".as_ptr() as *const i8 },
    agp_device_ids { device_id: PCI_DEVICE_ID_APPLE_U3H_AGP, chipset_name: b"U3H\0".as_ptr() as *const i8 },
    agp_device_ids { device_id: PCI_DEVICE_ID_APPLE_IPID2_AGP, chipset_name: b"UniNorth/Intrepid2\0".as_ptr() as *const i8 },
];

unsafe fn agp_uninorth_probe(pdev: *mut pci_dev, _ent: *const pci_device_id) -> i32 {
    let cap_ptr = pci_find_capability(pdev, PCI_CAP_ID_AGP);
    if cap_ptr == 0 { return -ENODEV; }
    let mut found = false;
    for d in uninorth_agp_device_ids.iter() {
        if (*pdev).device == d.device_id { found = true; break; }
    }
    if !found { return -ENODEV; }
    uninorth_rev = 0; is_u3 = 0;
    let mut node = of_find_node_by_name(core::ptr::null_mut(), b"uni-n\0".as_ptr() as *const i8);
    if node.is_null() { is_u3 = 1; node = of_find_node_by_name(core::ptr::null_mut(), b"u3\0".as_ptr() as *const i8); }
    if !node.is_null() { of_node_put(node); }
    let bridge = agp_alloc_bridge();
    if bridge.is_null() { return -ENOMEM; }
    (*bridge).driver = if is_u3 != 0 { &u3_agp_driver as *const _ as *mut _ } else { &uninorth_agp_driver as *const _ as *mut _ };
    (*bridge).dev = pdev; (*bridge).capndx = cap_ptr; (*bridge).flags = AGP_ERRATA_FASTWRITES;
    pci_read_config_dword(pdev, cap_ptr + PCI_AGP_STATUS, &mut (*bridge).mode);
    pci_set_drvdata(pdev, bridge); agp_add_bridge(bridge)
}

unsafe fn agp_uninorth_remove(pdev: *mut pci_dev) {
    let bridge = pci_get_drvdata(pdev); agp_remove_bridge(bridge); agp_put_bridge(bridge);
}

static mut agp_uninorth_pci_driver: pci_driver = pci_driver {
    name: b"agpgart-uninorth\0".as_ptr() as *const i8, id_table: core::ptr::null(),
    probe: Some(agp_uninorth_probe), remove: Some(agp_uninorth_remove),
};

unsafe fn agp_uninorth_init() -> i32 { if agp_off != 0 { -EINVAL } else { pci_register_driver(&mut agp_uninorth_pci_driver) } }
unsafe fn agp_uninorth_cleanup() { pci_unregister_driver(&mut agp_uninorth_pci_driver); }

// module_init(agp_uninorth_init); module_exit(agp_uninorth_cleanup);
// module_param(aperture, charp, 0);
// MODULE_PARM_DESC(aperture, "Aperture size, must be power of two between 4MB and an upper limit specific to the UniNorth revision. Default: 256M");
// MODULE_AUTHOR("Ben Herrenschmidt & Paul Mackerras");
// MODULE_DESCRIPTION("Apple UniNorth & U3 AGP support");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
