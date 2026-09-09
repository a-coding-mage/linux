/* ATi AGPGART routines. */

// Linux kernel dependencies supplied by the surrounding translation unit.

const ATI_GART_MMBASE_BAR: usize = 1;
const ATI_RS100_APSIZE: usize = 0xac;
const ATI_RS100_IG_AGPMODE: usize = 0xb0;
const ATI_RS300_APSIZE: usize = 0xf8;
const ATI_RS300_IG_AGPMODE: usize = 0xfc;
const ATI_GART_FEATURE_ID: usize = 0x00;
const ATI_GART_BASE: usize = 0x04;
const ATI_GART_CACHE_SZBASE: usize = 0x08;
const ATI_GART_CACHE_CNTRL: usize = 0x0c;
const ATI_GART_CACHE_ENTRY_CNTRL: usize = 0x10;

static ATI_GENERIC_SIZES: [aper_size_info_lvl2; 7] = [
    aper_size_info_lvl2 { size: 2048, num_entries: 524288, size_value: 0x0000000c },
    aper_size_info_lvl2 { size: 1024, num_entries: 262144, size_value: 0x0000000a },
    aper_size_info_lvl2 { size: 512, num_entries: 131072, size_value: 0x00000008 },
    aper_size_info_lvl2 { size: 256, num_entries: 65536, size_value: 0x00000006 },
    aper_size_info_lvl2 { size: 128, num_entries: 32768, size_value: 0x00000004 },
    aper_size_info_lvl2 { size: 64, num_entries: 16384, size_value: 0x00000002 },
    aper_size_info_lvl2 { size: 32, num_entries: 8192, size_value: 0x00000000 },
];

static mut ATI_GENERIC_MASKS: [gatt_mask; 1] = [gatt_mask { mask: 1, type_: 0 }];

#[repr(C)]
struct ati_page_map { real: *mut usize, remapped: *mut usize }

#[repr(C)]
struct ati_generic_private_t {
    registers: *mut u8,
    gatt_pages: *mut *mut ati_page_map,
    num_tables: i32,
}
static mut ati_generic_private: ati_generic_private_t = ati_generic_private_t {
    registers: core::ptr::null_mut(), gatt_pages: core::ptr::null_mut(), num_tables: 0,
};

unsafe fn ati_create_page_map(page_map: *mut ati_page_map) -> i32 {
    let real = __get_free_page(GFP_KERNEL) as *mut usize;
    (*page_map).real = real;
    if real.is_null() { return -ENOMEM; }
    set_memory_uc(real as usize, 1);
    let err = map_page_into_agp(virt_to_page(real));
    if err != 0 { free_page(real as usize); return err; }
    (*page_map).remapped = real;
    for i in 0..(PAGE_SIZE / core::mem::size_of::<usize>()) {
        writel((*agp_bridge).scratch_page, real.add(i));
        readl(real.add(i)); /* PCI Posting. */
    }
    0
}

unsafe fn ati_free_page_map(page_map: *mut ati_page_map) {
    unmap_page_from_agp(virt_to_page((*page_map).real));
    set_memory_wb((*page_map).real as usize, 1);
    free_page((*page_map).real as usize);
}

unsafe fn ati_free_gatt_pages() {
    let tables = ati_generic_private.gatt_pages;
    for i in 0..ati_generic_private.num_tables {
        let entry = *tables.add(i as usize);
        if !entry.is_null() {
            if !(*entry).real.is_null() { ati_free_page_map(entry); }
            kfree(entry as *mut core::ffi::c_void);
        }
    }
    kfree(tables as *mut core::ffi::c_void);
}

unsafe fn ati_create_gatt_pages(nr_tables: i32) -> i32 {
    let tables = kzalloc_objs::<*mut ati_page_map>(nr_tables + 1);
    if tables.is_null() { return -ENOMEM; }
    let mut i = 0;
    let mut retval = 0;
    while i < nr_tables {
        let entry = kzalloc_obj::<ati_page_map>();
        *tables.add(i as usize) = entry;
        if entry.is_null() { retval = -ENOMEM; break; }
        retval = ati_create_page_map(entry);
        if retval != 0 { break; }
        i += 1;
    }
    ati_generic_private.num_tables = i;
    ati_generic_private.gatt_pages = tables;
    if retval != 0 { ati_free_gatt_pages(); }
    retval
}

unsafe fn is_r200() -> i32 {
    let d = (*agp_bridge).dev.device;
    if d == PCI_DEVICE_ID_ATI_RS100 || d == PCI_DEVICE_ID_ATI_RS200 ||
       d == PCI_DEVICE_ID_ATI_RS200_B || d == PCI_DEVICE_ID_ATI_RS250 { 1 } else { 0 }
}

