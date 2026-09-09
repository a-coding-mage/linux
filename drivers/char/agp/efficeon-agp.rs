/* Transmeta's Efficeon AGPGART driver. Direct source-level Rust translation. */

const EFFICEON_ATTPAGE: u32 = 0xb8;
const EFFICEON_L1_SIZE: usize = 64;
const EFFICEON_PATI: u32 = 0 << 9;
const EFFICEON_PRESENT: u32 = 1 << 8;

#[repr(C)]
struct EfficeonPrivate {
    l1_table: [usize; EFFICEON_L1_SIZE],
}

static mut efficeon_private: EfficeonPrivate = EfficeonPrivate {
    l1_table: [0; EFFICEON_L1_SIZE],
};

static efficeon_generic_masks: [gatt_mask; 1] = [gatt_mask { mask: 0x00000001, type_: 0 }];

#[inline]
unsafe fn efficeon_mask_memory(page: *mut page) -> usize {
    page_to_phys(page) | 0x00000001
}

static efficeon_generic_sizes: [aper_size_info_lvl2; 4] = [
    aper_size_info_lvl2 { size: 256, num_entries: 65536, size_value: 0 },
    aper_size_info_lvl2 { size: 128, num_entries: 32768, size_value: 32 },
    aper_size_info_lvl2 { size: 64, num_entries: 16384, size_value: 48 },
    aper_size_info_lvl2 { size: 32, num_entries: 8192, size_value: 56 },
];

unsafe fn efficeon_fetch_size() -> i32 {
    let mut temp: u16 = 0;
    pci_read_config_word((*agp_bridge).dev, INTEL_APSIZE, &mut temp);
    let values = A_SIZE_LVL2((*(*agp_bridge).driver).aperture_sizes);
    for i in 0..(*(*agp_bridge).driver).num_aperture_sizes {
        if temp == (*values.add(i)).size_value {
            (*agp_bridge).previous_size = values.add(i) as *mut _;
            (*agp_bridge).current_size = values.add(i) as *mut _;
            (*agp_bridge).aperture_size_idx = i;
            return (*values.add(i)).size;
        }
    }
    0
}

unsafe fn efficeon_tlbflush(_mem: *mut agp_memory) {
    printk(KERN_DEBUG, PFX, "efficeon_tlbflush()\n");
    pci_write_config_dword((*agp_bridge).dev, INTEL_AGPCTRL, 0x2200);
    pci_write_config_dword((*agp_bridge).dev, INTEL_AGPCTRL, 0x2280);
}

unsafe fn efficeon_cleanup() {
    let mut temp: u16 = 0;
    let previous_size = A_SIZE_LVL2((*agp_bridge).previous_size);
    printk(KERN_DEBUG, PFX, "efficeon_cleanup()\n");
    pci_read_config_word((*agp_bridge).dev, INTEL_NBXCFG, &mut temp);
    pci_write_config_word((*agp_bridge).dev, INTEL_NBXCFG, temp & !(1 << 9));
    pci_write_config_word((*agp_bridge).dev, INTEL_APSIZE, (*previous_size).size_value);
}

unsafe fn efficeon_configure() -> i32 {
    let mut temp2: u16 = 0;
    let current_size = A_SIZE_LVL2((*agp_bridge).current_size);
    printk(KERN_DEBUG, PFX, "efficeon_configure()\n");
    pci_write_config_word((*agp_bridge).dev, INTEL_APSIZE, (*current_size).size_value);
    (*agp_bridge).gart_bus_addr = pci_bus_address((*agp_bridge).dev, AGP_APERTURE_BAR);
    pci_write_config_dword((*agp_bridge).dev, INTEL_AGPCTRL, 0x2280);
    pci_read_config_word((*agp_bridge).dev, INTEL_NBXCFG, &mut temp2);
    pci_write_config_word((*agp_bridge).dev, INTEL_NBXCFG, (temp2 & !(1 << 10)) | (1 << 9) | (1 << 11));
    pci_write_config_byte((*agp_bridge).dev, INTEL_ERRSTS + 1, 7);
    0
}

