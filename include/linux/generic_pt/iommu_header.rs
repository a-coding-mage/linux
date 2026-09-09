/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright (c) 2024-2025, NVIDIA CORPORATION & AFFILIATES */

// Dependencies supplied by the corresponding kernel translation units:
// linux/generic_pt/common.h, linux/iommu.h, linux/mm_types.h

pub struct iommu_iotlb_gather;
pub struct pt_iommu_ops;
pub struct pt_iommu_driver_ops;
pub struct iommu_dirty_bitmap;

/**
 * DOC: IOMMU Radix Page Table
 *
 * The IOMMU implementation of the Generic Page Table provides an ops struct
 * that is useful to go with an iommu_domain to serve the DMA API, IOMMUFD and
 * the generic map/unmap interface.
 *
 * This interface uses a caller provided locking approach. The caller must have
 * a VA range lock concept that prevents concurrent threads from calling ops on
 * the same VA. Generally the range lock must be at least as large as a single
 * map call.
 */

#[repr(C)]
pub struct pt_iommu {
    pub domain: iommu_domain,
    pub ops: *const pt_iommu_ops,
    pub driver_ops: *const pt_iommu_driver_ops,
    pub nid: core::ffi::c_int,
    pub iommu_device: *mut device,
}

#[inline]
pub unsafe fn iommupt_from_domain(domain: *mut iommu_domain) -> *mut pt_iommu {
    if !IS_ENABLED(CONFIG_IOMMU_PT) || !(*domain).is_iommupt {
        return core::ptr::null_mut();
    }
    container_of!(domain, pt_iommu, domain)
}

#[repr(C)]
pub struct pt_iommu_info {
    pub pgsize_bitmap: u64,
}

#[repr(C)]
pub struct pt_iommu_ops {
    pub map_range: Option<unsafe extern "C" fn(
        iommu_table: *mut pt_iommu,
        iova: dma_addr_t,
        paddr: phys_addr_t,
        len: dma_addr_t,
        prot: core::ffi::c_uint,
        gfp: gfp_t,
        mapped: *mut usize,
    ) -> core::ffi::c_int>,
    pub unmap_range: Option<unsafe extern "C" fn(
        iommu_table: *mut pt_iommu,
        iova: dma_addr_t,
        len: dma_addr_t,
        iotlb_gather: *mut iommu_iotlb_gather,
    ) -> usize>,
    pub set_dirty: Option<unsafe extern "C" fn(
        iommu_table: *mut pt_iommu,
        iova: dma_addr_t,
    ) -> core::ffi::c_int>,
    pub get_info: Option<unsafe extern "C" fn(
        iommu_table: *mut pt_iommu,
        info: *mut pt_iommu_info,
    )>,
    pub deinit: Option<unsafe extern "C" fn(iommu_table: *mut pt_iommu)>,
}

#[repr(C)]
pub struct pt_iommu_driver_ops {
    pub change_top: Option<unsafe extern "C" fn(
        iommu_table: *mut pt_iommu,
        top_paddr: phys_addr_t,
        top_level: core::ffi::c_uint,
    )>,
    pub get_top_lock: Option<unsafe extern "C" fn(
        iommu_table: *mut pt_iommu,
    ) -> *mut spinlock_t>,
}

#[inline]
pub unsafe fn pt_iommu_deinit(iommu_table: *mut pt_iommu) {
    if !(*iommu_table).ops.is_null() {
        if let Some(deinit) = (*(*iommu_table).ops).deinit {
            deinit(iommu_table);
        }
    }
}

#[repr(C)]
pub struct pt_iommu_cfg {
    pub features: core::ffi::c_uint,
    pub hw_max_vasz_lg2: u8,
    pub hw_max_oasz_lg2: u8,
}

// C macro equivalents for the domain operation initializers and domain-layout check.
#[macro_export]
macro_rules! IOMMU_PT_DOMAIN_OPS {
    ($iova_to_phys:path) => { .iova_to_phys = Some($iova_to_phys) };
}
#[macro_export]
macro_rules! IOMMU_PT_DIRTY_OPS {
    ($read_and_clear_dirty:path) => { .read_and_clear_dirty = Some($read_and_clear_dirty) };
}
#[macro_export]
macro_rules! PT_IOMMU_CHECK_DOMAIN {
    ($s:ty, $pt_iommu_memb:ident, $domain_memb:ident) => {
        const _: () = assert!(core::mem::offset_of!($s, $pt_iommu_memb.domain) == core::mem::offset_of!($s, $domain_memb));
    };
}

#[repr(C)]
pub struct pt_iommu_amdv1_cfg {
    pub common: pt_iommu_cfg,
    pub starting_level: core::ffi::c_uint,
}

#[repr(C)]
pub struct pt_iommu_amdv1_hw_info {
    pub host_pt_root: u64,
    pub mode: u8,
}

#[repr(C)]
pub struct pt_iommu_amdv1 {
    pub iommu: pt_iommu,
    pub amdpt: pt_amdpt,
}

