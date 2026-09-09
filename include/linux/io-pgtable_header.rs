/* SPDX-License-Identifier: GPL-2.0 */

// Translated from io-pgtable.h. Linux-provided types and macros are external
// dependencies of this header.

#[repr(C)]
#[derive(Copy, Clone)]
pub enum io_pgtable_fmt {
    ARM_32_LPAE_S1,
    ARM_32_LPAE_S2,
    ARM_64_LPAE_S1,
    ARM_64_LPAE_S2,
    ARM_V7S,
    ARM_MALI_LPAE,
    APPLE_DART,
    APPLE_DART2,
    IO_PGTABLE_NUM_FMTS,
}

#[repr(C)]
pub struct iommu_flush_ops {
    pub tlb_flush_all: Option<unsafe extern "C" fn(cookie: *mut core::ffi::c_void)>,
    pub tlb_flush_walk: Option<unsafe extern "C" fn(iova: libc::c_ulong, size: usize, granule: usize, cookie: *mut core::ffi::c_void)>,
    pub tlb_add_page: Option<unsafe extern "C" fn(gather: *mut iommu_iotlb_gather, iova: libc::c_ulong, granule: usize, cookie: *mut core::ffi::c_void)>,
}

#[repr(C)]
pub struct io_pgtable_cfg {
    pub quirks: libc::c_ulong,
    pub pgsize_bitmap: libc::c_ulong,
    pub ias: core::ffi::c_uint,
    pub oas: core::ffi::c_uint,
    pub coherent_walk: bool,
    pub tlb: *const iommu_flush_ops,
    pub iommu_dev: *mut device,
    pub alloc: Option<unsafe extern "C" fn(cookie: *mut core::ffi::c_void, size: usize, gfp: gfp_t) -> *mut core::ffi::c_void>,
    pub free: Option<unsafe extern "C" fn(cookie: *mut core::ffi::c_void, pages: *mut core::ffi::c_void, size: usize)>,
    pub __bindgen_anon_1: io_pgtable_cfg_union,
}

pub const IO_PGTABLE_QUIRK_ARM_NS: libc::c_ulong = 1 << 0;
pub const IO_PGTABLE_QUIRK_NO_PERMS: libc::c_ulong = 1 << 1;
pub const IO_PGTABLE_QUIRK_ARM_MTK_EXT: libc::c_ulong = 1 << 3;
pub const IO_PGTABLE_QUIRK_ARM_MTK_TTBR_EXT: libc::c_ulong = 1 << 4;
pub const IO_PGTABLE_QUIRK_ARM_TTBR1: libc::c_ulong = 1 << 5;
pub const IO_PGTABLE_QUIRK_ARM_OUTER_WBWA: libc::c_ulong = 1 << 6;
pub const IO_PGTABLE_QUIRK_ARM_HD: libc::c_ulong = 1 << 7;
pub const IO_PGTABLE_QUIRK_ARM_S2FWB: libc::c_ulong = 1 << 8;
pub const IO_PGTABLE_QUIRK_NO_WARN: libc::c_ulong = 1 << 9;

#[repr(C)]
pub union io_pgtable_cfg_union {
    pub arm_lpae_s1_cfg: arm_lpae_s1_cfg,
    pub arm_lpae_s2_cfg: arm_lpae_s2_cfg,
    pub arm_v7s_cfg: arm_v7s_cfg,
    pub arm_mali_lpae_cfg: arm_mali_lpae_cfg,
    pub apple_dart_cfg: apple_dart_cfg,
    pub amd: amd_cfg,
}
#[repr(C)] pub struct arm_lpae_s1_cfg { pub ttbr: u64, pub tcr: arm_lpae_s1_tcr, pub mair: u64 }
#[repr(C)] pub struct arm_lpae_s1_tcr { pub ips: u32, pub tg: u32, pub sh: u32, pub orgn: u32, pub irgn: u32, pub tsz: u32 }
#[repr(C)] pub struct arm_lpae_s2_cfg { pub vttbr: u64, pub vtcr: arm_lpae_s2_vtcr }
#[repr(C)] pub struct arm_lpae_s2_vtcr { pub ps: u32, pub tg: u32, pub sh: u32, pub orgn: u32, pub irgn: u32, pub sl: u32, pub tsz: u32 }
#[repr(C)] pub struct arm_v7s_cfg { pub ttbr: u32, pub tcr: u32, pub nmrr: u32, pub prrr: u32 }
#[repr(C)] pub struct arm_mali_lpae_cfg { pub transtab: u64, pub memattr: u64 }
#[repr(C)] pub struct apple_dart_cfg { pub ttbr: [u64; 4], pub n_ttbrs: u32, pub n_levels: u32 }
#[repr(C)] pub struct amd_cfg { pub nid: core::ffi::c_int }