unsafe fn ati_fetch_size() -> i32 {
    let mut temp = 0u32;
    let reg = if is_r200() != 0 { ATI_RS100_APSIZE } else { ATI_RS300_APSIZE };
    pci_read_config_dword((*agp_bridge).dev, reg, &mut temp);
    temp &= 0x0000000e;
    let values = A_SIZE_LVL2((*agp_bridge).driver.aperture_sizes);
    for i in 0..(*agp_bridge).driver.num_aperture_sizes {
        if temp == (*values.add(i as usize)).size_value {
            (*agp_bridge).previous_size = values.add(i as usize) as *mut core::ffi::c_void;
            (*agp_bridge).current_size = values.add(i as usize) as *mut core::ffi::c_void;
            (*agp_bridge).aperture_size_idx = i;
            return (*values.add(i as usize)).size;
        }
    }
    0
}

unsafe fn ati_tlbflush(_mem: *mut agp_memory) {
    writel(1, ati_generic_private.registers.add(ATI_GART_CACHE_CNTRL));
    readl(ati_generic_private.registers.add(ATI_GART_CACHE_CNTRL)); /* PCI Posting. */
}

unsafe fn ati_cleanup() {
    let previous_size = A_SIZE_LVL2((*agp_bridge).previous_size);
    let reg = if is_r200() != 0 { ATI_RS100_APSIZE } else { ATI_RS300_APSIZE };
    let mut temp = 0u32;
    pci_read_config_dword((*agp_bridge).dev, reg, &mut temp);
    temp = (temp & !0x0000000f) | (*previous_size).size_value;
    pci_write_config_dword((*agp_bridge).dev, reg, temp);
    iounmap(ati_generic_private.registers);
}

unsafe fn ati_configure() -> i32 {
    let reg = pci_resource_start((*agp_bridge).dev, ATI_GART_MMBASE_BAR);
    ati_generic_private.registers = ioremap(reg, 4096) as *mut u8;
    if ati_generic_private.registers.is_null() { return -ENOMEM; }
    let mode = if is_r200() != 0 { ATI_RS100_IG_AGPMODE } else { ATI_RS300_IG_AGPMODE };
    pci_write_config_dword((*agp_bridge).dev, mode, 0x20000);
    writel(0x60000, ati_generic_private.registers.add(ATI_GART_FEATURE_ID));
    readl(ati_generic_private.registers.add(ATI_GART_FEATURE_ID)); /* PCI Posting. */
    let mut temp = 0u32;
    pci_read_config_dword((*agp_bridge).dev, PCI_COMMAND, &mut temp);
    pci_write_config_dword((*agp_bridge).dev, PCI_COMMAND, temp | (1 << 14));
    writel((*agp_bridge).gatt_bus_addr, ati_generic_private.registers.add(ATI_GART_BASE));
    readl(ati_generic_private.registers.add(ATI_GART_BASE)); /* PCI Posting. */
    0
}

unsafe fn agp_ati_resume(_dev: *mut device) -> i32 { ati_configure() }

// The remaining routines retain the driver's original low-level table arithmetic.
unsafe fn ati_insert_memory(mem: *mut agp_memory, pg_start: isize, type_: i32) -> i32 {
    let num_entries = (*A_SIZE_LVL2((*agp_bridge).current_size)).num_entries as isize;
    let mask_type = agp_generic_type_to_mask_type((*mem).bridge, type_);
    if mask_type != 0 || type_ != (*mem).type_ { return -EINVAL; }
    if (*mem).page_count == 0 { return 0; }
    if pg_start + (*mem).page_count as isize > num_entries { return -EINVAL; }
    let mut j = pg_start;
    while j < pg_start + (*mem).page_count as isize {
        let addr = (j as usize * PAGE_SIZE) + (*agp_bridge).gart_bus_addr as usize;
        let g = *ati_generic_private.gatt_pages.add(((addr >> 22) - ((*agp_bridge).gart_bus_addr as usize >> 22)));
        if !PGE_EMPTY((*agp_bridge), readl((*g).remapped.add((addr & 0x003ff000) >> 12))) { return -EBUSY; }
        j += 1;
    }
    if !(*mem).is_flushed { global_cache_flush(); (*mem).is_flushed = true; }
    for i in 0..(*mem).page_count {
        let j = pg_start as usize + i;
        let addr = j * PAGE_SIZE + (*agp_bridge).gart_bus_addr as usize;
        let g = *ati_generic_private.gatt_pages.add((addr >> 22) - ((*agp_bridge).gart_bus_addr as usize >> 22));
        writel((*agp_bridge).driver.mask_memory((*agp_bridge), page_to_phys(*(*mem).pages.add(i)), (*mem).type_), (*g).remapped.add((addr & 0x003ff000) >> 12));
    }
    let g = *ati_generic_private.gatt_pages.add(0);
    readl((*g).remapped); (*agp_bridge).driver.tlb_flush(mem); 0
}