extern "C" {
    pub fn pt_iommu_amdv1_iova_to_phys(domain: *mut iommu_domain, iova: dma_addr_t) -> phys_addr_t;
    pub fn pt_iommu_amdv1_read_and_clear_dirty(
        domain: *mut iommu_domain, iova: core::ffi::c_ulong, size: usize,
        flags: core::ffi::c_ulong, dirty: *mut iommu_dirty_bitmap,
    ) -> core::ffi::c_int;
    pub fn pt_iommu_amdv1_init(table: *mut pt_iommu_amdv1, cfg: *const pt_iommu_amdv1_cfg, gfp: gfp_t) -> core::ffi::c_int;
    pub fn pt_iommu_amdv1_hw_info(table: *mut pt_iommu_amdv1, info: *mut pt_iommu_amdv1_hw_info);
}

pub type pt_iommu_amdv1_mock = pt_iommu_amdv1;
pub type pt_iommu_amdv1_mock_cfg = pt_iommu_amdv1_cfg;
pub struct pt_iommu_amdv1_mock_hw_info;
extern "C" {
    pub fn pt_iommu_amdv1_mock_iova_to_phys(domain: *mut iommu_domain, iova: dma_addr_t) -> phys_addr_t;
    pub fn pt_iommu_amdv1_mock_read_and_clear_dirty(domain: *mut iommu_domain, iova: core::ffi::c_ulong, size: usize, flags: core::ffi::c_ulong, dirty: *mut iommu_dirty_bitmap) -> core::ffi::c_int;
    pub fn pt_iommu_amdv1_mock_init(table: *mut pt_iommu_amdv1_mock, cfg: *const pt_iommu_amdv1_mock_cfg, gfp: gfp_t) -> core::ffi::c_int;
    pub fn pt_iommu_amdv1_mock_hw_info(table: *mut pt_iommu_amdv1_mock, info: *mut pt_iommu_amdv1_mock_hw_info);
}

#[repr(C)]
pub struct pt_iommu_vtdss_cfg { pub common: pt_iommu_cfg, pub top_level: core::ffi::c_uint }
#[repr(C)]
pub struct pt_iommu_vtdss_hw_info { pub ssptptr: u64, pub aw: u8 }
#[repr(C)]
pub struct pt_iommu_vtdss { pub iommu: pt_iommu, pub vtdss_pt: pt_vtdss_pt }

#[repr(C)]
pub struct pt_iommu_riscv_64_cfg { pub common: pt_iommu_cfg }
#[repr(C)]
pub struct pt_iommu_riscv_64_hw_info { pub ppn: u64, pub fsc_iosatp_mode: u8 }
#[repr(C)]
pub struct pt_iommu_riscv_64 { pub iommu: pt_iommu, pub riscv_64pt: pt_riscv_64pt }

#[repr(C)]
pub struct pt_iommu_x86_64_cfg { pub common: pt_iommu_cfg, pub top_level: core::ffi::c_uint }
#[repr(C)]
pub struct pt_iommu_x86_64_hw_info { pub gcr3_pt: u64, pub levels: u8 }
#[repr(C)]
pub struct pt_iommu_x86_64 { pub iommu: pt_iommu, pub x86_64_pt: pt_x86_64_pt }

extern "C" {
    pub fn pt_iommu_vtdss_iova_to_phys(domain: *mut iommu_domain, iova: dma_addr_t) -> phys_addr_t;
    pub fn pt_iommu_vtdss_read_and_clear_dirty(domain: *mut iommu_domain, iova: core::ffi::c_ulong, size: usize, flags: core::ffi::c_ulong, dirty: *mut iommu_dirty_bitmap) -> core::ffi::c_int;
    pub fn pt_iommu_vtdss_init(table: *mut pt_iommu_vtdss, cfg: *const pt_iommu_vtdss_cfg, gfp: gfp_t) -> core::ffi::c_int;
    pub fn pt_iommu_vtdss_hw_info(table: *mut pt_iommu_vtdss, info: *mut pt_iommu_vtdss_hw_info);
    pub fn pt_iommu_riscv_64_iova_to_phys(domain: *mut iommu_domain, iova: dma_addr_t) -> phys_addr_t;
    pub fn pt_iommu_riscv_64_read_and_clear_dirty(domain: *mut iommu_domain, iova: core::ffi::c_ulong, size: usize, flags: core::ffi::c_ulong, dirty: *mut iommu_dirty_bitmap) -> core::ffi::c_int;
    pub fn pt_iommu_riscv_64_init(table: *mut pt_iommu_riscv_64, cfg: *const pt_iommu_riscv_64_cfg, gfp: gfp_t) -> core::ffi::c_int;
    pub fn pt_iommu_riscv_64_hw_info(table: *mut pt_iommu_riscv_64, info: *mut pt_iommu_riscv_64_hw_info);
    pub fn pt_iommu_x86_64_iova_to_phys(domain: *mut iommu_domain, iova: dma_addr_t) -> phys_addr_t;
    pub fn pt_iommu_x86_64_read_and_clear_dirty(domain: *mut iommu_domain, iova: core::ffi::c_ulong, size: usize, flags: core::ffi::c_ulong, dirty: *mut iommu_dirty_bitmap) -> core::ffi::c_int;
    pub fn pt_iommu_x86_64_init(table: *mut pt_iommu_x86_64, cfg: *const pt_iommu_x86_64_cfg, gfp: gfp_t) -> core::ffi::c_int;
    pub fn pt_iommu_x86_64_hw_info(table: *mut pt_iommu_x86_64, info: *mut pt_iommu_x86_64_hw_info);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
