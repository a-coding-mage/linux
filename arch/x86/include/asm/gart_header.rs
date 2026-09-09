/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by the surrounding kernel translation unit:
// #include <asm/e820/api.h>

unsafe extern "C" {
    pub fn set_up_gart_resume(a: u32, b: u32);

    pub static mut fallback_aper_order: i32;
    pub static mut fallback_aper_force: i32;
    pub static mut fix_aperture: i32;
}

/* PTE bits. */
pub const GPTE_VALID: u32 = 1;
pub const GPTE_COHERENT: u32 = 2;

/* Aperture control register bits. */
pub const GARTEN: u32 = 1 << 0;
pub const DISGARTCPU: u32 = 1 << 4;
pub const DISGARTIO: u32 = 1 << 5;
pub const DISTLBWALKPRB: u32 = 1 << 6;

/* GART cache control register bits. */
pub const INVGART: u32 = 1 << 0;
pub const GARTPTEERR: u32 = 1 << 1;

/* K8 On-cpu GART registers */
pub const AMD64_GARTAPERTURECTL: u32 = 0x90;
pub const AMD64_GARTAPERTUREBASE: u32 = 0x94;
pub const AMD64_GARTTABLEBASE: u32 = 0x98;
pub const AMD64_GARTCACHECTL: u32 = 0x9c;

#[cfg(CONFIG_GART_IOMMU)]
unsafe extern "C" {
    pub static mut gart_iommu_aperture: i32;
    pub static mut gart_iommu_aperture_allowed: i32;
    pub static mut gart_iommu_aperture_disabled: i32;

    pub fn early_gart_iommu_check();
    pub fn gart_iommu_init() -> i32;
    pub fn gart_parse_options(options: *mut u8);
    pub fn gart_iommu_hole_init();
}

#[cfg(not(CONFIG_GART_IOMMU))]
pub const gart_iommu_aperture: i32 = 0;
#[cfg(not(CONFIG_GART_IOMMU))]
pub const gart_iommu_aperture_allowed: i32 = 0;
#[cfg(not(CONFIG_GART_IOMMU))]
pub const gart_iommu_aperture_disabled: i32 = 1;

#[cfg(not(CONFIG_GART_IOMMU))]
#[inline]
pub unsafe fn early_gart_iommu_check() {}
#[cfg(not(CONFIG_GART_IOMMU))]
#[inline]
pub unsafe fn gart_parse_options(_options: *mut u8) {}
#[cfg(not(CONFIG_GART_IOMMU))]
#[inline]
pub unsafe fn gart_iommu_hole_init() {}

unsafe extern "C" {
    pub fn agp_amd64_init() -> i32;
}

#[inline]
pub unsafe fn gart_set_size_and_enable(dev: *mut pci_dev, order: u32) {
    let ctl: u32;

    /*
     * Don't enable translation but enable GART IO and CPU accesses.
     * Also, set DISTLBWALKPRB since GART tables memory is UC.
     */
    ctl = order << 1;

    pci_write_config_dword(dev, AMD64_GARTAPERTURECTL, ctl);
}

#[inline]
pub unsafe fn enable_gart_translation(dev: *mut pci_dev, mut addr: u64) {
    let mut tmp: u32;
    let mut ctl: u32 = 0;

    /* address of the mappings table */
    addr >>= 12;
    tmp = addr as u32 << 4;
    tmp &= !0xf;
    pci_write_config_dword(dev, AMD64_GARTTABLEBASE, tmp);

    /* Enable GART translation for this hammer. */
    pci_read_config_dword(dev, AMD64_GARTAPERTURECTL, &mut ctl);
    ctl |= GARTEN | DISTLBWALKPRB;
    ctl &= !(DISGARTCPU | DISGARTIO);
    pci_write_config_dword(dev, AMD64_GARTAPERTURECTL, ctl);
}

#[inline]
pub unsafe fn aperture_valid(aper_base: u64, aper_size: u32, min_size: u32) -> i32 {
    if aper_base == 0 {
        return 0;
    }

    if aper_base.wrapping_add(aper_size as u64) > 0x1_0000_0000 {
        printk(KERN_INFO, b"Aperture beyond 4GB. Ignoring.\0".as_ptr());
        return 0;
    }
    if e820__mapped_any(
        aper_base,
        aper_base.wrapping_add(aper_size as u64),
        E820_TYPE_RAM,
    ) != 0 {
        printk(KERN_INFO, b"Aperture pointing to e820 RAM. Ignoring.\0".as_ptr());
        return 0;
    }
    if aper_size < min_size {
        printk(
            KERN_INFO,
            b"Aperture too small (%d MB) than (%d MB)\n\0".as_ptr(),
            aper_size >> 20,
            min_size >> 20,
        );
        return 0;
    }

    1
}

// Types, functions, constants, and configuration symbols below are supplied
// by the surrounding kernel translation unit.
unsafe extern "C" {
    pub fn pci_write_config_dword(dev: *mut pci_dev, where_: u32, value: u32);
    pub fn pci_read_config_dword(dev: *mut pci_dev, where_: u32, value: *mut u32);
    pub fn e820__mapped_any(start: u64, end: u64, type_: u32) -> i32;
    pub fn printk(fmt: *const u8, ...);
}

pub enum pci_dev {}

unsafe extern "C" {
    pub static KERN_INFO: *const u8;
    pub static E820_TYPE_RAM: u32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
