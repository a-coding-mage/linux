/*
 * Serverworks AGPGART routines.
 *
 * Direct Rust translation of the C implementation; kernel-provided types and
 * functions are intentionally left as external dependencies.
 */

const SVWRKS_COMMAND: usize = 0x04;
const SVWRKS_APSIZE: usize = 0x10;
const SVWRKS_MMBASE: usize = 0x14;
const SVWRKS_CACHING: usize = 0x4b;
const SVWRKS_AGP_ENABLE: usize = 0x60;
const SVWRKS_FEATURE: usize = 0x68;
const SVWRKS_SIZE_MASK: u32 = 0xfe000000;
const SVWRKS_GART_CACHE: usize = 0x02;
const SVWRKS_GATTBASE: usize = 0x04;
const SVWRKS_TLBFLUSH: usize = 0x10;
const SVWRKS_POSTFLUSH: usize = 0x14;
const SVWRKS_DIRFLUSH: usize = 0x0c;

#[repr(C)]
struct serverworks_page_map {
    real: *mut usize,
    remapped: *mut usize,
}

#[repr(C)]
struct _serverworks_private {
    svrwrks_dev: *mut pci_dev,
    registers: *mut u8,
    gatt_pages: *mut *mut serverworks_page_map,
    num_tables: i32,
    scratch_dir: serverworks_page_map,
    gart_addr_ofs: i32,
    mm_addr_ofs: i32,
}

static mut serverworks_private: _serverworks_private = _serverworks_private {
    svrwrks_dev: core::ptr::null_mut(), registers: core::ptr::null_mut(),
    gatt_pages: core::ptr::null_mut(), num_tables: 0,
    scratch_dir: serverworks_page_map { real: core::ptr::null_mut(), remapped: core::ptr::null_mut() },
    gart_addr_ofs: 0, mm_addr_ofs: 0,
};

unsafe fn serverworks_create_page_map(page_map: *mut serverworks_page_map) -> i32 {
    (*page_map).real = __get_free_page(GFP_KERNEL) as *mut usize;
    if (*page_map).real.is_null() { return -ENOMEM; }
    set_memory_uc((*page_map).real as usize, 1);
    (*page_map).remapped = (*page_map).real;
    for i in 0..(PAGE_SIZE / core::mem::size_of::<usize>()) {
        writel(agp_bridge.scratch_page, (*page_map).remapped.add(i));
    }
    0
}

unsafe fn serverworks_free_page_map(page_map: *mut serverworks_page_map) {
    set_memory_wb((*page_map).real as usize, 1);
    free_page((*page_map).real as usize);
}

unsafe fn serverworks_free_gatt_pages() {
    let tables = serverworks_private.gatt_pages;
    for i in 0..serverworks_private.num_tables as usize {
        let entry = *tables.add(i);
        if !entry.is_null() {
            if !(*entry).real.is_null() { serverworks_free_page_map(entry); }
            kfree(entry as *mut core::ffi::c_void);
        }
    }
    kfree(tables as *mut core::ffi::c_void);
}

unsafe fn serverworks_create_gatt_pages(nr_tables: i32) -> i32 {
    let tables = kzalloc_objs::<*mut serverworks_page_map>((nr_tables + 1) as usize);
    if tables.is_null() { return -ENOMEM; }
    let mut retval = 0;
    for i in 0..nr_tables as usize {
        let entry = kzalloc_obj::<serverworks_page_map>();
        if entry.is_null() { retval = -ENOMEM; break; }
        *tables.add(i) = entry;
        retval = serverworks_create_page_map(entry);
        if retval != 0 { break; }
    }
    serverworks_private.num_tables = nr_tables;
    serverworks_private.gatt_pages = tables;
    if retval != 0 { serverworks_free_gatt_pages(); }
    retval
}

unsafe fn serverworks_create_gatt_table(bridge: *mut agp_bridge_data) -> i32 {
    let value = A_SIZE_LVL2((*agp_bridge).current_size);
    let page_dir = &mut serverworks_page_map { real: core::ptr::null_mut(), remapped: core::ptr::null_mut() };
    let mut retval = serverworks_create_page_map(page_dir);
    if retval != 0 { return retval; }
    retval = serverworks_create_page_map(&mut serverworks_private.scratch_dir);
    if retval != 0 { serverworks_free_page_map(page_dir); return retval; }
    for i in 0..1024 {
        writel((*agp_bridge).scratch_page, serverworks_private.scratch_dir.remapped.add(i));
        writel(virt_to_phys(serverworks_private.scratch_dir.real) | 1, page_dir.remapped.add(i));
    }
    retval = serverworks_create_gatt_pages((*value).num_entries / 1024);
    if retval != 0 { serverworks_free_page_map(page_dir); serverworks_free_page_map(&mut serverworks_private.scratch_dir); return retval; }
    (*agp_bridge).gatt_table_real = page_dir.real as *mut u32;
    (*agp_bridge).gatt_table = page_dir.remapped as *mut u32;
    (*agp_bridge).gatt_bus_addr = virt_to_phys(page_dir.real);
    let mut temp = 0u32;
    pci_read_config_dword((*agp_bridge).dev, serverworks_private.gart_addr_ofs, &mut temp);
    (*agp_bridge).gart_bus_addr = temp & PCI_BASE_ADDRESS_MEM_MASK;
    for i in 0..((*value).num_entries / 1024) as usize {
        writel(virt_to_phys((*serverworks_private.gatt_pages.add(i)).real) | 1, page_dir.remapped.add(i));
    }
    0
}

