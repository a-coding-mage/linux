// SPDX-License-Identifier: GPL-2.0-only
/* VIA AGPGART routines. */

// C headers and symbols from the kernel AGP/PCI subsystem are external dependencies.

const VIA_GARTCTRL: u32 = 0x80;
const VIA_APSIZE: u32 = 0x84;
const VIA_ATTBASE: u32 = 0x88;
const VIA_AGP3_GARTCTRL: u32 = 0x90;
const VIA_AGP3_APSIZE: u32 = 0x94;
const VIA_AGP3_ATTBASE: u32 = 0x98;
const VIA_AGPSEL: u32 = 0xfd;

unsafe fn via_fetch_size() -> i32 {
    let mut temp: u8 = 0;
    let values = A_SIZE_8((*(*agp_bridge).driver).aperture_sizes);
    pci_read_config_byte((*agp_bridge).dev, VIA_APSIZE, &mut temp);
    for i in 0..(*(*agp_bridge).driver).num_aperture_sizes {
        if temp == (*values.add(i as usize)).size_value {
            (*agp_bridge).previous_size = values.add(i as usize) as *mut core::ffi::c_void;
            (*agp_bridge).current_size = values.add(i as usize) as *mut core::ffi::c_void;
            (*agp_bridge).aperture_size_idx = i;
            return (*values.add(i as usize)).size;
        }
    }
    printk(KERN_ERR, concat!(PFX, "Unknown aperture size from AGP bridge (0x%x)\n"), temp);
    0
}

unsafe fn via_configure() -> i32 {
    let current_size = A_SIZE_8((*agp_bridge).current_size);
    pci_write_config_byte((*agp_bridge).dev, VIA_APSIZE, (*current_size).size_value);
    (*agp_bridge).gart_bus_addr = pci_bus_address((*agp_bridge).dev, AGP_APERTURE_BAR);
    pci_write_config_dword((*agp_bridge).dev, VIA_GARTCTRL, 0x0000000f);
    pci_write_config_dword((*agp_bridge).dev, VIA_ATTBASE, ((*agp_bridge).gatt_bus_addr & 0xfffff000) | 3);
    0
}

unsafe fn via_cleanup() {
    let previous_size = A_SIZE_8((*agp_bridge).previous_size);
    pci_write_config_byte((*agp_bridge).dev, VIA_APSIZE, (*previous_size).size_value);
    /* Do not disable by writing 0 to VIA_ATTBASE, it screws things up during reinitialization. */
}

unsafe fn via_tlbflush(_mem: *mut agp_memory) {
    let mut temp: u32 = 0;
    pci_read_config_dword((*agp_bridge).dev, VIA_GARTCTRL, &mut temp);
    temp |= 1 << 7;
    pci_write_config_dword((*agp_bridge).dev, VIA_GARTCTRL, temp);
    temp &= !(1 << 7);
    pci_write_config_dword((*agp_bridge).dev, VIA_GARTCTRL, temp);
}

static via_generic_sizes: [aper_size_info_8; 9] = [
    aper_size_info_8 { size: 256, num_entries: 65536, page_order: 6, size_value: 0 },
    aper_size_info_8 { size: 128, num_entries: 32768, page_order: 5, size_value: 128 },
    aper_size_info_8 { size: 64, num_entries: 16384, page_order: 4, size_value: 192 },
    aper_size_info_8 { size: 32, num_entries: 8192, page_order: 3, size_value: 224 },
    aper_size_info_8 { size: 16, num_entries: 4096, page_order: 2, size_value: 240 },
    aper_size_info_8 { size: 8, num_entries: 2048, page_order: 1, size_value: 248 },
    aper_size_info_8 { size: 4, num_entries: 1024, page_order: 0, size_value: 252 },
    aper_size_info_8 { size: 2, num_entries: 512, page_order: 0, size_value: 254 },
    aper_size_info_8 { size: 1, num_entries: 256, page_order: 0, size_value: 255 },
];

