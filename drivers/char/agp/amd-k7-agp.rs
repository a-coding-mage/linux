/* AMD K7 AGPGART routines. */

// Kernel dependencies supplied by the surrounding translation unit.

const AMD_MMBASE_BAR: usize = 1;
const AMD_APSIZE: usize = 0xac;
const AMD_MODECNTL: usize = 0xb0;
const AMD_MODECNTL2: usize = 0xb2;
const AMD_GARTENABLE: usize = 0x02;
const AMD_ATTBASE: usize = 0x04;
const AMD_TLBFLUSH: usize = 0x0c;
const AMD_CACHEENTRY: usize = 0x10;

struct amd_page_map {
    real: *mut usize,
    remapped: *mut usize,
}

struct _amd_irongate_private {
    registers: *mut u8,
    gatt_pages: *mut *mut amd_page_map,
    num_tables: i32,
}

static mut amd_irongate_private: _amd_irongate_private = _amd_irongate_private {
    registers: core::ptr::null_mut(), gatt_pages: core::ptr::null_mut(), num_tables: 0,
};

unsafe fn amd_create_page_map(page_map: *mut amd_page_map) -> i32 {
    let mut i: i32;
    (*page_map).real = __get_free_page(GFP_KERNEL) as *mut usize;
    if (*page_map).real.is_null() { return -ENOMEM; }
    set_memory_uc((*page_map).real as usize, 1);
    (*page_map).remapped = (*page_map).real;
    i = 0;
    while i < PAGE_SIZE / core::mem::size_of::<usize>() as i32 {
        writel((*agp_bridge).scratch_page, (*page_map).remapped.add(i as usize));
        readl((*page_map).remapped.add(i as usize));
        i += 1;
    }
    0
}

unsafe fn amd_free_page_map(page_map: *mut amd_page_map) {
    set_memory_wb((*page_map).real as usize, 1);
    free_page((*page_map).real as usize);
}

unsafe fn amd_free_gatt_pages() {
    let tables = amd_irongate_private.gatt_pages;
    let mut i = 0;
    while i < amd_irongate_private.num_tables {
        let entry = *tables.add(i as usize);
        if !entry.is_null() {
            if !(*entry).real.is_null() { amd_free_page_map(entry); }
            kfree(entry as *mut core::ffi::c_void);
        }
        i += 1;
    }
    kfree(tables as *mut core::ffi::c_void);
    amd_irongate_private.gatt_pages = core::ptr::null_mut();
}

unsafe fn amd_create_gatt_pages(nr_tables: i32) -> i32 {
    let tables = kzalloc_objs::<*mut amd_page_map>(nr_tables + 1);
    if tables.is_null() { return -ENOMEM; }
    let mut retval = 0;
    let mut i = 0;
    while i < nr_tables {
        let entry = kzalloc_obj::<amd_page_map>();
        *tables.add(i as usize) = entry;
        if entry.is_null() { retval = -ENOMEM; break; }
        retval = amd_create_page_map(entry);
        if retval != 0 { break; }
        i += 1;
    }
    amd_irongate_private.num_tables = i;
    amd_irongate_private.gatt_pages = tables;
    if retval != 0 { amd_free_gatt_pages(); }
    retval
}

unsafe fn amd_create_gatt_table(bridge: *mut agp_bridge_data) -> i32 {
    let value = A_SIZE_LVL2((*bridge).current_size);
    let mut page_dir = amd_page_map { real: core::ptr::null_mut(), remapped: core::ptr::null_mut() };
    let retval = amd_create_page_map(&mut page_dir);
    if retval != 0 { return retval; }
    let retval = amd_create_gatt_pages((*value).num_entries / 1024);
    if retval != 0 { amd_free_page_map(&mut page_dir); return retval; }
    (*bridge).gatt_table_real = page_dir.real as *mut u32;
    (*bridge).gatt_table = page_dir.remapped as *mut u32;
    (*bridge).gatt_bus_addr = virt_to_phys(page_dir.real);
    let mut addr = pci_bus_address((*bridge).dev, AGP_APERTURE_BAR);
    (*bridge).gart_bus_addr = addr;
    let mut i = 0;
    while i < (*value).num_entries / 1024 {
        writel(virt_to_phys((*amd_irongate_private.gatt_pages.add(i as usize)).real) | 1,
               page_dir.remapped.add((addr >> 22) as usize));
        readl(page_dir.remapped.add((addr >> 22) as usize));
        i += 1; addr += 0x00400000;
    }
    i = 0;
    while i < (*value).num_entries {
        addr = (i as usize * PAGE_SIZE as usize) as u64 + (*bridge).gart_bus_addr;
        let cur_gatt = *amd_irongate_private.gatt_pages.add(((addr >> 22) - ((*bridge).gart_bus_addr >> 22)) as usize);
        writel((*bridge).scratch_page, (*cur_gatt).remapped.add(((addr & 0x003ff000) >> 12) as usize));
        readl((*cur_gatt).remapped.add(((addr & 0x003ff000) >> 12) as usize));
        i += 1;
    }
    0
}