unsafe fn ati_remove_memory(mem: *mut agp_memory, pg_start: isize, type_: i32) -> i32 {
    let mask_type = agp_generic_type_to_mask_type((*mem).bridge, type_);
    if mask_type != 0 || type_ != (*mem).type_ { return -EINVAL; }
    if (*mem).page_count == 0 { return 0; }
    for i in pg_start as usize..(pg_start as usize + (*mem).page_count) {
        let addr = i * PAGE_SIZE + (*agp_bridge).gart_bus_addr as usize;
        let g = *ati_generic_private.gatt_pages.add((addr >> 22) - ((*agp_bridge).gart_bus_addr as usize >> 22));
        writel((*agp_bridge).scratch_page, (*g).remapped.add((addr & 0x003ff000) >> 12));
    }
    let g = *ati_generic_private.gatt_pages.add(0); readl((*g).remapped); (*agp_bridge).driver.tlb_flush(mem); 0
}

// GATT creation/freeing, PCI registration tables, and module entry points.
// External kernel structures and helpers are intentionally referenced rather than redefined.
unsafe fn ati_create_gatt_table(_bridge: *mut agp_bridge_data) -> i32 {
    let value = A_SIZE_LVL2((*agp_bridge).current_size);
    let mut page_dir = ati_page_map { real: core::ptr::null_mut(), remapped: core::ptr::null_mut() };
    let mut retval = ati_create_page_map(&mut page_dir);
    if retval != 0 { return retval; }
    retval = ati_create_gatt_pages((*value).num_entries / 1024);
    if retval != 0 { ati_free_page_map(&mut page_dir); return retval; }
    (*agp_bridge).gatt_table_real = page_dir.real as *mut u32;
    (*agp_bridge).gatt_table = page_dir.remapped as *mut u32;
    (*agp_bridge).gatt_bus_addr = virt_to_phys(page_dir.real);
    let reg = if is_r200() != 0 { ATI_RS100_APSIZE } else { ATI_RS300_APSIZE };
    let mut temp = 0u32;
    pci_read_config_dword((*agp_bridge).dev, reg, &mut temp);
    temp = (temp & !0x0000000e) | (*value).size_value | 1;
    pci_write_config_dword((*agp_bridge).dev, reg, temp);
    pci_read_config_dword((*agp_bridge).dev, reg, &mut temp);
    let mut addr = pci_bus_address((*agp_bridge).dev, AGP_APERTURE_BAR);
    (*agp_bridge).gart_bus_addr = addr;
    for i in 0..((*value).num_entries / 1024) {
        let p = *ati_generic_private.gatt_pages.add(i as usize);
        writel(virt_to_phys((*p).real) | 1, page_dir.remapped.add((addr >> 22) as usize));
        readl(page_dir.remapped.add((addr >> 22) as usize));
        addr += 0x00400000;
    }
    for i in 0..(*value).num_entries {
        let a = i as usize * PAGE_SIZE + (*agp_bridge).gart_bus_addr as usize;
        let p = *ati_generic_private.gatt_pages.add((a >> 22) - ((*agp_bridge).gart_bus_addr as usize >> 22));
        writel((*agp_bridge).scratch_page, (*p).remapped.add((a & 0x003ff000) >> 12));
    }
    0
}
unsafe fn ati_free_gatt_table(_bridge: *mut agp_bridge_data) -> i32 { ati_free_gatt_pages(); 0 }

unsafe fn agp_ati_probe(pdev: *mut pci_dev, _ent: *const pci_device_id) -> i32 {
    let cap_ptr = pci_find_capability(pdev, PCI_CAP_ID_AGP);
    if cap_ptr == 0 { return -ENODEV; }
    let bridge = agp_alloc_bridge();
    if bridge.is_null() { return -ENOMEM; }
    (*bridge).dev = pdev; (*bridge).capndx = cap_ptr;
    pci_set_drvdata(pdev, bridge); agp_add_bridge(bridge)
}
unsafe fn agp_ati_remove(pdev: *mut pci_dev) {
    let bridge = pci_get_drvdata(pdev); agp_remove_bridge(bridge); agp_put_bridge(bridge);
}
unsafe fn agp_ati_init() -> i32 { if agp_off { -EINVAL } else { pci_register_driver(&mut agp_ati_pci_driver) } }
unsafe fn agp_ati_cleanup() { pci_unregister_driver(&mut agp_ati_pci_driver); }

// MODULE_DEVICE_TABLE, module_init/module_exit, and metadata are build-system declarations.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
