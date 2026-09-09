/* Nvidia AGPGART routines. Rust translation of nvidia-agp.c. */

/* Kernel dependencies supplied by the surrounding translation unit. */

const NVIDIA_0_APSIZE: u32 = 0x80;
const NVIDIA_1_WBC: u32 = 0xf0;
const NVIDIA_2_GARTCTRL: u32 = 0xd0;
const NVIDIA_2_APBASE: u32 = 0xd8;
const NVIDIA_2_APLIMIT: u32 = 0xdc;
const NVIDIA_3_APBASE: u32 = 0x50;
const NVIDIA_3_APLIMIT: u32 = 0x54;

#[inline]
const fn nvidia_2_attbase(i: u32) -> u32 { 0xe0 + i * 4 }

#[repr(C)]
struct _nvidia_private {
    dev_1: *mut pci_dev,
    dev_2: *mut pci_dev,
    dev_3: *mut pci_dev,
    aperture: *mut u32,
    num_active_entries: i32,
    pg_offset: off_t,
    wbc_mask: u32,
}

static mut nvidia_private: _nvidia_private = _nvidia_private {
    dev_1: core::ptr::null_mut(), dev_2: core::ptr::null_mut(),
    dev_3: core::ptr::null_mut(), aperture: core::ptr::null_mut(),
    num_active_entries: 0, pg_offset: 0, wbc_mask: 0,
};

unsafe fn nvidia_fetch_size() -> i32 {
    let mut size_value: u8 = 0;
    pci_read_config_byte((*agp_bridge).dev, NVIDIA_0_APSIZE, &mut size_value);
    size_value &= 0x0f;
    let values = A_SIZE_8((*agp_bridge).driver.aperture_sizes);
    for i in 0..(*agp_bridge).driver.num_aperture_sizes {
        if size_value == (*values.add(i as usize)).size_value {
            (*agp_bridge).previous_size = values.add(i as usize) as *mut _;
            (*agp_bridge).current_size = values.add(i as usize) as *mut _;
            (*agp_bridge).aperture_size_idx = i;
            return (*values.add(i as usize)).size;
        }
    }
    0
}

const SYSCFG: u32 = 0xC0010010;
const IORR_BASE0: u32 = 0xC0010016;
const IORR_MASK0: u32 = 0xC0010017;
const AMD_K7_NUM_IORR: u32 = 2;

unsafe fn nvidia_init_iorr(base: u32, size: u32) -> i32 {
    let (mut base_hi, mut base_lo, mut mask_hi, mut mask_lo) = (0u32, 0u32, 0u32, 0u32);
    let (mut sys_hi, mut sys_lo) = (0u32, 0u32);
    let mut free_iorr_addr = AMD_K7_NUM_IORR;
    let mut iorr_addr = 0;
    while iorr_addr < AMD_K7_NUM_IORR {
        rdmsr(IORR_BASE0 + 2 * iorr_addr, &mut base_lo, &mut base_hi);
        rdmsr(IORR_MASK0 + 2 * iorr_addr, &mut mask_lo, &mut mask_hi);
        if (base_lo & 0xfffff000) == (base & 0xfffff000) { break; }
        if (mask_lo & 0x00000800) == 0 { free_iorr_addr = iorr_addr; }
        iorr_addr += 1;
    }
    if iorr_addr >= AMD_K7_NUM_IORR {
        iorr_addr = free_iorr_addr;
        if iorr_addr >= AMD_K7_NUM_IORR { return -EINVAL; }
    }
    base_hi = 0; base_lo = (base & !0xfff) | 0x18;
    mask_hi = 0xf; mask_lo = (!(size - 1) & 0xfffff000) | 0x800;
    wrmsr(IORR_BASE0 + 2 * iorr_addr, base_lo, base_hi);
    wrmsr(IORR_MASK0 + 2 * iorr_addr, mask_lo, mask_hi);
    rdmsr(SYSCFG, &mut sys_lo, &mut sys_hi);
    sys_lo |= 0x00100000;
    wrmsr(SYSCFG, sys_lo, sys_hi);
    0
}