unsafe fn amd_free_gatt_table(bridge: *mut agp_bridge_data) -> i32 {
    let mut page_dir = amd_page_map { real: (*bridge).gatt_table_real as *mut usize, remapped: (*bridge).gatt_table as *mut usize };
    amd_free_gatt_pages(); amd_free_page_map(&mut page_dir); 0
}

unsafe fn amd_irongate_fetch_size() -> i32 {
    let mut temp = 0u32; pci_read_config_dword((*agp_bridge).dev, AMD_APSIZE, &mut temp);
    temp &= 0x0000000e; let values = A_SIZE_LVL2((*agp_bridge).driver).aperture_sizes;
    let mut i = 0; while i < (*agp_bridge).driver.num_aperture_sizes { if temp == (*values.add(i as usize)).size_value {
        (*agp_bridge).previous_size = (*agp_bridge).current_size = values.add(i as usize) as *mut _; (*agp_bridge).aperture_size_idx = i; return (*values.add(i as usize)).size; } i += 1; } 0
}

unsafe fn amd_irongate_configure() -> i32 {
    let current_size = A_SIZE_LVL2((*agp_bridge).current_size); let mut temp = 0u32;
    if amd_irongate_private.registers.is_null() { let reg = pci_resource_start((*agp_bridge).dev, AMD_MMBASE_BAR); amd_irongate_private.registers = ioremap(reg, 4096) as *mut u8; if amd_irongate_private.registers.is_null() { return -ENOMEM; } }
    writel((*agp_bridge).gatt_bus_addr, amd_irongate_private.registers.add(AMD_ATTBASE)); readl(amd_irongate_private.registers.add(AMD_ATTBASE));
    pci_write_config_byte((*agp_bridge).dev, AMD_MODECNTL, 0x80); pci_write_config_byte((*agp_bridge).dev, AMD_MODECNTL2, 0);
    let mut enable_reg = readw(amd_irongate_private.registers.add(AMD_GARTENABLE)); enable_reg |= 4; writew(enable_reg, amd_irongate_private.registers.add(AMD_GARTENABLE)); readw(amd_irongate_private.registers.add(AMD_GARTENABLE));
    pci_read_config_dword((*agp_bridge).dev, AMD_APSIZE, &mut temp); temp = (temp & !0x0000000e) | (*current_size).size_value | 1; pci_write_config_dword((*agp_bridge).dev, AMD_APSIZE, temp);
    writel(1, amd_irongate_private.registers.add(AMD_TLBFLUSH)); readl(amd_irongate_private.registers.add(AMD_TLBFLUSH)); 0
}

unsafe fn amd_irongate_cleanup() { let previous_size = A_SIZE_LVL2((*agp_bridge).previous_size); let mut enable_reg = readw(amd_irongate_private.registers.add(AMD_GARTENABLE)); enable_reg &= !4; writew(enable_reg, amd_irongate_private.registers.add(AMD_GARTENABLE)); readw(amd_irongate_private.registers.add(AMD_GARTENABLE)); let mut temp=0; pci_read_config_dword((*agp_bridge).dev, AMD_APSIZE, &mut temp); pci_write_config_dword((*agp_bridge).dev, AMD_APSIZE, (temp & !0xf)|(*previous_size).size_value); iounmap(amd_irongate_private.registers as *mut _); }
unsafe fn amd_irongate_tlbflush(_temp: *mut agp_memory) { writel(1, amd_irongate_private.registers.add(AMD_TLBFLUSH)); readl(amd_irongate_private.registers.add(AMD_TLBFLUSH)); }