#[repr(C)] pub struct arm_lpae_io_pgtable_walk_data { pub ptes: [u64; 4] }

#[repr(C)]
pub struct io_pgtable_ops {
    pub map_pages: Option<unsafe extern "C" fn(*mut io_pgtable_ops, libc::c_ulong, phys_addr_t, usize, usize, core::ffi::c_int, gfp_t, *mut usize) -> core::ffi::c_int>,
    pub unmap_pages: Option<unsafe extern "C" fn(*mut io_pgtable_ops, libc::c_ulong, usize, usize, *mut iommu_iotlb_gather) -> usize>,
    pub iova_to_phys: Option<unsafe extern "C" fn(*mut io_pgtable_ops, libc::c_ulong) -> phys_addr_t>,
    pub pgtable_walk: Option<unsafe extern "C" fn(*mut io_pgtable_ops, libc::c_ulong, *mut core::ffi::c_void) -> core::ffi::c_int>,
    pub read_and_clear_dirty: Option<unsafe extern "C" fn(*mut io_pgtable_ops, libc::c_ulong, usize, libc::c_ulong, *mut iommu_dirty_bitmap) -> core::ffi::c_int>,
}

extern "C" {
    pub fn alloc_io_pgtable_ops(fmt: io_pgtable_fmt, cfg: *mut io_pgtable_cfg, cookie: *mut core::ffi::c_void) -> *mut io_pgtable_ops;
    pub fn free_io_pgtable_ops(ops: *mut io_pgtable_ops);
}

#[repr(C)] pub struct io_pgtable { pub fmt: io_pgtable_fmt, pub cookie: *mut core::ffi::c_void, pub cfg: io_pgtable_cfg, pub ops: io_pgtable_ops }

#[inline]
pub unsafe fn io_pgtable_tlb_flush_all(iop: *mut io_pgtable) {
    if !(*iop).cfg.tlb.is_null() { if let Some(f) = (*(*iop).cfg.tlb).tlb_flush_all { f((*iop).cookie); } }
}
#[inline]
pub unsafe fn io_pgtable_tlb_flush_walk(iop: *mut io_pgtable, iova: libc::c_ulong, size: usize, granule: usize) {
    if !(*iop).cfg.tlb.is_null() { if let Some(f) = (*(*iop).cfg.tlb).tlb_flush_walk { f(iova, size, granule, (*iop).cookie); } }
}
#[inline]
pub unsafe fn io_pgtable_tlb_add_page(iop: *mut io_pgtable, gather: *mut iommu_iotlb_gather, iova: libc::c_ulong, granule: usize) {
    if !(*iop).cfg.tlb.is_null() { if let Some(f) = (*(*iop).cfg.tlb).tlb_add_page { f(gather, iova, granule, (*iop).cookie); } }
}

#[repr(C)] pub struct io_pgtable_init_fns { pub alloc: Option<unsafe extern "C" fn(*mut io_pgtable_cfg, *mut core::ffi::c_void) -> *mut io_pgtable>, pub free: Option<unsafe extern "C" fn(*mut io_pgtable)>, pub caps: u32 }
pub const IO_PGTABLE_CAP_CUSTOM_ALLOCATOR: u32 = 1 << 0;

extern "C" {
    pub static mut io_pgtable_arm_32_lpae_s1_init_fns: io_pgtable_init_fns;
    pub static mut io_pgtable_arm_32_lpae_s2_init_fns: io_pgtable_init_fns;
    pub static mut io_pgtable_arm_64_lpae_s1_init_fns: io_pgtable_init_fns;
    pub static mut io_pgtable_arm_64_lpae_s2_init_fns: io_pgtable_init_fns;
    pub static mut io_pgtable_arm_v7s_init_fns: io_pgtable_init_fns;
    pub static mut io_pgtable_arm_mali_lpae_init_fns: io_pgtable_init_fns;
    pub static mut io_pgtable_amd_iommu_v1_init_fns: io_pgtable_init_fns;
    pub static mut io_pgtable_amd_iommu_v2_init_fns: io_pgtable_init_fns;
    pub static mut io_pgtable_apple_dart_init_fns: io_pgtable_init_fns;
}

// External Linux types supplied by dependent headers: gfp_t, phys_addr_t,
// iommu_iotlb_gather, iommu_dirty_bitmap, and device.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