unsafe fn nvidia_configure() -> i32 {
    let current_size = A_SIZE_8((*agp_bridge).current_size);
    pci_write_config_byte((*agp_bridge).dev, NVIDIA_0_APSIZE, (*current_size).size_value);
    let apbase = pci_bus_address((*agp_bridge).dev, AGP_APERTURE_BAR);
    (*agp_bridge).gart_bus_addr = apbase;
    let aplimit = apbase + ((*current_size).size as u32 * 1024 * 1024) - 1;
    pci_write_config_dword(nvidia_private.dev_2, NVIDIA_2_APBASE, apbase);
    pci_write_config_dword(nvidia_private.dev_2, NVIDIA_2_APLIMIT, aplimit);
    pci_write_config_dword(nvidia_private.dev_3, NVIDIA_3_APBASE, apbase);
    pci_write_config_dword(nvidia_private.dev_3, NVIDIA_3_APLIMIT, aplimit);
    let rc = nvidia_init_iorr(apbase, (*current_size).size as u32 * 1024 * 1024);
    if rc != 0 { return rc; }
    let mut num_dirs = (*current_size).size / 64;
    nvidia_private.num_active_entries = (*current_size).num_entries;
    nvidia_private.pg_offset = 0;
    if num_dirs == 0 {
        num_dirs = 1;
        nvidia_private.num_active_entries /= 64 / (*current_size).size;
        nvidia_private.pg_offset = ((apbase & (64 * 1024 * 1024 - 1) & !((*current_size).size as u32 * 1024 * 1024 - 1)) as off_t) / PAGE_SIZE;
    }
    for i in 0..8 { pci_write_config_dword(nvidia_private.dev_2, nvidia_2_attbase(i), ((*agp_bridge).gatt_bus_addr + (i % num_dirs) * 64 * 1024) | 1); }
    let mut temp = 0u32;
    pci_read_config_dword(nvidia_private.dev_2, NVIDIA_2_GARTCTRL, &mut temp);
    pci_write_config_dword(nvidia_private.dev_2, NVIDIA_2_GARTCTRL, temp | 0x11);
    pci_read_config_dword((*agp_bridge).dev, NVIDIA_0_APSIZE, &mut temp);
    pci_write_config_dword((*agp_bridge).dev, NVIDIA_0_APSIZE, temp | 0x100);
    nvidia_private.aperture = ioremap(pci_resource_start((*agp_bridge).dev, AGP_APERTURE_BAR), 33 * PAGE_SIZE) as *mut u32;
    if nvidia_private.aperture.is_null() { return -ENOMEM; }
    0
}

unsafe fn nvidia_cleanup() {
    let mut temp = 0u32;
    pci_read_config_dword((*agp_bridge).dev, NVIDIA_0_APSIZE, &mut temp);
    pci_write_config_dword((*agp_bridge).dev, NVIDIA_0_APSIZE, temp & !0x100);
    pci_read_config_dword(nvidia_private.dev_2, NVIDIA_2_GARTCTRL, &mut temp);
    pci_write_config_dword(nvidia_private.dev_2, NVIDIA_2_GARTCTRL, temp & !0x11);
    iounmap(nvidia_private.aperture as *mut core::ffi::c_void);
    let previous_size = A_SIZE_8((*agp_bridge).previous_size);
    pci_write_config_byte((*agp_bridge).dev, NVIDIA_0_APSIZE, (*previous_size).size_value);
    nvidia_init_iorr((*agp_bridge).gart_bus_addr, (*previous_size).size as u32 * 1024 * 1024);
}

extern "C" {
    static mut agp_bridge: *mut agp_bridge_data;
    static mut agp_memory_reserved: i32;
}

unsafe fn nvidia_insert_memory(mem: *mut agp_memory, pg_start: off_t, type_: i32) -> i32 {
    let mask_type = agp_generic_type_to_mask_type((*mem).bridge, type_);
    if mask_type != 0 || type_ != (*mem).type_ { return -EINVAL; }
    if (*mem).page_count == 0 { return 0; }
    if pg_start + (*mem).page_count as off_t > (nvidia_private.num_active_entries as off_t - agp_memory_reserved as off_t / PAGE_SIZE) { return -EINVAL; }
    for j in pg_start..pg_start + (*mem).page_count as off_t {
        if !PGE_EMPTY(agp_bridge, readl((*agp_bridge).gatt_table.offset(nvidia_private.pg_offset + j))) { return -EBUSY; }
    }
    if !(*mem).is_flushed { global_cache_flush(); (*mem).is_flushed = true; }
    let mut j = pg_start;
    for i in 0..(*mem).page_count { writel((*agp_bridge).driver.mask_memory(agp_bridge, page_to_phys(*(*mem).pages.add(i)), mask_type), (*agp_bridge).gatt_table.offset(nvidia_private.pg_offset + j)); j += 1; }
    readl((*agp_bridge).gatt_table.offset(nvidia_private.pg_offset + j - 1));
    ((*agp_bridge).driver.tlb_flush)(mem); 0
}