unsafe fn amd_insert_memory(mem: *mut agp_memory, pg_start: i64, type_: i32) -> i32 {
    let num_entries = (*A_SIZE_LVL2((*agp_bridge).current_size)).num_entries;
    if type_ != (*mem).type_ || ((*agp_bridge).driver.agp_type_to_mask_type)(agp_bridge, type_) != 0 || pg_start + (*mem).page_count as i64 > num_entries as i64 { return -EINVAL; }
    let mut j = pg_start; while j < pg_start + (*mem).page_count as i64 { let addr = j as usize * PAGE_SIZE as usize + (*agp_bridge).gart_bus_addr as usize; let p = *amd_irongate_private.gatt_pages.add(((addr >> 22)-((*agp_bridge).gart_bus_addr as usize >> 22)) as usize); if !PGE_EMPTY(agp_bridge, readl((*p).remapped.add(((addr & 0x003ff000)>>12) as usize))) { return -EBUSY; } j+=1; }
    if !(*mem).is_flushed { global_cache_flush(); (*mem).is_flushed=true; }
    let mut i=0; j=pg_start; while i < (*mem).page_count { let addr=j as usize*PAGE_SIZE as usize+(*agp_bridge).gart_bus_addr as usize; let p=*amd_irongate_private.gatt_pages.add(((addr>>22)-((*agp_bridge).gart_bus_addr as usize>>22)) as usize); writel(agp_generic_mask_memory(agp_bridge,page_to_phys(*(*mem).pages.add(i as usize)),type_),(*p).remapped.add(((addr&0x003ff000)>>12) as usize)); readl((*p).remapped.add(((addr&0x003ff000)>>12) as usize)); i+=1;j+=1; } amd_irongate_tlbflush(mem); 0
}

unsafe fn amd_remove_memory(mem: *mut agp_memory, pg_start: i64, type_: i32) -> i32 { if type_ != (*mem).type_ || ((*agp_bridge).driver.agp_type_to_mask_type)(agp_bridge,type_) != 0 { return -EINVAL; } let mut i=pg_start; while i < (*mem).page_count as i64+pg_start { let addr=i as usize*PAGE_SIZE as usize+(*agp_bridge).gart_bus_addr as usize; let p=*amd_irongate_private.gatt_pages.add(((addr>>22)-((*agp_bridge).gart_bus_addr as usize>>22)) as usize); writel((*agp_bridge).scratch_page,(*p).remapped.add(((addr&0x003ff000)>>12) as usize)); readl((*p).remapped.add(((addr&0x003ff000)>>12) as usize)); i+=1; } amd_irongate_tlbflush(mem); 0 }

// The remaining driver-registration declarations and callback table are supplied by the kernel bindings.
// Their C definitions are preserved here as the corresponding external Rust interfaces.
extern "C" {
    static mut agp_bridge: *mut agp_bridge_data;
    fn __get_free_page(flags: i32) -> usize; fn set_memory_uc(addr: usize, pages: i32) -> i32; fn set_memory_wb(addr: usize, pages: i32) -> i32; fn free_page(addr: usize);
    fn writel(v: u32, p: *mut usize); fn readl(p: *mut usize) -> u32; fn readw(p: *mut u8) -> u16; fn writew(v:u16,p:*mut u8);
    fn pci_read_config_dword(d:*mut pci_dev,o:usize,v:*mut u32); fn pci_write_config_dword(d:*mut pci_dev,o:usize,v:u32); fn pci_write_config_byte(d:*mut pci_dev,o:usize,v:u8);
    fn pci_bus_address(d:*mut pci_dev,b:usize)->u64; fn pci_resource_start(d:*mut pci_dev,b:usize)->u64; fn ioremap(a:u64,s:usize)->*mut u8; fn iounmap(p:*mut core::ffi::c_void);
    fn virt_to_phys(p:*mut usize)->u64; fn kfree(p:*mut core::ffi::c_void); fn global_cache_flush(); fn page_to_phys(p:*mut core::ffi::c_void)->u64;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
