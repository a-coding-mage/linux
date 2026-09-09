// SPDX-License-Identifier: GPL-2.0
/* Firmware replacement code. */

// Kernel headers and configuration-dependent declarations are supplied by the surrounding translation unit.

const GART_MIN_ADDR: u64 = 512u64 << 20;
const GART_MAX_ADDR: u64 = 1u64 << 32;

pub static mut gart_iommu_aperture: i32 = 0;
pub static mut gart_iommu_aperture_disabled: i32 = 0;
pub static mut gart_iommu_aperture_allowed: i32 = 0;
pub static mut fallback_aper_order: i32 = 1;
pub static mut fallback_aper_force: i32 = 0;
pub static mut fix_aperture: i32 = 1;

extern "C" {
    fn memblock_phys_alloc_range(size: usize, align: usize, min: u64, max: u64) -> u64;
    fn register_nosave_region(start: u64, end: u64);
    fn read_pci_config_16(bus: i32, slot: i32, func: i32, reg: i32) -> u16;
    fn read_pci_config_byte(bus: i32, slot: i32, func: i32, reg: i32) -> u8;
    fn read_pci_config(bus: i32, slot: i32, func: i32, reg: i32) -> u32;
    fn write_pci_config(bus: i32, slot: i32, func: i32, reg: i32, value: u32);
    fn hweight16(v: u16) -> i32;
    fn aperture_valid(base: u64, size: u64, min_size: u64) -> bool;
    fn amd_gart_present() -> bool;
    fn early_pci_allowed() -> bool;
    fn early_is_amd_nb(id: u32) -> bool;
    fn e820__mapped_any(start: u64, end: u64, typ: u32) -> bool;
    fn e820__range_add(start: u64, size: u64, typ: u32);
    fn e820__update_table_print();
    fn gart_iommu_init();
    fn set_up_gart_resume(order: u32, base: u32);
    fn panic(msg: *const u8) -> !;
}

#[repr(C)]
pub struct AmdNbBusDevRange { pub bus: i32, pub dev_base: i32, pub dev_limit: i32 }
extern "C" {
    static mut amd_nb_bus_dev_ranges: *const AmdNbBusDevRange;
    static mut iommu_detected: i32;
    static mut x86_init: X86Init;
    static mut no_iommu: bool;
    static mut force_iommu: bool;
    static mut max_pfn: u64;
}
#[repr(C)] pub struct Iommu { pub iommu_init: Option<unsafe extern "C" fn()> }
#[repr(C)] pub struct X86Init { pub iommu: Iommu }

const PCI_STATUS: i32 = 0x06;
const PCI_STATUS_CAP_LIST: u16 = 0x10;
const PCI_CAPABILITY_LIST: i32 = 0x34;
const PCI_CAP_LIST_ID: i32 = 2;
const PCI_CAP_LIST_NEXT: i32 = 1;
const PCI_CAP_ID_AGP: i32 = 2;
const PCI_CLASS_REVISION: i32 = 0x08;
const PCI_CLASS_BRIDGE_HOST: u32 = 0x0600;
const PCI_CLASS_BRIDGE_OTHER: u32 = 0x0680;
const PCI_HEADER_TYPE: i32 = 0x0e;
const PCI_HEADER_TYPE_MFD: u8 = 0x80;
const AMD64_GARTAPERTURECTL: i32 = 0x90;
const AMD64_GARTAPERTUREBASE: i32 = 0x94;
const GARTEN: u32 = 1;
const MAX_DMA32_PFN: u64 = 0x100000;
const E820_TYPE_RAM: u32 = 1;
const E820_TYPE_RESERVED: u32 = 2;
const PAGE_SHIFT: u32 = 12;

static mut aperture_pfn_start: u64 = 0;
static mut aperture_page_count: u64 = 0;

unsafe fn exclude_from_core(aper_base: u64, aper_order: u32) {
    aperture_pfn_start = aper_base >> PAGE_SHIFT;
    aperture_page_count = ((32u64 * 1024 * 1024) << aper_order) >> PAGE_SHIFT;
}

unsafe fn allocate_aperture() -> u32 {
    if fallback_aper_order > 5 { fallback_aper_order = 5; }
    let aper_size: u64 = (32 * 1024 * 1024u64) << fallback_aper_order;
    let addr = memblock_phys_alloc_range(aper_size as usize, aper_size as usize, GART_MIN_ADDR, GART_MAX_ADDR);
    if addr == 0 { return 0; }
    register_nosave_region(addr >> PAGE_SHIFT, (addr + aper_size) >> PAGE_SHIFT);
    addr as u32
}

unsafe fn find_cap(bus: i32, slot: i32, func: i32, cap: i32) -> u32 {
    if read_pci_config_16(bus, slot, func, PCI_STATUS) & PCI_STATUS_CAP_LIST == 0 { return 0; }
    let mut pos = read_pci_config_byte(bus, slot, func, PCI_CAPABILITY_LIST);
    for _ in 0..48 {
        if pos < 0x40 { break; }
        pos &= !3;
        let id = read_pci_config_byte(bus, slot, func, pos as i32 + PCI_CAP_LIST_ID);
        if id == 0xff { break; }
        if id as i32 == cap { return pos as u32; }
        pos = read_pci_config_byte(bus, slot, func, pos as i32 + PCI_CAP_LIST_NEXT);
    }
    0
}