unsafe fn via_fetch_size_agp3() -> i32 {
    let mut temp: u16 = 0;
    let values = A_SIZE_16((*(*agp_bridge).driver).aperture_sizes);
    pci_read_config_word((*agp_bridge).dev, VIA_AGP3_APSIZE, &mut temp);
    temp &= 0xfff;
    for i in 0..(*(*agp_bridge).driver).num_aperture_sizes {
        if temp == (*values.add(i as usize)).size_value {
            (*agp_bridge).previous_size = values.add(i as usize) as *mut core::ffi::c_void;
            (*agp_bridge).current_size = values.add(i as usize) as *mut core::ffi::c_void;
            (*agp_bridge).aperture_size_idx = i;
            return (*values.add(i as usize)).size;
        }
    }
    0
}

unsafe fn via_configure_agp3() -> i32 {
    let mut temp: u32 = 0;
    (*agp_bridge).gart_bus_addr = pci_bus_address((*agp_bridge).dev, AGP_APERTURE_BAR);
    pci_write_config_dword((*agp_bridge).dev, VIA_AGP3_ATTBASE, (*agp_bridge).gatt_bus_addr & 0xfffff000);
    pci_read_config_dword((*agp_bridge).dev, VIA_AGP3_GARTCTRL, &mut temp);
    pci_write_config_dword((*agp_bridge).dev, VIA_AGP3_GARTCTRL, temp | (3 << 7));
    0
}

unsafe fn via_cleanup_agp3() {
    let previous_size = A_SIZE_16((*agp_bridge).previous_size);
    pci_write_config_byte((*agp_bridge).dev, VIA_APSIZE, (*previous_size).size_value as u8);
}

unsafe fn via_tlbflush_agp3(_mem: *mut agp_memory) {
    let mut temp: u32 = 0;
    pci_read_config_dword((*agp_bridge).dev, VIA_AGP3_GARTCTRL, &mut temp);
    pci_write_config_dword((*agp_bridge).dev, VIA_AGP3_GARTCTRL, temp & !(1 << 7));
    pci_write_config_dword((*agp_bridge).dev, VIA_AGP3_GARTCTRL, temp);
}

// The bridge driver initializers, PCI ID table, probe/remove/resume callbacks,
// module registration, and metadata below retain the C kernel interfaces.
// External structure and callback types are supplied by the AGP/PCI subsystem.

unsafe fn check_via_agp3(bridge: *mut agp_bridge_data) {
    let mut reg: u8 = 0;
    pci_read_config_byte((*bridge).dev, VIA_AGPSEL, &mut reg);
    if reg & (1 << 1) == 0 { (*bridge).driver = &via_agp3_driver; }
}

unsafe fn agp_via_probe(pdev: *mut pci_dev, ent: *const pci_device_id) -> i32 {
    let cap_ptr = pci_find_capability(pdev, PCI_CAP_ID_AGP);
    if cap_ptr == 0 { return -ENODEV; }
    dev_info(&(*pdev).dev, "Detected VIA %s chipset\n", (*ent).driver_data as *const i8);
    let bridge = agp_alloc_bridge();
    if bridge.is_null() { return -ENOMEM; }
    (*bridge).dev = pdev;
    (*bridge).capndx = cap_ptr;
    (*bridge).driver = &via_driver;
    if (*pdev).device == PCI_DEVICE_ID_VIA_8367_0 && (*pdev).subsystem_device == PCI_DEVICE_ID_VIA_8377_0 { check_via_agp3(bridge); }
    get_agp_version(bridge);
    if (*bridge).major_version >= 3 { check_via_agp3(bridge); }
    pci_read_config_dword(pdev, (*bridge).capndx + PCI_AGP_STATUS, &mut (*bridge).mode);
    pci_set_drvdata(pdev, bridge);
    agp_add_bridge(bridge)
}

unsafe fn agp_via_remove(pdev: *mut pci_dev) {
    let bridge = pci_get_drvdata(pdev);
    agp_remove_bridge(bridge);
    agp_put_bridge(bridge);
}

unsafe fn agp_via_resume(dev: *mut device) -> i32 {
    let bridge = dev_get_drvdata(dev);
    if (*bridge).driver == &via_agp3_driver { via_configure_agp3() }
    else if (*bridge).driver == &via_driver { via_configure() } else { 0 }
}