unsafe fn efficeon_free_gatt_table(_bridge: *mut agp_bridge_data) -> i32 {
    let mut freed = 0;
    for index in 0..EFFICEON_L1_SIZE {
        let page = efficeon_private.l1_table[index];
        if page != 0 {
            efficeon_private.l1_table[index] = 0;
            free_page(page);
            freed += 1;
        }
        printk(KERN_DEBUG, PFX, "efficeon_free_gatt_table(%p, %02x, %08x)\n", (*agp_bridge).dev, EFFICEON_ATTPAGE, index);
        pci_write_config_dword((*agp_bridge).dev, EFFICEON_ATTPAGE, index as u32);
    }
    printk(KERN_DEBUG, PFX, "efficeon_free_gatt_table() freed %d pages\n", freed);
    0
}

unsafe fn efficeon_create_gatt_table(_bridge: *mut agp_bridge_data) -> i32 {
    let pati = EFFICEON_PATI;
    let present = EFFICEON_PRESENT;
    let clflush_chunk = ((cpuid_ebx(1) >> 8) & 0xff) << 3;
    let num_entries = (*A_SIZE_LVL2((*agp_bridge).current_size)).num_entries;
    printk(KERN_DEBUG, PFX, "efficeon_create_gatt_table(%d)\n", num_entries);
    BUG_ON(num_entries & 0x3ff);
    let l1_pages = num_entries >> 10;
    for index in 0..l1_pages {
        BUG_ON(efficeon_private.l1_table[index] != 0);
        let page = get_zeroed_page(GFP_KERNEL);
        if page == 0 { efficeon_free_gatt_table(agp_bridge); return -ENOMEM; }
        let mut offset = 0;
        while offset < PAGE_SIZE { clflush((page as *mut u8).add(offset)); offset += clflush_chunk as usize; }
        efficeon_private.l1_table[index] = page;
        let value = virt_to_phys(page as *mut usize) as u32 | pati | present | index as u32;
        pci_write_config_dword((*agp_bridge).dev, EFFICEON_ATTPAGE, value);
    }
    0
}

unsafe fn efficeon_insert_memory(mem: *mut agp_memory, pg_start: isize, _type: i32) -> i32 {
    let count = (*mem).page_count;
    let clflush_chunk = ((cpuid_ebx(1) >> 8) & 0xff) << 3;
    let clflush_mask = !(clflush_chunk - 1);
    printk(KERN_DEBUG, PFX, "efficeon_insert_memory(%lx, %d)\n", pg_start, count);
    let num_entries = (*A_SIZE_LVL2((*agp_bridge).current_size)).num_entries;
    if pg_start + count as isize > num_entries as isize || (*mem).type_ != 0 { return -EINVAL; }
    if !(*mem).is_flushed { global_cache_flush(); (*mem).is_flushed = true; }
    let mut last_page: *mut u32 = core::ptr::null_mut();
    for i in 0..count {
        let index = pg_start as usize + i;
        let insert = efficeon_mask_memory(*(*mem).pages.add(i)) as u32;
        let page = efficeon_private.l1_table[index >> 10] as *mut u32;
        if page.is_null() { continue; }
        let page = page.add(index & 0x3ff);
        *page = insert;
        if !last_page.is_null() && (((page as usize ^ last_page as usize) & clflush_mask as usize) != 0) { clflush(last_page as *mut i8); }
        last_page = page;
    }
    if !last_page.is_null() { clflush(last_page as *mut i8); }
    ((*(*agp_bridge).driver).tlb_flush.unwrap())(mem);
    0
}

unsafe fn efficeon_remove_memory(mem: *mut agp_memory, pg_start: isize, _type: i32) -> i32 {
    let count = (*mem).page_count;
    printk(KERN_DEBUG, PFX, "efficeon_remove_memory(%lx, %d)\n", pg_start, count);
    let num_entries = (*A_SIZE_LVL2((*agp_bridge).current_size)).num_entries;
    if pg_start + count as isize > num_entries as isize || (*mem).type_ != 0 { return -EINVAL; }
    for i in 0..count { let index = pg_start as usize + i; let page = efficeon_private.l1_table[index >> 10] as *mut u32; if !page.is_null() { *page.add(index & 0x3ff) = 0; } }
    ((*(*agp_bridge).driver).tlb_flush.unwrap())(mem);
    0
}