unsafe fn read_agp(bus: i32, slot: i32, func: i32, cap: u32, order: *mut u32) -> u32 {
    let apsizereg = read_pci_config_16(bus, slot, func, cap as i32 + 0x14);
    if apsizereg == 0xffff { return 0; }
    let old_order = *order;
    let mut apsize = (apsizereg as u32) & 0xfff;
    if apsize & 0xff != 0 { apsize |= 0xf00; }
    let nbits = hweight16(apsize as u16);
    *order = 7u32.saturating_sub(nbits as u32);
    let aper_low = read_pci_config(bus, slot, func, 0x10);
    let aper_hi = read_pci_config(bus, slot, func, 0x14);
    let aper = ((aper_low & !((1u32 << 22) - 1)) as u64) | ((aper_hi as u64) << 32);
    if aper + (32u64 << (20 + *order)) > 0x1_0000_0000 { *order = old_order; }
    if !aperture_valid(aper, (32u64 * 1024 * 1024) << *order, 32u64 << 20) { return 0; }
    aper as u32
}

unsafe fn search_agp_bridge(order: *mut u32, valid_agp: *mut i32) -> u32 {
    for bus in 0..256 { for slot in 0..32 { for func in 0..8 {
        let class = read_pci_config(bus, slot, func, PCI_CLASS_REVISION);
        if class == 0xffff_ffff { break; }
        match class >> 16 {
            PCI_CLASS_BRIDGE_HOST | PCI_CLASS_BRIDGE_OTHER => {
                let cap = find_cap(bus, slot, func, PCI_CAP_ID_AGP);
                if cap != 0 { *valid_agp = 1; return read_agp(bus, slot, func, cap, order); }
            }, _ => {}
        }
        let typ = read_pci_config_byte(bus, slot, func, PCI_HEADER_TYPE);
        if typ & PCI_HEADER_TYPE_MFD == 0 { break; }
    }}}
    0
}

pub unsafe fn early_gart_iommu_check() {
    if !amd_gart_present() || !early_pci_allowed() { return; }
    let mut agp_order = 0; let mut valid_agp = 0;
    search_agp_bridge(&mut agp_order, &mut valid_agp);
    let mut aper_enabled = 0; let mut last_enabled = 0; let mut last_order = 0; let mut last_base = 0; let mut last_valid = false; let mut aper_base = 0; let mut aper_size = 0; let mut fix = false;
    let mut i = 0;
    while amd_nb_bus_dev_ranges.add(i).read().dev_limit != 0 {
        let r = amd_nb_bus_dev_ranges.add(i).read();
        for slot in r.dev_base..r.dev_limit { if early_is_amd_nb(read_pci_config(r.bus, slot, 3, 0)) {
            let ctl = read_pci_config(r.bus, slot, 3, AMD64_GARTAPERTURECTL); aper_enabled = (ctl & GARTEN) as i32; let order = (ctl >> 1) & 7; aper_size = (32u32 * 1024 * 1024) << order; aper_base = ((read_pci_config(r.bus, slot, 3, AMD64_GARTAPERTUREBASE) & 0x7fff) as u64) << 25;
            if last_valid && (order != last_order || aper_base != last_base || aper_enabled != last_enabled) { fix = true; break; } last_order = order; last_base = aper_base; last_enabled = aper_enabled; last_valid = true;
        }} i += 1;
    }
    if !fix && aper_enabled == 0 { return; }
    if aper_base == 0 || aper_size == 0 || aper_base + aper_size as u64 > 0x1_0000_0000 { fix = true; }
    if valid_agp == 0 { return; }
    if fix { return; }
}

pub unsafe fn gart_iommu_hole_init() {
    if !amd_gart_present() || gart_iommu_aperture_disabled != 0 || fix_aperture == 0 || !early_pci_allowed() { return; }
    let mut agp_order = 0; let mut valid_agp = 0; let agp_base = if fallback_aper_force == 0 { search_agp_bridge(&mut agp_order, &mut valid_agp) } else { 0 };
    let mut aper_alloc = agp_base; let mut aper_order = agp_order;
    if aper_alloc == 0 && ((!no_iommu && max_pfn > MAX_DMA32_PFN) || force_iommu || valid_agp != 0 || fallback_aper_force != 0) { aper_order = fallback_aper_order as u32; aper_alloc = allocate_aperture(); if aper_alloc == 0 { panic(b"Not enough memory for aperture\0".as_ptr()); } } else if aper_alloc == 0 { return; }
    exclude_from_core(aper_alloc as u64, aper_order);
    let mut i = 0; while amd_nb_bus_dev_ranges.add(i).read().dev_limit != 0 { let r = amd_nb_bus_dev_ranges.add(i).read(); let ctl = aper_order << 1; for slot in r.dev_base..r.dev_limit { if early_is_amd_nb(read_pci_config(r.bus, slot, 3, 0)) { write_pci_config(r.bus, slot, 3, AMD64_GARTAPERTURECTL, ctl); write_pci_config(r.bus, slot, 3, AMD64_GARTAPERTUREBASE, aper_alloc >> 25); }} i += 1; }
    set_up_gart_resume(aper_order, aper_alloc);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