unsafe fn nvidia_remove_memory(mem: *mut agp_memory, pg_start: off_t, type_: i32) -> i32 {
    let mask_type = agp_generic_type_to_mask_type((*mem).bridge, type_);
    if mask_type != 0 || type_ != (*mem).type_ { return -EINVAL; }
    if (*mem).page_count == 0 { return 0; }
    for i in pg_start..pg_start + (*mem).page_count as off_t { writel((*agp_bridge).scratch_page, (*agp_bridge).gatt_table.offset(nvidia_private.pg_offset + i)); }
    ((*agp_bridge).driver.tlb_flush)(mem); 0
}

unsafe fn nvidia_tlbflush(_mem: *mut agp_memory) {
    if nvidia_private.wbc_mask != 0 {
        let mut wbc_reg = 0u32;
        pci_read_config_dword(nvidia_private.dev_1, NVIDIA_1_WBC, &mut wbc_reg);
        pci_write_config_dword(nvidia_private.dev_1, NVIDIA_1_WBC, wbc_reg | nvidia_private.wbc_mask);
        let end = jiffies + 3 * HZ;
        loop { pci_read_config_dword(nvidia_private.dev_1, NVIDIA_1_WBC, &mut wbc_reg); if time_before_eq(end, jiffies) { printk(KERN_ERR, "TLB flush took more than 3 seconds.\n"); } if wbc_reg & nvidia_private.wbc_mask == 0 { break; } }
    }
    let mut temp: u32;
    for _ in 0..33 { temp = readl(nvidia_private.aperture.offset(PAGE_SIZE / core::mem::size_of::<u32>())); core::hint::black_box(temp); }
    for _ in 0..33 { temp = readl(nvidia_private.aperture.offset(PAGE_SIZE / core::mem::size_of::<u32>())); core::hint::black_box(temp); }
}

static nvidia_generic_sizes: [aper_size_info_8; 5] = [
    aper_size_info_8 { size: 512, num_entries: 131072, page_order: 7, size_value: 0 },
    aper_size_info_8 { size: 256, num_entries: 65536, page_order: 6, size_value: 8 },
    aper_size_info_8 { size: 128, num_entries: 32768, page_order: 5, size_value: 12 },
    aper_size_info_8 { size: 64, num_entries: 16384, page_order: 4, size_value: 14 },
    aper_size_info_8 { size: 32, num_entries: 16384, page_order: 4, size_value: 15 },
];
static nvidia_generic_masks: [gatt_mask; 1] = [gatt_mask { mask: 1, type_: 0 }];

static nvidia_driver: agp_bridge_driver = agp_bridge_driver {
    owner: THIS_MODULE, aperture_sizes: nvidia_generic_sizes.as_ptr(), size_type: U8_APER_SIZE,
    num_aperture_sizes: 5, needs_scratch_page: true, configure: Some(nvidia_configure),
    fetch_size: Some(nvidia_fetch_size), cleanup: Some(nvidia_cleanup), tlb_flush: Some(nvidia_tlbflush),
    mask_memory: Some(agp_generic_mask_memory), masks: nvidia_generic_masks.as_ptr(),
    agp_enable: Some(agp_generic_enable), cache_flush: Some(global_cache_flush),
    create_gatt_table: Some(agp_generic_create_gatt_table), free_gatt_table: Some(agp_generic_free_gatt_table),
    insert_memory: Some(nvidia_insert_memory), remove_memory: Some(nvidia_remove_memory),
    alloc_by_type: Some(agp_generic_alloc_by_type), free_by_type: Some(agp_generic_free_by_type),
    agp_alloc_page: Some(agp_generic_alloc_page), agp_alloc_pages: Some(agp_generic_alloc_pages),
    agp_destroy_page: Some(agp_generic_destroy_page), agp_destroy_pages: Some(agp_generic_destroy_pages),
    agp_type_to_mask_type: Some(agp_generic_type_to_mask_type),
};

