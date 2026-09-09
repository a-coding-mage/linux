/* SiS AGPGART routines. */

// Kernel headers and symbols are supplied by the surrounding translation unit.

const SIS_ATTBASE: u8 = 0x90;
const SIS_APSIZE: u8 = 0x94;
const SIS_TLBCNTRL: u8 = 0x97;
const SIS_TLBFLUSH: u8 = 0x98;
const PCI_DEVICE_ID_SI_662: u16 = 0x0662;
const PCI_DEVICE_ID_SI_671: u16 = 0x0671;

static mut agp_sis_force_delay: bool = false;
static mut agp_sis_agp_spec: i32 = -1;

unsafe fn sis_fetch_size() -> i32 {
    let mut temp_size: u8 = 0;
    let mut i: i32;
    let values: *mut aper_size_info_8;
    pci_read_config_byte((*agp_bridge).dev, SIS_APSIZE, &mut temp_size);
    values = A_SIZE_8((*agp_bridge).driver.aperture_sizes);
    i = 0;
    while i < (*agp_bridge).driver.num_aperture_sizes {
        let v = values.add(i as usize);
        if temp_size == (*v).size_value || ((temp_size & !0x07) == ((*v).size_value & !0x07)) {
            (*agp_bridge).previous_size = v as *mut _;
            (*agp_bridge).current_size = v as *mut _;
            (*agp_bridge).aperture_size_idx = i;
            return (*v).size;
        }
        i += 1;
    }
    0
}

unsafe fn sis_tlbflush(_mem: *mut agp_memory) {
    pci_write_config_byte((*agp_bridge).dev, SIS_TLBFLUSH, 0x02);
}

unsafe fn sis_configure() -> i32 {
    let current_size: *mut aper_size_info_8 = A_SIZE_8((*agp_bridge).current_size);
    pci_write_config_byte((*agp_bridge).dev, SIS_TLBCNTRL, 0x05);
    (*agp_bridge).gart_bus_addr = pci_bus_address((*agp_bridge).dev, AGP_APERTURE_BAR);
    pci_write_config_dword((*agp_bridge).dev, SIS_ATTBASE, (*agp_bridge).gatt_bus_addr);
    pci_write_config_byte((*agp_bridge).dev, SIS_APSIZE, (*current_size).size_value);
    0
}

unsafe fn sis_cleanup() {
    let previous_size: *mut aper_size_info_8 = A_SIZE_8((*agp_bridge).previous_size);
    pci_write_config_byte((*agp_bridge).dev, SIS_APSIZE, (*previous_size).size_value & !0x03);
}

unsafe fn sis_delayed_enable(bridge: *mut agp_bridge_data, mode: u32) {
    let mut device: *mut pci_dev = core::ptr::null_mut();
    let mut command: u32 = 0;
    let rate: i32;
    dev_info!(&(*(*agp_bridge).dev).dev, "AGP {}.{} bridge\n", (*agp_bridge).major_version, (*agp_bridge).minor_version);
    pci_read_config_dword((*agp_bridge).dev, (*agp_bridge).capndx + PCI_AGP_STATUS, &mut command);
    command = agp_collect_device_status(bridge, mode, command);
    command |= AGPSTAT_AGP_ENABLE;
    rate = ((command & 0x7) << 2) as i32;
    for_each_pci_dev!(device) {
        let agp: u8 = pci_find_capability(device, PCI_CAP_ID_AGP);
        if agp == 0 { continue; }
        dev_info!(&(*(*agp_bridge).dev).dev, "putting AGP V3 device at {} into {}x mode\n", pci_name(device), rate);
        pci_write_config_dword(device, agp + PCI_AGP_COMMAND, command);
        /* Weird: on some sis chipsets any rate change in the target command
         * register triggers a 5ms screwup during which the master cannot be configured. */
        if (*device).device == (*bridge).dev.device {
            dev_info!(&(*(*agp_bridge).dev).dev, "SiS delay workaround: giving bridge time to recover\n");
            msleep(10);
        }
    }
}

static mut sis_generic_sizes: [aper_size_info_8; 7] = [
    aper_size_info_8 { size: 256, num_entries: 65536, size_value: 6, page_order: 99 },
    aper_size_info_8 { size: 128, num_entries: 32768, size_value: 5, page_order: 83 },
    aper_size_info_8 { size: 64, num_entries: 16384, size_value: 4, page_order: 67 },
    aper_size_info_8 { size: 32, num_entries: 8192, size_value: 3, page_order: 51 },
    aper_size_info_8 { size: 16, num_entries: 4096, size_value: 2, page_order: 35 },
    aper_size_info_8 { size: 8, num_entries: 2048, size_value: 1, page_order: 19 },
    aper_size_info_8 { size: 4, num_entries: 1024, size_value: 0, page_order: 3 },
];

static mut sis_driver: agp_bridge_driver = agp_bridge_driver {
    owner: THIS_MODULE, aperture_sizes: sis_generic_sizes.as_mut_ptr() as *const _, size_type: U8_APER_SIZE,
    num_aperture_sizes: 7, needs_scratch_page: true, configure: Some(sis_configure), fetch_size: Some(sis_fetch_size),
    cleanup: Some(sis_cleanup), tlb_flush: Some(sis_tlbflush), mask_memory: Some(agp_generic_mask_memory), masks: core::ptr::null(),
    agp_enable: Some(agp_generic_enable), cache_flush: Some(global_cache_flush), create_gatt_table: Some(agp_generic_create_gatt_table),
    free_gatt_table: Some(agp_generic_free_gatt_table), insert_memory: Some(agp_generic_insert_memory), remove_memory: Some(agp_generic_remove_memory),
    alloc_by_type: Some(agp_generic_alloc_by_type), free_by_type: Some(agp_generic_free_by_type), agp_alloc_page: Some(agp_generic_alloc_page),
    agp_alloc_pages: Some(agp_generic_alloc_pages), agp_destroy_page: Some(agp_generic_destroy_page), agp_destroy_pages: Some(agp_generic_destroy_pages),
    agp_type_to_mask_type: Some(agp_generic_type_to_mask_type),
};