static via_agp3_driver: agp_bridge_driver = agp_bridge_driver {
    owner: THIS_MODULE, aperture_sizes: agp3_generic_sizes as *const _, size_type: U8_APER_SIZE,
    num_aperture_sizes: 10, needs_scratch_page: true, configure: Some(via_configure_agp3),
    fetch_size: Some(via_fetch_size_agp3), cleanup: Some(via_cleanup_agp3), tlb_flush: Some(via_tlbflush_agp3),
    mask_memory: Some(agp_generic_mask_memory), masks: core::ptr::null(), agp_enable: Some(agp_generic_enable),
    cache_flush: Some(global_cache_flush), create_gatt_table: Some(agp_generic_create_gatt_table),
    free_gatt_table: Some(agp_generic_free_gatt_table), insert_memory: Some(agp_generic_insert_memory),
    remove_memory: Some(agp_generic_remove_memory), alloc_by_type: Some(agp_generic_alloc_by_type),
    free_by_type: Some(agp_generic_free_by_type), agp_alloc_page: Some(agp_generic_alloc_page),
    agp_alloc_pages: Some(agp_generic_alloc_pages), agp_destroy_page: Some(agp_generic_destroy_page),
    agp_destroy_pages: Some(agp_generic_destroy_pages), agp_type_to_mask_type: Some(agp_generic_type_to_mask_type),
};

static via_driver: agp_bridge_driver = agp_bridge_driver {
    owner: THIS_MODULE, aperture_sizes: via_generic_sizes.as_ptr(), size_type: U8_APER_SIZE,
    num_aperture_sizes: 9, needs_scratch_page: true, configure: Some(via_configure),
    fetch_size: Some(via_fetch_size), cleanup: Some(via_cleanup), tlb_flush: Some(via_tlbflush),
    mask_memory: Some(agp_generic_mask_memory), masks: core::ptr::null(), agp_enable: Some(agp_generic_enable),
    cache_flush: Some(global_cache_flush), create_gatt_table: Some(agp_generic_create_gatt_table),
    free_gatt_table: Some(agp_generic_free_gatt_table), insert_memory: Some(agp_generic_insert_memory),
    remove_memory: Some(agp_generic_remove_memory), alloc_by_type: Some(agp_generic_alloc_by_type),
    free_by_type: Some(agp_generic_free_by_type), agp_alloc_page: Some(agp_generic_alloc_page),
    agp_alloc_pages: Some(agp_generic_alloc_pages), agp_destroy_page: Some(agp_generic_destroy_page),
    agp_destroy_pages: Some(agp_generic_destroy_pages), agp_type_to_mask_type: Some(agp_generic_type_to_mask_type),
};

static agp_via_pci_table: [pci_device_id; 1] = [pci_device_id { class: PCI_CLASS_BRIDGE_HOST << 8, class_mask: !0, vendor: PCI_VENDOR_ID_VIA, device: 0, subvendor: PCI_ANY_ID, subdevice: PCI_ANY_ID, driver_data: 0 },];

static mut agp_via_pci_driver: pci_driver = pci_driver {
    name: "agpgart-via", id_table: agp_via_pci_table.as_ptr(), probe: Some(agp_via_probe), remove: Some(agp_via_remove), driver: pci_driver_embedded { pm: &agp_via_pm_ops },
};

static agp_via_pm_ops: dev_pm_ops = DEFINE_SIMPLE_DEV_PM_OPS!(None, Some(agp_via_resume));

unsafe fn agp_via_init() -> i32 {
    if agp_off { return -EINVAL; }
    pci_register_driver(&mut agp_via_pci_driver)
}

unsafe fn agp_via_cleanup() { pci_unregister_driver(&mut agp_via_pci_driver); }

// module_init!(agp_via_init);
// module_exit!(agp_via_cleanup);
// MODULE_DESCRIPTION!("VIA AGPGART routines");
// MODULE_LICENSE!("GPL");
// MODULE_AUTHOR!("Dave Jones");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