unsafe fn agp_nvidia_probe(pdev: *mut pci_dev, _ent: *const pci_device_id) -> i32 {
    nvidia_private.dev_1 = pci_get_domain_bus_and_slot(pci_domain_nr((*pdev).bus), (*(*pdev).bus).number as u32, PCI_DEVFN(0, 1));
    nvidia_private.dev_2 = pci_get_domain_bus_and_slot(pci_domain_nr((*pdev).bus), (*(*pdev).bus).number as u32, PCI_DEVFN(0, 2));
    nvidia_private.dev_3 = pci_get_domain_bus_and_slot(pci_domain_nr((*pdev).bus), (*(*pdev).bus).number as u32, PCI_DEVFN(30, 0));
    if nvidia_private.dev_1.is_null() || nvidia_private.dev_2.is_null() || nvidia_private.dev_3.is_null() { printk(KERN_INFO, "Detected an NVIDIA nForce/nForce2 chipset, but could not find the secondary devices.\n"); return -ENODEV; }
    let cap_ptr = pci_find_capability(pdev, PCI_CAP_ID_AGP); if cap_ptr == 0 { return -ENODEV; }
    match (*pdev).device {
        PCI_DEVICE_ID_NVIDIA_NFORCE => { printk(KERN_INFO, "Detected NVIDIA nForce chipset\n"); nvidia_private.wbc_mask = 0x00010000; },
        PCI_DEVICE_ID_NVIDIA_NFORCE2 => { printk(KERN_INFO, "Detected NVIDIA nForce2 chipset\n"); nvidia_private.wbc_mask = 0x80000000; },
        _ => { printk(KERN_ERR, "Unsupported NVIDIA chipset (device id: %04x)\n", (*pdev).device); return -ENODEV; }
    }
    let bridge = agp_alloc_bridge(); if bridge.is_null() { return -ENOMEM; }
    (*bridge).driver = &nvidia_driver; (*bridge).dev_private_data = &mut nvidia_private as *mut _ as *mut _; (*bridge).dev = pdev; (*bridge).capndx = cap_ptr;
    pci_read_config_dword(pdev, (*bridge).capndx + PCI_AGP_STATUS, &mut (*bridge).mode);
    pci_set_drvdata(pdev, bridge); agp_add_bridge(bridge)
}

unsafe fn agp_nvidia_remove(pdev: *mut pci_dev) { let bridge = pci_get_drvdata(pdev); agp_remove_bridge(bridge); agp_put_bridge(bridge); }
unsafe fn agp_nvidia_resume(_dev: *mut device) -> i32 { nvidia_configure(); 0 }

static agp_nvidia_pci_table: [pci_device_id; 3] = [
    pci_device_id { class_: PCI_CLASS_BRIDGE_HOST << 8, class_mask: !0, vendor: PCI_VENDOR_ID_NVIDIA, device: PCI_DEVICE_ID_NVIDIA_NFORCE, subvendor: PCI_ANY_ID, subdevice: PCI_ANY_ID },
    pci_device_id { class_: PCI_CLASS_BRIDGE_HOST << 8, class_mask: !0, vendor: PCI_VENDOR_ID_NVIDIA, device: PCI_DEVICE_ID_NVIDIA_NFORCE2, subvendor: PCI_ANY_ID, subdevice: PCI_ANY_ID },
    pci_device_id { class_: 0, class_mask: 0, vendor: 0, device: 0, subvendor: 0, subdevice: 0 },
];

static agp_nvidia_pci_driver: pci_driver = pci_driver { name: "agpgart-nvidia", id_table: agp_nvidia_pci_table.as_ptr(), probe: Some(agp_nvidia_probe), remove: Some(agp_nvidia_remove), pm: Some(agp_nvidia_resume) };

unsafe fn agp_nvidia_init() -> i32 { if agp_off { return -EINVAL; } pci_register_driver(&agp_nvidia_pci_driver) }
unsafe fn agp_nvidia_cleanup() { pci_unregister_driver(&agp_nvidia_pci_driver); pci_dev_put(nvidia_private.dev_1); pci_dev_put(nvidia_private.dev_2); pci_dev_put(nvidia_private.dev_3); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