// Chipsets that require the 'delay hack'.
static mut sis_broken_chipsets: [i32; 3] = [PCI_DEVICE_ID_SI_648, PCI_DEVICE_ID_SI_746, 0];

unsafe fn sis_get_driver(bridge: *mut agp_bridge_data) {
    let mut i = 0;
    while sis_broken_chipsets[i as usize] != 0 {
        if (*bridge).dev.device == sis_broken_chipsets[i as usize] { break; }
        i += 1;
    }
    if sis_broken_chipsets[i as usize] != 0 || agp_sis_force_delay { sis_driver.agp_enable = Some(sis_delayed_enable); }
    if (((*agp_bridge).major_version == 3 && (*agp_bridge).minor_version >= 5 && agp_sis_agp_spec != 0) || agp_sis_agp_spec == 1) {
        sis_driver.aperture_sizes = agp3_generic_sizes;
        sis_driver.size_type = U16_APER_SIZE;
        sis_driver.num_aperture_sizes = AGP_GENERIC_SIZES_ENTRIES;
        sis_driver.configure = Some(agp3_generic_configure);
        sis_driver.fetch_size = Some(agp3_generic_fetch_size);
        sis_driver.cleanup = Some(agp3_generic_cleanup);
        sis_driver.tlb_flush = Some(agp3_generic_tlbflush);
    }
}

unsafe fn agp_sis_probe(pdev: *mut pci_dev, _ent: *const pci_device_id) -> i32 {
    let cap_ptr = pci_find_capability(pdev, PCI_CAP_ID_AGP);
    if cap_ptr == 0 { return -ENODEV; }
    dev_info!(&(*pdev).dev, "SiS chipset [{:04x}/{:04x}]\n", (*pdev).vendor, (*pdev).device);
    let bridge = agp_alloc_bridge();
    if bridge.is_null() { return -ENOMEM; }
    (*bridge).driver = &mut sis_driver;
    (*bridge).dev = pdev;
    (*bridge).capndx = cap_ptr;
    get_agp_version(bridge);
    pci_read_config_dword(pdev, (*bridge).capndx + PCI_AGP_STATUS, &mut (*bridge).mode);
    sis_get_driver(bridge);
    pci_set_drvdata(pdev, bridge);
    agp_add_bridge(bridge)
}

unsafe fn agp_sis_remove(pdev: *mut pci_dev) {
    let bridge = pci_get_drvdata(pdev);
    agp_remove_bridge(bridge);
    agp_put_bridge(bridge);
}

unsafe fn agp_sis_resume(_dev: *mut device) -> i32 { sis_driver.configure.unwrap()() }

// PCI device table; entries correspond to the C driver's host-bridge matches.
static agp_sis_pci_table: [pci_device_id; 23] = [
    pci_id!(PCI_DEVICE_ID_SI_5591), pci_id!(PCI_DEVICE_ID_SI_530), pci_id!(PCI_DEVICE_ID_SI_540),
    pci_id!(PCI_DEVICE_ID_SI_550), pci_id!(PCI_DEVICE_ID_SI_620), pci_id!(PCI_DEVICE_ID_SI_630),
    pci_id!(PCI_DEVICE_ID_SI_635), pci_id!(PCI_DEVICE_ID_SI_645), pci_id!(PCI_DEVICE_ID_SI_646),
    pci_id!(PCI_DEVICE_ID_SI_648), pci_id!(PCI_DEVICE_ID_SI_650), pci_id!(PCI_DEVICE_ID_SI_651),
    pci_id!(PCI_DEVICE_ID_SI_655), pci_id!(PCI_DEVICE_ID_SI_661), pci_id!(PCI_DEVICE_ID_SI_662),
    pci_id!(PCI_DEVICE_ID_SI_671), pci_id!(PCI_DEVICE_ID_SI_730), pci_id!(PCI_DEVICE_ID_SI_735),
    pci_id!(PCI_DEVICE_ID_SI_740), pci_id!(PCI_DEVICE_ID_SI_741), pci_id!(PCI_DEVICE_ID_SI_745),
    pci_id!(PCI_DEVICE_ID_SI_746),
    pci_device_id { ..Default::default() },
];

static agp_sis_pm_ops: simple_dev_pm_ops = simple_dev_pm_ops {
    resume: Some(agp_sis_resume),
};

static mut agp_sis_pci_driver: pci_driver = pci_driver {
    name: "agpgart-sis", id_table: agp_sis_pci_table.as_ptr(), probe: Some(agp_sis_probe), remove: Some(agp_sis_remove), pm: &agp_sis_pm_ops,
};

unsafe fn agp_sis_init() -> i32 {
    if agp_off { return -EINVAL; }
    pci_register_driver(&mut agp_sis_pci_driver)
}

unsafe fn agp_sis_cleanup() { pci_unregister_driver(&mut agp_sis_pci_driver); }

// Module initialization, exit, parameters, and metadata are supplied by the kernel binding layer.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
