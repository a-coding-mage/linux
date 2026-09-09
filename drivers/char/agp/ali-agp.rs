/*
 * ALi AGPGART routines.
 */

// External Linux/kernel declarations supplied by the surrounding translation unit.

const ALI_AGPCTRL: u32 = 0xb8;
const ALI_ATTBASE: u32 = 0xbc;
const ALI_TLBCTRL: u32 = 0xc0;
const ALI_TAGCTRL: u32 = 0xc4;
const ALI_CACHE_FLUSH_CTRL: u32 = 0xD0;
const ALI_CACHE_FLUSH_ADDR_MASK: u32 = 0xFFFFF000;
const ALI_CACHE_FLUSH_EN: u32 = 0x100;

unsafe fn ali_fetch_size() -> i32 {
    let mut temp: u32 = 0;
    let values: *mut aper_size_info_32;

    pci_read_config_dword((*agp_bridge).dev, ALI_ATTBASE, &mut temp);
    temp &= !0xfffffff0u32;
    values = A_SIZE_32((*(*agp_bridge).driver).aperture_sizes);

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

unsafe fn ali_tlbflush(_mem: *mut agp_memory) {
    let mut temp: u32 = 0;
    pci_read_config_dword((*agp_bridge).dev, ALI_TLBCTRL, &mut temp);
    temp &= 0xfffffff0;
    temp |= (1 << 0) | (1 << 1);
    pci_write_config_dword((*agp_bridge).dev, ALI_TAGCTRL, temp);
}

unsafe fn ali_cleanup() {
    let previous_size = A_SIZE_32((*agp_bridge).previous_size);
    let mut temp: u32 = 0;
    pci_read_config_dword((*agp_bridge).dev, ALI_TLBCTRL, &mut temp);
    // clear tag
    pci_write_config_dword((*agp_bridge).dev, ALI_TAGCTRL, (temp & 0xffffff00) | 0x00000001 | 0x00000002);
    pci_read_config_dword((*agp_bridge).dev, ALI_ATTBASE, &mut temp);
    pci_write_config_dword((*agp_bridge).dev, ALI_ATTBASE, (temp & 0x00000ff0) | (*previous_size).size_value);
}

unsafe fn ali_configure() -> i32 {
    let current_size = A_SIZE_32((*agp_bridge).current_size);
    let mut temp: u32 = 0;
    pci_read_config_dword((*agp_bridge).dev, ALI_ATTBASE, &mut temp);
    temp = (temp & 0x00000ff0) | ((*agp_bridge).gatt_bus_addr & 0xfffff000) | ((*current_size).size_value & 0xf);
    pci_write_config_dword((*agp_bridge).dev, ALI_ATTBASE, temp);
    pci_read_config_dword((*agp_bridge).dev, ALI_TLBCTRL, &mut temp);
    pci_write_config_dword((*agp_bridge).dev, ALI_TLBCTRL, (temp & 0xffffff00) | 0x00000010);
    (*agp_bridge).gart_bus_addr = pci_bus_address((*agp_bridge).dev, AGP_APERTURE_BAR);
    pci_read_config_dword((*agp_bridge).dev, ALI_TLBCTRL, &mut temp);
    temp &= 0xffffff7f; //enable TLB
    pci_write_config_dword((*agp_bridge).dev, ALI_TLBCTRL, temp);
    0
}

unsafe fn m1541_cache_flush() {
    let mut temp: u32 = 0;
    global_cache_flush();
    let page_count = 1 << (*A_SIZE_32((*agp_bridge).current_size)).page_order;
    let mut i = 0;
    while i < PAGE_SIZE * page_count {
        pci_read_config_dword((*agp_bridge).dev, ALI_CACHE_FLUSH_CTRL, &mut temp);
        pci_write_config_dword((*agp_bridge).dev, ALI_CACHE_FLUSH_CTRL, (temp & ALI_CACHE_FLUSH_ADDR_MASK) | ((*agp_bridge).gatt_bus_addr + i) | ALI_CACHE_FLUSH_EN);
        i += PAGE_SIZE;
    }
}

unsafe fn m1541_alloc_page(_bridge: *mut agp_bridge_data) -> *mut page {
    let page = agp_generic_alloc_page(agp_bridge);
    if page.is_null() { return core::ptr::null_mut(); }
    let mut temp: u32 = 0;
    pci_read_config_dword((*agp_bridge).dev, ALI_CACHE_FLUSH_CTRL, &mut temp);
    pci_write_config_dword((*agp_bridge).dev, ALI_CACHE_FLUSH_CTRL, (temp & ALI_CACHE_FLUSH_ADDR_MASK) | page_to_phys(page) | ALI_CACHE_FLUSH_EN);
    page
}

unsafe fn ali_destroy_page(page: *mut page, flags: i32) {
    if !page.is_null() {
        if flags & AGP_PAGE_DESTROY_UNMAP != 0 { global_cache_flush(); }
        agp_generic_destroy_page(page, flags);
    }
}

unsafe fn m1541_destroy_page(page: *mut page, flags: i32) {
    if page.is_null() { return; }
    if flags & AGP_PAGE_DESTROY_UNMAP != 0 {
        global_cache_flush();
        let mut temp: u32 = 0;
        pci_read_config_dword((*agp_bridge).dev, ALI_CACHE_FLUSH_CTRL, &mut temp);
        pci_write_config_dword((*agp_bridge).dev, ALI_CACHE_FLUSH_CTRL, (temp & ALI_CACHE_FLUSH_ADDR_MASK) | page_to_phys(page) | ALI_CACHE_FLUSH_EN);
    }
    agp_generic_destroy_page(page, flags);
}

static ali_generic_sizes: [aper_size_info_32; 7] = [
    aper_size_info_32 { size: 256, num_entries: 65536, page_order: 6, size_value: 10 },
    aper_size_info_32 { size: 128, num_entries: 32768, page_order: 5, size_value: 9 },
    aper_size_info_32 { size: 64, num_entries: 16384, page_order: 4, size_value: 8 },
    aper_size_info_32 { size: 32, num_entries: 8192, page_order: 3, size_value: 7 },
    aper_size_info_32 { size: 16, num_entries: 4096, page_order: 2, size_value: 6 },
    aper_size_info_32 { size: 8, num_entries: 2048, page_order: 1, size_value: 4 },
    aper_size_info_32 { size: 4, num_entries: 1024, page_order: 0, size_value: 3 },
];

// The bridge-driver objects, PCI tables, module registration, and external declarations
// retain the source interfaces and are supplied by the surrounding kernel bindings.
// C preprocessor/module metadata is intentionally represented as source-level comments.

extern "C" {
    static mut ali_generic_bridge: agp_bridge_driver;
    static mut ali_m1541_bridge: agp_bridge_driver;
}

#[repr(C)]
struct agp_device_ids {
    device_id: u16,
    chipset_name: *const u8,
}

static mut ali_agp_device_ids: [agp_device_ids; 12] = [
    agp_device_ids { device_id: PCI_DEVICE_ID_AL_M1541, chipset_name: b"M1541\0".as_ptr() },
    agp_device_ids { device_id: PCI_DEVICE_ID_AL_M1621, chipset_name: b"M1621\0".as_ptr() },
    agp_device_ids { device_id: PCI_DEVICE_ID_AL_M1631, chipset_name: b"M1631\0".as_ptr() },
    agp_device_ids { device_id: PCI_DEVICE_ID_AL_M1632, chipset_name: b"M1632\0".as_ptr() },
    agp_device_ids { device_id: PCI_DEVICE_ID_AL_M1641, chipset_name: b"M1641\0".as_ptr() },
    agp_device_ids { device_id: PCI_DEVICE_ID_AL_M1644, chipset_name: b"M1644\0".as_ptr() },
    agp_device_ids { device_id: PCI_DEVICE_ID_AL_M1647, chipset_name: b"M1647\0".as_ptr() },
    agp_device_ids { device_id: PCI_DEVICE_ID_AL_M1651, chipset_name: b"M1651\0".as_ptr() },
    agp_device_ids { device_id: PCI_DEVICE_ID_AL_M1671, chipset_name: b"M1671\0".as_ptr() },
    agp_device_ids { device_id: PCI_DEVICE_ID_AL_M1681, chipset_name: b"M1681\0".as_ptr() },
    agp_device_ids { device_id: PCI_DEVICE_ID_AL_M1683, chipset_name: b"M1683\0".as_ptr() },
    agp_device_ids { device_id: 0, chipset_name: core::ptr::null() },
];

unsafe fn agp_ali_probe(pdev: *mut pci_dev, _ent: *const pci_device_id) -> i32 {
    let mut cap_ptr: u8;
    let mut j = 0usize;
    cap_ptr = pci_find_capability(pdev, PCI_CAP_ID_AGP);
    if cap_ptr == 0 { return -ENODEV; }
    while !ali_agp_device_ids[j].chipset_name.is_null() {
        if (*pdev).device == ali_agp_device_ids[j].device_id { break; }
        j += 1;
    }
    if ali_agp_device_ids[j].chipset_name.is_null() { return -ENODEV; }
    let bridge = agp_alloc_bridge();
    if bridge.is_null() { return -ENOMEM; }
    (*bridge).dev = pdev;
    (*bridge).capndx = cap_ptr;
    match (*pdev).device {
        PCI_DEVICE_ID_AL_M1541 => (*bridge).driver = &mut ali_m1541_bridge,
        PCI_DEVICE_ID_AL_M1621 => {
            let mut hidden_1621_id = 0u8;
            pci_read_config_byte(pdev, 0xFB, &mut hidden_1621_id);
            let name = match hidden_1621_id {
                0x31 => b"M1631\0", 0x32 => b"M1632\0", 0x41 => b"M1641\0",
                0x43 => b"M1621\0", 0x47 => b"M1647\0", 0x51 => b"M1651\0", _ => core::ptr::null(),
            };
            if !name.is_null() { ali_agp_device_ids[j].chipset_name = name.as_ptr(); }
            (*bridge).driver = &mut ali_generic_bridge;
        },
        _ => (*bridge).driver = &mut ali_generic_bridge,
    }
    pci_read_config_dword(pdev, (*bridge).capndx as u32 + PCI_AGP_STATUS, &mut (*bridge).mode);
    pci_set_drvdata(pdev, bridge);
    agp_add_bridge(bridge)
}

unsafe fn agp_ali_remove(pdev: *mut pci_dev) {
    let bridge = pci_get_drvdata(pdev);
    agp_remove_bridge(bridge);
    agp_put_bridge(bridge);
}

extern "C" {
    static mut agp_ali_pci_table: [pci_device_id; 2];
    static mut agp_ali_pci_driver: pci_driver;
}

unsafe fn agp_ali_init() -> i32 {
    if agp_off { return -EINVAL; }
    pci_register_driver(&mut agp_ali_pci_driver)
}

unsafe fn agp_ali_cleanup() {
    pci_unregister_driver(&mut agp_ali_pci_driver);
}

// MODULE_DEVICE_TABLE(pci, agp_ali_pci_table);
// module_init(agp_ali_init);
// module_exit(agp_ali_cleanup);
// MODULE_AUTHOR("Dave Jones");
// MODULE_DESCRIPTION("ALi AGPGART routines");
// MODULE_LICENSE("GPL and additional rights");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
