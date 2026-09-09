/* SPDX-License-Identifier: GPL-2.0 */
/* iommu.h: Definitions for the sun5 IOMMU.
 *
 * Copyright (C) 1996, 1999, 2007 David S. Miller (davem@davemloft.net)
 */

/* The format of an iopte in the page tables. */
pub const IOPTE_VALID: u64 = 0x8000_0000_0000_0000;
pub const IOPTE_64K: u64 = 0x2000_0000_0000_0000;
pub const IOPTE_STBUF: u64 = 0x1000_0000_0000_0000;
pub const IOPTE_INTRA: u64 = 0x0800_0000_0000_0000;
pub const IOPTE_CONTEXT: u64 = 0x07ff_8000_0000_0000;
pub const IOPTE_PAGE: u64 = 0x0000_7fff_fffe_0000;
pub const IOPTE_CACHE: u64 = 0x0000_0000_0000_0010;
pub const IOPTE_WRITE: u64 = 0x0000_0000_0000_0002;

pub const IOMMU_NUM_CTXS: usize = 4096;

#[repr(C)]
pub struct iommu_arena {
    pub map: *mut ::core::ffi::c_ulong,
    pub hint: ::core::ffi::c_uint,
    pub limit: ::core::ffi::c_uint,
}

pub const ATU_64_SPACE_SIZE: u64 = 0x8000_0000_0; /* 32G */

/* Data structures for SPARC ATU architecture */
#[repr(C)]
pub struct atu_iotsb {
    pub table: *mut ::core::ffi::c_void, /* IOTSB table base virtual addr*/
    pub ra: u64,                         /* IOTSB table real addr */
    pub dvma_size: u64,                  /* ranges[3].size or OS slected 32G size */
    pub dvma_base: u64,                  /* ranges[3].base */
    pub table_size: u64,                 /* IOTSB table size */
    pub page_size: u64,                  /* IO PAGE size for IOTSB */
    pub iotsb_num: u32,                  /* tsbnum is same as iotsb_handle */
}

#[repr(C)]
pub struct atu_ranges {
    pub base: u64,
    pub size: u64,
}

#[repr(C)]
pub struct atu {
    pub ranges: *mut atu_ranges,
    pub iotsb: *mut atu_iotsb,
    pub tbl: iommu_map_table,
    pub base: u64,
    pub size: u64,
    pub dma_addr_mask: u64,
}

#[repr(C)]
pub struct iommu {
    pub tbl: iommu_map_table,
    pub atu: *mut atu,
    pub lock: spinlock_t,
    pub dma_addr_mask: u32,
    pub page_table: *mut iopte_t,
    pub iommu_control: ::core::ffi::c_ulong,
    pub iommu_tsbbase: ::core::ffi::c_ulong,
    pub iommu_flush: ::core::ffi::c_ulong,
    pub iommu_flushinv: ::core::ffi::c_ulong,
    pub iommu_tags: ::core::ffi::c_ulong,
    pub iommu_ctxflush: ::core::ffi::c_ulong,
    pub write_complete_reg: ::core::ffi::c_ulong,
    pub dummy_page: ::core::ffi::c_ulong,
    pub dummy_page_pa: ::core::ffi::c_ulong,
    pub ctx_lowest_free: ::core::ffi::c_ulong,
    pub ctx_bitmap: [::core::ffi::c_ulong; IOMMU_NUM_CTXS / (core::mem::size_of::<::core::ffi::c_ulong>() * 8)],
}

#[repr(C)]
pub struct strbuf {
    pub strbuf_enabled: ::core::ffi::c_int,
    pub strbuf_control: ::core::ffi::c_ulong,
    pub strbuf_pflush: ::core::ffi::c_ulong,
    pub strbuf_fsync: ::core::ffi::c_ulong,
    pub strbuf_err_stat: ::core::ffi::c_ulong,
    pub strbuf_tag_diag: ::core::ffi::c_ulong,
    pub strbuf_line_diag: ::core::ffi::c_ulong,
    pub strbuf_ctxflush: ::core::ffi::c_ulong,
    pub strbuf_ctxmatch_base: ::core::ffi::c_ulong,
    pub strbuf_flushflag_pa: ::core::ffi::c_ulong,
    pub strbuf_flushflag: *mut ::core::ffi::c_ulong,
    pub __flushflag_buf: [::core::ffi::c_ulong; (64 + (64 - 1)) / core::mem::size_of::<::core::ffi::c_ulong>()],
}

unsafe extern "C" {
    pub fn iommu_table_init(
        iommu: *mut iommu,
        tsbsize: ::core::ffi::c_int,
        dma_offset: u32,
        dma_addr_mask: u32,
        numa_node: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