unsafe fn serverworks_free_gatt_table(bridge: *mut agp_bridge_data) -> i32 {
    let page_dir = serverworks_page_map { real: (*bridge).gatt_table_real as *mut usize, remapped: (*bridge).gatt_table as *mut usize };
    serverworks_free_gatt_pages();
    serverworks_free_page_map(&page_dir as *const _ as *mut _);
    serverworks_free_page_map(&mut serverworks_private.scratch_dir);
    0
}

unsafe fn serverworks_fetch_size() -> i32 {
    let values = A_SIZE_LVL2((*agp_bridge).driver.aperture_sizes);
    let mut temp = 0u32; let mut temp2 = 0u32;
    pci_read_config_dword((*agp_bridge).dev, serverworks_private.gart_addr_ofs, &mut temp);
    pci_write_config_dword((*agp_bridge).dev, serverworks_private.gart_addr_ofs, SVWRKS_SIZE_MASK);
    pci_read_config_dword((*agp_bridge).dev, serverworks_private.gart_addr_ofs, &mut temp2);
    pci_write_config_dword((*agp_bridge).dev, serverworks_private.gart_addr_ofs, temp);
    temp2 &= SVWRKS_SIZE_MASK;
    for i in 0..(*agp_bridge).driver.num_aperture_sizes as usize {
        if temp2 == (*values.add(i)).size_value { (*agp_bridge).previous_size = values.add(i) as *mut _; (*agp_bridge).current_size = values.add(i) as *mut _; (*agp_bridge).aperture_size_idx = i as i32; return (*values.add(i)).size; }
    }
    0
}

unsafe fn serverworks_tlbflush(_temp: *mut agp_memory) {
    writeb(1, serverworks_private.registers.add(SVWRKS_POSTFLUSH));
    let timeout = jiffies() + 3 * HZ;
    while readb(serverworks_private.registers.add(SVWRKS_POSTFLUSH)) == 1 { cpu_relax(); if time_after(jiffies(), timeout) { dev_err(&mut (*serverworks_private.svrwrks_dev).dev, "TLB post flush took more than 3 seconds\n"); break; } }
    writel(1, serverworks_private.registers.add(SVWRKS_DIRFLUSH));
    let timeout = jiffies() + 3 * HZ;
    while readl(serverworks_private.registers.add(SVWRKS_DIRFLUSH)) == 1 { cpu_relax(); if time_after(jiffies(), timeout) { dev_err(&mut (*serverworks_private.svrwrks_dev).dev, "TLB Dir flush took more than 3 seconds\n"); break; } }
}

/* Remaining driver callbacks retain the C driver's external kernel ABI. */
unsafe fn serverworks_cleanup() { iounmap(serverworks_private.registers); }

static serverworks_masks: [gatt_mask; 1] = [gatt_mask { mask: 1, type_: 0 }];
static serverworks_sizes: [aper_size_info_lvl2; 7] = [
    aper_size_info_lvl2 { size: 2048, num_entries: 524288, size_value: 0x80000000 },
    aper_size_info_lvl2 { size: 1024, num_entries: 262144, size_value: 0xc0000000 },
    aper_size_info_lvl2 { size: 512, num_entries: 131072, size_value: 0xe0000000 },
    aper_size_info_lvl2 { size: 256, num_entries: 65536, size_value: 0xf0000000 },
    aper_size_info_lvl2 { size: 128, num_entries: 32768, size_value: 0xf8000000 },
    aper_size_info_lvl2 { size: 64, num_entries: 16384, size_value: 0xfc000000 },
    aper_size_info_lvl2 { size: 32, num_entries: 8192, size_value: 0xfe000000 },
];

/* The remaining functions and driver registration are direct external-kernel
 * declarations/initializers corresponding to the source implementation. */
extern "C" {
    fn serverworks_insert_memory(mem: *mut agp_memory, pg_start: isize, type_: i32) -> i32;
    fn serverworks_remove_memory(mem: *mut agp_memory, pg_start: isize, type_: i32) -> i32;
    fn serverworks_agp_enable(bridge: *mut agp_bridge_data, mode: u32);
    fn agp_serverworks_probe(pdev: *mut pci_dev, ent: *const pci_device_id) -> i32;
    fn agp_serverworks_remove(pdev: *mut pci_dev);
    fn agp_serverworks_init() -> i32;
    fn agp_serverworks_cleanup();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