/* External kernel types, constants, globals, and functions are supplied by the surrounding translation unit. */

static efficeon_driver: agp_bridge_driver = agp_bridge_driver {
    owner: THIS_MODULE, aperture_sizes: efficeon_generic_sizes.as_ptr(), size_type: LVL2_APER_SIZE,
    num_aperture_sizes: 4, configure: Some(efficeon_configure), fetch_size: Some(efficeon_fetch_size),
    cleanup: Some(efficeon_cleanup), tlb_flush: Some(efficeon_tlbflush), mask_memory: Some(agp_generic_mask_memory),
    masks: efficeon_generic_masks.as_ptr(), agp_enable: Some(agp_generic_enable), cache_flush: Some(global_cache_flush),
    create_gatt_table: Some(efficeon_create_gatt_table), free_gatt_table: Some(efficeon_free_gatt_table),
    insert_memory: Some(efficeon_insert_memory), remove_memory: Some(efficeon_remove_memory), cant_use_aperture: false,
    alloc_by_type: Some(agp_generic_alloc_by_type), free_by_type: Some(agp_generic_free_by_type),
    agp_alloc_page: Some(agp_generic_alloc_page), agp_alloc_pages: Some(agp_generic_alloc_pages),
    agp_destroy_page: Some(agp_generic_destroy_page), agp_destroy_pages: Some(agp_generic_destroy_pages),
    agp_type_to_mask_type: Some(agp_generic_type_to_mask_type),
};

unsafe fn agp_efficeon_probe(pdev: *mut pci_dev, _ent: *const pci_device_id) -> i32 {
    let cap_ptr = pci_find_capability(pdev, PCI_CAP_ID_AGP);
    if cap_ptr == 0 { return -ENODEV; }
    if (*pdev).device != PCI_DEVICE_ID_EFFICEON { printk(KERN_ERR, PFX, "Unsupported Efficeon chipset (device id: %04x)\n", (*pdev).device); return -ENODEV; }
    printk(KERN_INFO, PFX, "Detected Transmeta Efficeon TM8000 series chipset\n");
    let bridge = agp_alloc_bridge();
    if bridge.is_null() { return -ENOMEM; }
    (*bridge).driver = &efficeon_driver; (*bridge).dev = pdev; (*bridge).capndx = cap_ptr;
    if pci_enable_device(pdev) != 0 { printk(KERN_ERR, PFX, "Unable to Enable PCI device\n"); agp_put_bridge(bridge); return -ENODEV; }
    let r = &mut (*pdev).resource[0];
    if r.start == 0 && r.end != 0 && pci_assign_resource(pdev, 0) != 0 { printk(KERN_ERR, PFX, "could not assign resource 0\n"); agp_put_bridge(bridge); return -ENODEV; }
    pci_read_config_dword(pdev, (*bridge).capndx + PCI_AGP_STATUS, &mut (*bridge).mode);
    pci_set_drvdata(pdev, bridge);
    agp_add_bridge(bridge)
}

unsafe fn agp_efficeon_remove(pdev: *mut pci_dev) { let bridge = pci_get_drvdata(pdev); agp_remove_bridge(bridge); agp_put_bridge(bridge); }
unsafe fn agp_efficeon_resume(_dev: *mut device) -> i32 { printk(KERN_DEBUG, PFX, "agp_efficeon_resume()\n"); efficeon_configure() }

static agp_efficeon_pci_driver: pci_driver = pci_driver { name: "agpgart-efficeon", id_table: core::ptr::null(), probe: Some(agp_efficeon_probe), remove: Some(agp_efficeon_remove), pm: core::ptr::null() };

static mut agp_initialised: i32 = 0;
unsafe fn agp_efficeon_init() -> i32 { if agp_off { return -EINVAL; } if agp_initialised == 1 { return 0; } agp_initialised = 1; pci_register_driver(&agp_efficeon_pci_driver) }
unsafe fn agp_efficeon_module_cleanup() { pci_unregister_driver(&agp_efficeon_pci_driver); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
